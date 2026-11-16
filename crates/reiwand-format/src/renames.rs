use serde_json::{Value, json};
use std::collections::BTreeMap;

pub type Renames = BTreeMap<String, String>;

pub fn rename_json(manifest: &mut Value, scene: Option<&mut Value>, renames: &Renames) {
    rekey(manifest.get_mut("properties"), renames);
    let props = manifest.get_mut("properties").and_then(Value::as_object_mut);
    for prop in props.into_iter().flat_map(|p| p.values_mut()) {
        if let Some(group) = prop.get("group").and_then(Value::as_str).and_then(|g| renames.get(g)) {
            prop["group"] = json!(group);
        }
        for key in ["visible_if", "enabled_if"] {
            match prop.get_mut(key) {
                Some(Value::String(text)) => {
                    *text =
                        renames.iter().fold(text.clone(), |t, (from, to)| reiwand_scene::rename(&t, from, to))
                }
                found => rekey(found, renames),
            }
        }
    }
    for preset in manifest.get_mut("presets").and_then(Value::as_array_mut).into_iter().flatten() {
        rekey(preset.get_mut("values"), renames);
    }
    if let Some(name) = manifest.pointer_mut("/display/accent/prop")
        && let Some(to) = name.as_str().and_then(|n| renames.get(n))
    {
        *name = json!(to);
    }
    scene.into_iter().for_each(|scene| bind(scene, renames));
}

fn rekey(value: Option<&mut Value>, renames: &Renames) {
    let Some(map) = value.and_then(Value::as_object_mut) else { return };
    *map = std::mem::take(map).into_iter().map(|(k, v)| (renames.get(&k).cloned().unwrap_or(k), v)).collect();
}

fn bind(value: &mut Value, renames: &Renames) {
    match value {
        Value::Object(map) => map.iter_mut().for_each(|(key, v)| {
            let next = match (key.as_str(), v.as_str()) {
                ("prop", Some(id)) => renames.get(id).map(|n| json!(n)),
                ("trigger", Some(t)) => {
                    t.strip_prefix("prop:").and_then(|id| renames.get(id)).map(|n| json!(format!("prop:{n}")))
                }
                _ => None,
            };
            match next {
                Some(next) => *v = next,
                None => bind(v, renames),
            }
        }),
        Value::Array(list) => list.iter_mut().for_each(|v| bind(v, renames)),
        _ => {}
    }
}
