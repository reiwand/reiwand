use super::walk::{branches, deref};
use serde_json::Value;

fn tuples<'a>(root: &'a Value, schema: &'a Value, found: &mut Vec<&'a Value>) {
    let (schema, _) = deref(root, schema);
    if schema["prefixItems"].is_array() {
        found.push(schema);
    }
    for branch in branches(schema) {
        tuples(root, branch, found);
    }
}

pub(super) fn pad(root: &Value, schema: &Value, list: &mut Vec<Value>) {
    let mut found = Vec::new();
    tuples(root, schema, &mut found);
    let wanted = found.iter().filter_map(|t| t["minItems"].as_u64()).max().unwrap_or(0) as usize;
    while list.len() < wanted {
        let index = list.len();
        let default = found.iter().find_map(|t| deref(root, &t["prefixItems"][index]).0.get("default"));
        match default {
            Some(d) => list.push(d.clone()),
            None => break,
        }
    }
}
