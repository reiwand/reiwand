use crate::filter::Filter;
use crate::fit::Fit;
use crate::quality::QualityChoice;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputConfig {
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::empty")]
    pub image: Option<PathBuf>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::general")]
    pub fit: Option<Fit>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::general")]
    pub background: Option<String>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::empty")]
    pub playlist: Option<String>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::empty")]
    pub preset: Option<String>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::none")]
    pub blank: Option<crate::blank::Kept>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::general")]
    pub clicks: Option<bool>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::general")]
    pub filter: Option<Filter>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::general")]
    pub quality: Option<QualityChoice>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::general")]
    pub hdr: Option<crate::HdrChoice>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::general")]
    pub transition: Option<crate::TransitionKind>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    pub scenes: BTreeMap<String, BTreeMap<String, toml::Value>>,
}

impl OutputConfig {
    pub fn sets_content(&self) -> bool {
        self.image.is_some() || self.playlist.is_some() || self.blank.is_some()
    }
}
