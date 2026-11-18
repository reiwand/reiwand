use serde_json::{Value, json};

pub const BUNDLE_STARTERS: [&str; 1] = ["assets"];

pub(crate) fn document(name: &str, base: Value) -> Option<Value> {
    if name != "assets" {
        return None;
    }
    let mut manifest = base;
    manifest["kind"] = json!("assets");
    manifest["entry"] = json!("assets.json");
    manifest["requires"] = json!({ "features": ["assets"] });
    let scene = json!({"schema": reiwand_scene::SCHEMA,
        "canvas": reiwand_scene::prefill::offer("canvas", "", json!({})), "layers": []});
    Some(json!({ "manifest": manifest, "scene": scene }))
}
