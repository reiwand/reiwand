use crate::names::valid_id;
use crate::{Release, Result, ensure, refuse, require};
use reiwand_core::message::said;
use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const KINDS_FORMAT: &str = "wpak-repo-shard/4.0";
pub const REVOKED_FORMAT: &str = "wpak-repo-revoked/2.0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shard {
    pub format: String,
    pub repo_id: String,
    pub shard: usize,
    pub packages: Vec<Listing>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Listing {
    pub id: String,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    pub author: Option<String>,
    pub versions: Vec<Release>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Revoked {
    pub format: String,
    pub repo_id: String,
    pub packages: Vec<Revocation>,
    pub keys: Vec<KeyRevocation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Revocation {
    pub id: String,
    pub versions: Vec<String>,
    pub reason: Reason,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyRevocation {
    pub fingerprint: String,
    pub reason: Reason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reason {
    Malware,
    License,
    Takedown,
    Broken,
    Other,
}

pub(crate) fn format_is(what: &str, found: &str, wanted: &str) -> Result<()> {
    let wrong = || {
        said(
            "format.version",
            "%1 is in format %2 but this version reads only %3: whoever publishes the repository must publish it again with this version",
            &[&what, &found, &wanted],
        )
    };
    ensure(found == wanted, || refuse(what, wrong()))
}

impl Shard {
    pub fn parse(bytes: &[u8]) -> Result<Shard> {
        let mut shard: serde_json::Value = serde_json::from_slice(bytes)?;
        let known = |r: &serde_json::Value| {
            r["kind"].as_str().is_none_or(|k| serde_json::from_value::<crate::Kind>(k.into()).is_ok())
        };
        if let Some(packages) = shard["packages"].as_array_mut() {
            packages.retain_mut(|p| match p["versions"].as_array_mut() {
                Some(v) if v.iter().any(|r| !known(r)) => {
                    v.retain(known);
                    !v.is_empty()
                }
                _ => true,
            });
        }
        crate::strict::value("shard.json", &shard)
    }

    pub fn check(&self, format: &str, repo_id: &str, index: usize, bits: u8) -> Result<()> {
        format_is("shard", &self.format, format)?;
        let own = self.repo_id == repo_id && self.shard == index;
        require(own, "shard", "does not match its place in the entry")?;
        let mut ids = BTreeSet::new();
        self.packages.iter().try_for_each(|p| {
            let placed = valid_id(&p.id) && shard_of(&p.id, bits) == index && ids.insert(&p.id);
            let id = format!("{:?}", p.id);
            let wrong = || said("format.shard_id", "%1 is invalid, misplaced or repeated", &[&id]);
            ensure(placed, || refuse("shard.id", wrong()))?;
            let versions: Option<Vec<Version>> = p.versions.iter().map(Release::parse).collect();
            let sorted = versions.is_some_and(|v| !v.is_empty() && v.is_sorted_by(|a, b| a < b));
            let unsorted =
                || said("format.shard_versions", "%1 needs unique ascending valid releases", &[&id]);
            ensure(sorted, || refuse("shard.versions", unsorted()))
        })
    }
}

impl Revoked {
    pub fn check(&self, repo_id: &str) -> Result<()> {
        format_is("revoked", &self.format, REVOKED_FORMAT)?;
        let own = self.repo_id == repo_id;
        require(own, "revoked", "expected a wpak-repo-revoked/2.0 of this repository")
    }

    pub fn version(&self, id: &str, version: &str) -> Option<Reason> {
        let hit = |r: &&Revocation| r.id == id && r.versions.iter().any(|v| v == "*" || v == version);
        self.packages.iter().find(hit).map(|r| r.reason)
    }

    pub fn key(&self, fingerprint: &str) -> Option<Reason> {
        self.keys.iter().find(|k| k.fingerprint == fingerprint).map(|k| k.reason)
    }
}

pub fn shard_of(id: &str, bits: u8) -> usize {
    usize::from(Sha256::digest(id.as_bytes())[0]) >> (8 - bits.min(8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_older_format_is_named_with_what_to_do() {
        let revoked = Revoked {
            format: "wpak-repo-revoked/1.0".into(),
            repo_id: "r".into(),
            packages: Vec::new(),
            keys: Vec::new(),
        };
        let said = revoked.check("r").unwrap_err().to_string();
        assert!(said.contains("wpak-repo-revoked/1.0") && said.contains(REVOKED_FORMAT), "{said}");
        assert!(said.contains("publish it again with this version"), "{said}");
        assert!(format_is("shard", KINDS_FORMAT, KINDS_FORMAT).is_ok());
    }
}
