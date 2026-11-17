use crate::ops::Op;
use crate::schema::{at, child, choose, deref, fits, properties, root_of, sample};
use serde_json::Value;

fn fill(root: &Value, schema: &Value, value: &mut Value) {
    match value {
        Value::Object(map) => {
            let (resolved, _) = deref(root, schema);
            let shown = Value::Object(map.clone());
            let chosen = if resolved["properties"].is_object() {
                resolved
            } else {
                let objects = crate::schema::branches(resolved).iter().map(|b| deref(root, b).0);
                let shaped = objects
                    .clone()
                    .find(|b| b["properties"].is_object() && crate::schema::fits(root, b, &shown));
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

pub fn complete(doc: &Value, op: Op) -> Op {
    let (path, value) = match &op {
        Op::Add { path, value } | Op::Replace { path, value } => (path, value),
        _ => return op,
    };
    let Some((root, _)) = root_of(path).filter(|_| path.starts_with("/scene")) else { return op };
    let Some(schema) = at(doc, path) else { return op };
    let mut full = value.clone();
    fill(root, &schema, &mut full);
    match op {
        Op::Add { path, .. } => Op::Add { path, value: full },
        Op::Replace { path, .. } => Op::Replace { path, value: full },
        other => other,
    }
}

pub fn complete_all(doc: &Value, ops: Vec<Op>) -> Vec<Op> {
    ops.into_iter().map(|op| complete(doc, op)).collect()
}

pub fn scene_doc(doc: &mut Value) {
    let Some(scene) = doc.get("scene").cloned() else { return };
    if let Op::Replace { value, .. } = complete(doc, Op::Replace { path: "/scene".into(), value: scene }) {
        doc["scene"] = value;
    }
}
