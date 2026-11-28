use super::Uid;
use std::collections::BTreeMap;
use std::path::PathBuf;

model! {
    pub struct LockChoice {
        #[serde(deserialize_with = "crate::explicit::null")]
        pub image: Option<PathBuf>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub item: Option<Uid>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub playlist: Option<Uid>,
        pub name: String,
        pub desktop: bool,
    }

    pub struct LockReply {
        pub model: u32,
        pub serial: u64,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub every: Option<LockChoice>,
        pub outputs: BTreeMap<String, LockChoice>,
    }
}
