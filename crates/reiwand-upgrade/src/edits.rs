use crate::fill::{Format, complete};
use crate::sink::Sink;
use anyhow::Context;
use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;

pub const FORMAT: i64 = 3;

fn put(map: &mut Map<String, Value>, key: &str, value: Value) {
    map.entry(key).or_insert(value);
}

fn word(map: &mut Map<String, Value>, key: &str, word: &str) {
    match map.get(key) {
        None | Some(Value::Null) => {
            map.insert(key.to_owned(), json!(word));
        }
        Some(_) => {}
    }
}

fn blocks(doc: &mut Map<String, Value>) {
    word(doc, "fit", "general");
    if let Some(Value::Object(depth)) = doc.get_mut("depth") {
        for key in ["strokes", "fills", "joins", "splits", "moves"] {
            put(depth, key, json!([]));
        }
    }
    if let Some(Value::Object(parallax)) = doc.get_mut("parallax") {
        word(parallax, "fill", "auto");
        word(parallax, "layers", "auto");
    }
}

fn ops(doc: &mut Map<String, Value>) {
    for op in doc.get_mut("ops").and_then(Value::as_array_mut).into_iter().flatten() {
        if let Some(op) = op.as_object_mut() {
            word(op, "mask", "");
        }
    }
}

pub fn edit(text: &str) -> anyhow::Result<String> {
    let mut doc: Value = serde_json::from_str(text).context("the edit is not valid JSON")?;
    if doc.get("version").and_then(Value::as_i64) == Some(FORMAT) {
        return Ok(text.to_owned());
    }
    complete(Format::Edit, &mut doc);
    let map = doc.as_object_mut().context("the edit is not an object")?;
    map.insert("version".to_owned(), json!(FORMAT));
    blocks(map);
    ops(map);
    Ok(serde_json::to_string_pretty(&doc)? + "\n")
}

pub fn file(path: &Path, sink: &mut Sink) -> anyhow::Result<bool> {
    sink.text(path, edit)
}

pub fn folder(dir: &Path, sink: &mut Sink) -> anyhow::Result<usize> {
    let mut done = 0;
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            done += usize::from(file(&path, sink)?);
        }
    }
    Ok(done)
}

pub fn sidecars(dir: &Path, sink: &mut Sink) -> anyhow::Result<usize> {
    let mut done = 0;
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            done += sidecars(&path, sink)?;
        } else if path.file_name().is_some_and(|n| n.to_string_lossy().ends_with(".reiwand.json")) {
            done += usize::from(file(&path, sink)?);
        }
    }
    Ok(done)
}
