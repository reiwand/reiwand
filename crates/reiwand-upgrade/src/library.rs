use crate::sink::Sink;
use anyhow::{Context, bail};
use rusqlite::{Connection, params};
use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;

pub const SCHEMA: u32 = 9;

const OUTPUT: [&str; 7] = ["item", "path", "playlist", "fit", "background", "preset", "filter"];

fn put(map: &mut Map<String, Value>, key: &str, value: Value) {
    map.entry(key).or_insert(value);
}

pub fn rules(text: &str) -> anyhow::Result<String> {
    let mut doc: Value = serde_json::from_str(text)?;
    let map = doc.as_object_mut().context("rules are not an object")?;
    for (key, value) in [
        ("order", json!("sequential")),
        ("interval", json!("30m")),
        ("at", json!([])),
        ("days", json!([])),
        ("synced", json!(false)),
        ("resume", json!(true)),
        ("advance_while_paused", json!(false)),
        ("skip_videos_on_battery", json!(false)),
        ("transition_ms", Value::Null),
    ] {
        put(map, key, value);
    }
    Ok(doc.to_string())
}

pub fn outputs(text: &str) -> anyhow::Result<String> {
    let mut doc: Value = serde_json::from_str(text)?;
    for one in doc.as_object_mut().context("outputs are not an object")?.values_mut() {
        let one = one.as_object_mut().context("an output is not an object")?;
        OUTPUT.iter().for_each(|key| put(one, key, Value::Null));
    }
    Ok(doc.to_string())
}

pub fn auto(text: &str) -> anyhow::Result<String> {
    let mut doc: Value = serde_json::from_str(text)?;
    let map = doc.as_object_mut().context("auto is not an object")?;
    put(map, "outputs", Value::Null);
    put(map, "on_battery", Value::Null);
    Ok(doc.to_string())
}

pub fn preset(text: &str) -> anyhow::Result<String> {
    let mut doc: Value = serde_json::from_str(text)?;
    let map = doc.as_object_mut().context("a preset is not an object")?;
    match map.contains_key("package") {
        true => put(map, "values", json!({})),
        false => {
            put(map, "fit", Value::Null);
            put(map, "background", Value::Null);
        }
    }
    Ok(doc.to_string())
}

pub fn state(text: &str) -> anyhow::Result<String> {
    let mut doc: Value = serde_json::from_str(text)?;
    let map = doc.as_object_mut().context("a rotation state is not an object")?;
    put(map, "anchor", Value::Null);
    put(map, "since", Value::Null);
    for key in ["behind", "ahead"] {
        put(map, key, json!([]));
        for shown in map.get_mut(key).and_then(Value::as_array_mut).into_iter().flatten() {
            if let Some(shown) = shown.as_object_mut() {
                put(shown, "anchor", Value::Null);
            }
        }
    }
    Ok(doc.to_string())
}

fn rewrite(
    db: &Connection,
    table: &str,
    column: &str,
    key: &str,
    fix: fn(&str) -> anyhow::Result<String>,
) -> anyhow::Result<()> {
    let mut read = db.prepare(&format!("SELECT {key}, {column} FROM {table}"))?;
    let found: Vec<(rusqlite::types::Value, String)> =
        read.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<Result<_, _>>()?;
    for (id, text) in found {
        let fixed = fix(&text).with_context(|| format!("{table}.{column} of {id:?}"))?;
        db.execute(&format!("UPDATE {table} SET {column} = ?1 WHERE {key} = ?2"), params![fixed, id])?;
    }
    Ok(())
}

pub fn convert(db: &mut Connection) -> anyhow::Result<()> {
    let tx = db.transaction()?;
    rewrite(&tx, "playlists", "rules", "id", rules)?;
    rewrite(&tx, "profiles", "outputs", "id", outputs)?;
    rewrite(&tx, "profiles", "auto", "id", auto)?;
    rewrite(&tx, "presets", "value", "id", preset)?;
    rewrite(&tx, "rotation", "state", "output", state)?;
    tx.pragma_update(None, "user_version", SCHEMA)?;
    Ok(tx.commit()?)
}

pub fn run(data: &Path, sink: &mut Sink) -> anyhow::Result<bool> {
    let path = data.join("library.db");
    if !path.is_file() {
        return Ok(false);
    }
    let original = Connection::open_with_flags(&path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let found: u32 = original.pragma_query_value(None, "user_version", |r| r.get(0))?;
    if found == SCHEMA || found == 0 {
        return Ok(false);
    }
    if found != SCHEMA - 1 {
        bail!("{}: schema {found} cannot be converted; nothing was changed", path.display());
    }
    if sink.is_check() {
        sink.note(&path);
        return Ok(true);
    }
    let staged = data.join("library.db.upgrading");
    let _ = fs::remove_file(&staged);
    original.execute("VACUUM INTO ?1", [staged.to_string_lossy().as_ref()])?;
    let mut copy = Connection::open(&staged)?;
    convert(&mut copy).context("cannot convert the library database")?;
    copy.close().map_err(|(_, e)| e)?;
    original.close().map_err(|(_, e)| e)?;
    for suffix in ["", "-wal", "-shm"] {
        sink.keep(&data.join(format!("library.db{suffix}")))?;
    }
    for suffix in ["-wal", "-shm"] {
        let _ = fs::remove_file(data.join(format!("library.db{suffix}")));
    }
    fs::rename(staged, path)?;
    Ok(true)
}
