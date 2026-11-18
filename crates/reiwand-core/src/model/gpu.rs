model! {
    pub struct PreviewState {
        #[serde(deserialize_with = "crate::explicit::null")]
        pub error: Option<String>,
        pub console: Vec<String>,
    }

    pub struct GpuAdapter {
        pub name: String,
        pub kind: String,
        pub backend: String,
        pub used: bool,
        pub bus: String,
        pub compositor: bool,
    }
}
