use crate::{Format, complete as fill};
use serde_json::Value;

pub fn complete(doc: &mut Value) {
    fill(Format::Manifest, doc);
}

pub fn whole(text: &str) -> String {
    let Ok(mut doc) = serde_json::from_str::<Value>(text) else { return text.to_owned() };
    complete(&mut doc);
    doc.to_string()
}
