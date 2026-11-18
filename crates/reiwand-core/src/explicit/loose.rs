use super::walk::{branches, deref};
use serde_json::Value;

pub(super) fn loose(root: &Value, schema: &Value, value: &Value) -> bool {
    let (schema, _) = deref(root, schema);
    let list = branches(schema);
    if !list.is_empty() {
        return list.iter().any(|b| loose(root, b, value));
    }
    let types: Vec<&str> = match &schema["type"] {
        Value::String(t) => vec![t.as_str()],
        Value::Array(l) => l.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    };
    let named = kind(value);
    if !types.is_empty() && !types.iter().any(|t| *t == named || (*t == "number" && named == "integer")) {
        return false;
    }
    let item = |i: usize| schema["prefixItems"].get(i).or(schema.get("items").filter(|s| s.is_object()));
    value.as_array().into_iter().flatten().enumerate().all(|(i, v)| item(i).is_none_or(|s| loose(root, s, v)))
}

pub(super) fn kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(n) if n.is_i64() || n.is_u64() => "integer",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
