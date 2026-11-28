# State, jobs and the library

On this page: the types of the state, outputs, settings fields, jobs and library items.

## State and outputs

```rust
struct State {
    model: u32, version: String, adapter: String, tier: String, tier_measured: bool, config: PathBuf,
    config_error: Option<String>,
    library_error: Option<String>, serial: u64, paused: bool, on_battery: bool, battery_percent: Option<u8>, locked: bool,
    volume: f64, muted: bool, profile: Option<Uid>, jobs: u32,
    options: BTreeMap<String, Value>,
    outputs: BTreeMap<String, OutputState>, previews: BTreeMap<String, PreviewState>, calibrating: bool, compute: bool,
    adapters: Vec<GpuAdapter>, gpu_warning: Option<String>, gpu_note: Option<String>, gpu_restart: bool,
    qualities: BTreeMap<String, String>, hdrs: BTreeMap<String, String>, layer_shell: LayerShellState,
}
struct LayerShellState { namespace: Option<String>, from: String, restart: bool, ignored: Option<String> }
struct GpuAdapter { name: String, kind: String, backend: String, used: bool, bus: String, compositor: bool }
struct OutputState {
    width: u32, height: u32, image: Option<PathBuf>, fit: Fit, background: String,
    kind: Shown, playing: bool, active: bool, covered: bool, still_reason: Option<StillReason>, clients: u32, error: Option<String>,
    item: Option<Uid>, preset: Option<Uid>, playlist: Option<OutputPlaylist>, credit: Option<Credit>,
    edited: bool, screen: Option<Screen>, mm: Option<[f64; 4]>, parallax: bool, frames: u64, page_frames: u64, refresh: u32, fps: u32,
    buffers: String, sound: String, shown_fit: Fit, flashes: String, left_out: Vec<String>,
    particles: String, quality: String, quality_reason: String, quality_choice: Option<String>,
    quality_cost: f64, quality_budget: f64, quality_auto: String, quality_frame_ms: Option<f64>,
    quality_period_ms: Option<f64>, quality_step: String, quality_pace: u32, sandbox: String,
    accent: Option<String>, tones: Option<Tones>, filter: Option<Filter>, hdr: HdrState,
    away: Option<String>,
}
struct Tones { night: String, night_high: String, night_line: String, mist: String, mist_low: String,
    moon: String, moon_deep: String, veil: f64 }
struct PreviewState { error: Option<String>, console: Vec<String> }
struct HdrState {
    choice: String, own: Option<String>, active: bool, offered: bool, display: bool,
    peak_nits: Option<f64>, reference_nits: Option<f64>, reason: String, encoding: String,
    max_cll: Option<u32>, max_fall: Option<u32>,
}
struct Filter { look: String, strength: f32, exposure: f32, contrast: f32, saturation: f32, warmth: f32, hue: f32 }
struct Screen { x: i32, y: i32, width: u32, height: u32, scale: f64, px_per_mm: f64, identity: String }
enum Shown { Color, Image, Video, Scene, Web }
struct OutputPlaylist { uid: Uid, name: String, position: u32, count: u32, next_at: Option<Time> }
```

## Scene properties, settings fields and provider kinds

```rust
struct SceneProperties {
    model: u32, serial: u64, output: String, package: String, title: String,
    properties: BTreeMap<String, Value>, values: BTreeMap<String, Value>, presets: Vec<Value>,
    fields: Vec<SceneField>, preset_labels: Vec<SceneOption>, warnings: Vec<String>,
    user_presets: Vec<SceneOption>, shared: bool, kind: String,
}
struct SceneField {
    key: String, kind: String, label: String, help: String, value: Value, visible: bool,
    min: Option<f64>, max: Option<f64>, step: Option<f64>, integer: bool, unit: String,
    options: Vec<SceneOption>, alpha: bool, multiline: bool, max_len: Option<u32>, accept: Vec<String>,
    enabled: bool, group: String, icon: String, decimals: Option<u32>, editable: bool, log: bool,
    collapsed: bool, secret: bool, filled: bool,
}
struct SceneOption { value: Value, label: String, icon: String }

struct Schema {
    model: u32, serial: u64, options: Vec<Field>, rules: Vec<Field>, auto: Vec<Field>,
    preset: Vec<Field>, providers: Vec<ProviderKind>, query: Vec<QueryKey>, thumb_sizes: Vec<u32>,
    fits: Vec<Fit>,
}
struct Field {
    key: String, kind: FieldKind, default: Value, min: Option<f64>, max: Option<f64>,
    step: Option<f64>, values: Vec<String>, required: bool, clearable: bool,
}
enum FieldKind { Bool, Int, Float, Text, Enum, Color, Interval, Times, Weekdays, List, Url, Flags, Palette }
struct WallhavenCollection { id: u64, label: String, count: u64, public: bool }
struct CollectionsReply { model: u32, serial: u64, collections: Vec<WallhavenCollection> }
struct ProviderKind { kind: String, hosts: Vec<String>, options: Vec<Field>, secret: Option<SecretField> }
struct SecretField { name: String, required: bool }
struct QueryKey { key: String, hint: String }
```

## Jobs

```rust
struct Job {
    id: u64, kind: JobKind, target: String, state: JobState, done: u64, total: u64,
    started_at: Time, finished_at: Option<Time>, error: Option<JobError>, output: Option<Value>,
}
enum JobKind { Scan, Import, Analyze, ProviderSync, RepoAdd, RepoRefresh, Install, Account, Link, AccountSync, Upload, Layers }
enum JobState { Queued, Running, Done, Failed, Cancelled }
struct JobError { name: String, message: String }
struct JobsReply { model: u32, serial: u64, jobs: Vec<Job> }
struct JobReply { model: u32, serial: u64, job: Job }
```

## Library items

```rust
enum ItemKind { Image, Video, Package }
struct PackageInfo {
    id: String, version: String, kind: String, license: String, content: Option<String>,
    photosensitive: Option<bool>, facets: Vec<String>, confirmed: Vec<String>, exposes: Vec<String>,
    unshown: Option<String>, flash_disabled: bool,
}
struct About {
    schema: String, title: String, description: String, changes: String, authors: Vec<String>,
    license: String, content: Option<String>, descriptors: Vec<String>, photosensitive: Option<bool>,
    origin: Option<String>, derived_from: Vec<Work>, credits: Vec<PackageCredit>, links: Vec<Link>,
    inputs: Vec<String>, features: Vec<Feature>, languages: Vec<String>, provider: Option<String>,
    source_url: Option<String>, platforms: Vec<String>, packages: Vec<String>,
}
struct Work { author: String, title: Option<String>, url: Option<String>, license: Option<String> }
struct PackageCredit {
    paths: Vec<String>, license: String, attribution: String, url: Option<String>, source: Option<String>,
}
struct Link { kind: String, url: String }
struct Feature { name: String, why: String }
struct Item {
    uid: Uid, source: Option<Uid>, kind: ItemKind, path: PathBuf, title: String, size: u64,
    mtime: Time, width: Option<u32>, height: Option<u32>, duration_ms: Option<u32>,
    audio: Option<bool>, color: Option<u32>, error: Option<String>, package: Option<PackageInfo>,
    tags: Vec<String>, rating: u8, favorite: bool, added_at: Time, used_at: Option<Time>,
    use_count: u32, missing: bool,
}
struct ItemSummary {
    uid: Uid, kind: ItemKind, title: String, width: Option<u32>, height: Option<u32>,
    duration_ms: Option<u32>, color: Option<u32>, tags: Vec<String>, rating: u8, favorite: bool,
    missing: bool, error: Option<String>, revoked: Option<Reason>, removable: bool, imported: bool,
    thumb: Option<PathBuf>, package: Option<PackageInfo>,
}
struct SearchPage {
    model: u32, serial: u64, query: String, total: u32, offset: u32, items: Vec<ItemSummary>, away: Vec<Away>,
}
struct ItemDetails {
    model: u32, serial: u64, item: Item, thumb: Option<PathBuf>, presets: Vec<Preset>,
    playlists: Vec<PlaylistRef>, credit: Option<Credit>, installed: Option<InstalledPackage>,
    revoked: Option<Reason>, about: Option<About>,
}
struct PlaylistRef { uid: Uid, name: String }
struct Source {
    uid: Uid, path: PathBuf, recursive: bool, scanned_at: Option<Time>, error: Option<String>,
    items: u32, provider: Option<String>,
}
struct SourcesReply { model: u32, serial: u64, sources: Vec<Source> }
struct Tag { name: String, count: u32 }
struct TagsReply { model: u32, serial: u64, tags: Vec<Tag> }
struct SourceScan { source: Option<Uid>, added: u32, updated: u32, moved: u32, missing: u32 }
struct ScanOutput { reports: Vec<SourceScan> }
struct Imported { path: PathBuf, uid: Uid }
struct Kept { path: PathBuf, uid: Uid, version: String, newer: bool }
struct PathError { path: PathBuf, error: String }
struct ImportReport { imported: Vec<Imported>, kept: Vec<Kept>, failed: Vec<PathError> }
struct AnalyzeReport { analyzed: u32, failed: u32 }
struct Credit { provider: String, title: String, attribution: Option<String>, license: Option<String>, link: Option<String> }
```
