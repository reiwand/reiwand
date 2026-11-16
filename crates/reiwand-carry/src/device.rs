use serde_json::Value;
use std::collections::BTreeMap;

pub const DEVICE_KEYS: [&str; 12] = [
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
    "pause.on_battery",
];

// per-screen choices are named after the screen (`playback.quality.DP-1`), which belongs to this machine
const DEVICE_PREFIXES: [&str; 5] =
    ["outputs.", "screens.", "device.", "playback.quality.", "transition.kind."];

pub fn is_device_local(key: &str) -> bool {
    DEVICE_KEYS.contains(&key) || DEVICE_PREFIXES.iter().any(|p| key.starts_with(p))
}

pub fn portable(settings: &BTreeMap<String, Value>) -> BTreeMap<String, Value> {
    settings.iter().filter(|(k, _)| !is_device_local(k)).map(|(k, v)| (k.clone(), v.clone())).collect()
}
