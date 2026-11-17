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
    if !upgrade(&mut doc) {
        return Ok(false);
    }
    let staged = path.with_extension("json.upgrading");
    let mut text = serde_json::to_string_pretty(&doc)?;
    text.push('\n');
    fs::write(&staged, text)?;
    fs::rename(staged, path)?;
    Ok(true)
}

pub fn upgrade(doc: &mut Value) -> bool {
    let mut done = false;
    if let Some(audio) = doc.get_mut("audio").and_then(Value::as_object_mut) {
        done |= fill(audio, &[("gain_release", json!(5))]);
    }
    if let Some(templates) = doc.get_mut("templates").and_then(Value::as_object_mut) {
        done |= templates.values_mut().fold(false, |any, one| layer(one) | any);
    }
    walk_value(doc) | done
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

fn layer(one: &mut Value) -> bool {
    one.as_object_mut().is_some_and(|map| fill(map, &[("blend_space", json!("canvas"))]))
}

fn system(map: &mut Map<String, Value>) -> bool {
    let mut done = false;
    let init = map.entry("init").or_insert_with(|| json!({}));
    if let Some(fields) = init.as_object_mut() {
        let tilt = json!({"angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]]});
        done |= fill(fields, &[("tilt", tilt)]);
    }
    let render = map.entry("render").or_insert_with(|| json!({}));
    if let Some(fields) = render.as_object_mut() {
        done |= fill(fields, &[("aspect", json!(1))]);
    }
    done
}

fn field(key: &str, fields: &mut Map<String, Value>) -> bool {
    match key {
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
    }
}

fn chroma_key(map: &mut Map<String, Value>) -> bool {
    if map.get("effect").and_then(Value::as_str) != Some("chroma_key") {
        return false;
    }
    let params = map.entry("params").or_insert_with(|| json!({}));
    let Some(params) = params.as_object_mut() else { return false };
    fill(params, &[("distance", json!(0)), ("keep", json!(0))])
}

fn walk_value(v: &mut Value) -> bool {
    match v {
        Value::Object(map) => {
            let mut done = chroma_key(map);
            if map.contains_key("emitters") && map.contains_key("capacity") {
                done |= system(map);
            }
            for (key, inner) in map.iter_mut() {
                if key == "layers"
                    && let Some(list) = inner.as_array_mut()
                {
                    done |= list.iter_mut().fold(false, |any, one| layer(one) | any);
                }
                if let Some(fields) = inner.as_object_mut() {
                    done |= field(key, fields);
                }
            }
            map.values_mut().fold(done, |any, inner| walk_value(inner) | any)
        }
        Value::Array(list) => list.iter_mut().fold(false, |any, inner| walk_value(inner) | any),
        _ => false,
    }
}
