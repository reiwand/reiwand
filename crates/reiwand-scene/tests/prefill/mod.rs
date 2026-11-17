#![allow(dead_code)]
use reiwand_core::explicit::{Fault, read};
use reiwand_scene::System;
use serde_json::Value;
use std::collections::BTreeMap;

pub const TABLE: &str = include_str!("particles.tsv");

pub fn rows() -> BTreeMap<&'static str, Option<Value>> {
    TABLE
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .map(|(key, text)| (key, (text != "!").then(|| serde_json::from_str(text).unwrap())))
        .collect()
}

fn key(path: &str) -> String {
    let names: Vec<&str> = path
        .split('.')
        .map(|part| part.split('[').next().unwrap_or(part))
        .filter(|part| !part.is_empty())
        .collect();
    match names.as_slice() {
        [only] => format!("system.{only}"),
        [.., parent, field] => format!("{parent}.{field}"),
        [] => String::new(),
    }
}

fn slot<'a>(root: &'a mut Value, path: &str) -> Option<&'a mut Value> {
    let mut at = root;
    for part in path.split('.') {
        let (name, indexes) = part.split_once('[').map_or((part, ""), |(n, i)| (n, i));
        if !name.is_empty() {
            at = at.get_mut(name)?;
        }
        for index in indexes.split('[').filter(|i| !i.is_empty()) {
            at = at.get_mut(index.trim_end_matches(']').parse::<usize>().ok()?)?;
        }
    }
    Some(at)
}

pub fn completed(mut system: Value) -> Value {
    let table = rows();
    loop {
        let Err(e) = read::<System>("system", &system) else { return system };
        if e.fault != Fault::Missing {
            let Some(at) = slot(&mut system, &e.path) else { return system };
            match at.as_array_mut() {
                Some(list) if list.len() == 2 => {
                    list.push(Value::from(1.0));
                    continue;
                }
                _ => return system,
            }
        }
        let name = key(&e.path);
        let Some(value) = table
            .get(name.as_str())
            .unwrap_or_else(|| panic!("no prefill row for {name} ({})", e.path))
            .clone()
        else {
            return system;
        };
        let (parent, field) = e.path.rsplit_once('.').unwrap_or(("", &e.path));
        let Some(target) = (if parent.is_empty() { Some(&mut system) } else { slot(&mut system, parent) })
        else {
            return system;
        };
        target[field] = value;
    }
}

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

pub fn system(value: Value) -> System {
    serde_json::from_value(completed(value)).unwrap()
}

pub fn all(systems: Value) -> Value {
    let mut systems = systems;
    for (_, one) in systems.as_object_mut().unwrap().iter_mut() {
        *one = completed(one.take());
    }
    systems
}

pub fn render(render: Value) -> Value {
    completed(serde_json::json!({ "capacity": 1, "emitters": [], "render": render }))["render"].clone()
}

pub fn instance(mut one: Value) -> Value {
    for name in ["alpha", "size", "count", "speed", "life", "time_scale", "brightness", "color"] {
        one.as_object_mut().unwrap().entry(name).or_insert(Value::Null);
    }
    one.as_object_mut().unwrap().entry("points").or_insert_with(|| serde_json::json!([]));
    one
}
