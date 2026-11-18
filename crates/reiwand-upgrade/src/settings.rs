use crate::fill::{Format, complete_toml};
use crate::sink::Sink;
use anyhow::Context;
use std::path::Path;
use toml_edit::{DocumentMut, Item, Table, TableLike, value};

pub const FORMAT: i64 = 3;

const GENERAL: [&str; 7] = ["fit", "background", "clicks", "filter", "quality", "hdr", "transition"];
const EMPTY: [&str; 3] = ["image", "playlist", "preset"];

fn text(item: &mut dyn TableLike, key: &str, word: &str) {
    if !item.contains_key(key) {
        item.insert(key, value(word));
    }
}

fn table(item: &mut dyn TableLike, key: &str) {
    if !item.contains_key(key) {
        item.insert(key, Item::Table(Table::new()));
    }
}

fn output(section: &mut dyn TableLike) {
    EMPTY.iter().for_each(|key| text(section, key, ""));
    GENERAL.iter().for_each(|key| text(section, key, "general"));
    text(section, "blank", "none");
    if let Some(kept) = section.get_mut("blank").and_then(Item::as_table_like_mut) {
        EMPTY.iter().for_each(|key| text(kept, key, ""));
    }
    table(section, "scenes");
}

pub fn config(original: &str) -> anyhow::Result<String> {
    let found: toml::Value = toml::from_str(original).context("config.toml is not valid TOML")?;
    if found.get("format").and_then(toml::Value::as_integer) == Some(FORMAT) {
        return Ok(original.to_owned());
    }
    let (done, _) = complete_toml(Format::Config, original)?;
    let mut doc: DocumentMut = done.parse()?;
    let root = doc.as_table_mut();
    root.insert("format", value(FORMAT));
    text(root, "profile", "");
    table(root, "default");
    table(root, "outputs");
    for key in ["layout", "render", "surfaces", "providers", "rgb"] {
        table(root, key);
    }
    text_in(root, "layout", |t| {
        table(t, "monitors");
        table(t, "follow");
    });
    text_in(root, "render", |t| text(t, "gpu", "auto"));
    text_in(root, "surfaces", |t| text(t, "layer_namespace", "auto"));
    text_in(root, "rgb", |t| table(t, "places"));
    if let Some(section) = root.get_mut("default").and_then(Item::as_table_like_mut) {
        output(section);
    }
    for (_, one) in
        root.get_mut("outputs").and_then(Item::as_table_like_mut).into_iter().flat_map(|t| t.iter_mut())
    {
        if let Some(section) = one.as_table_like_mut() {
            output(section);
        }
    }
    Ok(doc.to_string())
}

fn text_in(root: &mut Table, key: &str, apply: impl FnOnce(&mut dyn TableLike)) {
    if let Some(inner) = root.get_mut(key).and_then(Item::as_table_like_mut) {
        apply(inner);
    }
}

pub fn run(config_dir: &Path, sink: &mut Sink) -> anyhow::Result<bool> {
    sink.text(&config_dir.join("config.toml"), config)
}
