use reiwand_upgrade::descriptors::{complete, text};
use serde_json::Value;
use std::path::PathBuf;

fn pair(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/pairs").join(name)
}

#[test]
fn an_old_effect_descriptor_gains_every_field_with_the_value_applied_until_now() {
    let original = std::fs::read_to_string(pair("effect.original.json")).unwrap();
    let mut doc: Value = serde_json::from_str(&original).unwrap();
    assert!(complete(&mut doc));
    let done = text(&doc, 2);
    if std::env::var_os("REIWAND_BLESS").is_some() {
        std::fs::write(pair("effect.completed.json"), &done).unwrap();
    }
    assert_eq!(done, std::fs::read_to_string(pair("effect.completed.json")).unwrap());
    assert_eq!(doc["params"][0]["prefill"], "#ffffff");
    assert_eq!(doc["passes"][0]["inputs"], serde_json::json!(["previous"]));
    assert_eq!(doc["targets"]["half"]["format"], "rgba8");
    assert_eq!(doc["lighter"]["cost"]["scale"], 1);
    assert_eq!(doc["state"]["scale"], 1);
    assert!(!complete(&mut doc));
}
