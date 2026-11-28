//! The arrangement earlier versions kept in `window.conf` (group `editor`, key `arrangement`): the
//! panels on the left, on the right and under the canvas, and their sizes. Until a `workbench.json`
//! exists, the Scene layout starts from it, with the panels it did not know closed. Once that
//! Scene layout is saved in the file of layouts, the key is removed (the rest of the file stays).

use crate::bench::{Dir, Node, Panel, group, split};
use crate::benchdesk::Desk;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

/// The width and height of the workbench the shares of the layouts are measured on.
const WIDE: f64 = 1920.0;
const HIGH: f64 = 1008.0;

#[derive(Deserialize)]
struct Arrangement {
    #[serde(default)]
    places: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    sizes: BTreeMap<String, f64>,
}

/// The value of `key` in `[section]` of a settings file as Qt writes it (quoted, with backslash escapes).
pub fn value(text: &str, section: &str, key: &str) -> Option<String> {
    let mut inside = false;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            inside = line == format!("[{section}]");
        } else if let Some(raw) =
            line.strip_prefix(key).and_then(|rest| rest.strip_prefix('=')).filter(|_| inside)
        {
            let raw = raw.strip_prefix('"').and_then(|r| r.strip_suffix('"')).unwrap_or(raw);
            let mut out = String::new();
            let mut chars = raw.chars();
            while let Some(c) = chars.next() {
                out.push(if c == '\\' { chars.next().unwrap_or('\\') } else { c });
            }
            return Some(out);
        }
    }
    None
}

fn panels(names: Option<&Vec<String>>, seen: &mut Vec<Panel>) -> Vec<Panel> {
    let read =
        names.into_iter().flatten().filter_map(|name| serde_json::from_value(name.as_str().into()).ok());
    let mut found = Vec::new();
    for panel in read {
        if panel != Panel::Canvas && !seen.contains(&panel) {
            seen.push(panel);
            found.push(panel);
        }
    }
    found
}

/// The Scene layout an old arrangement describes, or `None` when it describes nothing usable.
pub fn tree(text: &str) -> Option<Node> {
    let old: Arrangement = serde_json::from_str(text).ok()?;
    let mut seen = Vec::new();
    let [left, right, bottom] = ["left", "right", "bottom"].map(|at| panels(old.places.get(at), &mut seen));
    if seen.is_empty() {
        return None;
    }
    let size = |at: &str, usual: f64, most: f64| {
        old.sizes.get(at).copied().filter(|n| n.is_finite() && *n > 0.0).unwrap_or(usual).min(most)
    };
    let canvas = group(&[Panel::Canvas]);
    let middle = if bottom.is_empty() {
        canvas
    } else {
        let low = size("bottom", HIGH * 0.24, HIGH * 0.6);
        split(Dir::Column, vec![(HIGH - low, canvas), (low, group(&bottom))])
    };
    let mut row = Vec::new();
    let (wide_left, wide_right) = (size("left", 300.0, WIDE * 0.35), size("right", 380.0, WIDE * 0.35));
    if !left.is_empty() {
        row.push((wide_left, group(&left)));
    }
    row.push((WIDE - wide_left - wide_right, middle.clone()));
    if !right.is_empty() {
        row.push((wide_right, group(&right)));
    }
    Some(if row.len() == 1 { middle } else { split(Dir::Row, row) })
}

/// The layouts to start with when there is no file of layouts yet: those that come with the
/// editor, with the Scene layout made from the arrangement of `conf` if it holds one.
pub fn carried(conf: &Path) -> Option<Desk> {
    let text = std::fs::read_to_string(conf).ok()?;
    let root = tree(&value(&text, "editor", "arrangement")?)?;
    Desk::default().put(root)
}

/// `text` without the line of `key` in `[section]`, or `None` when it has no such line.
pub fn without(text: &str, section: &str, key: &str) -> Option<String> {
    let mut inside = false;
    let mut found = false;
    let kept: Vec<&str> = text
        .split_inclusive('\n')
        .filter(|line| {
            let bare = line.trim();
            if bare.starts_with('[') {
                inside = bare == format!("[{section}]");
            }
            let ours = inside && bare.strip_prefix(key).is_some_and(|rest| rest.starts_with('='));
            found |= ours;
            !ours
        })
        .collect();
    found.then(|| kept.concat())
}

/// Removes the old arrangement from `conf`, writing the rest of it to a file beside it first so a
/// failed write leaves it whole.
pub fn forget(conf: &Path) -> std::io::Result<()> {
    let text = std::fs::read_to_string(conf)?;
    let Some(rest) = without(&text, "editor", "arrangement") else { return Ok(()) };
    let beside = conf.with_extension("conf.new");
    std::fs::write(&beside, rest)?;
    std::fs::rename(&beside, conf)
}
