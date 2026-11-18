use crate::ops::Op;
use serde_json::{Value, json};

pub(crate) fn fill(value: &mut Value) {
    match value {
        Value::Array(list) => list.iter_mut().for_each(fill),
        Value::Object(map) => {
            map.values_mut().for_each(fill);
            let keyed = map.get("keys").is_some_and(Value::is_array)
                && map.get("timeline").is_none_or(Value::is_null)
                && map.get("loop").is_none_or(Value::is_null);
            if keyed {
                map.insert("loop".into(), json!("once"));
            }
            let indexed = map.get("audio").is_some_and(|a| a.is_u64() || a.is_array());
            if indexed && map.get("of").is_some_and(Value::is_null) {
                map.insert("of".into(), reiwand_scene::prefill::offer("audio", "", json!({}))["of"].clone());
            }
        }
        _ => {}
    }
}

fn solid(value: &Value) -> bool {
    match value {
        Value::Array(list) => list.iter().any(solid),
        Value::Object(map) => {
            let kind = map.get("type").and_then(Value::as_str);
            matches!(kind, Some("mesh" | "model"))
                || (kind == Some("group") && map.get("space").and_then(Value::as_str) == Some("3d"))
                || map.values().any(solid)
        }
        _ => false,
    }
}

fn written(op: &Op) -> Option<(&str, &Value)> {
    match op {
        Op::Add { path, value } | Op::Replace { path, value } => Some((path.as_str(), value)),
        _ => None,
    }
}

fn space_3d(op: &Op) -> bool {
    written(op).is_some_and(|(path, value)| path.ends_with("/space") && value == "3d")
}

fn clears(op: &Op) -> bool {
    match op {
        Op::Remove { path } => path == "/scene/camera3d",
        other => written(other).is_some_and(|(path, value)| path == "/scene/camera3d" && !value.is_object()),
    }
}

pub(crate) fn camera(doc: &Value, ops: &[Op]) -> Option<Value> {
    let scene = doc.pointer("/scene")?;
    let has = doc.pointer("/scene/camera3d").is_some_and(Value::is_object);
    let sets =
        ops.iter().filter_map(written).any(|(path, value)| path == "/scene/camera3d" && value.is_object());
    let places = ops.iter().any(space_3d)
        || ops
            .iter()
            .filter_map(written)
            .any(|(path, value)| path.starts_with("/scene") && path != "/scene/camera3d" && solid(value));
    let cleared = ops.iter().any(clears) && solid(scene);
    ((places && !has || cleared) && !sets).then(crate::stage3d::camera_suggested)
}
