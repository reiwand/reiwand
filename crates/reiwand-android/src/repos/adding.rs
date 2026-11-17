use crate::account::{pins, using};
use crate::library::now;
use crate::{Error, net, said, store};
use reiwand_account::origin;
use serde_json::{Value, json};
use std::io::ErrorKind;

fn io_of(e: &reiwand_format::Error) -> Option<&std::io::Error> {
    match e {
        reiwand_format::Error::Io(io) => Some(io),
        _ => None,
    }
}

pub(super) fn add(url: &str, trust: Option<&str>) -> Result<Value, Error> {
    super::server::https(url)?;
    net::reach(url)?;
    let site = origin(url);
    let trust =
        trust.map(|t| t.chars().filter(char::is_ascii_hexdigit).collect::<String>().to_ascii_lowercase());
    if trust.as_deref().is_some_and(|t| !reiwand_account::valid_pin(t)) {
        return Err(said("android.fingerprint_digits", "A certificate fingerprint has 64 hex digits", &[]));
    }
    let extra = site.as_deref().zip(trust.as_deref());
    std::fs::create_dir_all(super::dir()?)?;
    let fetch = net::fetch_with(extra)?;
    match reiwand_format::add_repo(&super::dir()?, &fetch, url, now()) {
        Ok(info) => {
            if let Some((site, print)) = extra {
                pins::pin(site, print)?;
            }
            store::changed();
            Ok(json!({"repo_id": info.repo_id, "name": info.name, "packages": info.packages}))
        }
        Err(e) => Err(explain(url, trust.is_some(), e)),
    }
}

fn explain(url: &str, trusted: bool, e: reiwand_format::Error) -> Error {
    let Some(site) = origin(url) else { return e.into() };
    let host = site.split("://").nth(1).unwrap_or(&site).to_owned();
    if io_of(&e).is_some_and(|io| io.kind() == ErrorKind::PermissionDenied) {
        return using::needed(&site);
    }
    if io_of(&e).is_some_and(|io| reiwand_account::untrusted(io)) {
        if trusted || pins::all().contains_key(&site) {
            return said(
                "android.cert_changed",
                "The certificate of %1 is not the one you trusted; nothing was added",
                &[&host],
            );
        }
        let seen = tokio::runtime::Builder::new_current_thread().enable_all().build();
        let print = seen.map_err(Error::from).and_then(|rt| rt.block_on(net::peek(&site)));
        return match print {
            Ok(print) => said(
                "android.cert_unknown",
                "The certificate of %1 is not from a known authority; its fingerprint is %2",
                &[&host, &print],
            ),
            Err(e) => e,
        };
    }
    if matches!(&e, reiwand_format::Error::Missing(what) if what == "root/1.json") {
        return said(
            "android.repo_is_server",
            "This is a server, not a repository: pick one of its repositories",
            &[&host, &site],
        );
    }
    e.into()
}
