use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Repos {
    pub auto_refresh: bool,
    pub auto_update: bool,
    pub vote_counts: bool,
}

impl Default for Repos {
    fn default() -> Self {
        Self { auto_refresh: true, auto_update: false, vote_counts: true }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Models {
    pub allow_download: bool,
}

impl Default for Models {
    fn default() -> Self {
        Self { allow_download: true }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Media {
    pub remote_covers: bool,
}
