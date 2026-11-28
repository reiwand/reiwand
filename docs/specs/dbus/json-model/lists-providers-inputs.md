# Lists, providers, inputs and accounts

On this page: the types of playlists, profiles, presets, providers, repositories, inputs and accounts, and notes on some fields.

## Playlists, history and the lock screen

```rust
enum Order { Sequential, Shuffle }
struct Rules {
    order: Order, interval: String, at: Vec<String>, days: Vec<String>, synced: bool,
    resume: bool, advance_while_paused: bool, skip_videos_on_battery: bool,
    transition_ms: Option<u32>,
}
struct Playlist { uid: Uid, name: String, query: Option<String>, rules: Rules, count: u32, outputs: Vec<String> }
struct PlaylistsReply { model: u32, serial: u64, playlists: Vec<Playlist> }
struct PlaylistPage {
    model: u32, serial: u64, playlist: Playlist, total: u32, offset: u32, items: Vec<ItemSummary>, away: Vec<Away>,
}
struct HistoryEntry { at: Time, item: Option<ItemSummary> }
struct HistoryReply { model: u32, serial: u64, output: String, entries: Vec<HistoryEntry> }
struct LockChoice { image: Option<PathBuf>, item: Option<Uid>, playlist: Option<Uid>, name: String }
struct LockReply { model: u32, serial: u64, every: Option<LockChoice>, outputs: BTreeMap<String, LockChoice> }
```

## Profiles and presets

```rust
struct ProfileOutput {
    item: Option<Uid>, path: Option<PathBuf>, playlist: Option<Uid>, fit: Option<Fit>,
    background: Option<String>, preset: Option<Uid>, filter: Option<Filter>,
}
struct ProfileAuto { outputs: Option<Vec<String>>, on_battery: Option<bool> }
struct Profile {
    uid: Uid, name: String, outputs: BTreeMap<String, ProfileOutput>,
    options: BTreeMap<String, String>, auto: ProfileAuto, priority: i32, active: bool,
}
struct ProfilesReply { model: u32, serial: u64, active: Option<Uid>, profiles: Vec<Profile> }
struct ProfileReply { model: u32, serial: u64, profile: Profile }
struct PresetValue { fit: Option<Fit>, background: Option<String> }
struct Preset { uid: Uid, item: Option<Uid>, name: String, value: PresetValue }
struct PresetsReply { model: u32, serial: u64, item: Option<Uid>, presets: Vec<Preset> }
```

## Providers, repositories and packages

```rust
enum ProviderState { Idle, Syncing, Locked, Error, NeedsSecret }
struct Provider {
    id: String, kind: String, options: BTreeMap<String, String>, source: Option<Uid>,
    state: ProviderState, has_secret: bool, hosts: Vec<String>, items: u32, cache_bytes: u64,
    last_sync: Option<Time>, next_sync: Option<Time>, last_error: Option<String>, job: Option<u64>,
    orphan_secret: bool, plain_http: bool, offline: bool,
}
struct ProvidersReply {
    model: u32, serial: u64, providers: Vec<Provider>, store: String, keyring_found: bool,
}
struct SyncReport {
    provider: String, listed: u32, downloaded: u32, reused: u32, skipped: u32, failed: u32,
    scan: SourceScan,
}
struct SyncOutput { reports: Vec<SyncReport> }
enum RepoWarning {
    RootExpiring, RootExpired, EntryExpiring, Stale, SignatureChanged, CertificateChanged,
}
struct Repo {
    repo_id: String, url: String, name: String, hosts: Vec<String>, fingerprints: Vec<String>,
    root_version: u64, root_expires: Time, entry_version: u64, entry_expires: Time, checked: Time,
    stale: bool, packages: u32, warnings: Vec<RepoWarning>, last_error: Option<String>,
    job: Option<u64>, broken: Option<String>, bundled: bool,
}
struct ReposReply { model: u32, serial: u64, repos: Vec<Repo> }
enum Reason { Malware, License, Takedown, Broken, Other }
struct RepoPackage {
    repo_id: String, repo_name: String, id: String, author: Option<String>, version: String,
    kind: String, title: String, description: String, tags: Vec<String>, license: String,
    authors: Vec<String>, size: u64, published: Time, preview: bool, animated: bool, versions: Vec<String>,
    installed: Vec<String>, update: bool, content: Option<String>, photosensitive: Option<bool>,
    facets: Vec<String>, changes: String, asks: Vec<String>, flash_disabled: bool,
}
struct BrowsePage { model: u32, serial: u64, query: String, total: u32, offset: u32, packages: Vec<RepoPackage> }
struct InstalledPackage {
    repo_id: String, id: String, version: String, sha256: String, size: u64,
    signer: Option<String>, revoked: Option<Reason>, item: Option<Uid>, update: Option<String>,
    changes: String, asks: Vec<String>,
}
struct InstalledReply { model: u32, serial: u64, packages: Vec<InstalledPackage> }
struct Update { repo_id: String, id: String, installed: String, available: String, changes: String }
struct RefreshOutput { repos: Vec<Repo>, updates: Vec<Update>, revoked: Vec<InstalledPackage> }
```

## Inputs and accounts

```rust
enum Input { Audio, Pointer, Clicks, Network, Rgb, Media, MediaNetwork, AudioNetwork }
struct SceneInputs {
    scene: String, audio: bool, pointer: bool, clicks: bool, network: bool, rgb: bool, media: bool, title: String,
    key: String, granted: Vec<Input>, without: BTreeSet<Input>, undecided: BTreeSet<Input>,
}
struct MediaStatus {
    listening: bool, players: u32, playing: bool, track: bool, cover: bool, remote_covers: bool, using: Vec<String>,
}
struct InputsReply {
    model: u32, serial: u64, capturing: bool, tracking: bool,
    grants: BTreeMap<String, Vec<Input>>, shown: BTreeMap<String, SceneInputs>, parallax: Vec<String>,
    declined: BTreeMap<String, Vec<Input>>, asking: BTreeMap<String, Vec<Input>>,
    quiet: BTreeMap<String, Vec<Input>>, clicks: Vec<String>, clicking: Vec<String>, media: MediaStatus,
}
struct ServerAccountRepo { url: String, repo_id: Option<String> }
struct ServerAccount {
    id: String, server: String, user: String, label: String, pending: bool, repos: Vec<ServerAccountRepo>,
}
struct AccountsReply { model: u32, serial: u64, accounts: Vec<ServerAccount> }
struct SignIn { user_code: String, verification_uri: String, job: u64 }
struct UploadOutput { id: String, state: String, package: Option<String>, version: Option<String>, errors: Vec<String> }
```

## Notes on some fields


`outputs.<name>.still_reason` says why the wallpaper of one output is not moving, or is `null` while it plays (`active` is true exactly then). One function of the daemon decides it, from these in order: `paused` (`SetPaused`, the manual pause), `locked` (the screen is locked and `pause.when_locked` holds), `battery` (`pause.on_battery` says to pause with the present power source and charge) and `covered` (a window covers the output and `pause.when_covered` holds); the first that applies is reported, so the other reasons show up once it ends. `reiwand-ctl state` prints the JSON as before and, on a terminal, one line per screen on the error output with the reason in words. The app does not show it. Whether a window covers an output is known as follows: on Plasma, the wallpaper plugin of each screen reports it with `SetOutputCovered` (an output with no claimant reports nothing, so it is never covered); on layer-shell and X11 outputs the daemon works it out itself, as described above (frame callbacks that stop, fullscreen or maximized windows, Hyprland workspaces, COSMIC overlap notices, X11 window geometry and visibility). It cannot be known on a compositor that sends frame callbacks all the time and offers no way to list its windows (a window then covers the screen without the wallpaper pausing), nor for a window that covers the screen only through transparency or only in part.

`battery_percent` is the charge of the machine's battery as UPower reports it (0 to 100, rounded), read from the same service as `on_battery` and followed when it changes. It is `null` when the machine has no battery or when the power service cannot be reached, so a script must treat `null` as "unknown" and not as zero. Together with `on_battery` it feeds the settings `pause.below_percent` and `playback.lowest_below_percent`.

Bulk replies (`Search`, playlist pages, history, installed packages) carry no file paths except
cached thumbnail paths; an item's path appears only in `GetItem` and in `GetState().outputs`.
