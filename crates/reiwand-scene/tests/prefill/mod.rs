#![allow(dead_code)]
use serde_json::{Value, json};

pub use reiwand_scene::prefill::*;

const OPAQUE: [&str; 11] = [
    "Value_", "Binding_", "Key_", "Spread_", "Color", "Ease", "Handles", "Num", "Param", "Gradient", "Easing",
];

fn named<'a>(schema: &'a Value, node: &'a Value) -> Option<&'a Value> {
    let Some(target) = node.get("$ref").and_then(Value::as_str) else { return Some(node) };
    let name = target.rsplit('/').next().unwrap_or("");
    (!OPAQUE.iter().any(|o| name.starts_with(o))).then(|| &schema["$defs"][name])
}

fn gather(schema: &Value, node: &Value, parent: &str, tag: bool, depth: usize, out: &mut Vec<String>) {
    let Some(node) = named(schema, node) else { return };
    if depth > 10 {
        return;
    }
    for kind in ["oneOf", "anyOf", "allOf"] {
        for branch in node.get(kind).and_then(Value::as_array).into_iter().flatten() {
            let props = branch.get("properties").and_then(Value::as_object).map_or(0, |p| p.len());
            let required = branch.get("required").and_then(Value::as_array).map_or(0, Vec::len);
            gather(schema, branch, parent, kind == "oneOf" && props == 1 && required == 1, depth + 1, out);
        }
    }
    for list in [node.get("items"), node.get("additionalProperties")].into_iter().flatten() {
        if list.is_object() {
            gather(schema, list, parent, false, depth + 1, out);
        }
    }
    for (name, child) in node.get("properties").and_then(Value::as_object).into_iter().flatten() {
        if !tag {
            out.push(format!("{parent}.{name}"));
        }
        gather(schema, child, name, false, depth + 1, out);
    }
}

pub fn schema_keys(schema: &Value) -> Vec<String> {
    let mut out = Vec::new();
    gather(schema, &serde_json::json!({ "$ref": "#/$defs/System" }), "system", false, 0, &mut out);
    out.sort();
    out.dedup();
    out
}
