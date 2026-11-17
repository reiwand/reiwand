use anyhow::Context;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

const SCHEMA: &str = "reiwand.scene/1";

fn fill(layers: &mut Value) -> usize {
    let Some(list) = layers.as_array_mut() else { return 0 };
    let mut added = 0;
    for layer in list {
        let Some(fields) = layer.as_object_mut() else { continue };
        if !fields.contains_key("blend_space") {
            fields.insert("blend_space".into(), json!("canvas"));
            added += 1;
        }
        if let Some(children) = fields.get_mut("layers") {
            added += fill(children);
        }
    }
    added
}

pub fn upgrade(text: &str) -> anyhow::Result<Option<(String, usize)>> {
    let Ok(mut scene) = serde_json::from_str::<Value>(text) else { return Ok(None) };
    if scene["schema"] != SCHEMA {
        return Ok(None);
    }
    let added = fill(&mut scene["layers"])
        + scene["templates"].as_object_mut().map_or(0, |t| t.values_mut().map(fill_one).sum());
    let mut out = serde_json::to_string_pretty(&scene)?;
    out.push('\n');
    Ok(Some((out, added)))
}

fn fill_one(layer: &mut Value) -> usize {
    let mut one = Value::Array(vec![layer.take()]);
    let added = fill(&mut one);
    *layer = one[0].take();
    added
}

fn files(path: &Path, out: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    let meta = fs::symlink_metadata(path).with_context(|| path.display().to_string())?;
    if meta.is_dir() {
        let mut names: Vec<PathBuf> = fs::read_dir(path)?.filter_map(|e| Some(e.ok()?.path())).collect();
        names.sort();
        names.iter().try_for_each(|p| files(p, out))
    } else if meta.is_file() && path.extension().is_some_and(|e| e == "json") {
        out.push(path.to_path_buf());
        Ok(())
    } else {
        Ok(())
    }
}

pub fn run(roots: &[PathBuf]) -> anyhow::Result<String> {
    let mut found = Vec::new();
    for root in roots {
        files(root, &mut found)?;
    }
    let (mut scenes, mut layers) = (0, 0);
    for path in found {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let Some((out, added)) = upgrade(&text)? else { continue };
        fs::write(&path, out).with_context(|| path.display().to_string())?;
        println!("{}: {added} layers", path.display());
        (scenes, layers) = (scenes + 1, layers + added);
    }
    Ok(match scenes {
        0 => "nothing to upgrade".into(),
        _ => format!("upgraded {scenes} scenes, {layers} layers now say blend_space canvas"),
    })
}
