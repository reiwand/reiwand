use reiwand_editor::starters::starter;
use reiwand_scene::Scene;
use serde_json::json;

#[test]
fn every_starting_point_loads_without_missing_values() {
    for name in reiwand_editor::STARTERS {
        let doc = starter(name, "org.example.test", "Test", "en").unwrap();
        if doc["scene"].is_object() {
            let found = Scene::from_json(&doc["scene"].to_string());
            assert!(found.is_ok(), "{name}: {:?}", found.err());
        }
    }
}

#[test]
fn an_added_layer_is_written_whole() {
    let mut doc = starter("empty", "org.example.test", "Test", "en").unwrap();
    let layer = json!({"id": "sun", "type": "shape", "shape": {"ellipse": {"size": [10, 10]}}});
    let op = reiwand_editor::ops::Op::Add { path: "/scene/layers/0".into(), value: layer };
    let op = reiwand_editor::completing::complete(&doc, op);
    reiwand_editor::ops::apply(&mut doc, &[op]).unwrap();
    let found = Scene::from_json(&doc["scene"].to_string());
    assert!(found.is_ok(), "{:?}", found.err());
}
