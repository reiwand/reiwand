use crate::Reason;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Installed {
    pub repo_id: String,
    pub id: String,
    pub version: String,
    pub sha256: String,
    pub size: u64,
    pub path: PathBuf,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    pub signer: Option<String>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    pub revoked: Option<Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}
