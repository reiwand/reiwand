use crate::blank::Kept;
use crate::config::{Config, OutputConfig};
use crate::filter::Filter;
use crate::gpu::Render;
use crate::layout::Layout;
use crate::settings::{Library, Media, Models, Pause, Playback, Repos, Transition};
use std::collections::BTreeMap;
use toml::{Table, Value};

fn table_of<T: serde::Serialize>(value: &T) -> Table {
    Table::try_from(value).unwrap_or_default()
}

fn merged(mut base: Table, over: Table) -> Table {
    for (key, value) in over {
        match (base.get_mut(&key), value) {
            (Some(Value::Table(b)), Value::Table(o)) => *b = merged(std::mem::take(b), o),
            (_, value) => drop(base.insert(key, value)),
        }
    }
    base
}

fn output(section: &mut Table) {
    for (key, whole) in [("filter", table_of(&Filter::default())), ("blank", table_of(&Kept::default()))] {
        if let Some(Value::Table(own)) = section.get_mut(key) {
            *own = merged(whole, std::mem::take(own));
        }
    }
    *section = merged(table_of(&OutputConfig::default()), std::mem::take(section));
}

impl Config {
    pub fn template() -> String {
        toml::to_string_pretty(&Config::default()).unwrap_or_default()
    }

    pub fn complete(partial: &str) -> String {
        let Ok(mut over) = toml::from_str::<Table>(partial) else { return partial.to_owned() };
        if let Some(Value::Table(default)) = over.get_mut("default") {
            output(default);
        }
        if let Some(Value::Table(outputs)) = over.get_mut("outputs") {
            outputs.iter_mut().filter_map(|(_, v)| v.as_table_mut()).for_each(output);
        }
        toml::to_string_pretty(&merged(table_of(&Config::default()), over))
            .unwrap_or_else(|_| partial.to_owned())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format: Self::FORMAT,
            profile: None,
            default: OutputConfig::default(),
            outputs: BTreeMap::new(),
            lock: None,
            playback: Playback::default(),
            transition: Transition::default(),
            pause: Pause::default(),
            library: Library::default(),
            repos: Repos::default(),
            models: Models::default(),
            media: Media::default(),
            network: crate::net::Network::default(),
            updates: None,
            rgb: crate::rgb::Rgb::default(),
            shortcuts: crate::shortcuts::Shortcuts::default(),
            layout: Layout::default(),
            render: Render::default(),
            surfaces: crate::Surfaces::default(),
            providers: BTreeMap::new(),
        }
    }
}
