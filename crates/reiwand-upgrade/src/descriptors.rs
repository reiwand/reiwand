use serde_json::Value;
use std::fs;
use std::path::Path;

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
    use super::renamed;
    use serde_json::json;

    #[test]
    fn defaults_become_prefills_and_required_goes() {
        let mut d = json!({ "params": [{ "id": "a", "type": "float", "default": 1, "required": true }],
            "options": [{ "id": "o", "values": [1, 2], "default": 1 }] });
        assert!(renamed(&mut d));
        assert_eq!(d["params"][0], json!({ "id": "a", "type": "float", "prefill": 1 }));
        assert_eq!(d["options"][0]["prefill"], json!(1));
        assert!(!renamed(&mut d));
    }
}
