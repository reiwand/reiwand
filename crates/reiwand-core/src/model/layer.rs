model! {
    #[derive(Default)]
    pub struct LayerShellState {
        #[serde(deserialize_with = "crate::explicit::null")]
        pub namespace: Option<String>,
        pub from: String,
        pub restart: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub ignored: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub ignored_from: Option<String>,
        pub ignored_empty: bool,
    }
}
