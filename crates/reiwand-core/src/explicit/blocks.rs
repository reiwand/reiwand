use super::omit::steps;
use super::walk::{child, properties, settled};
use serde_json::Value;

fn is_block(schema: &Value) -> bool {
    schema.get("x-block") == Some(&Value::Bool(true))
}

pub fn block_at(root: &Value, schema: &Value, sample: &Value, path: &str) -> bool {
    let tokens = steps(path);
    let Some((last, parents)) = tokens.split_last() else { return false };
    let (mut schema, mut value) = (schema.clone(), sample);
    for token in parents {
        let Some(next) = child(root, &schema, token, value) else { return false };
        let inner = match value {
            Value::Array(items) => token.parse::<usize>().ok().and_then(|i| items.get(i)),
            other => other.get(token),
        };
        let Some(inner) = inner else { return false };
        (schema, value) = (next, inner);
    }
    properties(root, settled(root, &schema, value), value).get(last).is_some_and(is_block)
}

pub fn null_blocks(root: &Value, schema: &Value, doc: &Value) -> Vec<String> {
    let mut found = Vec::new();
    visit(root, schema, doc, "", &mut found);
    found
}

fn visit(root: &Value, schema: &Value, value: &Value, at: &str, found: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            let props = properties(root, settled(root, schema, value), value);
            for (key, inner) in map {
                let path = if at.is_empty() { key.clone() } else { format!("{at}.{key}") };
                if inner.is_null() && props.get(key).is_some_and(is_block) {
                    found.push(path.clone());
                }
                if let Some(next) = child(root, schema, key, value) {
                    visit(root, &next, inner, &path, found);
                }
            }
        }
        Value::Array(items) => {
            for (i, inner) in items.iter().enumerate() {
                if let Some(next) = child(root, schema, &i.to_string(), value) {
                    visit(root, &next, inner, &format!("{at}[{i}]"), found);
                }
            }
        }
        _ => {}
    }
}

pub fn prune_blocks(root: &Value, schema: &Value, value: &mut Value) {
    match value {
        Value::Object(map) => {
            let shown = Value::Object(map.clone());
            let props = properties(root, settled(root, schema, &shown), &shown);
            map.retain(|key, inner| !(inner.is_null() && props.get(key).is_some_and(is_block)));
            for (key, inner) in map.iter_mut() {
                if let Some(next) = child(root, schema, key, &shown) {
                    prune_blocks(root, &next, inner);
                }
            }
        }
        Value::Array(items) => {
            let whole = Value::Array(items.clone());
            for (i, inner) in items.iter_mut().enumerate() {
                if let Some(next) = child(root, schema, &i.to_string(), &whole) {
                    prune_blocks(root, &next, inner);
                }
            }
        }
        _ => {}
    }
}
