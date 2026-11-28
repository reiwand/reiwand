use super::plugged::{self, PREFIX};
use crate::{Error, said};
use reiwand_plugin::Manifest;
use reiwand_providers::{catalog, wallhaven::SORTINGS};
use serde_json::{Value, json};
use std::collections::BTreeMap;

pub(crate) type Options = BTreeMap<String, String>;

const REFRESH: [&str; 5] = ["6h", "12h", "1d", "3d", "7d"];

#[derive(Clone, Copy)]
pub(super) enum Shape {
    Text,
    Url,
    Int(u32, u32),
    Pick(&'static [&'static str]),
}

pub(super) struct Field(pub(super) &'static str, pub(super) Shape, pub(super) &'static str, pub(super) bool);

pub(super) struct Kind {
    pub(super) name: &'static str,
    pub(super) refresh: &'static str,
    pub(super) fields: &'static [Field],
}

pub(super) const KEPT: Field = Field("keep", Shape::Int(5, 200), "30", false);
pub(super) const WHEN: Field = Field("refresh", Shape::Pick(&REFRESH), "1d", false);

const KINDS: [Kind; 7] = [
    Kind {
        name: "bing",
        refresh: "12h",
        fields: &[Field("market", Shape::Text, "en-US", false), Field("count", Shape::Int(1, 8), "8", false)],
    },
    Kind { name: "apod", refresh: "12h", fields: &[Field("days", Shape::Int(1, 30), "1", false)] },
    Kind { name: "wikimedia", refresh: "12h", fields: &[Field("days", Shape::Int(1, 30), "1", false)] },
    Kind {
        name: "wallhaven",
        refresh: "1d",
        fields: &[
            Field("query", Shape::Text, "", false),
            Field("sorting", Shape::Pick(&SORTINGS), "toplist", false),
            Field("max_items", Shape::Int(1, 24), "12", false),
        ],
    },
    Kind {
        name: "webdav",
        refresh: "1d",
        fields: &[Field("url", Shape::Url, "", true), Field("user", Shape::Text, "", true)],
    },
    Kind {
        name: "nextcloud",
        refresh: "1d",
        fields: &[
            Field("server", Shape::Url, "", true),
            Field("user", Shape::Text, "", true),
            Field("folder", Shape::Text, "", false),
        ],
    },
    Kind {
        name: "immich",
        refresh: "1d",
        fields: &[Field("server", Shape::Url, "", true), Field("album", Shape::Text, "", true)],
    },
];

/// A kind of source: one of the phone's own, or one an installed provider plugin adds.
pub(super) enum Known {
    Own(&'static Kind),
    Plugin(Box<Manifest>),
}

pub(super) fn kind(name: &str) -> Result<Known, Error> {
    if let Some(id) = name.strip_prefix(PREFIX) {
        return plugged::manifest(id).map(|m| Known::Plugin(Box::new(m)));
    }
    let found = KINDS.iter().find(|k| k.name == name).map(Known::Own);
    found.ok_or_else(|| said("android.provider_kind", "%1 is not a kind of picture source", &[&name]))
}

pub(super) fn describe(field: &Field, default: &str) -> Value {
    let Field(key, shape, _, required) = field;
    let (kind, values, min, max) = match shape {
        Shape::Text => ("text", json!([]), 0, 0),
        Shape::Url => ("url", json!([]), 0, 0),
        Shape::Int(min, max) => ("int", json!([]), *min, *max),
        Shape::Pick(values) => ("pick", json!(values), 0, 0),
    };
    json!({"key": key, "type": kind, "default": default, "required": required, "values": values, "min": min, "max": max})
}

pub(super) fn schema() -> Value {
    let kinds = KINDS.iter().map(|k| {
        let own = k.fields.iter().map(|f| describe(f, f.2));
        let common = [describe(&WHEN, k.refresh), describe(&KEPT, KEPT.2)];
        let secret = secret_of(&Known::Own(k)).map(|(name, required)| json!({"name": name, "required": required}));
        json!({"kind": k.name, "hosts": base(k).map_or(&[][..], |b| b.hosts), "options": own.chain(common).collect::<Vec<_>>(), "secret": secret})
    });
    json!(kinds.chain(plugged::kinds()).collect::<Vec<_>>())
}

pub(super) fn hosts(known: &Known, options: &Options) -> Vec<String> {
    let kind = match known {
        Known::Own(kind) => kind,
        Known::Plugin(manifest) => return manifest.hosts.clone(),
    };
    let own = ["url", "server"].iter().find_map(|key| reqwest::Url::parse(options.get(*key)?).ok());
    let own = own.and_then(|url| url.host_str().map(String::from));
    let fixed = base(kind).map_or(&[][..], |b| b.hosts);
    if fixed.is_empty() { own.into_iter().collect() } else { fixed.iter().map(|h| (*h).to_owned()).collect() }
}

pub(super) fn secret_of(known: &Known) -> Option<(String, bool)> {
    match known {
        Known::Own(kind) => {
            base(kind).and_then(|b| b.secret).map(|(name, required)| (name.to_owned(), required))
        }
        Known::Plugin(manifest) => plugged::secret_of(manifest),
    }
}

fn base(kind: &Kind) -> Option<&'static catalog::Kind> {
    catalog::find(kind.name)
}
