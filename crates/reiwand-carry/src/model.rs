use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub format: u32,
    pub program: String,
    pub version: String,
    pub created: String,
    pub sections: Vec<String>,
    pub encrypted: bool,
    pub sums: BTreeMap<String, String>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Item {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    pub id: String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    pub address: String,
    pub repo_id: String,
    pub origin: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provider {
    pub kind: String,
    pub name: String,
    #[serde(default)]
    pub config: Value,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct List {
    pub name: String,
    pub items: Vec<Item>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<serde_json::Value>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rating {
    pub item: Item,
    pub rating: u8,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tagging {
    pub item: Item,
    pub tags: Vec<String>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Installed {
    pub repo_id: String,
    pub id: String,
    pub version: String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OwnFile {
    pub id: String,
    pub name: String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub repo_id: String,
    pub origin: String,
    pub account: String,
    pub token: String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Secrets {
    pub sessions: Vec<Session>,
    pub provider_keys: BTreeMap<String, String>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Content {
    #[serde(default)]
    pub repositories: Vec<Repository>,
    #[serde(default)]
    pub providers: Vec<Provider>,
    #[serde(default)]
    pub lists: Vec<List>,
    #[serde(default)]
    pub favourites: Vec<Item>,
    #[serde(default)]
    pub ratings: Vec<Rating>,
    #[serde(default)]
    pub tags: Vec<Tagging>,
    #[serde(default)]
    pub installed: Vec<Installed>,
    #[serde(default)]
    pub settings: BTreeMap<String, Value>,
    #[serde(default)]
    pub own: Vec<OwnFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Secrets>,
}

impl Content {
    pub fn sections(&self) -> Vec<String> {
        let held = [
            ("repositories", !self.repositories.is_empty()),
            ("providers", !self.providers.is_empty()),
            ("lists", !self.lists.is_empty()),
            ("favourites", !self.favourites.is_empty()),
            ("ratings", !self.ratings.is_empty()),
            ("tags", !self.tags.is_empty()),
            ("installed", !self.installed.is_empty()),
            ("settings", !self.settings.is_empty()),
            ("own", !self.own.is_empty()),
            ("secrets", self.secrets.is_some()),
        ];
        held.into_iter().filter(|(_, on)| *on).map(|(n, _)| n.to_string()).collect()
    }
}
