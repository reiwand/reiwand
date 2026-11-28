mod applied;
mod blocks;
mod hidden;
mod params;
mod particles;
mod walk;

use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::OnceLock;

pub mod descriptors;
pub mod edits;
pub mod manifests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Scene,
    Manifest,
    Edit,
}

impl Format {
    fn frozen(self) -> &'static Value {
        static FROZEN: OnceLock<[Value; 3]> = OnceLock::new();
        let all = FROZEN.get_or_init(|| {
            [
                parse(include_str!("../frozen/scene.json")),
                parse(include_str!("../frozen/manifest.json")),
                parse(include_str!("../frozen/edit.json")),
            ]
        });
        &all[self as usize]
    }
}

fn parse(text: &str) -> Value {
    serde_json::from_str(text).unwrap_or(Value::Null)
}

pub(crate) fn effects() -> &'static BTreeMap<String, Vec<(String, Value)>> {
    static EFFECTS: OnceLock<BTreeMap<String, Vec<(String, Value)>>> = OnceLock::new();
    EFFECTS.get_or_init(|| {
        serde_json::from_value(parse(include_str!("../frozen/effects.json"))).unwrap_or_default()
    })
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Report {
    pub written: usize,
    pub fields: BTreeMap<String, usize>,
}

impl Report {
    fn note(&mut self, def: &str, field: &str) {
        self.written += 1;
        *self.fields.entry(format!("{def}.{field}")).or_default() += 1;
    }
}

pub type Customs = BTreeMap<String, Vec<(String, Value)>>;

pub fn complete(format: Format, doc: &mut Value) {
    complete_with(format, doc, &Customs::new());
}

pub fn complete_with(format: Format, doc: &mut Value, customs: &Customs) {
    walk::Walk::new(format.frozen(), customs, false).start(doc);
}

pub fn whole(text: &str) -> String {
    let Ok(mut doc) = serde_json::from_str::<Value>(text) else { return text.to_owned() };
    complete(Format::Scene, &mut doc);
    serde_json::to_string(&doc).unwrap_or_else(|_| text.to_owned())
}
