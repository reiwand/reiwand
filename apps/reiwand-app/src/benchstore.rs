use crate::benchdesk::Desk;
use crate::tr::tr;
use reiwand_core::explicit::{Fault, ReadError, read};
use reiwand_core::message::render;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};

pub const FORMAT: u32 = 8;

/// The panels each format added: a file of an older format is read with them closed in every
/// view it holds, where the Window menu brings them back.
const ADDED: [(u32, &[&str]); 2] =
    [(6, &["particles", "cost"]), (7, &["scripts", "code", "reference", "console"])];

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Stored {
    format: u32,
    content: Desk,
}

#[derive(Debug, PartialEq)]
pub enum Loaded {
    Fresh,
    Kept(Box<Desk>),
    /// Read in part: the layouts that could not be read went back to how they come.
    Mended(Box<Desk>, String),
    Refused(String),
}
/// The file of a page's layouts: a scene's in `workbench.json`; a package of another kind (web,
/// image, video, preset) in its own, `workbench-<kind>.json`, so its arrangement never takes the
/// place of the scene's. A place that is not a word of small letters is the scene's.
pub fn file_for(place: &str) -> PathBuf {
    let name = if !place.is_empty() && place.bytes().all(|b| b.is_ascii_lowercase()) {
        format!("workbench-{place}.json")
    } else {
        "workbench.json".to_owned()
    };
    if crate::bus::checking() {
        return crate::scenefile::projects_dir().join(format!(".{name}"));
    }
    dirs::config_dir().unwrap_or_default().join("reiwand").join(name)
}

pub fn parse(file: &str, text: &str) -> Result<Desk, String> {
    let invalid =
        |path: String, why: String| ReadError { file: file.to_owned(), path, fault: Fault::Invalid(why) };
    let mut value: Value =
        serde_json::from_str(text).map_err(|e| invalid(String::new(), e.to_string()).to_string())?;
    upgrade(&mut value);
    let found = value.get("format").and_then(Value::as_u64);
    if found != Some(u64::from(FORMAT)) {
        let found = found.map_or_else(|| tr("none"), |n| n.to_string());
        let said =
            tr("%1 was written by another version of Reiwand (format %2) and this one reads only format %3.");
        return Err(render(&said, &[file.to_owned(), found, FORMAT.to_string()]));
    }
    let stored: Stored = read(file, &value).map_err(|e| e.to_string())?;
    match stored.content.fault() {
        Some((path, why)) => Err(invalid(format!("content.{path}"), why).to_string()),
        None => Ok(stored.content),
    }
}

/// Brings a file of an older format up to this one, a format at a time.
fn upgrade(value: &mut Value) {
    crate::benchmend::older(value);
    for &(from, panels) in crate::benchmend::ADDED.iter().chain(ADDED.iter()) {
        if value.get("format").and_then(Value::as_u64) != Some(u64::from(from)) {
            continue;
        }
        if let Some(content) = value.get_mut("content") {
            close(content, panels);
        }
        value["format"] = Value::from(from + 1);
    }
}

/// Every view (an object with a root and the panels closed in it) gets `panels` closed, unless
/// its tree already shows them.
fn close(value: &mut Value, panels: &[&str]) {
    match value {
        Value::Object(map) if map.contains_key("root") && map.get("closed").is_some_and(Value::is_array) => {
            let tree = map.get("root").map(Value::to_string).unwrap_or_default();
            if let Some(Value::Array(closed)) = map.get_mut("closed") {
                for panel in panels {
                    let name = Value::from(*panel);
                    if !tree.contains(&format!("\"{panel}\"")) && !closed.contains(&name) {
                        closed.push(name);
                    }
                }
            }
        }
        Value::Object(map) => map.values_mut().for_each(|inner| close(inner, panels)),
        Value::Array(items) => items.iter_mut().for_each(|inner| close(inner, panels)),
        _ => {}
    }
}

pub fn text(desk: &Desk) -> String {
    let stored = Stored { format: FORMAT, content: desk.clone() };
    serde_json::to_string_pretty(&stored).unwrap_or_default() + "\n"
}

pub fn load_from(path: &Path) -> Loaded {
    match std::fs::read_to_string(path) {
        Ok(text) => match parse(&path.display().to_string(), &text) {
            Ok(desk) => Loaded::Kept(Box::new(desk)),
            Err(said) => mended(&path.display().to_string(), &text).unwrap_or(Loaded::Refused(said)),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Loaded::Fresh,
        Err(e) => Loaded::Refused(format!("{}: {e}", path.display())),
    }
}

/// A file of this format that does not read whole, read layout by layout.
fn mended(file: &str, text: &str) -> Option<Loaded> {
    let mut value: Value = serde_json::from_str(text).ok()?;
    upgrade(&mut value);
    if value.get("format").and_then(Value::as_u64) != Some(u64::from(FORMAT)) {
        return None;
    }
    let (desk, lost) = crate::benchmend::mend(&value)?;
    Some(Loaded::Mended(Box::new(desk), crate::benchmend::told(file, &lost)))
}

pub fn save_to(path: &Path, desk: &Desk) -> Result<(), String> {
    if let Some((at, why)) = desk.fault() {
        return Err(format!("content.{at}: {why}"));
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let partial = path.with_extension("json.part");
    std::fs::write(&partial, text(desk)).map_err(|e| e.to_string())?;
    std::fs::rename(&partial, path).map_err(|e| e.to_string())
}
