#![allow(dead_code)]
use reiwand_editor::Project;
use reiwand_upgrade::fill::{Format, complete_with};
use reiwand_upgrade::scenes::customs;
use serde_json::Value;

fn place(scene: &mut Value, pointer: &str, value: &Value) -> bool {
    if let Some(slot) = scene.pointer_mut(pointer) {
        *slot = value.clone();
        return true;
    }
    let Some((parent, last)) = pointer.rsplit_once('/') else { return false };
    match scene.pointer_mut(parent) {
        Some(Value::Array(list)) => {
            list.push(value.clone());
            true
        }
        Some(Value::Object(map)) => {
            map.insert(last.to_owned(), value.clone());
            true
        }
        _ => false,
    }
}

pub fn at(project: &Project, pointer: &str, value: &Value) -> Value {
    let Some(inner) = pointer.strip_prefix("/scene") else { return value.clone() };
    let mut scene = project.doc["scene"].clone();
    let landing = match inner.rsplit_once('/') {
        Some((parent, "-")) => {
            format!("{parent}/{}", scene.pointer(parent).and_then(Value::as_array).map_or(0, Vec::len))
        }
        _ => inner.to_owned(),
    };
    if !place(&mut scene, &landing, value) {
        return value.clone();
    }
    let found = customs(&project.dir, &scene);
    complete_with(Format::Scene, &mut scene, &found);
    scene.pointer(&landing).cloned().unwrap_or_else(|| value.clone())
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
