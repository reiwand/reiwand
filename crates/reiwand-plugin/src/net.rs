use crate::guard::{Private, Public};
use crate::{Manifest, Secret};
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{HeaderValue, LOCATION};
use reqwest::redirect::Policy;
use reqwest::{Method, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Read;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub(crate) const AGENT: &str =
    concat!("reiwand/", env!("CARGO_PKG_VERSION"), " (+https://github.com/reiwand/reiwand)");
pub(crate) const HOPS: usize = 5;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Answer {
    pub status: u16,
    pub url: String,
    pub headers: BTreeMap<String, String>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Fault {
    Refused(String),
    Limit(String),
    Failed(String),
}

pub struct Net {
    hosts: Vec<String>,
    secret: Option<(Secret, String)>,
    forbidden: Manifest,
    client: Client,
}

pub fn declared(manifest: &Manifest, url: &str) -> bool {
    Url::parse(url).is_ok_and(|url| within(&manifest.hosts, &url))
}

pub(crate) fn within(hosts: &[String], url: &Url) -> bool {
    let plain = url.username().is_empty() && url.password().is_none() && url.port().is_none();
    url.scheme() == "https" && plain && url.host_str().is_some_and(|h| hosts.iter().any(|d| d == h))
}

pub(crate) fn keyed(
    manifest: &Manifest,
    secret: Option<&str>,
) -> Result<Option<(Secret, String)>, crate::Error> {
    match (&manifest.secret, secret) {
        (Some(declared), Some(value)) => Ok(Some((declared.clone(), value.to_owned()))),
        (Some(declared), None) if declared.required => {
            Err(crate::Error::Refused(format!("the plugin needs its secret: {}", declared.label)))
        }
        (None, Some(_)) => Err(crate::Error::Refused("the plugin declares no secret".into())),
        _ => Ok(None),
    }
}

impl Net {
    pub fn new(manifest: &Manifest, secret: Option<&str>) -> Result<Net, crate::Error> {
        Net::with(manifest, secret, |builder| builder)
    }

    pub fn with(
        manifest: &Manifest,
        secret: Option<&str>,
        setup: impl FnOnce(ClientBuilder) -> ClientBuilder,
    ) -> Result<Net, crate::Error> {
        let secret = keyed(manifest, secret)?;
        let builder = setup(reiwand_core::net::blocking());
        #[cfg(target_os = "android")]
        let builder = builder.tls_certs_only(reiwand_core::roots::certificates());
        let builder =
            builder.no_proxy().https_only(true).redirect(Policy::none()).referer(false).user_agent(AGENT);
        let builder = builder.dns_resolver(Arc::new(Public)).connect_timeout(Duration::from_secs(10));
        let client = builder.build().map_err(|e| crate::Error::Host(e.to_string()))?;
        Ok(Net { hosts: manifest.hosts.clone(), secret, forbidden: manifest.clone(), client })
    }

    pub fn fetch(&self, request: &Request, until: Instant, max: usize) -> Result<Answer, Fault> {
        let mut method = match request.method.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            other => {
                return Err(Fault::Refused(format!("the method {other:?} is not allowed: use GET or POST")));
            }
        };
        if let Some((name, _)) = request.headers.iter().find(|(name, _)| self.forbidden.header(name)) {
            return Err(Fault::Refused(format!("the plugin may not set the header {name}")));
        }
        let (mut url, mut body) = (Url::parse(&request.url).ok(), request.body.clone());
        for hop in 0..=HOPS {
            let Some(at) = url.take().filter(|u| within(&self.hosts, u)) else {
                let what = if hop == 0 { "the address" } else { "a redirect" };
                return Err(Fault::Refused(format!(
                    "{what} leaves the hosts the plugin declares ({}), or is not https",
                    self.hosts.join(", ")
                )));
            };
            let left = until.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return Err(Fault::Limit("the plugin ran out of time".into()));
            }
            let mut asked = self.client.request(method.clone(), at.clone()).timeout(left);
            for (name, value) in &request.headers {
                asked = asked.header(name, value);
            }
            if let Some((declared, value)) = self.secret.as_ref().filter(|(d, _)| within(&d.hosts, &at)) {
                let mut value = HeaderValue::from_str(&format!("{}{value}", declared.prefix))
                    .map_err(|_| Fault::Refused("the secret is not a valid header value".into()))?;
                value.set_sensitive(true);
                asked = asked.header(&declared.header, value);
            }
            if let Some(body) = &body {
                asked = asked.body(body.clone());
            }
            let response = asked.send().map_err(failed)?;
            let status = response.status();
            if status.is_redirection() && status != StatusCode::NOT_MODIFIED {
                let next = response.headers().get(LOCATION).and_then(|l| l.to_str().ok());
                url = next.and_then(|next| at.join(next).ok());
                if status == StatusCode::SEE_OTHER || method == Method::POST && status.as_u16() <= 302 {
                    (method, body) = (Method::GET, None);
                }
                continue;
            }
            return answer(response, max);
        }
        Err(Fault::Failed(format!("more than {HOPS} redirects")))
    }
}

fn answer(response: reqwest::blocking::Response, max: usize) -> Result<Answer, Fault> {
    let (status, url) = (response.status().as_u16(), response.url().to_string());
    let mut headers = BTreeMap::<String, String>::new();
    for (name, value) in response.headers() {
        let value = String::from_utf8_lossy(value.as_bytes()).into_owned();
        headers.entry(name.as_str().to_owned()).and_modify(|v| *v = format!("{v}, {value}")).or_insert(value);
    }
    let mut bytes = Vec::new();
    response.take(max as u64 + 1).read_to_end(&mut bytes).map_err(|e| Fault::Failed(e.to_string()))?;
    if bytes.len() > max {
        return Err(Fault::Limit(format!("a response is larger than {} KiB", max >> 10)));
    }
    Ok(Answer { status, url, headers, body: String::from_utf8_lossy(&bytes).into_owned() })
}

fn failed(error: reqwest::Error) -> Fault {
    let mut chain = std::iter::successors(Some(&error as &(dyn std::error::Error + 'static)), |e| e.source());
    if let Some(private) = chain.find_map(|e| e.downcast_ref::<Private>()) {
        return Fault::Refused(private.to_string());
    }
    if reiwand_core::net::refused(&error) {
        return Fault::Refused(reiwand_core::net::REFUSED.into());
    }
    match error.is_timeout() {
        true => Fault::Limit("the plugin ran out of time".into()),
        false => Fault::Failed(chain_text(&error.without_url())),
    }
}

fn chain_text(error: &(dyn std::error::Error + 'static)) -> String {
    let chain = std::iter::successors(Some(error), |e| e.source()).map(ToString::to_string);
    chain.collect::<Vec<_>>().join(": ")
}
