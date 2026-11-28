use crate::daemon::Daemon;
use crate::error::{Error, Result, failed, invalid};
use crate::jobs::Ctl;
use crate::kinds::{local, parse_url};
use crate::repos::pins::origin;
use reiwand_core::dbus::error::{CERTIFICATE_CHANGED, CERTIFICATE_UNKNOWN, OFFLINE};
use reiwand_format::Fetch;
use serde_json::{Value, json};

pub(super) fn checked(url: &str, trust: &str) -> Result<()> {
    let parsed = parse_url(url, true)?;
    if !(matches!(parsed.scheme(), "https" | "file") || parsed.scheme() == "http" && local(&parsed)) {
        return Err(invalid("use an https:// or file:// URL, or http:// to this computer"));
    }
    if !trust.is_empty() && !reiwand_account::valid_pin(&trust.to_ascii_lowercase()) {
        return Err(invalid("a certificate fingerprint has 64 hex digits"));
    }
    Ok(())
}

fn unreachable(e: &reiwand_format::Error) -> Option<&std::io::Error> {
    match e {
        reiwand_format::Error::Io(io) => Some(io),
        _ => None,
    }
}

/// A repository that cannot be reached says which host and what the network answered, not only the
/// HTTP library's "error sending request"; `after` says what was kept.
fn offline(site: &str, e: &reiwand_format::Error, after: &str) -> Option<Error> {
    let io = unreachable(e).filter(|io| super::named::unreachable(io.kind()))?;
    let mut cause: &(dyn std::error::Error + 'static) = io.get_ref().map_or(io as _, |inner| inner as _);
    while let Some(deeper) = cause.source() {
        cause = deeper;
    }
    let host = site.split("://").nth(1).unwrap_or(site);
    Some(Error::Named(
        OFFLINE,
        format!("cannot reach {host} ({cause}): there is no network or the server does not answer; {after}"),
    ))
}

fn denied(e: &reiwand_format::Error) -> bool {
    unreachable(e).is_some_and(|io| io.kind() == std::io::ErrorKind::PermissionDenied)
}

impl Daemon {
    pub(super) fn explain_refresh(&self, bases: &[String], e: reiwand_format::Error) -> Error {
        if denied(&e) {
            return self.account_needed(&bases.iter().find_map(|base| origin(base)).unwrap_or_default());
        }
        explain_changed(bases, e)
    }
}

fn explain_changed(bases: &[String], e: reiwand_format::Error) -> Error {
    let site = bases.iter().find_map(|base| origin(base)).unwrap_or_default();
    if !unreachable(&e).is_some_and(|io| reiwand_account::untrusted(io)) {
        return offline(&site, &e, "the repository keeps what it had").unwrap_or_else(|| e.into());
    }
    let host = site.split("://").nth(1).unwrap_or(&site);
    Error::Named(CERTIFICATE_CHANGED, format!("the certificate of {host} changed; nothing was applied"))
}

impl Daemon {
    pub(super) fn explain_add(&self, url: &str, trust: &str, e: reiwand_format::Error, ctl: &Ctl) -> Error {
        let Some(site) = origin(url) else { return e.into() };
        if denied(&e) {
            return self.account_needed(&site);
        }
        if unreachable(&e).is_some_and(|io| reiwand_account::untrusted(io)) {
            let host = site.split("://").nth(1).unwrap_or(&site);
            if !trust.is_empty() || self.repo_pins().contains_key(&site) {
                let text = format!("the certificate of {host} is not the one you trusted; nothing was added");
                return Error::Named(CERTIFICATE_CHANGED, text);
            }
            return match crate::accounts::block_on(reiwand_account::peek(&site)) {
                Ok(print) => {
                    let text = format!(
                        "the certificate of {host} is not from a known authority; its fingerprint is {print}; \
                         add the repository again confirming that fingerprint to trust it"
                    );
                    Error::Named(CERTIFICATE_UNKNOWN, text)
                }
                Err(e) => failed(e.to_string()),
            };
        }
        if let Some(gone) = offline(&site, &e, "nothing was added") {
            return gone;
        }
        if matches!(e, reiwand_format::Error::Missing(ref what) if what == "root/1.json")
            && let Some((text, listed)) = self.server_page(&site, ctl)
        {
            return Error::Listing(text, json!({ "repositories": listed }));
        }
        e.into()
    }

    fn server_page(&self, site: &str, ctl: &Ctl) -> Option<(String, Vec<Value>)> {
        let fetch = crate::repos::fetcher(self, ctl, &[format!("{site}/")]).ok()?;
        let body = fetch.get(&format!("{site}/"), ".well-known/reiwand-server", 256 << 10).ok()??;
        let found: Value = serde_json::from_slice(&body).ok()?;
        let listed: Vec<Value> = found["repositories"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|r| Some(json!({ "name": r["name"].as_str()?, "url": r["url"].as_str()? })))
            .take(5)
            .collect();
        let named = found["name"].as_str().unwrap_or(site);
        let text = match listed.is_empty() {
            false => {
                let text = |r: &Value, key: &str| r[key].as_str().unwrap_or_default().to_owned();
                let names: Vec<String> =
                    listed.iter().map(|r| format!("{} ({})", text(r, "name"), text(r, "url"))).collect();
                format!(
                    "this is a server, not a repository: paste the address of one of its repositories: {}",
                    names.join(", ")
                )
            }
            true => format!("this is a server, not a repository: {named} has no public repository"),
        };
        Some((text, listed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn an_unreachable_repository_names_its_host_and_what_the_network_answered() {
        let reset = io::Error::new(io::ErrorKind::ConnectionReset, "connection reset by peer");
        let sending = io::Error::new(io::ErrorKind::NotConnected, io::Error::other(reset));
        let e = reiwand_format::Error::Io(sending);
        let said = offline("https://repo.example:8443", &e, "nothing was added").map(|e| e.to_string());
        let said = said.unwrap_or_default();
        assert!(said.contains("cannot reach repo.example:8443 (connection reset by peer)"), "{said}");
        assert!(said.contains("no network") && said.ends_with("nothing was added"), "{said}");
        let other = reiwand_format::Error::Io(io::Error::other("HTTP status 500"));
        assert!(offline("https://repo.example", &other, "").is_none());
    }
}
