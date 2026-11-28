model! {
    pub struct PluginSecret {
        pub label: String,
        pub header: String,
        pub hosts: Vec<String>,
        pub required: bool,
    }

    pub struct PluginInfo {
        pub id: String,
        pub kind: String,
        pub version: String,
        pub title: String,
        pub license: String,
        pub hosts: Vec<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub secret: Option<PluginSecret>,
        pub sources: Vec<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub error: Option<String>,
    }

    pub struct PluginsReply {
        pub model: u32,
        pub serial: u64,
        pub plugins: Vec<PluginInfo>,
    }

    pub struct PluginOffer {
        pub model: u32,
        pub plugin: PluginInfo,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub installed: Option<String>,
        /// `new`, `newer`, `same` or `older` than the installed version.
        pub step: String,
        pub added_hosts: Vec<String>,
        pub secret_changed: bool,
        pub token: String,
    }
}
