use crate::guard::Public;
use crate::net::{AGENT, HOPS, keyed, within};
use crate::{Error, Manifest, Secret};
use reqwest::header::HeaderValue;
use reqwest::redirect::Policy;
use reqwest::{Client, ClientBuilder, RequestBuilder, Url};
use std::sync::Arc;
use std::time::Duration;

pub struct Files {
    client: Client,
    hosts: Vec<String>,
    secret: Option<(Secret, String)>,
}

impl Files {
    pub fn new(manifest: &Manifest, secret: Option<&str>) -> Result<Files, Error> {
        Files::with(manifest, secret, |builder| builder)
    }

    pub fn with(
        manifest: &Manifest,
        secret: Option<&str>,
        setup: impl FnOnce(ClientBuilder) -> ClientBuilder,
    ) -> Result<Files, Error> {
        let secret = keyed(manifest, secret)?;
        let hosts = manifest.hosts.clone();
        let keyed_hosts = secret.as_ref().map(|(declared, _)| declared.hosts.clone());
        let declared = hosts.clone();
        let policy = Policy::custom(move |attempt| {
            let first = attempt.previous().first();
            let carries = keyed_hosts.as_ref().filter(|k| first.is_some_and(|f| within(k, f)));
            match attempt.previous().len() {
                hops if hops > HOPS => attempt.error("too many redirects"),
                _ if within(carries.unwrap_or(&declared), attempt.url()) => attempt.follow(),
                _ => attempt.stop(),
            }
        });
        let builder = setup(reiwand_core::net::client());
        #[cfg(target_os = "android")]
        let builder = builder.tls_certs_only(reiwand_core::roots::certificates());
        let builder = builder.no_proxy().https_only(true).redirect(policy).referer(false).user_agent(AGENT);
        let builder = builder.dns_resolver(Arc::new(Public)).connect_timeout(Duration::from_secs(10));
        let client = builder.read_timeout(Duration::from_secs(30)).build();
        Ok(Files { client: client.map_err(|e| Error::Host(e.to_string()))?, hosts, secret })
    }

    pub fn get(&self, url: &str) -> Result<RequestBuilder, Error> {
        let url = Url::parse(url).ok().filter(|u| within(&self.hosts, u)).ok_or_else(|| {
            Error::Refused(format!(
                "a file address leaves the hosts the plugin declares ({}), or is not https",
                self.hosts.join(", ")
            ))
        })?;
        let request = self.client.get(url.clone());
        match self.secret.as_ref().filter(|(declared, _)| within(&declared.hosts, &url)) {
            Some((declared, value)) => {
                let mut value = HeaderValue::from_str(&format!("{}{value}", declared.prefix))
                    .map_err(|_| Error::Refused("the secret is not a valid header value".into()))?;
                value.set_sensitive(true);
                Ok(request.header(&declared.header, value))
            }
            None => Ok(request),
        }
    }
}
