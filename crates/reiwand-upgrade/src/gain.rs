use serde_json::{Value, json};

pub fn fixed(doc: &mut Value) -> bool {
    let Some(audio) = doc.get_mut("audio").and_then(Value::as_object_mut) else { return false };
    let Some(gain) = audio.remove("gain") else { return false };
    if gain == "fixed"
        && let Some(root) = doc.as_object_mut()
    {
        for (_, value) in root.iter_mut().filter(|(k, _)| k.as_str() != "audio") {
            absolute(value);
        }
    }
    true
}

fn absolute(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let band = map.get("audio").is_some_and(|a| !a.is_object());
            let input = map.get("input").and_then(Value::as_str).is_some_and(|i| i.starts_with("audio:"));
            if (band || input) && !map.contains_key("kind") {
                map.insert("kind".into(), json!("absolute"));
                map.insert("unit".into(), json!("db"));
            }
            for v in map.values_mut() {
                absolute(v);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(absolute),
        _ => {}
    }
}
