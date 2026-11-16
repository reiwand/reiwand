use crate::config::Config;
use crate::db::now;
use anyhow::{Context, ensure};
use reiwand_format::{
    Added, Entry, Keypair, Published, Reason, RootStatus, Settings, Signer, fingerprint, keygen, settings,
};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::fs::{self, DirBuilder, OpenOptions};
use std::io::{ErrorKind, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};

mod listed;
mod manage;

pub use listed::Shown;
pub use manage::{import, import_repo, init, init_repo, status, status_repo, statuses, valid_name};

pub struct Repo {
    pub name: String,
    pub dir: PathBuf,
    folder: String,
    key_name: String,
    key: PathBuf,
    lock: Mutex<()>,
}

impl Repo {
    pub fn at(data: &Path, name: &str, folder: &str, key_name: &str) -> Repo {
        let (dir, key) = (data.join(folder), data.join(key_name));
        Repo {
            name: name.into(),
            folder: folder.into(),
            key_name: key_name.into(),
            dir,
            key,
            lock: Mutex::new(()),
        }
    }

    pub fn folder(&self) -> &str {
        &self.folder
    }

    pub fn key_name(&self) -> &str {
        &self.key_name
    }

    pub fn main(data: &Path) -> Repo {
        Repo::at(data, "main", "repo", "keys/online.key")
    }

    pub fn ready(&self) -> bool {
        self.dir.join("root/1.json").is_file() && self.key.is_file()
    }

    pub fn add(&self, wpak: &Path, now: i64, shown: listed::Shown) -> reiwand_format::Result<Added> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        let added = reiwand_format::add(&self.dir, wpak, None, now / 1000)?;
        shown.mark(&self.dir, &added.id, &added.release.version)?;
        Ok(added)
    }

    pub fn revoke(&self, id: &str, version: &str, reason: Reason, note: &str) -> reiwand_format::Result<()> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        reiwand_format::revoke(&self.dir, id, Some(version), reason, note)?;
        let catalog: Value = serde_json::from_slice(&fs::read(self.dir.join("catalog.json"))?)?;
        let list = |v: &Value, key: &str| v[key].as_array().cloned().unwrap_or_default();
        let revoked = list(&catalog, "revoked");
        let withdrawn = |id: &Value, v: &Value| {
            revoked.iter().any(|r| r["id"] == *id && list(r, "versions").iter().any(|x| x == v || x == "*"))
        };
        let shown: HashSet<String> = list(&catalog, "packages")
            .iter()
            .flat_map(|l| list(l, "versions").into_iter().filter(|r| !withdrawn(&l["id"], &r["version"])))
            .filter_map(|r| {
                Some(format!("{}.{}", r["preview"]["sha256"].as_str()?, r["preview"]["ext"].as_str()?))
            })
            .collect();
        for item in fs::read_dir(self.dir.join("previews")).into_iter().flatten().flatten() {
            if !shown.contains(&*item.file_name().to_string_lossy()) {
                let _ = fs::remove_file(item.path());
            }
        }
        Ok(())
    }

    pub fn hide(&self, id: &str, hidden: bool) -> reiwand_format::Result<()> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        reiwand_format::hide(&self.dir, id, hidden)
    }

    pub fn restore(
        &self,
        id: &str,
        version: &str,
        held: &Path,
        now: i64,
        shown: listed::Shown,
    ) -> anyhow::Result<()> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        let dest = self.dir.join("packages").join(id).join(format!("{version}.wpak"));
        reiwand_format::add(&self.dir, held, None, now / 1000)?;
        shown.mark(&self.dir, id, version)?;
        DirBuilder::new().recursive(true).mode(0o700).create(dest.parent().context("bad path")?)?;
        fs::copy(held, &dest)?;
        Ok(reiwand_format::unrevoke(&self.dir, id, version)?)
    }

    pub fn publish(&self, days: u32, now: i64) -> anyhow::Result<Published> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        let signer = Signer::new(&fs::read_to_string(&self.key)?, None)?;
        let days = days.min(settings(&self.dir)?.entry_max_days);
        Ok(reiwand_format::publish(&self.dir, &signer, days, now / 1000)?)
    }

    pub fn authors(&self) -> Option<(Vec<String>, bool, bool)> {
        let s = settings(&self.dir).ok()?;
        let keys = s.authors.into_values().flatten().filter_map(|k| fingerprint(&k).ok()).collect();
        Some((keys, s.require_author_signature, s.online_authors))
    }

    pub fn online(&self) -> bool {
        self.authors().is_some_and(|(_, _, online)| online)
    }

    pub fn register(&self, user: i64, key: &str) -> reiwand_format::Result<String> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        reiwand_format::register_author(&self.dir, &author(user), key)
    }

    pub fn rotate(&self, user: i64, from: &str, key: &str, sig: &str) -> reiwand_format::Result<String> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        reiwand_format::rotate_author(&self.dir, &author(user), from, key, sig)
    }

    pub fn drop_key(&self, user: i64, fp: &str) -> reiwand_format::Result<()> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        reiwand_format::drop_author_key(&self.dir, &author(user), fp)
    }

    pub fn info(&self, cfg: &Config) -> Option<Value> {
        let root = reiwand_format::status(&self.dir).ok()?;
        let latest =
            fs::read_to_string(self.dir.join("entry/latest")).ok().and_then(|t| t.trim().parse::<u64>().ok());
        let entry = latest.and_then(|n| fs::read(self.dir.join(format!("entry/{n}.json"))).ok());
        let entry: Option<Entry> = entry.and_then(|bytes| serde_json::from_slice(&bytes).ok());
        let (entry_version, entry_expires) =
            (entry.as_ref().map(|e| e.version), entry.map(|e| e.expires.to_rfc3339()));
        let base = if self.folder == "repo" { "/repo/".to_string() } else { format!("/r/{}/", self.name) };
        Some(json!({
            "name": self.name, "repo_id": root.repo_id, "root_version": root.version,
            "root_signed": root.complete, "query": root.query,
            "url": cfg.url(&format!("{base}?{}", root.query)),
            "address": cfg.url(&format!("/r/{}/?{}", self.name, root.query)), "online_key": self.key.is_file(),
            "entry_version": entry_version, "entry_expires": entry_expires,
        }))
    }
}

pub fn author(user: i64) -> String {
    format!("u{user}")
}

pub(crate) fn store_key(path: &Path, secret: &str) -> anyhow::Result<()> {
    DirBuilder::new().recursive(true).mode(0o700).create(path.parent().context("bad key path")?)?;
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(path)?;
    Ok(file.write_all(secret.as_bytes())?)
}
