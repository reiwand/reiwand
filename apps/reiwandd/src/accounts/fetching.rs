use crate::daemon::Daemon;
use crate::error::{Error, Result, failed};
use crate::jobs::now;
use crate::repos::pins::origin;
use reiwand_core::dbus::error::ACCOUNT_NEEDED;
use reiwand_providers::HttpFetch;

impl Daemon {
    pub(crate) fn with_account_tokens(&self, mut fetch: HttpFetch) -> Result<HttpFetch> {
        let recs: Vec<_> =
            self.account_list().into_iter().filter(|r| !r.pending && !r.links.is_empty()).collect();
        if recs.is_empty() {
            return Ok(fetch);
        }
        let known = reiwand_format::repos(&self.store.data, now()).unwrap_or_default();
        for rec in recs {
            let Some(token) = self.account_token(&rec.id) else { continue };
            for link in &rec.links {
                let mut bases = vec![link.url.clone()];
                let repo = link.repo_id.as_ref().and_then(|id| known.iter().find(|r| &r.repo_id == id));
                bases.extend(repo.into_iter().flat_map(|r| r.hosts.iter().cloned().chain([r.url.clone()])));
                for base in bases.iter().filter(|b| origin(b).as_deref() == Some(rec.origin.as_str())) {
                    fetch = fetch.account(base, &token).map_err(|e| failed(e.to_string()))?;
                }
            }
        }
        Ok(fetch)
    }

    pub(crate) fn account_needed(&self, site: &str) -> Error {
        let users: Vec<String> =
            self.account_list().into_iter().filter(|r| r.origin == site).map(|r| r.user).collect();
        Error::Named(ACCOUNT_NEEDED, needed_text(site, &users))
    }
}

pub(super) fn needed_text(site: &str, users: &[String]) -> String {
    let host = site.split("://").nth(1).unwrap_or(site);
    match users.is_empty() {
        true => format!("this repository is private: sign in with an account of {host} to open it"),
        false => format!(
            "this repository is private: use one of your accounts on {host} ({}) or sign in with another one",
            users.join(", ")
        ),
    }
}
