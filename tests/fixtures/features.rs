use base64::Engine;
use serde_json::{Map, Value, json};
use std::fs;
use std::path::{Path, PathBuf};

pub fn cases() -> Vec<(String, Value)> {
    let table: Map<String, Value> = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/features.json"
    )))
    .unwrap();
    table.into_iter().collect()
}

fn merge(base: &mut Value, more: &Value) {
    if let (Some(base), Some(more)) = (base.as_object_mut(), more.as_object()) {
        base.extend(more.clone());
    }
}

fn bytes(text: &str) -> Vec<u8> {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures");
    match (text.strip_prefix("fixture:"), text.strip_prefix("base64:")) {
        (Some(name), _) => fs::read(fixtures.join(name)).unwrap(),
        (_, Some(data)) => base64::engine::general_purpose::STANDARD.decode(data).unwrap(),
        _ => text.as_bytes().to_vec(),
    }
}

pub fn source(name: &str, case: &Value, dir: &Path) -> PathBuf {
    let source = dir.join(name);
    let (kind, entry) =
        (case["kind"].as_str().unwrap_or("scene"), case["entry"].as_str().unwrap_or("scene.json"));
    let id = format!("org.example.{}", name.replace(['.', '_'], "-"));
    let mut manifest = json!({ "schema": reiwand_format::SCHEMA, "id": id, "version": "1.0.0", "title": name,
        "license": "CC0-1.0", "kind": kind, "entry": entry, "content": { "rating": "general" },
        "photosensitive": false, "origin": { "kind": "original" }, "previews": { "thumbnail": "preview.png" } });
    merge(&mut manifest, &case["manifest"]);
    let mut scene = json!({ "schema": reiwand_scene::SCHEMA, "canvas": { "size": [64, 32] },
        "assets": { "sky": { "path": "sky.png" } } });
    merge(&mut scene, &case["scene"]);
    let scene = serde_json::from_str::<Value>(&reiwand_upgrade::fill::whole(&scene.to_string())).unwrap();
    let mut files = vec![("preview.png".to_owned(), bytes("fixture:green-64x32.png"))];
    files.push(("sky.png".to_owned(), bytes("fixture:green-64x32.png")));
    files.push(("manifest.json".to_owned(), manifest.to_string().into_bytes()));
    if matches!(kind, "scene" | "assets") {
        files.push((entry.to_owned(), scene.to_string().into_bytes()));
    }
    let given = case["files"].as_object().into_iter().flatten();
    files.extend(given.map(|(path, text)| (path.clone(), bytes(text.as_str().unwrap_or_default()))));
    for (path, data) in files {
        fs::create_dir_all(source.join(&path).parent().unwrap()).unwrap();
        fs::write(source.join(path), data).unwrap();
    }
    source
}

pub fn build(name: &str, case: &Value, dir: &Path) -> reiwand_format::Result<PathBuf> {
    let out = dir.join(format!("{name}.wpak"));
    let source = source(name, case, dir);
    reiwand_format::build(&source, &out, &reiwand_format::Limits::default()).map(|_| out)
}

fn whole(d: &reiwand_scene::Descriptor) -> Value {
    let params: Map<String, Value> = d
        .params
        .iter()
        .map(|p| (p.id.clone(), if p.kind.asset() { json!("sky") } else { json!(p.prefill) }))
        .collect();
    json!({ "effect": d.id, "params": params })
}

pub fn effects() -> Value {
    let first = |id: &str| reiwand_format::features::BUILTIN.split_whitespace().any(|b| b == id);
    let newer = reiwand_scene::builtin().iter().filter(move |d| !first(&d.id));
    let used = reiwand_scene::undrawn().iter().chain(newer).map(whole);
    let built =
        reiwand_scene::builtin().iter().filter(|d| reiwand_scene::newer_params(&d.id).next().is_some());
    let set = built.map(whole);
    let layer = |(i, e): (usize, Value)| json!({ "blend_space": "canvas", "id": format!("e{i}"), "type": "image", "image": "sky", "effects": [e] });
    let layers: Vec<Value> = used.chain(set).enumerate().map(layer).collect();
    json!({ "manifest": { "inputs": ["audio", "pointer", "clicks"] }, "scene": { "layers": layers } })
}

#[allow(dead_code)]
pub fn newer_effects() -> impl Iterator<Item = &'static str> {
    let first = |id: &str| reiwand_format::features::BUILTIN.split_whitespace().any(|b| b == id);
    let newer = reiwand_scene::builtin().iter().filter(move |d| !first(&d.id));
    reiwand_scene::undrawn().iter().chain(newer).map(|d| d.id.as_str())
}
