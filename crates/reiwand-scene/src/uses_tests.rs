use super::reads;
use crate::Binding;

#[test]
fn a_null_source_is_not_a_read() {
    let mut bound = serde_json::to_value(Binding::<serde_json::Value>::default()).unwrap();
    assert!(!reads(&bound, "clock"));
    assert!(!reads(&serde_json::json!({ "params": { "a": bound.clone() } }), "clock"));
    bound["clock"] = serde_json::json!("seconds");
    assert!(reads(&bound, "clock"));
    assert!(reads(&serde_json::json!({ "params": { "a": bound } }), "clock"));
}
