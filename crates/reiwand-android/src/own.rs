use crate::settings::{BACKEND, COLORS, NETWORK};
use crate::{Error, said, store};
use std::fs;
use std::io::ErrorKind;

pub(crate) const FORMAT: i64 = 2;
const FILE: &str = "settings.toml";
const CHARGER: &str = "charger_fps";

pub(crate) const PREFILL: [(&str, &[&str], &str); 6] = [
    (COLORS, &["chosen", "always", "never"], "chosen"),
    (CHARGER, &["60", "30"], "60"),
    (BACKEND, &["auto", "vulkan", "gl"], "auto"),
    (NETWORK, &["unmetered", "any"], "unmetered"),
    ("video_convert", &["auto", "keep", "off"], "auto"),
    (crate::net::PRIVATE, &["off", "on"], "off"),
];

pub(crate) fn allowed(key: &str) -> Option<&'static [&'static str]> {
    PREFILL.iter().find(|(k, ..)| *k == key).map(|(_, allowed, _)| *allowed)
}

fn template() -> toml::Table {
    let mut table = toml::Table::new();
    table.insert("format".into(), FORMAT.into());
    for (key, _, prefill) in PREFILL {
        table.insert(key.into(), prefill.into());
    }
    table
}

fn refuse(why: String) -> Error {
    said(
        "android.settings_refused",
        "settings.toml cannot be read: %1. Reiwand does not fill in values: clear the app data to start with the default settings.",
        &[&why],
    )
}

fn check(table: &toml::Table) -> Result<(), Error> {
    match table.get("format").and_then(toml::Value::as_integer) {
        Some(FORMAT) => {}
        Some(other) => return Err(refuse(format!("format {other} is not {FORMAT}"))),
        None => return Err(refuse("\"format\" is missing, so the file is from an older version".into())),
    }
    for (key, allowed, _) in PREFILL {
        match table.get(key).and_then(toml::Value::as_str) {
            Some(value) if allowed.contains(&value) => {}
            Some(value) => return Err(refuse(format!("\"{key}\" cannot be \"{value}\""))),
            None => return Err(refuse(format!("\"{key}\" is missing"))),
        }
    }
    Ok(())
}

pub(crate) fn write(table: &toml::Table) -> Result<(), Error> {
    store::write(&store::dir()?.join(FILE), table.to_string().as_bytes())
}

pub(crate) fn read() -> Result<toml::Table, Error> {
    let text = match fs::read_to_string(store::dir()?.join(FILE)) {
        Ok(text) => text,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let first = template();
            write(&first)?;
            return Ok(first);
        }
        Err(e) => return Err(e.into()),
    };
    let table: toml::Table = text.parse().map_err(|e: toml::de::Error| refuse(e.message().to_owned()))?;
    check(&table)?;
    Ok(table)
}

pub(crate) fn value(key: &str) -> Result<String, Error> {
    let table = read()?;
    let found = table.get(key).and_then(toml::Value::as_str);
    found.map(str::to_owned).ok_or_else(|| refuse(format!("\"{key}\" is missing")))
}
