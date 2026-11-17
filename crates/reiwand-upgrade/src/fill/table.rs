use super::{Customs, Format, legacy, walk::Walk};
use serde_json::Value;

pub struct Row {
    pub format: &'static str,
    pub def: String,
    pub prop: String,
    pub shape: String,
    pub required: bool,
    pub value: Value,
    pub class: u8,
}

fn class(def: &str, prop: &str, value: &Value) -> u8 {
    let empty = value.is_null()
        || value.as_array().is_some_and(Vec::is_empty)
        || value.as_object().is_some_and(|o| o.is_empty());
    if legacy::contextual(def, prop) {
        2
    } else if empty {
        3
    } else {
        1
    }
}

pub fn rows(format: Format) -> Vec<Row> {
    let root = format.frozen();
    let customs = Customs::new();
    let walk = Walk::new(root, &customs, false);
    let title = root.get("title").and_then(Value::as_str).unwrap_or("").to_string();
    let mut out = Vec::new();
    let mut add = |def: &str, schema: &Value| {
        let required = |n: &str| {
            schema.get("required").and_then(Value::as_array).is_some_and(|r| r.iter().any(|x| x == n))
        };
        for (prop, ps) in schema.get("properties").and_then(Value::as_object).into_iter().flatten() {
            let Some(value) =
                legacy::heard_value(def, prop).or_else(|| walk.fallback(ps, required(prop), def, prop))
            else {
                continue;
            };
            let row = Row {
                format: format.name(),
                def: def.to_string(),
                prop: prop.clone(),
                shape: walk.describe(ps),
                required: required(prop),
                class: class(def, prop, &value),
                value,
            };
            out.push(row);
        }
    };
    add(&title, root);
    for (def, schema) in root.get("$defs").and_then(Value::as_object).into_iter().flatten() {
        add(def, schema);
        for branch in
            ["oneOf", "anyOf"].iter().filter_map(|k| schema.get(k)).filter_map(Value::as_array).flatten()
        {
            add(def, branch);
        }
    }
    out.sort_by(|a, b| (&a.def, &a.prop, a.required).cmp(&(&b.def, &b.prop, b.required)));
    out.dedup_by(|a, b| a.def == b.def && a.prop == b.prop);
    out
}

pub fn tsv(format: Format) -> String {
    rows(format)
        .iter()
        .map(|r| {
            let (req, value) = (if r.required { "required" } else { "optional" }, r.value.to_string());
            format!("{}\t{}.{}\t{}\t{}\t{}\t{}\n", r.format, r.def, r.prop, r.shape, req, r.class, value)
        })
        .collect()
}
