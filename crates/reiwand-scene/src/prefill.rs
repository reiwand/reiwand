use serde_json::Value as Json;
use std::sync::LazyLock;

const TABLE: &str = include_str!("prefill.json");

static ROWS: LazyLock<Vec<(String, String)>> = LazyLock::new(|| {
    let table: serde_json::Map<String, Json> = serde_json::from_str(TABLE).unwrap_or_default();
    table.into_iter().map(|(row, value)| (row, value.to_string())).collect()
});

pub fn prefill(raw: &mut Json) -> reiwand_core::explicit::Annotated {
    let rows: Vec<(&str, &str)> = ROWS.iter().map(|(row, value)| (row.as_str(), value.as_str())).collect();
    reiwand_core::explicit::annotate_rows(raw, &rows)
}

static ROOT: LazyLock<Json> = LazyLock::new(crate::schema::schema);

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
