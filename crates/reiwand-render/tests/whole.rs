use serde_json::{Value, json};

pub fn whole(text: &str) -> String {
    let Ok(mut doc) = serde_json::from_str::<Value>(text) else { return text.to_owned() };
    let done = reiwand_upgrade::fill::whole(&doc.to_string());
    let Ok(mut doc) = serde_json::from_str::<Value>(&done) else { return done };
    settle(&mut doc);
    doc.to_string()
}

fn settle(value: &mut Value) {
    match value {
        Value::Array(items) => items.iter_mut().for_each(settle),
        Value::Object(map) => {
            if let (Some(Value::String(kind)), true) = (map.get("type").cloned(), map.contains_key("id")) {
                let table: Value = serde_json::from_str(include_str!("../../reiwand-scene/src/prefill.json")).unwrap();
                let nulls: Vec<String> = map.iter().filter(|(_, v)| v.is_null()).map(|(k, _)| k.clone()).collect();
                for key in nulls {
                    let row = table.get(format!("Layer[{kind}].{key}")).or_else(|| table.get(format!("Layer.{key}")));
                    if let Some(value) = row.filter(|v| !v.is_null()) {
                        map.insert(key, value.clone());
                    }
                }
            }
            if let Some(Value::Object(t)) = map.get_mut("transform")
                && t.get("origin").is_some_and(Value::is_null)
            {
                t.insert("origin".into(), json!("anchor"));
            }
            if map.get("playback").is_some_and(Value::is_null) {
                map.insert("playback".into(), reiwand_scene::prefill::of("Playback", json!({})));
            }
            if let Some(Value::Object(m)) = map.get_mut("mask") {
                let texture = m.get("texture").is_some_and(|t| !t.is_null());
                if m.get("channel").is_some_and(Value::is_null) {
                    m.insert("channel".into(), json!(if texture { "luminance" } else { "a" }));
                }
                if m.get("strength").is_some_and(Value::is_null) {
                    m.insert("strength".into(), json!(1.0));
                }
            }
            map.values_mut().for_each(settle);
        }
        _ => {}
    }
}
