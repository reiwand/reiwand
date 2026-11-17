use crate::fill::{Customs, Format, complete_with};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn run(root: &Path) -> anyhow::Result<usize> {
    let mut changed = 0;
    walk(root, &mut changed)?;
    Ok(changed)
}

fn walk(dir: &Path, changed: &mut usize) -> anyhow::Result<()> {
    let Ok(items) = fs::read_dir(dir) else { return Ok(()) };
    for item in items.flatten() {
        let path = item.path();
        if path.is_dir() {
            walk(&path, changed)?;
        } else if path.file_name().is_some_and(|n| n == "scene.json") {
            *changed += usize::from(scene(&path)?);
        }
    }
    Ok(())
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

pub fn scene(path: &Path) -> anyhow::Result<bool> {
    let Ok(mut doc) = serde_json::from_slice::<Value>(&fs::read(path)?) else { return Ok(false) };
    let customs = customs(path.parent().unwrap_or(Path::new(".")), &doc);
    let dropped = crate::gain::fixed(&mut doc);
    let renamed = crate::descriptors::rewrite(path.parent().unwrap_or(Path::new(".")), &doc)? > 0;
    let versioned = version(&mut doc);
    if complete_with(Format::Scene, &mut doc, &customs).written == 0 && !dropped && !versioned {
        return Ok(renamed);
    }
    let staged = path.with_extension("json.upgrading");
    fs::write(&staged, text(&doc)?)?;
    fs::rename(staged, path)?;
    Ok(true)
}
