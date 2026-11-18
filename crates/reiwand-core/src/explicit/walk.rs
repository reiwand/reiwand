use super::loose::kind;
pub use super::sample::sample;
use serde_json::{Map, Value};

static NULL: Value = Value::Null;

pub fn deref<'a>(root: &'a Value, mut schema: &'a Value) -> (&'a Value, &'a str) {
    let mut name = "";
    while let Some(target) = schema.get("$ref").and_then(Value::as_str) {
        name = target.rsplit('/').next().unwrap_or_default();
        schema = root.pointer(target.trim_start_matches('#')).unwrap_or(&NULL);
    }
    (schema, name)
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
    let extra = schema.get("additionalProperties").filter(|a| a.is_object());
    if let (Some(extra), Some(object)) = (extra, value.as_object()) {
        return object.values().all(|v| fits(root, extra, v));
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
        .or(list.iter().filter(usable).find(|b| super::loose::loose(root, b, value)))
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

fn objects_of<'a>(root: &'a Value, schema: &'a Value) -> Vec<&'a Value> {
    let mut found = Vec::new();
    for branch in branches(schema) {
        let (branch, _) = deref(root, branch);
        if branch["properties"].is_object() {
            found.extend([branch]);
        } else {
            found.extend(objects_of(root, branch));
        }
    }
    found
}

pub fn fill(root: &Value, schema: &Value, value: &mut Value) {
    match value {
        Value::Object(map) => {
            let (resolved, _) = deref(root, schema);
            let shown = Value::Object(map.clone());
            let chosen = if resolved["properties"].is_object() {
                resolved
            } else {
                let objects = objects_of(root, resolved);
                let fitting = objects.iter().copied().find(|b| fits(root, b, &shown));
                let picked = deref(root, choose(root, resolved, &shown)).0;
                let covers = |b: &&Value| {
                    b["properties"].as_object().is_some_and(|p| map.keys().all(|k| p.contains_key(k)))
                };
                fitting
                    .or_else(|| objects.iter().copied().find(covers))
                    .or_else(|| Some(picked).filter(|p| fits(root, p, &shown)))
                    .or_else(|| objects.first().copied())
                    .unwrap_or(picked)
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
            super::pad::pad(root, schema, list);
            let whole = Value::Array(list.clone());
            for (i, inner) in list.iter_mut().enumerate() {
                if let Some(next) = child(root, schema, &i.to_string(), &whole) {
                    fill(root, &next, inner);
                }
            }
        }
        _ => {}
    }
}
