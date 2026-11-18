use serde_json::{Value, json};

pub(super) fn param(id: &str, kind: &str, prefill: Value) -> Value {
    let mut made = json!({ "id": id, "type": kind, "prefill": prefill });
    for key in super::form::PARAM_KEYS_ALL.iter().skip(2) {
        made[key] = cleared_param(key);
    }
    made
}

pub(super) fn cleared_param(key: &str) -> Value {
    match key {
        "tools" | "values" => json!([]),
        "label" | "help" => json!({}),
        _ => Value::Null,
    }
}

pub(super) fn pass(entry: String) -> Value {
    let mut made = json!({ "entry": entry });
    for key in super::form::PASS_KEYS.iter().skip(1) {
        made[key] = cleared_pass(key);
    }
    made
}

pub(super) fn cleared_pass(key: &str) -> Value {
    match key {
        "scale" => json!(1),
        "inputs" => json!(["previous"]),
        "output" => json!("next"),
        _ => Value::Null,
    }
}

pub(super) fn cleared_field(key: &str) -> Value {
    match key {
        "name" | "quality" | "targets" => json!({}),
        "uses" | "bounded" | "options" => json!([]),
        "compose" => json!(false),
        "mask_mode" => json!("mix"),
        "cost" => json!({ "taps": 1, "passes": 1, "scale": 1 }),
        _ => Value::Null,
    }
}
