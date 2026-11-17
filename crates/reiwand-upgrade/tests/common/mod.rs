#![allow(dead_code)]
use serde_json::Value;
use std::path::{Path, PathBuf};

pub fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub fn walk(dir: &Path, name: &str, out: &mut Vec<PathBuf>) {
    let Ok(items) = std::fs::read_dir(dir) else { return };
    for item in items.flatten() {
        let path = item.path();
        if path.is_dir() {
            if !matches!(
                path.file_name().and_then(|n| n.to_str()),
                Some("target-ci" | ".git" | "node_modules")
            ) {
                walk(&path, name, out);
            }
        } else if path.file_name().is_some_and(|n| n == name) {
            out.push(path);
        }
    }
}

pub fn corpus(name: &str) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for sub in ["scenes", "docs/crear/ejemplos", "tests/fixtures", "crates/reiwand-scene/tests"] {
        walk(&repo().join(sub), name, &mut found);
    }
    if let Some(extra) = std::env::var_os("REIWAND_CORPUS") {
        walk(Path::new(&extra), name, &mut found);
    }
    found.sort();
    found
}

pub fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => match (x.as_f64(), y.as_f64()) {
            (Some(x), Some(y)) => (x as f32) == (y as f32),
            _ => x == y,
        },
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(a, b)| same(a, b)),
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| same(v, w)))
        }
        _ => a == b,
    }
}

pub fn first_diff(a: &Value, b: &Value, path: &str) -> Option<String> {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            let keys: std::collections::BTreeSet<_> = x.keys().chain(y.keys()).collect();
            keys.into_iter().find_map(|k| match (x.get(k), y.get(k)) {
                (Some(p), Some(q)) => first_diff(p, q, &format!("{path}/{k}")),
                (p, q) => Some(format!("{path}/{k}: {p:?} vs {q:?}")),
            })
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => {
            x.iter().zip(y).enumerate().find_map(|(i, (p, q))| first_diff(p, q, &format!("{path}[{i}]")))
        }
        _ if same(a, b) => None,
        _ => Some(format!("{path}: {a} vs {b}")),
    }
}

pub fn live_schemas() -> Vec<(&'static str, Value)> {
    let to = |s: schemars::Schema| Value::from(s);
    vec![
        ("scene", reiwand_scene::schema()),
        ("manifest", reiwand_format::manifest_schema()),
        ("config", to(schemars::schema_for!(reiwand_core::Config))),
        ("edit", to(schemars::schema_for!(reiwand_core::edit::Edit))),
        ("carry", to(schemars::schema_for!(reiwand_carry::Content))),
    ]
}
