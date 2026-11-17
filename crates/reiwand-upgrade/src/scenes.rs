use serde_json::{Value, json};
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

fn give(v: &mut Value) -> bool {
    match v {
        Value::Object(map) => {
            let mut done = false;
            if matches!(map.get("kind").and_then(Value::as_str), Some("vortex" | "around"))
                && !map.contains_key("axis")
            {
                map.insert("axis".into(), json!([0, 0, 1]));
                done = true;
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
