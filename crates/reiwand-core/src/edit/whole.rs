use super::{Depth, Edit, Geometry, Parallax, Source, Span};
use serde_json::{Value, json};

fn fill(target: &mut Value, base: &Value) {
    let (Some(target), Some(base)) = (target.as_object_mut(), base.as_object()) else { return };
    for (key, value) in base {
        target.entry(key.clone()).or_insert_with(|| value.clone());
    }
}

fn each(doc: &mut Value, key: &str, base: &Value) {
    if let Some(inner) = doc.get_mut(key).filter(|v| v.is_object()) {
        fill(inner, base);
    }
}

fn extras(kind: &str) -> Value {
    match kind {
        "white_balance" => json!({"tint": 0.0}),
        "saturation" => json!({"saturation": 0.0, "vibrance": 0.0}),
        "curves" => json!({"rgb": [], "r": [], "g": [], "b": []}),
        "overlay" => json!({"blend": "normal"}),
        "linear" | "brush" => json!({"invert": false}),
        "radial" => json!({"feather": 0.0, "invert": false}),
        "luminance" | "depth" => json!({"soft": 0.0, "invert": false}),
        _ => json!({}),
    }
}

fn each_in(doc: &mut Value, key: &str, own: Value) {
    let list = doc.get_mut(key);
    let items: Vec<&mut Value> = match list {
        Some(Value::Array(items)) => items.iter_mut().collect(),
        Some(Value::Object(map)) => map.values_mut().collect(),
        _ => return,
    };
    for item in items {
        let kind = item.get("type").and_then(Value::as_str).unwrap_or_default().to_owned();
        fill(item, &own);
        fill(item, &extras(&kind));
        if let Some(strokes) = item.get_mut("strokes").and_then(Value::as_array_mut) {
            strokes.iter_mut().for_each(|s| fill(s, &json!({"erase": false})));
        }
    }
}

impl Edit {
    pub fn whole(mut doc: Value) -> Value {
        let base = serde_json::to_value(Edit::default()).unwrap_or_default();
        fill(&mut doc, &base);
        doc["version"] = json!(super::VERSION);
        for (key, value) in [
            ("source", serde_json::to_value(Source::default())),
            ("geometry", serde_json::to_value(Geometry::default())),
            ("span", serde_json::to_value(Span::default())),
            ("depth", serde_json::to_value(Depth::default())),
            ("parallax", serde_json::to_value(Parallax::default())),
        ] {
            each(&mut doc, key, &value.unwrap_or_default());
        }
        each_in(&mut doc, "ops", json!({"enabled": true, "mask": "", "opacity": null}));
        each_in(&mut doc, "masks", json!({}));
        each_in(&mut doc, "outputs", json!({"connector": ""}));
        doc
    }

    pub fn parse_whole(text: &str) -> Result<Edit, crate::message::Message> {
        Edit::from_whole(serde_json::from_str(text).unwrap_or_default())
    }

    pub fn from_whole(doc: Value) -> Result<Edit, crate::message::Message> {
        Edit::from_value(Edit::whole(doc))
    }
}
