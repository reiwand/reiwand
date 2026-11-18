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
    match op {
        Op::Add { path, .. } => Op::Add { path, value: full },
        Op::Replace { path, .. } => Op::Replace { path, value: full },
        other => other,
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
    ops.into_iter().map(|op| complete(doc, op)).collect()
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
    let prop = properties(root, &schema, shown).get(key)?.clone();
    doc.pointer(path)?;
    let required = crate::schema::deref(root, &schema).0["required"].as_array()?.iter().any(|r| r == key);
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
