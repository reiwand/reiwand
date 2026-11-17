use crate::daemon::Daemon;
use crate::error::{Error, Result, failed, invalid};
use crate::jobs::Ctl;
use crate::kinds::{local, parse_url};
use crate::repos::pins::origin;
use reiwand_core::dbus::error::{CERTIFICATE_CHANGED, CERTIFICATE_UNKNOWN, NOT_A_REPOSITORY};
use reiwand_format::Fetch;
use serde_json::Value;

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
    if !unreachable(&e).is_some_and(|io| reiwand_account::untrusted(io)) {
        return e.into();
    }
    let site = bases.iter().find_map(|base| origin(base)).unwrap_or_default();
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
        if matches!(e, reiwand_format::Error::Missing(ref what) if what == "root/1.json")
            && let Some(found) = self.server_page(&site, ctl)
        {
            return Error::Named(NOT_A_REPOSITORY, found);
        }
        e.into()
    }

    fn server_page(&self, site: &str, ctl: &Ctl) -> Option<String> {
        let fetch = crate::repos::fetcher(self, ctl, &[format!("{site}/")]).ok()?;
        let body = fetch.get(&format!("{site}/"), ".well-known/reiwand-server", 256 << 10).ok()??;
        let found: Value = serde_json::from_slice(&body).ok()?;
        let listed: Vec<String> = found["repositories"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|r| Some(format!("{} ({})", r["name"].as_str()?, r["url"].as_str()?)))
            .take(5)
            .collect();
        let named = found["name"].as_str().unwrap_or(site);
        Some(match listed.is_empty() {
            false => format!(
                "this is a server, not a repository: paste the address of one of its repositories: {}",
                listed.join(", ")
            ),
            true => format!("this is a server, not a repository: {named} has no public repository"),
        })
    }
}
