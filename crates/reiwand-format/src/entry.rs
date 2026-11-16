use crate::repo::hex64;
use crate::{Error, KINDS_FORMAT, Result, Revoked, Root, Shard, Time, ensure, require, sha256};
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const ENTRY_FORMAT: &str = "wpak-repo-entry/1.0";
const MAX_BLOB: u64 = 32 << 20;
const MAX_BLOBS: u64 = 256 << 20;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub format: String,
    pub repo_id: String,
    pub version: u64,
    pub timestamp: Time,
    pub expires: Time,
    pub root: Pin,
    pub shard_bits: u8,
    pub shards: Vec<Blob>,
    pub revoked: Blob,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authors: Option<Blob>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<Blob>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pin {
    pub version: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Blob {
    pub sha256: String,
    pub size: u64,
}

impl Entry {
    pub fn check(&self, version: u64, repo_id: &str, root: &Root, root_sha256: &str) -> Result<()> {
        let own = self.format == ENTRY_FORMAT && self.repo_id == repo_id && self.version == version;
        require(own, "entry.version", "does not match its file name or repository")?;
        let pin = Pin { version: root.version, sha256: root_sha256.into() };
        require(self.root == pin, "entry.root", "does not pin the current root")?;
        let listed = || self.shards.iter().chain([&self.revoked]).chain(&self.authors).chain(&self.featured);
        let blobs = listed().all(|b| hex64(&b.sha256));
        let total =
            listed().try_fold(0u64, |sum, b| (b.size <= MAX_BLOB).then(|| sum.saturating_add(b.size)));
        require(
            total.is_some_and(|t| t <= MAX_BLOBS),
            "entry.shards",
            "lists files larger than a client reads (32 MiB each, 256 MiB in all)",
        )?;
        let online = self.authors.is_some() == root.policy.online();
        require(
            online,
            "entry.authors",
            "must be listed exactly when the root lets authors register online",
        )?;
        let shards = self.shard_bits <= 8 && self.shards.len() == 1 << self.shard_bits;
        require(blobs && shards, "entry.shards", "expected 2^shard_bits shards named by SHA-256")?;
        let life = self.expires - self.timestamp;
        let max = TimeDelta::days(root.policy.entry_max_days.into());
        require(
            life > TimeDelta::zero() && life <= max,
            "entry.expires",
            "exceeds the lifetime the root allows",
        )
    }
}

impl Blob {
    pub fn matches(&self, bytes: &[u8]) -> Result<()> {
        let ok = bytes.len() as u64 == self.size && sha256(bytes)? == self.sha256;
        ensure(ok, || Error::Mismatch(self.sha256.clone()))
    }
}

pub(crate) fn contents(
    entry: &Entry,
    repo_id: &str,
    blobs: &BTreeMap<String, Vec<u8>>,
) -> Result<(Vec<Shard>, Revoked)> {
    let get = |blob: &Blob| blobs.get(&blob.sha256).ok_or_else(|| Error::Missing(blob.sha256.clone()));
    let shards = entry.shards.iter().enumerate().map(|(index, blob)| {
        let shard = Shard::parse(get(blob)?)?;
        shard.check(KINDS_FORMAT, repo_id, index, entry.shard_bits).map(|()| shard)
    });
    let shards = shards.collect::<Result<Vec<_>>>()?;
    let revoked: Revoked = serde_json::from_slice(get(&entry.revoked)?)?;
    revoked.check(repo_id)?;
    Ok((shards, revoked))
}
