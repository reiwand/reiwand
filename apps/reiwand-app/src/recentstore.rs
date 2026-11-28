use crate::scenefile::projects_dir;
use reiwand_editor::project;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

const KEPT: usize = 200;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
    #[serde(default)]
    pub known: Vec<Known>,
    #[serde(default)]
    pub forgotten: Vec<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Known {
    pub folder: PathBuf,
    #[serde(default)]
    pub title: String,
    #[serde(default = "shown")]
    pub recent: bool,
}

fn shown() -> bool {
    true
}

fn file() -> PathBuf {
    if crate::bus::checking() {
        return projects_dir().join(".recent.json");
    }
    dirs::state_dir()
        .or_else(dirs::data_local_dir)
        .unwrap_or_default()
        .join("reiwand")
        .join("recent-projects.json")
}

pub fn parse(text: &str) -> Store {
    serde_json::from_str(text)
        .or_else(|_| serde_json::from_str(text).map(|known| Store { known, forgotten: Vec::new() }))
        .unwrap_or_default()
}

pub fn snapshot() -> String {
    std::fs::read_to_string(file()).unwrap_or_default()
}

pub fn restore(text: &str) {
    keep(&parse(text));
}

fn saved() -> Store {
    parse(&snapshot())
}

fn keep(store: &Store) {
    if let Some(parent) = file().parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(file(), serde_json::to_string(store).unwrap_or_default()).ok();
}

pub fn put(mut store: Store, folder: &Path, title: &str) -> Store {
    store.known.retain(|k| k.folder != folder);
    store.forgotten.retain(|f| f != folder);
    store.known.insert(0, Known { folder: folder.to_owned(), title: title.to_owned(), recent: true });
    store.known.truncate(KEPT);
    store
}

pub fn hide(mut store: Store, folders: &[PathBuf]) -> Store {
    for folder in folders {
        match store.known.iter_mut().find(|k| k.folder == *folder) {
            Some(known) => known.recent = false,
            None => store.known.push(Known { folder: folder.clone(), title: String::new(), recent: false }),
        }
    }
    store.known.truncate(KEPT);
    store
}

pub fn drop(mut store: Store, folder: &Path) -> Store {
    store.known.retain(|k| k.folder != folder);
    store.forgotten.retain(|f| f != folder);
    store.forgotten.insert(0, folder.to_owned());
    store.forgotten.truncate(KEPT);
    store
}

pub fn record(folder: &Path, title: &str) {
    keep(&put(saved(), folder, title));
}

pub fn forget(folder: &Path) {
    keep(&drop(saved(), folder));
}

pub fn hide_one(folder: &Path) {
    keep(&hide(saved(), &[folder.to_owned()]));
}

pub fn clear() {
    let folders: Vec<PathBuf> =
        list().iter().filter_map(|r| r["folder"].as_str().map(PathBuf::from)).collect();
    keep(&hide(saved(), &folders));
}

pub fn order(rows: &mut [Value]) {
    rows.sort_by_key(|r| (r["missing"] == true, std::cmp::Reverse(r["edited"].as_u64().unwrap_or(0))));
}

fn edited(folder: &Path, doc: &Value) -> u64 {
    let entry = project::entry(doc).unwrap_or_default();
    ["manifest.json", entry.as_str()]
        .iter()
        .filter_map(|n| {
            std::fs::metadata(folder.join(n)).ok()?.modified().ok()?.duration_since(UNIX_EPOCH).ok()
        })
        .map(|d| d.as_secs())
        .max()
        .unwrap_or(0)
}

fn poster(folder: &Path, doc: &Value) -> String {
    let named = doc["manifest"]["preview"].as_str().map(|p| folder.join(p));
    named.filter(|p| p.is_file()).map(|p| p.to_string_lossy().into_owned()).unwrap_or_default()
}

fn row(folder: &Path, title: &str, recent: bool) -> Value {
    let languages = reiwand_core::locale::languages();
    match project::read(folder) {
        Ok(doc) => json!({
            "folder": folder,
            "title": reiwand_editor::meta::title(&doc, &languages),
            "kind": reiwand_editor::pages::kind(&doc),
            "version": doc["manifest"]["version"].as_str().unwrap_or_default(),
            "edited": edited(folder, &doc),
            "poster": poster(folder, &doc),
            "missing": false,
            "recent": recent,
        }),
        Err(_) => json!({"folder": folder, "title": title, "kind": "", "edited": 0, "poster": "",
            "missing": true, "recent": recent}),
    }
}

pub fn list() -> Vec<Value> {
    let store = saved();
    let mut rows: Vec<Value> = store.known.iter().map(|k| row(&k.folder, &k.title, k.recent)).collect();
    for entry in std::fs::read_dir(projects_dir()).into_iter().flatten().flatten() {
        let path = entry.path();
        if !store.known.iter().any(|k| k.folder == path) && !store.forgotten.contains(&path) {
            let found = row(&path, "", true);
            if found["missing"] == false {
                rows.push(found);
            }
        }
    }
    order(&mut rows);
    rows
}
