use crate::fill::{Customs, Format, complete_with};
use serde_json::Value;

const OLD: &str = "reiwand.wpak/2";
const NEW: &str = "reiwand.wpak/3";

pub fn old(doc: &Value) -> bool {
    doc.get("schema").is_some_and(|s| s == OLD)
}

pub fn reshape(doc: &mut Value) {
    let Some(map) = doc.as_object_mut() else { return };
    let preview = map.remove("preview");
    if let Some(Value::String(path)) = preview {
        let previews = map.entry("previews").or_insert(Value::Null);
        if previews.is_null() {
            *previews = serde_json::json!({ "animated": null, "thumbnail": null });
        }
        if previews["thumbnail"].is_null() {
            previews["thumbnail"] = Value::String(path);
        }
    }
    map.insert("schema".into(), Value::from(NEW));
}

pub fn convert(doc: &mut Value) -> bool {
    if !old(doc) {
        return false;
    }
    complete_with(Format::Manifest, doc, &Customs::new());
    reshape(doc);
    true
}

pub fn whole(text: &str) -> String {
    let Ok(mut doc) = serde_json::from_str::<Value>(text) else { return text.to_owned() };
    if doc.get("schema").is_some_and(|s| s == NEW) {
        doc["schema"] = Value::from(OLD);
    }
    convert(&mut doc);
    doc.to_string()
}

pub fn complete(doc: &mut Value) {
    if let Ok(done) = serde_json::from_str(&whole(&doc.to_string())) {
        *doc = done;
    }
}
