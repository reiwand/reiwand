use crate::ops::Op;
use serde_json::{Map, Value, json};

const LISTS: [&str; 7] = ["clips", "animations", "ik", "physics", "colliders", "rules", "hit"];
const TABLES: [&str; 2] = ["shapes", "expressions"];

fn blank(key: &str) -> Value {
    if TABLES.contains(&key) { json!({}) } else { json!([]) }
}

fn put(map: &mut Map<String, Value>, rows: &[(&str, Value)]) {
    for (key, value) in rows {
        map.entry(*key).or_insert_with(|| value.clone());
    }
}

fn track(keys: &mut Value) {
    if let Some(map) = keys.as_object_mut() {
        let none = json!([]);
        put(map, &[("rotation", none.clone()), ("position", none.clone()), ("scale", none.clone())]);
        put(map, &[("angles", none.clone()), ("shift", none)]);
    }
}

fn clip(row: &mut Value) {
    let Some(map) = row.as_object_mut() else { return };
    let (none, table) = (json!([]), json!({}));
    put(map, &[("animation", Value::Null), ("from", json!(0.0)), ("to", Value::Null)]);
    put(map, &[("loop", json!("repeat")), ("events", none)]);
    put(map, &[("bones", table.clone()), ("shapes", table.clone()), ("order", table)]);
    map.get_mut("bones")
        .and_then(Value::as_object_mut)
        .into_iter()
        .flat_map(|b| b.values_mut())
        .for_each(track);
}

fn entry(key: &str, row: &mut Value) {
    if key == "clips" {
        return clip(row);
    }
    let Some(map) = row.as_object_mut() else { return };
    let rows: Vec<(&str, Value)> = match key {
        "animations" => vec![
            ("weight", json!(1.0)),
            ("rate", json!(1.0)),
            ("additive", json!(false)),
            ("blend_in", json!(0.0)),
            ("blend_out", json!(0.0)),
            ("enabled", json!(true)),
            ("bones", json!([])),
        ],
        "ik" => vec![("bend", json!("clockwise")), ("limits", json!({})), ("weight", json!(1.0))],
        "physics" => vec![
            ("mode", json!("spring")),
            ("stiffness", json!(0.5)),
            ("damping", json!(0.5)),
            ("inertia", json!(0.5)),
            ("gravity", json!(0.0)),
            ("wind", json!(0.0)),
            ("rotation", Value::Null),
            ("translation", Value::Null),
        ],
        "colliders" => vec![("shape", json!("capsule")), ("offset", json!([0.0, 0.0, 0.0]))],
        "rules" => vec![("weight", json!(1.0))],
        _ => Vec::new(),
    };
    put(map, &rows);
}

fn list(key: &str, rows: &mut Value) {
    rows.as_array_mut().into_iter().flatten().for_each(|row| entry(key, row));
}

fn layer(value: &mut Value) {
    let Some(map) = value.as_object_mut() else { return };
    if matches!(map.get("type").and_then(Value::as_str), Some("mesh" | "model")) {
        for key in LISTS.into_iter().chain(TABLES) {
            map.entry(key).or_insert_with(|| blank(key));
        }
        for key in LISTS {
            map.get_mut(key).into_iter().for_each(|rows| list(key, rows));
        }
    }
    layers(map.get_mut("layers"));
}

fn layers(value: Option<&mut Value>) {
    value.and_then(Value::as_array_mut).into_iter().flatten().for_each(layer);
}

fn is_row(part: &str) -> bool {
    part == "-" || part.parse::<usize>().is_ok()
}

fn value_at(path: &str, value: &mut Value) {
    let parts: Vec<&str> = path.split('/').collect();
    let (last, before) = (parts[parts.len() - 1], parts.get(parts.len().wrapping_sub(2)).copied());
    match (before, last) {
        (_, "layers") => layers(Some(value)),
        (Some("layers"), row) if is_row(row) => layer(value),
        (Some(key), "bones") if key.parse::<usize>().is_ok() => {
            value.as_object_mut().into_iter().flat_map(|b| b.values_mut()).for_each(track);
        }
        (_, key) if LISTS.contains(&key) => list(key, value),
        (Some(key), row) if LISTS.contains(&key) && is_row(row) => entry(key, value),
        _ => {}
    }
}

fn parent_is_rigged(doc: &Value, path: &str) -> bool {
    let Some((parent, key)) = path.rsplit_once('/') else { return false };
    let rigged = doc.pointer(parent).and_then(|l| l["type"].as_str());
    path.starts_with("/scene/")
        && matches!(rigged, Some("mesh" | "model"))
        && (LISTS.contains(&key) || TABLES.contains(&key))
}

pub(crate) fn complete_all(doc: &Value, ops: Vec<Op>) -> Vec<Op> {
    ops.into_iter()
        .map(|op| match op {
            Op::Add { path, mut value } => {
                value_at(&path, &mut value);
                Op::Add { path, value }
            }
            Op::Replace { path, mut value } => {
                value_at(&path, &mut value);
                Op::Replace { path, value }
            }
            Op::Remove { path } if parent_is_rigged(doc, &path) => {
                let key = path.rsplit('/').next().unwrap_or_default();
                Op::Replace { path: path.clone(), value: blank(key) }
            }
            other => other,
        })
        .collect()
}
