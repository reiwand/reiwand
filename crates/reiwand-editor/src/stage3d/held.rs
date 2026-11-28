use serde_json::Value;

const SOURCES: [&str; 10] =
    ["audio", "clock", "cursor", "date", "prop", "media", "script", "layer", "fn", "map"];

pub(super) fn held(value: &Value) -> &'static str {
    match value {
        Value::Object(o) if o.get("keys").is_some_and(Value::is_array) => {
            match SOURCES.iter().all(|k| o.get(*k).is_none_or(Value::is_null)) {
                true => "keyed",
                false => "bound",
            }
        }
        Value::Object(_) => "bound",
        _ => "fixed",
    }
}
