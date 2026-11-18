use crate::ops::Op;
use crate::schema::{at, branches, deref, fill, properties, root_of, sample};
use serde_json::Value;

pub fn complete(doc: &Value, op: Op) -> Op {
    let (path, value) = match &op {
        Op::Add { path, value } | Op::Replace { path, value } => (path, value),
        Op::Remove { path } => return blank(doc, path).unwrap_or(op),
        _ => return op,
    };
    let Some((root, _)) = root_of(path)
        .filter(|_| path.starts_with("/scene") || path == "/manifest" || path.starts_with("/manifest/"))
    else {
        return op;
    };
    let Some(schema) = at(doc, path) else { return op };
    let schema = unwrapped(root, schema);
    let mut full = value.clone();
    fill(root, &schema, &mut full);
    crate::contextual::fill(&mut full);
    if path.starts_with("/manifest") {
        no_conditions(&mut full);
    }
    match op {
        Op::Add { path, .. } => Op::Add { path, value: full },
        Op::Replace { path, .. } => Op::Replace { path, value: full },
        other => other,
    }
}

fn no_conditions(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, inner) in map.iter_mut() {
                if matches!(key.as_str(), "visible_if" | "enabled_if") && inner == "" {
                    *inner = Value::Object(Default::default());
                } else {
                    no_conditions(inner);
                }
            }
        }
        Value::Array(list) => list.iter_mut().for_each(no_conditions),
        _ => {}
    }
}

fn unwrapped(root: &Value, mut schema: Value) -> Value {
    loop {
        let list = branches(&schema);
        let present: Vec<&Value> = list.iter().filter(|b| deref(root, b).0["type"] != "null").collect();
        match present[..] {
            [only] if list.len() == 2 => schema = only.clone(),
            _ => return schema,
        }
    }
}

pub fn complete_all(doc: &Value, ops: Vec<Op>) -> Vec<Op> {
    let mut done: Vec<Op> = ops.into_iter().map(|op| complete(doc, op)).collect();
    if let Some(camera) = crate::contextual::camera(doc, &done) {
        done.extend([Op::Add { path: "/scene/camera3d".into(), value: camera }]);
    }
    done
}

pub fn manifest_doc(doc: &mut Value) {
    let Some(manifest) = doc.get("manifest").cloned() else { return };
    if let Op::Replace { value, .. } =
        complete(doc, Op::Replace { path: "/manifest".into(), value: manifest })
    {
        doc["manifest"] = value;
    }
}

pub fn scene_doc(doc: &mut Value) {
    let Some(scene) = doc.get("scene").cloned() else { return };
    if let Op::Replace { value, .. } = complete(doc, Op::Replace { path: "/scene".into(), value: scene }) {
        doc["scene"] = value;
    }
}

fn blank(doc: &Value, path: &str) -> Option<Op> {
    let (parent, key) = path.rsplit_once('/')?;
    let (root, _) = root_of(path).filter(|_| path.starts_with("/scene/"))?;
    let schema = at(doc, parent)?;
    let shown = doc.pointer(parent)?;
    let mut resolved = crate::schema::deref(root, &schema).0;
    while !resolved["properties"].is_object() && !crate::schema::branches(resolved).is_empty() {
        let next = crate::schema::deref(root, crate::schema::choose(root, resolved, shown)).0;
        if std::ptr::eq(next, resolved) {
            break;
        }
        resolved = next;
    }
    let prop = properties(root, resolved, shown).get(key)?.clone();
    doc.pointer(path)?;
    let variant =
        resolved["oneOf"].as_array().into_iter().flatten().find(|v| crate::schema::fits(root, v, shown));
    let listed_in = |s: &Value| s["required"].as_array().is_some_and(|r| r.iter().any(|r| r == key));
    let required = listed_in(resolved) || variant.is_some_and(listed_in);
    required.then(|| Op::Replace { path: path.to_owned(), value: sample(root, &prop) })
}

pub fn ops(doc: &Value) -> Vec<Op> {
    let mut found = Vec::new();
    let mut paths: Vec<String> = Vec::new();
    for id in doc["scene"]["particles"].as_object().into_iter().flatten().map(|(id, _)| id) {
        paths.push(format!("/scene/particles/{}", crate::ops::escape(id)));
    }
    layers(&doc["scene"]["layers"], "/scene/layers", &mut paths);
    for path in paths {
        let Some(old) = doc.pointer(&path) else { continue };
        let Op::Replace { value, .. } = complete(doc, Op::Replace { path: path.clone(), value: old.clone() })
        else {
            continue;
        };
        if &value != old {
            found.extend(crate::ops::set(doc, &path, value));
        }
    }
    found
}

fn layers(list: &Value, base: &str, out: &mut Vec<String>) {
    for (i, layer) in list.as_array().into_iter().flatten().enumerate() {
        let here = format!("{base}/{i}");
        if layer["instance"].is_object() {
            out.push(format!("{here}/instance"));
        }
        layers(&layer["layers"], &format!("{here}/layers"), out);
    }
}

pub fn require(doc: &Value, path: &str, value: &Value) -> anyhow::Result<()> {
    match lacking(doc, path, value) {
        Some(field) => anyhow::bail!(reiwand_core::message::said(
            "editor.missing_value",
            "%1 needs a value for %2",
            &[&path.rsplit('/').next().unwrap_or_default(), &field]
        )),
        None => Ok(()),
    }
}

pub fn lacking(doc: &Value, path: &str, value: &Value) -> Option<String> {
    let (root, _) = root_of(path)?;
    let schema = at(doc, path)?;
    gap(root, &schema, value, "")
}

fn gap(root: &Value, schema: &Value, value: &Value, base: &str) -> Option<String> {
    match value {
        Value::Object(map) => {
            let (resolved, _) = crate::schema::deref(root, schema);
            let shown = value.clone();
            let chosen = if resolved["properties"].is_object() {
                resolved
            } else {
                let objects =
                    crate::schema::branches(resolved).iter().map(|b| crate::schema::deref(root, b).0);
                objects
                    .clone()
                    .find(|b| b["properties"].is_object() && crate::schema::fits(root, b, &shown))
                    .or_else(|| objects.clone().find(|b| b["properties"].is_object()))
                    .unwrap_or(resolved)
            };
            let props = properties(root, chosen, &shown);
            let variant = chosen["oneOf"]
                .as_array()
                .into_iter()
                .flatten()
                .find(|v| crate::schema::fits(root, v, &shown));
            let listed = |s: &Value| s["required"].as_array().cloned().unwrap_or_default();
            let required = listed(chosen).into_iter().chain(variant.map(listed).unwrap_or_default());
            for key in required.filter_map(|k| k.as_str().map(str::to_owned)) {
                let given = map.contains_key(&key);
                let prop = props.get(&key);
                let fixed = prop.is_some_and(|p| {
                    p.get("default").or_else(|| crate::schema::deref(root, p).0.get("default")).is_some()
                        || sample(root, p).is_null()
                });
                if !given && !fixed {
                    return Some(format!("{base}/{key}"));
                }
            }
            map.iter().find_map(|(key, inner)| {
                let next = crate::schema::child(root, chosen, key, &shown)?;
                gap(root, &next, inner, &format!("{base}/{key}"))
            })
        }
        Value::Array(list) => list.iter().enumerate().find_map(|(i, inner)| {
            let next = crate::schema::child(root, schema, &i.to_string(), &Value::Null)?;
            gap(root, &next, inner, &format!("{base}/{i}"))
        }),
        _ => None,
    }
}
