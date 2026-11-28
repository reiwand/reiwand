use serde_json::Value;
use std::collections::BTreeMap;

pub const DEVICE_KEYS: [&str; 19] = [
    "playback.hdr",
    "layout.bezel_mm",
    "playback.volume",
    "playback.muted",
    "playback.software_parallax",
    "playback.max_fps",
    "library.thumb_cache_mb",
    "models.allow_download",
    "render.gpu",
    "playback.quality",
    "surfaces.layer_namespace",
    "media.remote_covers",
    "network.private",
    "updates.auto",
    // What each device consents to by itself: a file never turns these on or off.
    "playback.allow_flashing",
    "playback.scene_capture",
    "library.show_mature",
    "repos.auto_update",
    "pause.on_battery",
];

const DEVICE_PREFIXES: [&str; 8] = [
    "outputs.",
    "screens.",
    "device.",
    "playback.quality.",
    "playback.hdr.",
    "transition.kind.",
    "layout.",
    "rgb.",
];

pub fn is_device_local(key: &str) -> bool {
    DEVICE_KEYS.contains(&key) || DEVICE_PREFIXES.iter().any(|p| key.starts_with(p))
}

pub fn portable(settings: &BTreeMap<String, Value>) -> BTreeMap<String, Value> {
    settings.iter().filter(|(k, _)| !is_device_local(k)).map(|(k, v)| (k.clone(), v.clone())).collect()
}
