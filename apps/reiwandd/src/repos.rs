use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::{Error, Result, invalid, not_found};
use crate::items::reply;
use crate::jobs::{Ctl, now};
use reiwand_core::dbus::error::{CERTIFICATE_CHANGED, SIGNATURE_CHANGED};
use reiwand_core::model::{JobKind, MODEL, RefreshOutput, Repo, RepoWarning, ReposReply, Update};
use reiwand_format::{Anchor, RepoInfo};
use serde_json::Value;
use settle::{Settled, settle, vanished};
use std::sync::Arc;

mod adding;
mod named;
pub(crate) mod pins;
use pins::fetcher_trusting;
pub use pins::{fetcher, pinned};
mod settle;

pub fn newer(a: &str, b: &str) -> bool {
    semver::Version::parse(a).ok() > semver::Version::parse(b).ok()
}

fn warning(text: &str) -> RepoWarning {
    match (text.starts_with("root"), text.contains("within")) {
        (true, true) => RepoWarning::RootExpiring,
        (true, false) => RepoWarning::RootExpired,
        (false, true) => RepoWarning::EntryExpiring,
        (false, false) => RepoWarning::Stale,
    }
}

impl Daemon {
    fn repo_model(&self, info: RepoInfo) -> Repo {
        let host =
            |base: &String| reqwest::Url::parse(base).ok().and_then(|u| u.host_str().map(String::from));
        let hosts = info.hosts.iter().map(|base| host(base).unwrap_or_else(|| base.clone())).collect();
        let (last_error_name, last_error) =
            lock(&self.remote.repo_errors).get(&info.repo_id).cloned().unzip();
        let changed = match last_error_name.as_deref() {
            Some(SIGNATURE_CHANGED) => Some(RepoWarning::SignatureChanged),
            Some(CERTIFICATE_CHANGED) => Some(RepoWarning::CertificateChanged),
            _ => None,
        };
        let added = info.url.split('?').next().unwrap_or_default();
        let jobs = [
            (JobKind::RepoRefresh, info.repo_id.as_str()),
            (JobKind::RepoRefresh, ""),
            (JobKind::RepoAdd, added),
        ];
        let job = jobs.iter().flat_map(|(kind, target)| self.jobs.find(*kind, target)).min();
        Repo {
            hosts,
            name: info.name,
            url: info.url,
            fingerprints: info.fingerprints,
            root_version: info.root_version,
            root_expires: info.root_expires.timestamp(),
            entry_version: info.entry_version,
            entry_expires: info.entry_expires.timestamp(),
            checked: info.checked.timestamp(),
            stale: info.stale,
            packages: info.packages as u32,
            warnings: info.warnings.iter().map(|w| warning(w)).chain(changed).collect(),
            last_error,
            last_error_name,
            job,
            broken: info.broken,
            bundled: self.is_examples(&info.repo_id),
            repo_id: info.repo_id,
        }
    }

    pub fn list_repos(&self) -> Result<String> {
        let serial = self.changes.serial();
        let repos =
            reiwand_format::repos(&self.store.data, now())?.into_iter().map(|i| self.repo_model(i)).collect();
        reply(ReposReply { model: MODEL, serial, repos })
    }

    pub fn bases(&self, repo_id: &str) -> Vec<String> {
        let repos = reiwand_format::repos(&self.store.data, now()).unwrap_or_default();
        repos.into_iter().find(|r| r.repo_id == repo_id).map(|r| r.hosts).unwrap_or_default()
    }

    pub fn repo_id(&self, repo: &str) -> Result<String> {
        let repo = repo.trim().to_ascii_lowercase();
        if !(8..=64).contains(&repo.len()) || !repo.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(invalid("give a repository id or at least its first 8 hex digits"));
        }
        let ids = reiwand_format::repos(&self.store.data, now())?.into_iter().map(|r| r.repo_id);
        match ids.filter(|id| id.starts_with(&repo)).collect::<Vec<_>>().as_slice() {
            [one] => Ok(one.clone()),
            [] => Err(not_found(format!("repository {repo} not found"))),
            _ => Err(invalid(format!("{repo} matches several repositories; give more digits"))),
        }
    }

    fn gone(&self, id: &str) -> bool {
        vanished(&lock(&self.remote.removing), &self.store.data, id)
    }

    pub fn add_repo(self: &Arc<Self>, url: &str, trust: &str) -> Result<u64> {
        adding::checked(url, trust)?;
        Anchor::parse(url)?;
        let (url, trust) = (url.to_owned(), trust.to_ascii_lowercase());
        let target = url.clone();
        Ok(self.start_job(JobKind::RepoAdd, &target, move |d, ctl| {
            let site = pins::origin(&url);
            let extra = site.as_deref().filter(|_| !trust.is_empty()).map(|s| (s, trust.as_str()));
            let fetch = fetcher_trusting(d, ctl, &[], extra)?;
            let info = reiwand_format::add_repo(&d.store.data, &fetch, &url, now())
                .map_err(|e| d.explain_add(&url, &trust, e, ctl))?;
            if let (Some(site), false) = (&site, trust.is_empty()) {
                d.pin_origin(site, &trust)?;
            }
            lock(&d.remote.repo_errors).remove(&info.repo_id);
            tracing::info!(repo = &info.repo_id[..8], "repository added");
            d.changes.changed("repos", vec![info.repo_id.clone()]);
            Ok(serde_json::to_value(d.repo_model(info)).unwrap_or_default())
        }))
    }

    pub fn refresh_repos(self: &Arc<Self>, repo: &str) -> Result<u64> {
        let target = if repo.is_empty() { String::new() } else { self.repo_id(repo)? };
        Ok(self.refresh_job(&target))
    }

    pub fn refresh_job(self: &Arc<Self>, target: &str) -> u64 {
        let owned = target.to_owned();
        self.start_job(JobKind::RepoRefresh, target, move |d, ctl| d.refresh(&owned, ctl))
    }

    fn refresh(self: &Arc<Self>, target: &str, ctl: &Ctl) -> Result<Value> {
        let known = reiwand_format::repos(&self.store.data, now())?;
        let ids: Vec<String> = match target {
            "" => known.iter().map(|r| r.repo_id.clone()).collect(),
            id => vec![id.to_owned()],
        };
        let mut out = RefreshOutput { repos: Vec::new(), updates: Vec::new(), revoked: Vec::new() };
        let mut failure = None;
        for id in ids {
            let bases = known.iter().find(|r| r.repo_id == id).map_or(&[][..], |r| &r.hosts);
            let updated = ctl.check().and_then(|()| {
                let fetch = fetcher(self, ctl, bases)?;
                reiwand_format::update_repo(&self.store.data, &fetch, &id, now())
                    .map_err(|e| adding::explain_refresh(bases, e))
            });
            match settle(updated, || self.gone(&id)) {
                Settled::Done(outcome) => {
                    lock(&self.remote.repo_errors).remove(&id);
                    let latest = self.latest().unwrap_or_default();
                    let update = |u: reiwand_format::Update| Update {
                        changes: latest
                            .get(&(id.clone(), u.id.clone()))
                            .map(|l| l.0.clone())
                            .unwrap_or_default(),
                        repo_id: id.clone(),
                        id: u.id,
                        installed: u.installed,
                        available: u.available,
                    };
                    out.updates.extend(outcome.updates.into_iter().map(update));
                    out.revoked.extend(outcome.revoked.iter().map(|i| self.package_model(i, None)));
                    out.repos.push(self.repo_model(outcome.repo));
                }
                Settled::Stopped => {
                    failure = Some(Error::Cancelled);
                    break;
                }
                Settled::Gone => {}
                Settled::Failed(e) => {
                    tracing::warn!(repo = &id[..8], "repository refresh failed");
                    lock(&self.remote.repo_errors).insert(id.clone(), e.parts());
                    failure = failure.or(Some(e)).filter(|_| !target.is_empty());
                }
            }
        }
        self.reload_installed();
        self.update_installed();
        failure.map_or_else(|| Ok(serde_json::to_value(out).unwrap_or_default()), Err)
    }
}
