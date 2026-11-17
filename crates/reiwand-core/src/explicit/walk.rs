use serde_json::{Map, Value, json};

static NULL: Value = Value::Null;

pub fn deref<'a>(root: &'a Value, mut schema: &'a Value) -> (&'a Value, &'a str) {
    let mut name = "";
    while let Some(target) = schema.get("$ref").and_then(Value::as_str) {
        name = target.rsplit('/').next().unwrap_or_default();
        schema = root.pointer(target.trim_start_matches('#')).unwrap_or(&NULL);
    }
    (schema, name)
}

fn kind(value: &Value) -> &'static str {
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

pub fn branches(schema: &Value) -> &[Value] {
    let list = schema.get("anyOf").or_else(|| schema.get("oneOf")).and_then(Value::as_array);
    list.map_or(&[], Vec::as_slice)
}

pub fn fits(root: &Value, schema: &Value, value: &Value) -> bool {
    let (schema, _) = deref(root, schema);
    if let Some(c) = schema.get("const") {
        return c == value;
    }
    if let Some(options) = schema.get("enum").and_then(Value::as_array) {
        return options.contains(value);
    }
    let types: Vec<&str> = match &schema["type"] {
        Value::String(t) => vec![t.as_str()],
        Value::Array(list) => list.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    };
    let named = kind(value);
    if !types.is_empty() && !types.iter().any(|t| *t == named || (*t == "number" && named == "integer")) {
        return false;
    }
    let n = value.as_array().map(|l| l.len() as u64);
    let bound = |key: &str| schema[key].as_u64().zip(n);
    if bound("minItems").is_some_and(|(m, n)| n < m) || bound("maxItems").is_some_and(|(m, n)| n > m) {
        return false;
    }
    let item = |i: usize| schema["prefixItems"].get(i).or(schema.get("items").filter(|s| s.is_object()));
    let mut items = value.as_array().into_iter().flatten().enumerate();
    if !items.all(|(i, v)| item(i).is_none_or(|s| fits(root, s, v))) {
        return false;
    }
    if let (Some(props), Some(object)) = (schema["properties"].as_object(), value.as_object()) {
        let tag = props.iter().find(|(_, p)| p.get("const").is_some());
        if let Some((key, p)) = tag {
            return object.get(key) == p.get("const");
        }
        let required = schema["required"].as_array().into_iter().flatten().filter_map(Value::as_str);
        let closed = schema["additionalProperties"] == false;
        return required.clone().all(|k| object.contains_key(k))
            && (!closed || object.keys().all(|k| props.contains_key(k)));
    }
    let list = branches(schema);
    list.is_empty() || list.iter().any(|b| fits(root, b, value))
}

pub fn choose<'a>(root: &'a Value, schema: &'a Value, value: &Value) -> &'a Value {
    let (schema, _) = deref(root, schema);
    let list = branches(schema);
    let usable = |b: &&Value| value.is_null() || deref(root, b).0["type"] != "null";
    list.iter()
        .filter(usable)
        .find(|b| fits(root, b, value))
        .or(list.iter().find(|b| usable(b)))
        .unwrap_or(schema)
}

pub fn properties(root: &Value, schema: &Value, value: &Value) -> Map<String, Value> {
    let (schema, _) = deref(root, schema);
    let mut props = schema["properties"].as_object().cloned().unwrap_or_default();
    if let Some(variant) = schema.get("oneOf").filter(|_| !props.is_empty()) {
        let found = variant.as_array().into_iter().flatten().find(|v| fits(root, v, value));
        props.extend(found.and_then(|v| v["properties"].as_object()).cloned().unwrap_or_default());
    }
    props
}

pub fn child<'a>(root: &'a Value, schema: &'a Value, token: &str, value: &Value) -> Option<Value> {
    let (mut chosen, _) = deref(root, schema);
    while !chosen["properties"].is_object() && !branches(chosen).is_empty() {
        let next = deref(root, choose(root, chosen, value)).0;
        if std::ptr::eq(next, chosen) {
            break;
        }
        chosen = next;
    }
    if let Some(p) = properties(root, chosen, value).get(token) {
        return Some(p.clone());
    }
    if let Some(extra) = chosen.get("additionalProperties").filter(|a| a.is_object()) {
        return Some(extra.clone());
    }
    let i = token.parse::<usize>().ok();
    let prefix = i.and_then(|i| chosen["prefixItems"].get(i)).cloned();
    prefix.or_else(|| chosen.get("items").cloned())
}

pub fn sample(root: &Value, schema: &Value) -> Value {
    if let Some(d) = schema.get("default") {
        return d.clone();
    }
    let (schema, _) = deref(root, schema);
    if let Some(d) = schema.get("default").or_else(|| schema.get("const")) {
        return d.clone();
    }
    if let Some(first) = schema["enum"].get(0) {
        return first.clone();
    }
    if let Some(first) = branches(schema).first() {
        return sample(root, first);
    }
    let t = match &schema["type"] {
        Value::Array(list) => list.first().and_then(Value::as_str).unwrap_or("null"),
        t => t.as_str().unwrap_or("object"),
    };
    match t {
        "boolean" => json!(false),
        "integer" | "number" => schema.get("minimum").cloned().unwrap_or(json!(0)),
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

pub fn fill(root: &Value, schema: &Value, value: &mut Value) {
    match value {
        Value::Object(map) => {
            let (resolved, _) = deref(root, schema);
            let shown = Value::Object(map.clone());
            let chosen = if resolved["properties"].is_object() {
                resolved
            } else {
                let objects = branches(resolved).iter().map(|b| deref(root, b).0);
                let shaped = objects.clone().find(|b| b["properties"].is_object() && fits(root, b, &shown));
                shaped
                    .or_else(|| objects.clone().find(|b| b["properties"].is_object()))
                    .unwrap_or_else(|| deref(root, choose(root, resolved, &shown)).0)
            };
            let props = properties(root, chosen, &shown);
            let mut required: Vec<&str> =
                chosen["required"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
            let variant = chosen["oneOf"].as_array().into_iter().flatten().find(|v| fits(root, v, &shown));
            required.extend(
                variant
                    .map_or(&[][..], |v| v["required"].as_array().map_or(&[], Vec::as_slice))
                    .iter()
                    .filter_map(Value::as_str),
            );
            for (key, prop) in &props {
                if !map.contains_key(key) && required.contains(&key.as_str()) {
                    map.insert(key.clone(), sample(root, prop));
                }
            }
            for (key, inner) in map.iter_mut() {
                if let Some(next) = child(root, chosen, key, &shown) {
                    fill(root, &next, inner);
                }
            }
        }
        Value::Array(list) => {
            for (i, inner) in list.iter_mut().enumerate() {
                if let Some(next) = child(root, schema, &i.to_string(), &Value::Null) {
                    fill(root, &next, inner);
                }
            }
        }
        _ => {}
    }
}
