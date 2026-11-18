use serde_json::Value;

pub fn all_required(schema: &Value) -> Vec<String> {
    let mut found = Vec::new();
    walk(schema, "", &mut found);
    found
}

fn walk(node: &Value, at: &str, found: &mut Vec<String>) {
    match node {
        Value::Object(map) => {
            if let Some(Value::Object(props)) = map.get("properties") {
                let required: Vec<&str> = map
                    .get("required")
                    .and_then(Value::as_array)
                    .map(|a| a.iter().filter_map(Value::as_str).collect())
                    .unwrap_or_default();
                let allowed = |k: &String| props[k.as_str()].get("x-block") == Some(&Value::Bool(true));
                for name in props.keys().filter(|k| !required.contains(&k.as_str()) && !allowed(k)) {
                    found.push(format!("{at}/properties/{name}"));
                }
            }
            for (key, child) in map {
                walk(child, &format!("{at}/{key}"), found);
            }
        }
        Value::Array(items) => {
            for (i, child) in items.iter().enumerate() {
                walk(child, &format!("{at}/{i}"), found);
            }
        }
        _ => {}
    }
}
