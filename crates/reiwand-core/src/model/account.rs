use super::Uid;

model! {
    pub struct AwayProfile { pub profile: Uid, pub output: String }

    pub struct Away {
        pub key: String,
        pub title: String,
        #[serde(default)]
        pub playlists: Vec<Uid>,
        #[serde(default)]
        pub profiles: Vec<AwayProfile>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub repo: Option<String>,
    }

    pub struct UploadOutput {
        pub id: String,
        pub state: String,
        pub package: Option<String>,
        pub version: Option<String>,
        pub errors: Vec<String>,
    }
}
