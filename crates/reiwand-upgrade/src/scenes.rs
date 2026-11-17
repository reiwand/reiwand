use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;

pub fn run(root: &Path) -> anyhow::Result<usize> {
    let mut changed = 0;
    walk(root, &mut changed)?;
    Ok(changed)
}

fn walk(dir: &Path, changed: &mut usize) -> anyhow::Result<()> {
    let Ok(items) = fs::read_dir(dir) else { return Ok(()) };
    for item in items.flatten() {
        let path = item.path();
        if path.is_dir() {
            walk(&path, changed)?;
        } else if path.file_name().is_some_and(|n| n == "scene.json") {
            *changed += usize::from(scene(&path)?);
        }
    }
    Ok(())
}

fn scene(path: &Path) -> anyhow::Result<bool> {
    let Ok(mut doc) = serde_json::from_slice::<Value>(&fs::read(path)?) else { return Ok(false) };
    if !give(&mut doc) {
        return Ok(false);
    }
    let staged = path.with_extension("json.upgrading");
    fs::write(&staged, serde_json::to_vec_pretty(&doc)?)?;
    fs::rename(staged, path)?;
    Ok(true)
}

fn fill(map: &mut Map<String, Value>, fields: &[(&str, Value)]) -> bool {
    let mut done = false;
    for (name, value) in fields {
        if !map.contains_key(*name) {
            map.insert((*name).into(), value.clone());
            done = true;
        }
    }
    done
}

fn give(v: &mut Value) -> bool {
    match v {
        Value::Object(map) => {
            let mut done = false;
            for (key, inner) in map.iter_mut() {
                let Some(fields) = inner.as_object_mut() else { continue };
                done |= match key.as_str() {
                    "vortex" | "around" => fill(fields, &[("axis", json!([0, 0, 1]))]),
                    "oscillate" => fill(fields, &[("from", json!("top"))]),
                    "stretch" => fill(
                        fields,
                        &[
                            ("mode", json!("add")),
                            ("depth", json!("flat")),
                            ("time", json!(0)),
                            ("min", json!(0)),
                            ("max", json!(10000)),
                        ],
                    ),
                    _ => false,
                };
            }
            for inner in map.values_mut() {
                done |= give(inner);
            }
            done
        }
        Value::Array(list) => list.iter_mut().fold(false, |done, inner| give(inner) | done),
        _ => false,
    }
}
