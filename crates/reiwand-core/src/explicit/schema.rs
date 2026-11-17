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

fn object_of<'a>(schema: &'a mut Value, name: &str) -> Option<&'a mut Value> {
    if schema.get("$defs").and_then(|d| d.get(name)).is_some() {
        return schema.get_mut("$defs")?.get_mut(name);
    }
    (schema.get("title").and_then(Value::as_str) == Some(name)).then_some(schema)
}

fn branches(node: &mut Value) -> impl Iterator<Item = &mut Value> {
    let mut found = Vec::new();
    if let Value::Object(map) = node {
        for (key, list) in map.iter_mut() {
            if key == "oneOf" || key == "anyOf" {
                found.extend(list.as_array_mut().into_iter().flatten().filter(|b| b.is_object()));
            }
        }
    }
    found.into_iter()
}

fn tagged(node: &Value, tag: Option<&str>) -> bool {
    let Some(tag) = tag else { return true };
    let own = node.pointer("/properties/type/const").and_then(Value::as_str);
    own == Some(tag)
}

fn fill(node: &mut Value, field: &str, tag: Option<&str>, value: &Value) -> bool {
    let mut hit = false;
    if tagged(node, tag)
        && let Some(Value::Object(property)) = node.get_mut("properties").and_then(|p| p.get_mut(field))
    {
        if value.is_null() {
            allow_null(property);
        }
        property.insert("default".into(), value.clone());
        hit = true;
    }
    for branch in branches(node) {
        hit |= fill(branch, field, tag, value);
    }
    hit
}

fn widen(node: &mut Value) {
    if let Some(Value::Object(props)) = node.get_mut("properties") {
        for property in props.values_mut().filter_map(Value::as_object_mut) {
            if property.remove("x-null").is_some() {
                allow_null(property);
            }
        }
    }
    if let Value::Object(map) = node {
        for child in map.values_mut() {
            widen(child);
        }
    }
    if let Value::Array(list) = node {
        list.iter_mut().for_each(widen);
    }
}

fn allow_null(property: &mut Map<String, Value>) {
    let nullable = |p: &Map<String, Value>| {
        let has = |v: &Value| {
            v.get("type")
                .is_some_and(|t| t == "null" || t.as_array().is_some_and(|a| a.iter().any(|x| x == "null")))
        };
        has(&Value::Object(p.clone()))
            || p.get("anyOf").and_then(Value::as_array).is_some_and(|a| a.iter().any(has))
    };
    if nullable(property) {
        return;
    }
    let notes: Vec<String> = property.keys().filter(|k| k.starts_with("x-")).cloned().collect();
    let mut kept = Map::new();
    for key in notes {
        if let Some(v) = property.remove(&key) {
            kept.insert(key, v);
        }
    }
    let inner = Value::Object(std::mem::take(property));
    kept.insert("anyOf".into(), Value::Array(vec![inner, serde_json::json!({"type": "null"})]));
    *property = kept;
}

fn names(node: &Value, into: &mut Vec<String>) {
    if let Some(Value::Object(props)) = node.get("properties") {
        into.extend(props.keys().cloned());
    }
    for key in ["oneOf", "anyOf"] {
        for branch in node.get(key).and_then(Value::as_array).into_iter().flatten() {
            names(branch, into);
        }
    }
}

fn require(node: &mut Value) {
    if let Some(Value::Object(props)) = node.get("properties") {
        let mut fields: Vec<String> = props.keys().cloned().collect();
        fields.sort();
        node["required"] = fields.into_iter().map(Value::String).collect();
    }
    for branch in branches(node) {
        require(branch);
    }
}

pub fn annotate(schema: &mut Value, table: &Prefill) -> Annotated {
    annotate_rows(schema, table.0)
}

pub fn annotate_rows(schema: &mut Value, rows: &[(&str, &str)]) -> Annotated {
    let mut result = Annotated::default();
    widen(schema);
    for (row, text) in rows {
        let value = serde_json::from_str::<Value>(text);
        let hit = match (row.split_once('.'), value) {
            (Some((ty, field)), Ok(value)) => {
                let (ty, tag) = match ty.split_once('[') {
                    Some((name, rest)) => (name, rest.strip_suffix(']')),
                    None => (ty, None),
                };
                object_of(schema, ty).is_some_and(|o| fill(o, field, tag, &value))
            }
            _ => false,
        };
        if !hit {
            result.unknown.push((*row).to_owned());
        }
    }
    for name in type_names(schema) {
        let Some(object) = object_of(schema, &name) else { continue };
        let mut fields = Vec::new();
        names(object, &mut fields);
        fields.sort();
        fields.dedup();
        for field in &fields {
            let own = |row: &str| {
                row.split_once('.').is_some_and(|(ty, f)| f == field && ty.split('[').next() == Some(&name))
            };
            if !rows.iter().any(|(row, _)| own(row)) {
                result.uncovered.push(format!("{name}.{field}"));
            }
        }
        require(object);
    }
    result
}
