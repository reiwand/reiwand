use reiwand_fill::{Customs, Format};
use serde_json::Value;
use std::fs;
use std::path::Path;

fn read(path: &Path) -> Option<Value> {
    serde_json::from_slice(&fs::read(path).ok()?).ok()
}

fn listed(doc: &Value, key: &str) -> Vec<Value> {
    doc.get(key).and_then(Value::as_array).cloned().unwrap_or_default()
}

fn descriptors<'a>(
    dir: &'a Path,
    scene: &'a Value,
) -> impl Iterator<Item = (String, std::path::PathBuf)> + 'a {
    let effects = scene.get("effects").and_then(Value::as_object).into_iter().flatten();
    effects.filter_map(|(name, entry)| Some((name.clone(), dir.join(entry.get("descriptor")?.as_str()?))))
}

fn customs(dir: &Path, scene: &Value) -> Customs {
    let read = |(name, path): (String, std::path::PathBuf)| {
        let descriptor = read(&path)?;
        let all = [listed(&descriptor, "params"), listed(&descriptor, "options")].concat();
        let values =
            all.iter().filter_map(|p| Some((p.get("id")?.as_str()?.to_owned(), p.get("prefill")?.clone())));
        Some((name, values.collect()))
    };
    descriptors(dir, scene).filter_map(read).collect()
}

fn file(path: &Path) {
    let Some(mut doc) = read(path) else { return };
    let before = doc.clone();
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let dir = path.parent().unwrap_or(Path::new("."));
    match name.as_str() {
        "scene.json" => {
            descriptors(dir, &before).for_each(|(_, descriptor)| file(&descriptor));
            reiwand_fill::complete_with(Format::Scene, &mut doc, &customs(dir, &before));
        }
        "manifest.json" => reiwand_fill::manifests::complete(&mut doc),
        _ if name.ends_with(".reiwand.json") => reiwand_fill::edits::complete(&mut doc),
        _ => reiwand_fill::descriptors::complete(&mut doc),
    }
    if doc != before {
        let text = serde_json::to_string_pretty(&doc).expect("a JSON document") + "\n";
        fs::write(path, text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    }
}

fn walk(dir: &Path) {
    for path in fs::read_dir(dir).into_iter().flatten().flatten().map(|entry| entry.path()) {
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        if path.is_dir() && !path.is_symlink() {
            walk(&path);
        } else if ["scene.json", "manifest.json"].contains(&name.as_str()) || name.ends_with(".reiwand.json")
        {
            file(&path);
        }
    }
}

fn main() {
    for arg in std::env::args_os().skip(1) {
        let path = Path::new(&arg);
        if path.is_dir() { walk(path) } else { file(path) }
    }
}
