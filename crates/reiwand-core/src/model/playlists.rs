use super::{ItemSummary, Time, Uid};
use crate::Fit;
use std::collections::BTreeMap;
use std::path::PathBuf;

model! {
    #[derive(Copy, Eq, Hash)]
    pub enum Order { Sequential, Shuffle, #[serde(other)] Unknown }

    pub struct Rules {
        pub order: Order,
        pub interval: String,
        pub at: Vec<String>,
        pub days: Vec<String>,
        pub synced: bool,
        pub resume: bool,
        pub advance_while_paused: bool,
        pub skip_videos_on_battery: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub transition_ms: Option<u32>,
    }

    pub struct Playlist {
        pub uid: Uid,
        pub name: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub query: Option<String>,
        pub rules: Rules,
        pub count: u32,
        pub outputs: Vec<String>,
    }

    pub struct PlaylistsReply { pub model: u32, pub serial: u64, pub playlists: Vec<Playlist> }

    pub struct PlaylistPage {
        pub model: u32,
        pub serial: u64,
        pub playlist: Playlist,
        pub total: u32,
        pub offset: u32,
        pub items: Vec<ItemSummary>,
    }

    pub struct HistoryEntry {
        pub at: Time,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub item: Option<ItemSummary>,
    }

    pub struct HistoryReply { pub model: u32, pub serial: u64, pub output: String, pub entries: Vec<HistoryEntry> }

    pub struct ProfileOutput {
        #[serde(deserialize_with = "crate::explicit::null")]
        pub item: Option<Uid>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub path: Option<PathBuf>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub playlist: Option<Uid>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub fit: Option<Fit>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub background: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub preset: Option<Uid>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub filter: Option<crate::Filter>,
    }

    pub struct ProfileAuto {
        #[serde(deserialize_with = "crate::explicit::null")]
        pub outputs: Option<Vec<String>>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub on_battery: Option<bool>,
    }

    pub struct Profile {
        pub uid: Uid,
        pub name: String,
        pub outputs: BTreeMap<String, ProfileOutput>,
        pub options: BTreeMap<String, String>,
        pub auto: ProfileAuto,
        pub priority: i32,
        pub active: bool,
    }

    pub struct ProfilesReply {
        pub model: u32,
        pub serial: u64,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub active: Option<Uid>,
        pub profiles: Vec<Profile>,
    }

    pub struct ProfileReply { pub model: u32, pub serial: u64, pub profile: Profile }

    pub struct PresetValue {
        #[serde(deserialize_with = "crate::explicit::null")]
        pub fit: Option<Fit>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub background: Option<String>,
    }

    pub struct Preset {
        pub uid: Uid,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub item: Option<Uid>,
        pub name: String,
        pub value: PresetValue,
    }

    pub struct PresetsReply {
        pub model: u32,
        pub serial: u64,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub item: Option<Uid>,
        pub presets: Vec<Preset>,
    }

    #[derive(Copy, Eq, Hash)]
    pub enum BrowseSource { Playlist, Library, #[serde(other)] Unknown }

    #[derive(Copy, Eq, Hash)]
    pub enum BrowseEnd { Applied, Cancelled, Expired, #[serde(other)] Unknown }

    #[derive(Copy, Eq, Hash)]
    pub enum ShortcutsState { Off, Asking, Bound, Refused, Unavailable, #[serde(other)] Unknown }

    #[derive(Default)]
    pub struct BrowsePlays { pub kind: String, pub why: String, pub duration_ms: u32 }

    pub struct BrowseState {
        pub model: u32,
        pub serial: u64,
        pub output: String,
        pub active: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub ended: Option<BrowseEnd>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub source: Option<BrowseSource>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub playlist: Option<Uid>,
        pub playlist_name: String,
        pub steps: i32,
        pub position: u32,
        pub count: u32,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub shown: Option<Uid>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub candidate: Option<ItemSummary>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub now: Option<ItemSummary>,
        pub ahead: Vec<ItemSummary>,
        pub behind: Vec<ItemSummary>,
        pub plays: BrowsePlays,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub path: Option<PathBuf>,
        pub transition: String,
        pub idle_ms: u32,
        pub shortcuts: ShortcutsState,
    }
}
