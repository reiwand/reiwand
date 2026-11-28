mod shape;

use serde_json::Value;

pub fn completed(text: &str) -> String {
    let Ok(mut doc) = serde_json::from_str::<Value>(text) else { return text.to_owned() };
    complete(&mut doc);
    doc.to_string()
}

pub fn complete(descriptor: &mut Value) {
    shape::descriptor(descriptor);
}
