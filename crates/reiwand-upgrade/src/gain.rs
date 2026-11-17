use serde_json::{Value, json};

pub fn fixed(doc: &mut Value) -> bool {
    let Some(audio) = doc.get_mut("audio").and_then(Value::as_object_mut) else { return false };
    let Some(gain) = audio.remove("gain") else { return false };
    if gain == "fixed"
        && let Some(root) = doc.as_object_mut()
    {
        for (key, value) in root.iter_mut().filter(|(k, _)| k.as_str() != "audio") {
            absolute(key, value);
        }
    }
    true
}

fn absolute(key: &str, value: &mut Value) {
    match value {
        Value::Object(map) => {
            let heard = map.get("audio").is_some_and(|a| !a.is_object());
            if heard && key != "turbulent" && !map.contains_key("kind") {
                map.insert("kind".into(), json!("absolute"));
                map.insert("unit".into(), json!("db"));
            }
            for (k, v) in map.iter_mut() {
                absolute(k, v);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(|v| absolute(key, v)),
        _ => {}
    }
}
