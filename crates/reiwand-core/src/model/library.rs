use super::{About, InstalledPackage, Preset, Reason, Time, Uid};
use std::path::PathBuf;

model! {
    #[derive(Copy, Eq, Hash)]
    pub enum ItemKind { Image, Video, Package, #[serde(other)] Unknown }

    pub struct PackageInfo {
        pub id: String,
        pub version: String,
        pub kind: String,
        pub license: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub content: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub photosensitive: Option<bool>,
        pub facets: Vec<String>,
        pub confirmed: Vec<String>,
        pub exposes: Vec<String>,
        pub flash_disabled: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub unshown: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub base: Option<String>,
    }

    pub struct Item {
        pub uid: Uid,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub source: Option<Uid>,
        pub kind: ItemKind,
        pub path: PathBuf,
        pub title: String,
        pub size: u64,
        pub mtime: Time,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub width: Option<u32>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub height: Option<u32>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub duration_ms: Option<u32>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub audio: Option<bool>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub color: Option<u32>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub error: Option<String>,
        #[serde(default, deserialize_with = "crate::explicit::block")]
        pub package: Option<PackageInfo>,
        pub tags: Vec<String>,
        pub rating: u8,
        pub favorite: bool,
        pub added_at: Time,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub used_at: Option<Time>,
        pub use_count: u32,
        pub missing: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub sound_muted: Option<bool>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub sound_volume: Option<u8>,
    }

    pub struct ItemSummary {
        pub uid: Uid,
        pub kind: ItemKind,
        pub title: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub width: Option<u32>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub height: Option<u32>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub duration_ms: Option<u32>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub color: Option<u32>,
        pub tags: Vec<String>,
        pub rating: u8,
        pub favorite: bool,
        pub missing: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub error: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub revoked: Option<Reason>,
        pub removable: bool,
        pub imported: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub sound_muted: Option<bool>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub sound_volume: Option<u8>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub thumb: Option<PathBuf>,
        #[serde(default, deserialize_with = "crate::explicit::block")]
        pub package: Option<PackageInfo>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub provider: Option<String>,
    }

    pub struct SearchPage {
        pub model: u32,
        pub serial: u64,
        pub query: String,
        pub total: u32,
        pub offset: u32,
        pub items: Vec<ItemSummary>,
    }

    pub struct ItemDetails {
        pub model: u32,
        pub serial: u64,
        pub item: Item,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub thumb: Option<PathBuf>,
        pub presets: Vec<Preset>,
        pub playlists: Vec<PlaylistRef>,
        #[serde(default, deserialize_with = "crate::explicit::block")]
        pub credit: Option<Credit>,
        #[serde(default, deserialize_with = "crate::explicit::block")]
        pub installed: Option<InstalledPackage>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub revoked: Option<Reason>,
        #[serde(default, deserialize_with = "crate::explicit::block")]
        pub about: Option<About>,
        pub variants: Vec<PlaylistRef>,
        #[serde(default, deserialize_with = "crate::explicit::block")]
        pub base: Option<PlaylistRef>,
    }

    pub struct PlaylistRef { pub uid: Uid, pub name: String }

    pub struct Credit {
        pub provider: String,
        pub title: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub attribution: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub license: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub link: Option<String>,
    }
}

pub fn asks(what: &str, content: Option<&str>, confirmed: &[String]) -> bool {
    what == "mature" && content == Some("mature") && !confirmed.iter().any(|c| c == what)
}

pub fn exposes(kind: &str, facets: &[String], confirmed: &[String], network: bool) -> Vec<String> {
    let has = |list: &[String], name: &str| list.iter().any(|f| f == name);
    let parts = [("zone", true), ("language", true), ("gpu", true), ("storage", has(facets, "storage"))];
    let shown = parts.into_iter().filter(|(name, on)| *on && !has(confirmed, &format!("hide_{name}")));
    let network = (network && has(facets, "network")).then_some("network");
    let all = shown.map(|(name, _)| name).chain(network).map(String::from);
    if kind == "web" { all.collect() } else { Vec::new() }
}

impl PackageInfo {
    pub fn asks(&self, what: &str) -> bool {
        asks(what, self.content.as_deref(), &self.confirmed)
    }
}
