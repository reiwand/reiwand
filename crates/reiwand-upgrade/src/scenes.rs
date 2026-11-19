use crate::fill::{Customs, Format, complete_with};
use crate::sink::Sink;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn run(root: &Path) -> anyhow::Result<usize> {
    walk_in(root, &mut Sink::plain())
}

pub fn walk_in(root: &Path, sink: &mut Sink) -> anyhow::Result<usize> {
    let mut changed = 0;
    walk(root, &mut changed, sink)?;
    Ok(changed)
}

fn walk(dir: &Path, changed: &mut usize, sink: &mut Sink) -> anyhow::Result<()> {
    let Ok(items) = fs::read_dir(dir) else { return Ok(()) };
    for item in items.flatten() {
        let path = item.path();
        let Ok(kind) = item.file_type() else { continue };
        if kind.is_dir() {
            walk(&path, changed, sink)?;
        } else if kind.is_symlink() && path.is_dir() {
            continue;
        } else if let Some(done) = sink.attempt(&path, |sink| entry(&path, sink)) {
            *changed += usize::from(done);
        }
    }
    Ok(())
}

fn entry(path: &Path, sink: &mut Sink) -> anyhow::Result<bool> {
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    if name == "scene.json" {
        scene(path, sink)
    } else if name == "manifest.json" {
        manifest(path, sink)
    } else if path.extension().is_some_and(|e| e == "wpak") {
        crate::packages::wpak(path, sink)
    } else if name.ends_with(".reiwand.json") {
        crate::edits::file(path, sink)
    } else {
        Ok(false)
    }
}

pub fn customs(dir: &Path, doc: &Value) -> Customs {
    let mut found = Customs::new();
    let files = doc.get("effects").and_then(Value::as_object);
    for (name, entry) in files.into_iter().flatten() {
        let Some(path) = entry.get("descriptor").and_then(Value::as_str) else { continue };
        let Ok(bytes) = fs::read(dir.join(path)) else { continue };
        let Ok(descriptor) = serde_json::from_slice::<Value>(&bytes) else { continue };
        let listed = |key: &str| descriptor.get(key).and_then(Value::as_array).into_iter().flatten().cloned();
        let list = listed("params")
            .chain(listed("options"))
            .filter_map(|p| {
                Some((
                    p.get("id")?.as_str()?.to_owned(),
                    p.get("default").or_else(|| p.get("prefill"))?.clone(),
                ))
            })
            .collect();
        found.insert(name.clone(), list);
    }
    found
}

pub fn effect(path: &Path, sink: &mut Sink) -> anyhow::Result<bool> {
    crate::descriptors::file(path, sink)
}

pub fn text(doc: &Value) -> anyhow::Result<String> {
    let mut text = serde_json::to_string_pretty(doc)?;
    text.push('\n');
    Ok(text)
}

fn version(doc: &mut Value) -> bool {
    let Some(schema) = doc.get_mut("schema").filter(|s| *s == "reiwand.scene/1") else { return false };
    *schema = Value::from("reiwand.scene/2");
    true
}

pub fn scene(path: &Path, sink: &mut Sink) -> anyhow::Result<bool> {
    let Ok(mut doc) = serde_json::from_slice::<Value>(&fs::read(path)?) else { return Ok(false) };
    let dir = path.parent().unwrap_or(Path::new("."));
    let customs = customs(dir, &doc);
    let dropped = crate::gain::fixed(&mut doc);
    let renamed = crate::descriptors::rewrite(dir, &doc, sink)? > 0;
    let versioned = version(&mut doc);
    if complete_with(Format::Scene, &mut doc, &customs).written == 0 && !dropped && !versioned {
        return Ok(renamed);
    }
    sink.save(path, text(&doc)?.as_bytes())?;
    Ok(true)
}

pub fn manifest(path: &Path, sink: &mut Sink) -> anyhow::Result<bool> {
    let Ok(mut doc) = serde_json::from_slice::<Value>(&fs::read(path)?) else { return Ok(false) };
    if !crate::manifests::convert(&mut doc) {
        return Ok(false);
    }
    sink.save(path, text(&doc)?.as_bytes())?;
    Ok(true)
}
