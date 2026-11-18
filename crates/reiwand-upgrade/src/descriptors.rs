mod shape;
mod write;

use serde_json::Value;
use std::fs;
use std::path::Path;

pub use write::text;

pub fn completed(text: &str) -> String {
    let Ok(mut doc) = serde_json::from_str::<Value>(text) else { return text.to_owned() };
    complete(&mut doc);
    doc.to_string()
}

pub fn complete(descriptor: &mut Value) -> bool {
    renamed(descriptor)
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
    changed | shape::descriptor(descriptor)
}

pub fn file(path: &Path) -> anyhow::Result<bool> {
    let mut doc: Value = serde_json::from_slice(&fs::read(path)?)?;
    let listing = doc.get("effects").is_some() && doc.get("id").is_none();
    let changed = if listing { shape::catalog(&mut doc) } else { renamed(&mut doc) };
    if changed {
        let staged = path.with_extension("json.upgrading");
        fs::write(&staged, text(&doc, if listing { 3 } else { 2 }))?;
        fs::rename(staged, path)?;
    }
    Ok(changed)
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
        fs::write(&staged, text(&descriptor, 2))?;
        fs::rename(staged, &path)?;
        changed += 1;
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::renamed;
    use serde_json::json;

    #[test]
    fn defaults_become_prefills_and_required_goes() {
        let mut d = json!({ "params": [{ "id": "a", "type": "float", "default": 1, "required": true }],
            "options": [{ "id": "o", "values": [1, 2], "default": 1 }] });
        assert!(renamed(&mut d));
        assert_eq!(d["params"][0]["prefill"], json!(1));
        assert!(d["params"][0].get("default").is_none() && d["params"][0].get("required").is_none());
        assert_eq!(d["options"][0]["prefill"], json!(1));
        assert!(!renamed(&mut d));
    }

    #[test]
    fn omitted_fields_are_written_with_the_value_applied_until_now() {
        let mut d = json!({ "id": "x", "params": [{ "id": "a", "type": "color", "prefill": "#fff" }],
            "passes": [{ "entry": "fs_x" }], "targets": { "t": {} } });
        assert!(renamed(&mut d));
        assert_eq!(d["passes"][0]["inputs"], json!(["previous"]));
        assert_eq!(d["passes"][0]["scale"], json!(1));
        assert_eq!(d["targets"]["t"], json!({ "scale": 1, "format": "rgba8" }));
        assert_eq!(d["params"][0]["min"], json!(null));
        assert_eq!(d["cost"], json!({ "taps": 1, "passes": 1, "scale": 1 }));
        assert_eq!(d["lighter"], json!(null));
        assert!(!renamed(&mut d));
    }
}
