use anyhow::{Context, Result};
use base64::Engine;
use reiwand_format::{Limits, Manifest};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

type Files = &'static [(&'static str, &'static [u8])];

pub static SHIPPED: &[(&str, Files)] = include!(concat!(env!("OUT_DIR"), "/builtin.rs"));

fn file(files: Files, name: &str) -> Option<&'static [u8]> {
    files.iter().find(|(n, _)| *n == name).map(|(_, bytes)| *bytes)
}

fn picture(manifest: &Value, files: Files) -> String {
    let named = manifest["previews"]["thumbnail"].as_str().unwrap_or_default();
    let Some(bytes) = file(files, named) else { return String::new() };
    let kind = if named.ends_with(".png") { "png" } else { "jpeg" };
    format!("data:image/{kind};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes))
}

pub fn entry(name: &str, files: Files) -> Result<Value> {
    let bytes = file(files, reiwand_format::MANIFEST).context("an example without a manifest")?;
    let manifest = Manifest::from_json(bytes)?;
    let raw: Value = serde_json::from_slice(bytes)?;
    Ok(json!({
        "name": name,
        "title": manifest.title_in(&reiwand_core::locale::languages()),
        "kind": manifest.kind.name(),
        "license": manifest.license,
        "picture": picture(&raw, files),
    }))
}

pub fn list() -> Vec<Value> {
    SHIPPED.iter().filter_map(|(name, files)| entry(name, files).ok()).collect()
}

pub fn unpack(files: Files, dir: &Path) -> Result<()> {
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    for (name, bytes) in files {
        let path = dir.join(name);
        fs::create_dir_all(path.parent().unwrap_or(dir))?;
        fs::write(path, bytes)?;
    }
    Ok(())
}

pub fn package(name: &str, room: &Path) -> Result<PathBuf> {
    let (_, files) = SHIPPED.iter().find(|(n, _)| *n == name).context("no example of that name")?;
    let source = room.join(name);
    unpack(files, &source)?;
    let out = room.join(format!("{name}.wpak"));
    reiwand_format::build(&source, &out, &Limits::default())?;
    fs::remove_dir_all(&source).ok();
    Ok(out)
}

pub fn room() -> PathBuf {
    dirs::cache_dir().unwrap_or_else(std::env::temp_dir).join("reiwand").join("examples")
}

#[cfg(test)]
#[path = "tests/builtin.rs"]
mod tests;
