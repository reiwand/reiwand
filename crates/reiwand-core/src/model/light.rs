model! {
    #[derive(Default)]
    pub struct HdrState {
        pub choice: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub own: Option<String>,
        pub active: bool,
        pub offered: bool,
        pub display: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub peak_nits: Option<f64>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub reference_nits: Option<f64>,
        pub reason: String,
        pub encoding: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub max_cll: Option<u32>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub max_fall: Option<u32>,
    }
}
