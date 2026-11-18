use super::walk::{branches, deref};
use serde_json::{Value, json};

pub fn sample(root: &Value, schema: &Value) -> Value {
    if let Some(d) = schema.get("default") {
        return d.clone();
    }
    if schema["x-condition"] == true {
        return json!({});
    }
    let (schema, _) = deref(root, schema);
    if let Some(d) = schema.get("default").or_else(|| schema.get("const")) {
        return d.clone();
    }
    if schema["x-null"] == true {
        return Value::Null;
    }
    if let Some(first) = schema["enum"].get(0) {
        return first.clone();
    }
    if branches(schema).iter().any(|b| deref(root, b).0["type"] == "null") {
        return Value::Null;
    }
    if let Some(first) = branches(schema).first() {
        return sample(root, first);
    }
    let t = match &schema["type"] {
        Value::Array(list) if list.iter().any(|t| t == "null") => "null",
        Value::Array(list) => list.first().and_then(Value::as_str).unwrap_or("null"),
        t => t.as_str().unwrap_or("object"),
    };
    match t {
        "boolean" => json!(false),
        "integer" | "number" => schema.get("minimum").cloned().unwrap_or(json!(0)),
        "string" if schema["pattern"].as_str().is_some_and(|p| p.starts_with("^#")) => json!("#ffffff"),
        "string" => json!(""),
        "array" => {
            let n = schema["minItems"].as_u64().unwrap_or(0) as usize;
            let items = (0..n).map(|i| {
                schema["prefixItems"].get(i).or(schema.get("items")).map_or(Value::Null, |s| sample(root, s))
            });
            Value::Array(items.collect())
        }
        "object" => {
            let required = schema["required"].as_array().into_iter().flatten().filter_map(Value::as_str);
            let props = &schema["properties"];
            Value::Object(required.map(|k| (k.to_owned(), sample(root, &props[k]))).collect())
        }
        _ => Value::Null,
    }
}
