use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;

fn put(object: &mut Map<String, Value>, key: &str, value: Value) -> bool {
    let absent = !object.contains_key(key);
    if absent {
        object.insert(key.to_owned(), value);
    }
    absent
}

fn fill_all(items: Option<&mut Value>, fill: fn(&mut Map<String, Value>) -> bool) -> bool {
    let list = items.and_then(Value::as_array_mut).into_iter().flatten();
    list.filter_map(Value::as_object_mut).fold(false, |changed, item| fill(item) | changed)
}

fn param(item: &mut Map<String, Value>) -> bool {
    let mut changed = false;
    for key in ["min", "max", "uses", "unit", "channels", "paint", "yields_to", "waits_for", "tool"] {
        changed |= put(item, key, Value::Null);
    }
    for key in ["tools", "values"] {
        changed |= put(item, key, json!([]));
    }
    for key in ["label", "help"] {
        changed |= put(item, key, json!({}));
    }
    changed
}

fn pass(item: &mut Map<String, Value>) -> bool {
    let mut changed = put(item, "entry", json!("")) | put(item, "scale", json!(1.0));
    changed |= put(item, "inputs", json!(["previous"])) | put(item, "output", json!("next"));
    for key in ["target", "copy", "blend", "vertex", "grid", "swap"] {
        changed |= put(item, key, Value::Null);
    }
    changed
}

fn cost(item: &mut Map<String, Value>) -> bool {
    put(item, "taps", json!(1.0)) | put(item, "passes", json!(1)) | put(item, "scale", json!(1.0))
}

fn cost_of(item: &mut Map<String, Value>, key: &str) -> bool {
    let missing = put(item, key, json!({}));
    missing | item.get_mut(key).and_then(Value::as_object_mut).is_some_and(cost)
}

fn each<'a>(item: &'a mut Map<String, Value>, key: &str) -> impl Iterator<Item = &'a mut Map<String, Value>> {
    item.get_mut(key).and_then(Value::as_object_mut).into_iter().flat_map(|m| m.values_mut()).filter_map(Value::as_object_mut)
}

fn descriptor_fields(item: &mut Map<String, Value>) -> bool {
    let mut changed = false;
    for key in ["category", "description", "fuse", "reach", "min_fps", "velocity", "state", "solve", "pace", "lighter"] {
        changed |= put(item, key, Value::Null);
    }
    for key in ["uses", "audio", "params", "passes", "bounded", "options"] {
        changed |= put(item, key, json!([]));
    }
    for key in ["name", "quality", "targets"] {
        changed |= put(item, key, json!({}));
    }
    changed |= put(item, "mask_mode", json!("mix")) | put(item, "stage", json!("layer"));
    changed |= put(item, "compose", json!(false)) | cost_of(item, "cost");
    changed |= fill_all(item.get_mut("params"), param) | fill_all(item.get_mut("passes"), pass);
    changed |= fill_all(item.get_mut("audio"), listen) | fill_all(item.get_mut("options"), variant);
    for tune in each(item, "quality") {
        changed |= put(tune, "params", json!({})) | put(tune, "scale", Value::Null) | put(tune, "freeze", json!(false));
    }
    for target in each(item, "targets") {
        changed |= put(target, "scale", json!(1.0)) | put(target, "format", json!("rgba8"));
    }
    if let Some(state) = item.get_mut("state").and_then(Value::as_object_mut) {
        changed |= put(state, "scale", json!(1.0)) | put(state, "settle", json!(0.0)) | put(state, "fade", Value::Null);
    }
    if let Some(light) = item.get_mut("lighter").and_then(Value::as_object_mut) {
        changed |= put(light, "passes", json!([])) | put(light, "up_to", Value::Null) | cost_of(light, "cost");
        changed |= fill_all(light.get_mut("passes"), pass);
    }
    changed
}

fn listen(item: &mut Map<String, Value>) -> bool {
    ["window", "reference", "hold_silence"].iter().fold(false, |changed, key| put(item, key, Value::Null) | changed)
}

fn variant(item: &mut Map<String, Value>) -> bool {
    put(item, "when_bound", Value::Null)
}

fn renamed(descriptor: &mut Value) -> bool {
    let mut changed = false;
    for key in ["params", "options"] {
        let list = descriptor.get_mut(key).and_then(Value::as_array_mut);
        for item in list.into_iter().flatten().filter_map(Value::as_object_mut) {
            if let Some(value) = item.remove("default") {
                item.entry("prefill").or_insert(value);
                changed = true;
            }
            changed |= item.remove("required").is_some();
        }
    }
    if let Some(item) = descriptor.as_object_mut() {
        changed |= descriptor_fields(item);
    }
    changed
}

pub fn rewrite(dir: &Path, doc: &Value) -> anyhow::Result<usize> {
    let mut changed = 0;
    let files = doc.get("effects").and_then(Value::as_object);
    for entry in files.into_iter().flat_map(|f| f.values()) {
        let Some(path) = entry.get("descriptor").and_then(Value::as_str) else { continue };
        let path = dir.join(path);
        let Ok(bytes) = fs::read(&path) else { continue };
        let Ok(mut descriptor) = serde_json::from_slice::<Value>(&bytes) else { continue };
        if !renamed(&mut descriptor) {
            continue;
        }
        let staged = path.with_extension("json.upgrading");
        fs::write(&staged, super::scenes::text(&descriptor)?)?;
        fs::rename(staged, &path)?;
        changed += 1;
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::renamed as descriptor;
    use serde_json::json;

    #[test]
    fn defaults_become_prefills_and_every_value_is_written() {
        let mut d = json!({ "id": "x", "params": [{ "id": "a", "type": "float", "default": 1, "required": true }],
            "options": [{ "id": "o", "values": [1, 2], "default": 1 }] });
        assert!(descriptor(&mut d));
        assert_eq!(d["params"][0]["prefill"], json!(1));
        assert_eq!(d["params"][0]["min"], json!(null));
        assert_eq!(d["params"][0]["tools"], json!([]));
        assert_eq!(d["options"][0]["when_bound"], json!(null));
        assert_eq!(d["cost"], json!({ "taps": 1.0, "passes": 1, "scale": 1.0 }));
        assert_eq!(d["stage"], json!("layer"));
        assert!(!descriptor(&mut d));
    }
}
