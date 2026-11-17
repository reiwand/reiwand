use anyhow::Context;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

const FILE: &str = "scene.json";

pub fn run(roots: &[PathBuf]) -> anyhow::Result<Vec<PathBuf>> {
    let mut changed = Vec::new();
    for root in roots {
        walk(root, &mut changed)?;
    }
    Ok(changed)
}

fn walk(dir: &Path, changed: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    for item in fs::read_dir(dir).with_context(|| dir.display().to_string())?.flatten() {
        let path = item.path();
        if path.is_dir() {
            walk(&path, changed)?;
        } else if path.file_name().is_some_and(|n| n == FILE) && rewrite(&path)? {
            changed.extend([path]);
        }
    }
    Ok(())
}

fn rewrite(path: &Path) -> anyhow::Result<bool> {
    let text = fs::read_to_string(path).with_context(|| path.display().to_string())?;
    let mut doc: Value = serde_json::from_str(&text).with_context(|| path.display().to_string())?;
    if !chroma_keys(&mut doc) {
        return Ok(false);
    }
    let staged = path.with_extension("json.upgrading");
    fs::write(&staged, format!("{}\n", serde_json::to_string_pretty(&doc)?))?;
    fs::rename(staged, path)?;
    Ok(true)
}

fn chroma_keys(value: &mut Value) -> bool {
    match value {
        Value::Array(list) => list.iter_mut().fold(false, |any, v| chroma_keys(v) | any),
        Value::Object(map) => {
            let mut any = map.values_mut().fold(false, |any, v| chroma_keys(v) | any);
            if map.get("effect").and_then(Value::as_str) == Some("chroma_key") {
                let params = map.entry("params").or_insert_with(|| json!({}));
                if let Some(params) = params.as_object_mut() {
                    for (id, value) in [("distance", json!(0)), ("keep", json!(0))] {
                        any |= params.get(id).is_none();
                        params.entry(id).or_insert(value);
                    }
                }
            }
            any
        }
        _ => false,
    }
}
