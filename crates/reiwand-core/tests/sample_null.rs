use reiwand_core::explicit::walk::sample;
use serde_json::{Value, json};

#[test]
fn a_field_marked_null_samples_as_null_not_an_empty_string() {
    let root = Value::Null;
    assert_eq!(sample(&root, &json!({ "type": "string", "x-null": true, "x-ref": "icon" })), Value::Null);
    assert_eq!(sample(&root, &json!({ "type": "string" })), json!(""));
}
