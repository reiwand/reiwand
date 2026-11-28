use crate::{Format, complete as fill};
use serde_json::{Map, Value, json};

fn put(map: &mut Map<String, Value>, key: &str, value: Value) {
    map.entry(key).or_insert(value);
}

fn word(map: &mut Map<String, Value>, key: &str, word: &str) {
    if map.get(key).is_none_or(Value::is_null) {
        map.insert(key.to_owned(), json!(word));
    }
}

pub fn complete(doc: &mut Value) {
    fill(Format::Edit, doc);
    let Some(map) = doc.as_object_mut() else { return };
    put(map, "version", json!(reiwand_core::edit::VERSION));
    word(map, "fit", "general");
    if let Some(Value::Object(depth)) = map.get_mut("depth") {
        for key in ["strokes", "fills", "joins", "splits", "moves"] {
            put(depth, key, json!([]));
        }
    }
    if let Some(Value::Object(parallax)) = map.get_mut("parallax") {
        word(parallax, "fill", "auto");
        word(parallax, "layers", "auto");
    }
    for op in map.get_mut("ops").and_then(Value::as_array_mut).into_iter().flatten() {
        if let Some(op) = op.as_object_mut() {
            word(op, "mask", "");
        }
    }
}
