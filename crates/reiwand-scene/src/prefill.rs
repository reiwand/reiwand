use serde_json::Value as Json;
use std::sync::LazyLock;

const TABLE: &str = include_str!("prefill.json");

static TABLE_ROWS: LazyLock<serde_json::Map<String, Json>> =
    LazyLock::new(|| serde_json::from_str(TABLE).unwrap_or_default());

static TABLE_ROWS_TEXT: LazyLock<String> = LazyLock::new(|| Json::Object(TABLE_ROWS.clone()).to_string());

fn offered(row: &str) -> bool {
    row.starts_with("Action.") || row.starts_with("Offer.")
}

static ROWS: LazyLock<Vec<(String, String)>> = LazyLock::new(|| {
    TABLE_ROWS
        .iter()
        .filter(|(row, _)| !offered(row))
        .map(|(row, value)| (row.clone(), value.to_string()))
        .collect()
});

pub fn action(path: &str) -> Option<&'static Json> {
    TABLE_ROWS.get(&format!("Action.{path}"))
}

fn row(name: &str) -> &'static Json {
    TABLE_ROWS.get(name).expect("a row of the prefill table")
}

pub fn arg<T: serde::de::DeserializeOwned>(path: &str) -> T {
    serde_json::from_value(row(&format!("Action.{path}")).clone()).expect("a prefill row of the type asked")
}

pub fn offer(name: &str, def: &str, mut value: Json) -> Json {
    let base = row(&format!("Offer.{name}"));
    if let (Some(into), Some(base)) = (value.as_object_mut(), base.as_object()) {
        for (key, put) in base {
            if !into.contains_key(key) {
                into.insert(key.clone(), put.clone());
            }
        }
    }
    match def.is_empty() {
        true => value,
        false => of(def, value),
    }
}

pub fn offered_row(name: &str) -> Json {
    row(&format!("Offer.{name}")).clone()
}

pub fn canvas() -> [f64; 2] {
    serde_json::from_value(row("Offer.canvas")["size"].clone()).expect("the canvas size of the offers")
}

pub fn table() -> String {
    TABLE_ROWS_TEXT.clone()
}

pub fn rows() -> impl Iterator<Item = (&'static String, &'static Json)> {
    TABLE_ROWS.iter().filter(|(row, _)| offered(row))
}

pub fn prefill(raw: &mut Json) -> reiwand_core::explicit::Annotated {
    let rows: Vec<(&str, &str)> = ROWS.iter().map(|(row, value)| (row.as_str(), value.as_str())).collect();
    reiwand_core::explicit::annotate_rows(raw, &rows)
}

static ROOT: LazyLock<Json> = LazyLock::new(crate::schema::schema);

static PROPERTY: LazyLock<Json> = LazyLock::new(|| {
    let mut raw = crate::schema::property_schema();
    prefill(&mut raw);
    raw
});

pub fn of(def: &str, mut value: Json) -> Json {
    let target = serde_json::json!({ "$ref": format!("#/$defs/{def}") });
    reiwand_core::explicit::walk::fill(&ROOT, &target, &mut value);
    value
}

pub fn completed(system: Json) -> Json {
    of("System", system)
}

pub fn system(value: Json) -> crate::System {
    serde_json::from_value(completed(value)).unwrap()
}

pub fn all(mut systems: Json) -> Json {
    for (_, one) in systems.as_object_mut().unwrap().iter_mut() {
        *one = completed(one.take());
    }
    systems
}

pub fn render(render: Json) -> Json {
    completed(serde_json::json!({ "capacity": 1, "emitters": [], "render": render }))["render"].clone()
}

pub fn instance(one: Json) -> Json {
    of("Instance", one)
}

pub fn part(path: &str, value: Json) -> Json {
    let mut nested = value;
    for step in path.rsplit('.') {
        nested = match step.strip_suffix("[0]") {
            Some(name) => serde_json::json!({ name: [nested] }),
            None => serde_json::json!({ step: nested }),
        };
    }
    let mut root = serde_json::json!({ "capacity": 1, "emitters": [] });
    root.as_object_mut().unwrap().extend(nested.as_object().unwrap().clone());
    let mut found = completed(root);
    for step in path.split('.') {
        found = match step.strip_suffix("[0]") {
            Some(name) => found[name][0].take(),
            None => found[step].take(),
        };
    }
    found
}

pub fn properties(mut declared: Json) -> std::collections::BTreeMap<String, crate::Property> {
    for (_, one) in declared.as_object_mut().unwrap().iter_mut() {
        let mut value = one.take();
        reiwand_core::explicit::walk::fill(&PROPERTY, &serde_json::json!({ "$ref": "#" }), &mut value);
        *one = value;
    }
    serde_json::from_value(declared).unwrap()
}
