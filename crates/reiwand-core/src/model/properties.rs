use super::{SceneField, SceneOption, Time, Uid};
use serde_json::Value;
use std::collections::BTreeMap;

model! {
    pub struct SceneProperties {
        pub model: u32,
        pub serial: u64,
        pub output: String,
        pub package: String,
        pub title: String,
        pub properties: BTreeMap<String, Value>,
        pub values: BTreeMap<String, Value>,
        pub presets: Vec<Value>,
        pub fields: Vec<SceneField>,
        pub preset_labels: Vec<SceneOption>,
        pub warnings: Vec<String>,
        pub user_presets: Vec<SceneOption>,
        pub shared: bool,
        pub kind: String,
    }

    pub struct OutputPlaylist {
        pub uid: Uid,
        pub name: String,
        pub position: u32,
        pub count: u32,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub next_at: Option<Time>,
    }
}
