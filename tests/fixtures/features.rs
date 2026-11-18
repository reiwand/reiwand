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

fn bare(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|_, v| !v.is_null());
            map.values_mut().for_each(bare);
        }
        Value::Array(items) => items.iter_mut().for_each(bare),
        _ => {}
    }
}

fn bindings(value: &mut Value) {
    match value {
        Value::Object(map) => map.values_mut().for_each(bindings),
        Value::Array(items) if items.len() == 3 && items[2] == json!(1.0) => {
            for item in items.iter_mut().filter(|i| i.get("keys").is_some()) {
                *item = reiwand_scene::prefill::of("Binding_for_float", item.take());
                item["loop"] = json!("repeat");
            }
        }
        Value::Array(items) => items.iter_mut().for_each(bindings),
        _ => {}
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
    reiwand_upgrade::fill::complete(reiwand_upgrade::fill::Format::Manifest, &mut manifest);
    if let Some(playback) = case["manifest"].get("playback") {
        manifest["playback"] = playback.clone();
    }
    let mut scene = json!({ "schema": reiwand_scene::SCHEMA, "canvas": { "size": [64, 32] },
        "assets": { "sky": { "path": "sky.png" } } });
    merge(&mut scene, &case["scene"]);
    scene["assets"].as_object_mut().unwrap().entry("sky").or_insert(json!({ "path": "sky.png" }));
    if let Some(particles) = scene.get_mut("particles") {
        bare(particles);
        bindings(particles);
    }
    let scene = serde_json::from_str::<Value>(&reiwand_upgrade::fill::whole(&scene.to_string())).unwrap();
    let mut files = vec![("preview.png".to_owned(), bytes("fixture:green-64x32.png"))];
    files.push(("sky.png".to_owned(), bytes("fixture:green-64x32.png")));
    files.push(("manifest.json".to_owned(), manifest.to_string().into_bytes()));
    if matches!(kind, "scene" | "assets") {
        files.push((entry.to_owned(), scene.to_string().into_bytes()));
    }
    let given = case["files"].as_object().into_iter().flatten();
    files.extend(given.map(|(path, text)| {
        let data = bytes(text.as_str().unwrap_or_default());
        let descriptor =
            path.ends_with(".json") && (path.starts_with("effects/") || path.starts_with("transitions/"));
        match (descriptor, String::from_utf8(data.clone())) {
            (true, Ok(text)) => (path.clone(), reiwand_upgrade::descriptors::completed(&text).into_bytes()),
            _ => (path.clone(), data),
        }
    }));
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

fn moved(p: &reiwand_scene::ParamDef) -> Value {
    let prefill = json!(p.prefill);
    match &prefill {
        Value::Bool(b) => json!(!b),
        Value::Number(n) => {
            let n = n.as_f64().unwrap_or_default();
            json!(
                p.min
                    .map(f64::from)
                    .filter(|m| *m != n)
                    .or(p.max.map(f64::from).filter(|m| *m != n))
                    .unwrap_or(n + 1.0)
            )
        }
        Value::Array(list) => {
            json!(list.iter().map(|x| x.as_f64().unwrap_or_default() * 0.5 + 0.25).collect::<Vec<_>>())
        }
        Value::String(s) if s.starts_with('#') => json!("#123456"),
        Value::String(s) if p.values.is_empty() => json!(if s == "add" { "multiply" } else { "add" }),
        Value::String(s) => json!(
            p.values
                .iter()
                .map(|e| json!(e)["value"].clone())
                .find(|v| v != &json!(s))
                .unwrap_or(prefill.clone())
        ),
        _ => prefill,
    }
}

fn whole(d: &reiwand_scene::Descriptor) -> Value {
    let params: Map<String, Value> = d
        .params
        .iter()
        .map(|p| (p.id.clone(), if p.kind.asset() { json!("sky") } else { moved(p) }))
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
