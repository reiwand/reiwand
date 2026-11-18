#![allow(dead_code)]
use reiwand_editor::Project;
use reiwand_editor::completing::complete;
use reiwand_editor::ops::Op;
use serde_json::Value;

pub fn at(project: &Project, pointer: &str, value: &Value) -> Value {
    at_doc(&project.doc, pointer, value)
}

pub fn at_doc(doc: &Value, pointer: &str, value: &Value) -> Value {
    if !pointer.starts_with("/scene") && !pointer.starts_with("/manifest") {
        return value.clone();
    }
    let landing = match pointer.rsplit_once('/') {
        Some((parent, "-")) => {
            format!("{parent}/{}", doc.pointer(parent).and_then(Value::as_array).map_or(0, Vec::len))
        }
        _ => pointer.to_owned(),
    };
    match complete(doc, Op::Replace { path: landing, value: value.clone() }) {
        Op::Replace { value, .. } => value,
        _ => value.clone(),
    }
}

pub fn action(project: &Project, action: Value) -> Value {
    action_doc(&project.doc, action)
}

pub fn action_doc(doc: &Value, action: Value) -> Value {
    let mut action = action;
    for verb in ["set", "add"] {
        let Some(body) = action.get_mut(verb) else { continue };
        let (Some(pointer), Some(value)) = (body.get("pointer").and_then(Value::as_str), body.get("value"))
        else {
            continue;
        };
        let filled = at_doc(doc, pointer, value);
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

pub fn document(doc: &mut Value) {
    reiwand_editor::completing::scene_doc(doc);
    reiwand_editor::completing::manifest_doc(doc);
    let mut paths: Vec<String> = Vec::new();
    for key in ["properties", "presets"] {
        match doc["manifest"].get(key) {
            Some(Value::Object(map)) => paths.extend(map.keys().map(|k| format!("/manifest/{key}/{k}"))),
            Some(Value::Array(list)) => paths.extend((0..list.len()).map(|i| format!("/manifest/{key}/{i}"))),
            _ => {}
        }
    }
    for path in paths {
        let Some(old) = doc.pointer(&path).cloned() else { continue };
        if let Op::Replace { value, .. } = complete(doc, Op::Replace { path: path.clone(), value: old }) {
            *doc.pointer_mut(&path).unwrap() = value;
        }
    }
}

pub fn holds(actual: &Value, expected: Value) {
    let Some(map) = expected.as_object() else { return assert_eq!(actual, &expected) };
    for (key, value) in map {
        assert_eq!(&actual[key], value, "{key} of {actual}");
    }
}

pub fn descriptor(text: &str) -> String {
    let mut json: Value = serde_json::from_str(text).unwrap();
    reiwand_upgrade::descriptors::complete(&mut json);
    json.to_string()
}

pub fn unset(value: &Value) -> bool {
    matches!(value, Value::Bool(false)) || empty(value)
}

pub fn manifest(value: Value) -> Value {
    let text = reiwand_upgrade::manifests::whole(&value.to_string());
    serde_json::from_str(&text).unwrap()
}
