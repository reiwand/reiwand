use super::Uid;

model! {
    pub struct AwayProfile { pub profile: Uid, pub output: String }

    pub struct Away {
        pub key: String,
        pub title: String,
        pub playlists: Vec<Uid>,
        pub profiles: Vec<AwayProfile>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub repo: Option<String>,
    }

    pub struct UploadOutput {
        pub id: String,
        pub state: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub package: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub version: Option<String>,
        pub errors: Vec<String>,
    }
}
