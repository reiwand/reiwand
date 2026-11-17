use serde_json::{Map, Value};

pub struct Prefill(pub &'static [(&'static str, &'static str)]);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Annotated {
    pub unknown: Vec<String>,
    pub uncovered: Vec<String>,
}

pub fn all_required(schema: &Value) -> Vec<String> {
    let mut found = Vec::new();
    walk(schema, "", &mut found);
    found
}

fn walk(node: &Value, at: &str, found: &mut Vec<String>) {
    match node {
        Value::Object(map) => {
            if let Some(Value::Object(props)) = map.get("properties") {
                let required: Vec<&str> = map
                    .get("required")
                    .and_then(Value::as_array)
                    .map(|a| a.iter().filter_map(Value::as_str).collect())
                    .unwrap_or_default();
                for name in props.keys().filter(|k| !required.contains(&k.as_str())) {
                    found.push(format!("{at}/properties/{name}"));
                }
            }
            for (key, child) in map {
                walk(child, &format!("{at}/{key}"), found);
            }
        }
        Value::Array(items) => {
            for (i, child) in items.iter().enumerate() {
                walk(child, &format!("{at}/{i}"), found);
            }
        }
        _ => {}
    }
}

fn type_names(schema: &Value) -> Vec<String> {
    let defs = schema.get("$defs").and_then(Value::as_object);
    let title = schema.get("title").and_then(Value::as_str);
    defs.into_iter().flat_map(|d| d.keys().cloned()).chain(title.map(str::to_owned)).collect()
}

fn object_of<'a>(schema: &'a mut Value, name: &str) -> Option<&'a mut Map<String, Value>> {
    if schema.get("$defs").and_then(|d| d.get(name)).is_some() {
        return schema.get_mut("$defs")?.get_mut(name)?.as_object_mut();
    }
    schema.as_object_mut().filter(|root| root.get("title").and_then(Value::as_str) == Some(name))
}

pub fn annotate(schema: &mut Value, table: &Prefill) -> Annotated {
    let mut result = Annotated::default();
    for (row, text) in table.0 {
        let hit = row.split_once('.').and_then(|(ty, field)| {
            object_of(schema, ty)?.get_mut("properties")?.as_object_mut()?.get_mut(field)
        });
        match (hit, serde_json::from_str::<Value>(text)) {
            (Some(Value::Object(property)), Ok(value)) => {
                property.insert("default".into(), value);
            }
            _ => result.unknown.push((*row).to_owned()),
        }
    }
    for name in type_names(schema) {
        let Some(object) = object_of(schema, &name) else { continue };
        let Some(Value::Object(props)) = object.get("properties") else { continue };
        let mut fields: Vec<String> = props.keys().cloned().collect();
        fields.sort();
        for field in &fields {
            if !table.0.iter().any(|(row, _)| *row == format!("{name}.{field}")) {
                result.uncovered.push(format!("{name}.{field}"));
            }
        }
        object.insert("required".into(), fields.into_iter().map(Value::String).collect());
    }
    result
}
