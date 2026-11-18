use crate::error::{Result, failed};
use reiwand_library::Item;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

pub fn merged<T: DeserializeOwned>(base: impl Serialize, extra: Value) -> Result<T> {
    let mut value = serde_json::to_value(base).map_err(|e| failed(e.to_string()))?;
    if let (Some(object), Value::Object(extra)) = (value.as_object_mut(), extra) {
        object.extend(extra);
    }
    serde_json::from_value(value).map_err(|e| failed(e.to_string()))
}

pub fn whole_item<T: DeserializeOwned>(item: Item, extra: Value) -> Result<T> {
    let mut value = serde_json::to_value(item).map_err(|e| failed(e.to_string()))?;
    if let Some(package) = value.get_mut("package").and_then(Value::as_object_mut) {
        package.insert("exposes".into(), json!([]));
        package.insert("flash_disabled".into(), json!(false));
        package.insert("unshown".into(), Value::Null);
    }
    merged(value, extra)
}
