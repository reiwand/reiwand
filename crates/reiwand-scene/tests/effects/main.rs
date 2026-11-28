use reiwand_scene::*;
use serde_json::json;
use std::collections::BTreeMap;
use std::sync::Arc;

mod checks;
mod costs;
mod fusion;
mod heard;
mod inputs;
mod lighter;
mod locality;
mod spoken;
mod spread;

const VIEW: [f32; 4] = [0.0, 0.0, 64.0, 32.0];

fn probe(json: serde_json::Value, wgsl: &str) -> &'static Descriptor {
    register(Descriptor::parse(&reiwand_fill::descriptors::completed(&json.to_string()), wgsl).unwrap())
        .unwrap()
}

fn scene(effects: serde_json::Value) -> Scene {
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [64, 32] },
        "assets": { "a": { "path": "a.png" }, "grade": { "path": "grade.cube" } },
        "layers": [{ "blend_space": "canvas", "id": "l", "type": "image", "image": "a", "effects": effects }] });
    Scene::from_json(&reiwand_fill::whole(&text.to_string())).unwrap()
}

fn runtime(effects: serde_json::Value) -> Runtime {
    Runtime::new(Arc::new(scene(effects)), Values::new())
}
