use serde_json::{Map, Value, json};

pub(super) fn value(def: &str, name: &str) -> Option<Value> {
    Some(match (def, name) {
        ("Layer", "blend_space") => json!("canvas"),
        ("Oscillate", "from") => json!("top"),
        ("Stretch", "mode") => json!("add"),
        ("Stretch", "depth") => json!("flat"),
        ("Stretch", "time") => json!(0),
        ("Stretch", "min") => json!(0),
        ("Stretch", "max") => json!(10000),
        ("Hearing", "gain_release") => json!(5),
        ("vortex" | "around", "axis") => json!([0, 0, 1]),
        _ => return None,
    })
}

pub(super) fn corrected(def: &str, name: &str) -> Option<Value> {
    Some(match (def, name) {
        ("Layer", "parallax") => json!([0, 0]),
        ("Glide", "offset") => json!([0, 0]),
        ("System", "space") => json!("local"),
        ("Manifest", "description") => json!(""),
        ("Canvas", "clear") => json!(true),
        ("Layer", "solid") => json!(false),
        ("Property", "mode") => json!("one"),
        ("Property", "max_files") => json!(1000),
        _ => return None,
    })
}

pub(super) fn contextual(def: &str, name: &str) -> bool {
    matches!((def, name), ("Layer", "blend_space")) || heard_value(def, name).is_some()
}

pub(super) fn heard_value(def: &str, name: &str) -> Option<Value> {
    if !(def == "Remap" || def == "Turbulent" || def.starts_with("Binding_for_")) {
        return None;
    }
    match name {
        "kind" => Some(json!("relative")),
        "unit" => Some(json!("db")),
        _ => None,
    }
}

pub(super) fn heard(def: &str, map: &Map<String, Value>) -> Vec<(&'static str, Value)> {
    let listens = match def {
        "Remap" => map.get("input").and_then(Value::as_str).is_some_and(|i| i.starts_with("audio:")),
        "Turbulent" => map.get("audio").is_some_and(|a| !a.is_null()),
        _ => {
            def.starts_with("Binding_for_")
                && map.get("audio").is_some_and(|a| !a.is_object() && !a.is_null())
        }
    };
    if !listens {
        return Vec::new();
    }
    ["kind", "unit"]
        .into_iter()
        .filter(|name| map.get(*name).is_none_or(Value::is_null))
        .filter_map(|name| Some((name, heard_value(def, name)?)))
        .collect()
}
