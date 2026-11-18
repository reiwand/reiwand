use crate::ops::Op;
use crate::schema::{at, branches, deref, properties, root_of, sample};
use serde_json::Value;

pub(crate) fn blank(doc: &Value, path: &str) -> Option<Op> {
    let (parent, key) = path.rsplit_once('/')?;
    let (root, _) =
        root_of(path).filter(|_| path.starts_with("/scene/") || path.starts_with("/manifest/"))?;
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
    let nullable =
        prop["x-null"] == true || branches(&prop).iter().any(|b| deref(root, b).0["type"] == "null");
    let value = if nullable { Value::Null } else { sample(root, &prop) };
    required.then(|| Op::Replace { path: path.to_owned(), value })
}
