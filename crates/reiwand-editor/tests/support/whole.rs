#![allow(dead_code)]
use reiwand_editor::Project;
use reiwand_editor::completing::complete;
use reiwand_editor::ops::Op;
use serde_json::Value;

pub fn at(project: &Project, pointer: &str, value: &Value) -> Value {
    if !pointer.starts_with("/scene") {
        return value.clone();
    }
    let landing = match pointer.rsplit_once('/') {
        Some((parent, "-")) => {
            format!("{parent}/{}", project.doc.pointer(parent).and_then(Value::as_array).map_or(0, Vec::len))
        }
        _ => pointer.to_owned(),
    };
    match complete(&project.doc, Op::Replace { path: landing, value: value.clone() }) {
        Op::Replace { value, .. } => value,
        _ => value.clone(),
    }
}

pub fn action(project: &Project, action: Value) -> Value {
    let mut action = action;
    for verb in ["set", "add"] {
        let Some(body) = action.get_mut(verb) else { continue };
        let (Some(pointer), Some(value)) = (body.get("pointer").and_then(Value::as_str), body.get("value"))
        else {
            continue;
        };
        let filled = at(project, pointer, value);
        body["value"] = filled;
    }
    action
}

pub fn empty(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Array(list) => list.is_empty(),
        Value::Object(map) => map.is_empty(),
        _ => false,
    }
}

pub fn same(project: &Project, pointer: &str, expected: Value) -> Value {
    at(project, pointer, &expected)
}
