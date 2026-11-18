use crate::sink::Sink;
use anyhow::Context;
use serde_json::Value;
use std::fs;
use std::path::Path;

const NAME: &str = "Example wallpapers";
const RECORD: &str = "examples-repo.json";

fn rewrite(value: &mut Value, old: &str, new: &str) -> bool {
    match value {
        Value::Object(map) => {
            let mut changed = false;
            if map.get("repo_id").and_then(Value::as_str) == Some(old) {
                map.insert("repo_id".into(), new.into());
                changed = true;
            }
            for inner in map.values_mut() {
                changed |= rewrite(inner, old, new);
            }
            changed
        }
        Value::Array(list) => {
            let mut changed = false;
            for item in list {
                changed |= rewrite(item, old, new);
            }
            changed
        }
        _ => false,
    }
}

fn rewritten(data: &Path, file: &str, old: &str, new: &str, sink: &mut Sink) -> anyhow::Result<()> {
    let path = data.join(file);
    let Ok(text) = fs::read_to_string(&path) else { return Ok(()) };
    let mut doc: Value = serde_json::from_str(&text).with_context(|| format!("{file} is not valid JSON"))?;
    if rewrite(&mut doc, old, new) {
        sink.save(&path, (serde_json::to_string_pretty(&doc)? + "\n").as_bytes())?;
    }
    Ok(())
}

fn record(data: &Path, sink: &mut Sink) -> anyhow::Result<()> {
    let path = data.join(RECORD);
    let Ok(text) = fs::read_to_string(&path) else { return Ok(()) };
    let doc: Value = serde_json::from_str(&text).with_context(|| format!("{RECORD} is not valid JSON"))?;
    if doc.get("stamp").is_none() && doc.get("format").is_none() {
        sink.save(&path, crate::stored::wrap(&text)?.as_bytes())?;
    }
    Ok(())
}

fn repoint(data: &Path, old: &str, new: &str, sink: &mut Sink) -> anyhow::Result<()> {
    rewritten(data, "installed.json", old, new, sink)?;
    let path = data.join(RECORD);
    let Ok(text) = fs::read_to_string(&path) else { return Ok(()) };
    let mut doc: Value = serde_json::from_str(&text)?;
    let target = if doc.get("content").is_some() { &mut doc["content"] } else { &mut doc };
    if let Some(map) = target.as_object_mut().filter(|m| m.contains_key("stamp")) {
        map.insert("stamp".into(), 0.into());
    }
    rewrite(&mut doc, old, new);
    sink.save(&path, (serde_json::to_string_pretty(&doc)? + "\n").as_bytes())
}

pub(crate) fn repaired(data: &Path, sink: &mut Sink) -> anyhow::Result<()> {
    record(data, sink)?;
    let (repo, keys) = (data.join("examples-repo"), data.join("examples-keys"));
    let root = repo.join("root/1.json");
    let Ok(bytes) = fs::read(&root) else { return Ok(()) };
    if reiwand_format::status(&repo).is_ok() {
        return Ok(());
    }
    sink.note(&root);
    if sink.is_check() {
        return Ok(());
    }
    let name = serde_json::from_slice::<Value>(&bytes)
        .ok()
        .and_then(|v| v.get("name").and_then(Value::as_str).map(str::to_owned))
        .unwrap_or_else(|| NAME.to_owned());
    let old = reiwand_format::sha256(&bytes[..])?;
    fs::create_dir_all(sink.copies())?;
    for key in fs::read_dir(&keys).into_iter().flatten().flatten() {
        sink.keep(&key.path())?;
    }
    fs::rename(&repo, sink.copies().join("examples-repo"))?;
    let now = std::time::UNIX_EPOCH.elapsed().map_or(0, |d| d.as_secs() as i64);
    let made = reiwand_format::bundled(&repo, &keys, &name, &[], now)?;
    repoint(data, &old, &made.repo_id, sink)?;
    let client = data.join("repos").join(&old);
    if client.is_dir() {
        fs::rename(&client, sink.copies().join(format!("repo-{old}")))?;
    }
    Ok(())
}
