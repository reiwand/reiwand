use anyhow::Context;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

pub const FORMAT: u64 = 3;

pub fn wrap(text: &str) -> anyhow::Result<String> {
    let found: Value = serde_json::from_str(text).context("the file is not valid JSON")?;
    if found.get("format").and_then(Value::as_u64) == Some(FORMAT) {
        return Ok(text.to_owned());
    }
    Ok(serde_json::to_string_pretty(&json!({"format": FORMAT, "content": found}))? + "\n")
}

pub fn keep(
    path: &Path,
    copies: &Path,
    convert: impl Fn(&str) -> anyhow::Result<String>,
) -> anyhow::Result<bool> {
    let Ok(original) = fs::read_to_string(path) else { return Ok(false) };
    let converted = convert(&original).with_context(|| format!("cannot convert {}", path.display()))?;
    if converted == original {
        return Ok(false);
    }
    fs::create_dir_all(copies)?;
    fs::write(copies.join(path.file_name().context("no file name")?), &original)?;
    let staged = path.with_extension("upgrading");
    fs::write(&staged, converted)?;
    fs::rename(staged, path)?;
    Ok(true)
}

pub fn installed(text: &str) -> anyhow::Result<String> {
    let mut found: Value = serde_json::from_str(text).context("the file is not valid JSON")?;
    if found.get("format").and_then(Value::as_u64) == Some(FORMAT) {
        return Ok(text.to_owned());
    }
    if found.is_array() {
        for one in found.as_array_mut().into_iter().flatten().filter_map(Value::as_object_mut) {
            for key in ["signer", "revoked"] {
                one.entry(key).or_insert(Value::Null);
            }
        }
    }
    wrap(&found.to_string())
}

pub fn files(data: &Path, copies: &Path) -> anyhow::Result<usize> {
    let mut done = 0;
    for name in ["inputs.json", "inputs-declined.json", "accounts.json", "anchors.json", "repo-pins.json"] {
        done += usize::from(keep(&data.join(name), copies, wrap)?);
    }
    done += usize::from(keep(&data.join("installed.json"), copies, installed)?);
    Ok(done)
}
