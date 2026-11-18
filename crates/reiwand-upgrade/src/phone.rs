use anyhow::Context;
use std::fs;
use std::path::Path;

const FORMAT: i64 = 2;
const FILE: &str = "settings.toml";

const PREFILL: [(&str, &[&str], &str); 6] = [
    ("system_colors", &["chosen", "always", "never"], "chosen"),
    ("charger_fps", &["60", "30"], "60"),
    ("backend", &["auto", "vulkan", "gl"], "auto"),
    ("sync_network", &["unmetered", "any"], "unmetered"),
    ("video_convert", &["auto", "keep", "off"], "auto"),
    ("private_mode", &["off", "on"], "off"),
];

pub(crate) fn settings(data: &Path) -> anyhow::Result<()> {
    let path = data.join(FILE);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e).with_context(|| format!("reading {FILE}")),
    };
    let mut table: toml::Table = text.parse().with_context(|| format!("{FILE} is not valid TOML"))?;
    let mut changed = table.get("format").and_then(toml::Value::as_integer) != Some(FORMAT);
    table.insert("format".into(), FORMAT.into());
    for (key, allowed, prefill) in PREFILL {
        let held = table.get(key).and_then(toml::Value::as_str);
        if !held.is_some_and(|v| allowed.contains(&v)) {
            table.insert(key.into(), prefill.into());
            changed = true;
        }
    }
    if changed {
        fs::write(&path, table.to_string()).with_context(|| format!("writing {FILE}"))?;
    }
    Ok(())
}
