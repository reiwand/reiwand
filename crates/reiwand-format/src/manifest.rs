use crate::meta::{Content, Credit, Links, Previews};
use crate::needs::{Input, Permissions, Platform, Requires};
use crate::origin::Origin;
use crate::video::Playback;
use crate::web::Web;
use crate::{Display, Preset, Property, Result, Text};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod kind;
mod read;

pub use kind::Kind;

pub const SCHEMA: &str = "reiwand.wpak/3";
const PREFIX: &str = "reiwand.wpak/";

pub type Strings = BTreeMap<String, BTreeMap<String, String>>;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: String,
    pub id: String,
    pub version: String,
    pub title: Text,
    #[schemars(extend("x-markup" = true))]
    pub description: Text,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    #[schemars(extend("x-markup" = true))]
    pub changes: Option<Text>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    #[schemars(extend("x-ref" = "language"))]
    pub default_locale: Option<String>,
    pub strings: Strings,
    #[serde(skip)]
    #[schemars(skip)]
    pub string_files: Strings,
    pub authors: Vec<String>,
    pub license: String,
    pub credits: Vec<Credit>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    #[schemars(extend("x-required" = true))]
    pub origin: Option<Origin>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    #[schemars(extend("x-required" = true))]
    pub content: Option<Content>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    #[schemars(extend("x-required" = true))]
    pub photosensitive: Option<bool>,
    pub kind: Kind,
    pub entry: String,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub previews: Option<Previews>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub links: Option<Links>,
    pub tags: Vec<String>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub requires: Option<Requires>,
    pub platforms: Vec<Platform>,
    pub inputs: Vec<Input>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub permissions: Option<Permissions>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub web: Option<Web>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub playback: Option<Playback>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub display: Option<Display>,
    pub properties: BTreeMap<String, Property>,
    pub presets: Vec<Preset>,
}

pub fn manifest_schema() -> serde_json::Value {
    serde_json::Value::from(schemars::schema_for!(Manifest))
}

impl Manifest {
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn thumbnail(&self) -> Option<&String> {
        self.previews.as_ref().and_then(|p| p.thumbnail.as_ref())
    }

    pub fn animated(&self) -> Option<&String> {
        self.previews.as_ref().and_then(|p| p.animated.as_ref())
    }

    pub fn features(&self) -> &[String] {
        self.requires.as_ref().map_or(&[], |r| &r.features)
    }
}
