use super::{SourceScan, Time, Uid};
use std::collections::BTreeMap;

model! {
    #[derive(Copy, Eq, Hash)]
    pub enum ProviderState { Idle, Syncing, Locked, Error, NeedsSecret, #[serde(other)] Unknown }

    pub struct Provider {
        pub id: String,
        pub kind: String,
        pub options: BTreeMap<String, String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub source: Option<Uid>,
        pub state: ProviderState,
        pub has_secret: bool,
        pub hosts: Vec<String>,
        pub items: u32,
        pub cache_bytes: u64,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub last_sync: Option<Time>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub next_sync: Option<Time>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub last_error: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub job: Option<u64>,
        pub orphan_secret: bool,
        pub plain_http: bool,
        pub offline: bool,
    }

    pub struct ProvidersReply {
        pub model: u32,
        pub serial: u64,
        pub providers: Vec<Provider>,
        pub store: String,
        pub keyring_found: bool,
    }

    pub struct WallhavenCollection { pub id: u64, pub label: String, pub count: u64, pub public: bool }

    pub struct CollectionsReply {
        pub model: u32,
        pub serial: u64,
        pub collections: Vec<WallhavenCollection>,
    }

    pub struct SyncReport {
        pub provider: String,
        pub listed: u32,
        pub downloaded: u32,
        pub reused: u32,
        pub skipped: u32,
        pub failed: u32,
        pub scan: SourceScan,
    }

    pub struct SyncOutput { pub reports: Vec<SyncReport> }

    #[derive(Copy, Eq, Hash)]
    pub enum RepoWarning { RootExpiring, RootExpired, EntryExpiring, Stale, SignatureChanged, CertificateChanged, #[serde(other)] Unknown }

    pub struct Repo {
        pub repo_id: String,
        pub url: String,
        pub name: String,
        pub hosts: Vec<String>,
        pub fingerprints: Vec<String>,
        pub root_version: u64,
        pub root_expires: Time,
        pub entry_version: u64,
        pub entry_expires: Time,
        pub checked: Time,
        pub stale: bool,
        pub packages: u32,
        pub warnings: Vec<RepoWarning>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub last_error: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub last_error_name: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub job: Option<u64>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub broken: Option<String>,
        pub bundled: bool,
    }

    pub struct ReposReply { pub model: u32, pub serial: u64, pub repos: Vec<Repo> }

    #[derive(Copy, Eq, Hash)]
    pub enum Reason { Malware, License, Takedown, Broken, Other, #[serde(other)] Unknown }

    pub struct RepoPackage {
        pub repo_id: String,
        pub repo_name: String,
        pub id: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub author: Option<String>,
        pub version: String,
        pub kind: String,
        pub title: String,
        pub description: String,
        pub tags: Vec<String>,
        pub license: String,
        pub authors: Vec<String>,
        pub size: u64,
        pub published: Time,
        pub preview: bool,
        pub animated: bool,
        pub versions: Vec<String>,
        pub installed: Vec<String>,
        pub update: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub content: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub photosensitive: Option<bool>,
        pub facets: Vec<String>,
        pub asks: Vec<String>,
        pub flash_disabled: bool,
        pub changes: String,
        pub revealed: bool,
        pub featured: bool,
    }

    pub struct BrowsePage {
        pub model: u32,
        pub serial: u64,
        pub query: String,
        pub total: u32,
        pub hidden: u32,
        pub offset: u32,
        pub packages: Vec<RepoPackage>,
    }

    pub struct InstalledPackage {
        pub repo_id: String,
        pub id: String,
        pub version: String,
        pub sha256: String,
        pub size: u64,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub signer: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub revoked: Option<Reason>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub item: Option<Uid>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub update: Option<String>,
        pub changes: String,
        pub missing: Vec<String>,
        pub asks: Vec<String>,
    }

    pub struct InstalledReply { pub model: u32, pub serial: u64, pub packages: Vec<InstalledPackage> }

    pub struct Update {
        pub repo_id: String,
        pub id: String,
        pub installed: String,
        pub available: String,
        pub changes: String,
    }

    pub struct RefreshOutput { pub repos: Vec<Repo>, pub updates: Vec<Update>, pub revoked: Vec<InstalledPackage> }
}
