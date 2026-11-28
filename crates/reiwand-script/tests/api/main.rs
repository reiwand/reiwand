#[path = "../common/mod.rs"]
mod common;

use common::{at, load, staged, text};
use reiwand_scene::{AudioFrame, Extent, LayerRead, PointerState, Values, script_key};
use reiwand_script::{Frame, Mode, Touch, TouchKind};
use serde_json::json;

mod geometry;
mod hierarchy;
mod inputs;
mod keep;
mod layers;
mod lifecycle;
mod media;
mod modules;
mod molds;
mod shortcuts;
mod stage;

fn spin_scene() -> serde_json::Value {
    let params = json!({ "speed": { "type": "number", "default": 1, "min": 0, "max": 10 } });
    let spin = json!({ "script": "spin.js", "fn": "spin", "params": { "speed": 2 } });
    let fade = json!({ "script": "spin.js", "fn": "fade" });
    json!({
        "scripts": [{ "path": "spin.js", "events": ["init"], "params": params }],
        "layers": [{ "blend_space": "canvas", "id": "dot", "type": "solid", "color": "#ffffff", "opacity": fade, "transform": { "rotation": spin } }]
    })
}
