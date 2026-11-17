use crate::System;
use reiwand_core::explicit::{Fault, read};
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
