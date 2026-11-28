use serde_json::Value;

model! {
    pub struct Application {
        pub id: String,
        pub name: String,
        pub icon: String,
    }

    pub struct ApplicationsReply {
        pub model: u32,
        pub serial: u64,
        pub available: bool,
        pub applications: Vec<Application>,
    }

    pub struct SceneField {
        pub key: String,
        pub kind: String,
        pub label: String,
        pub help: String,
        pub value: Value,
        pub visible: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub min: Option<f64>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub max: Option<f64>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub step: Option<f64>,
        pub integer: bool,
        pub unit: String,
        pub options: Vec<SceneOption>,
        pub alpha: bool,
        pub multiline: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub max_len: Option<u32>,
        pub accept: Vec<String>,
        pub enabled: bool,
        pub group: String,
        pub icon: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub decimals: Option<u32>,
        pub editable: bool,
        pub log: bool,
        pub collapsed: bool,
        pub secret: bool,
        pub filled: bool,
    }

    pub struct SceneOption {
        pub value: Value,
        pub label: String,
        pub icon: String,
    }
}
