model! {
    #[derive(Default)]
    pub struct TransitionState {
        pub choice: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub own: Option<String>,
        pub plays: String,
        pub why: String,
    }
}

model! {
    pub struct Tones {
        pub night: String,
        pub night_high: String,
        pub night_line: String,
        pub mist: String,
        pub mist_low: String,
        pub moon: String,
        pub moon_deep: String,
        pub veil: f64,
    }
}
