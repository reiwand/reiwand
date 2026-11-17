use serde_json::{Value, json};
use std::fs;

#[test]
fn every_chroma_key_gets_the_values_that_reproduce_the_old_look() {
    let dir = tempfile::tempdir().unwrap();
    let folder = dir.path().join("pack/deep");
    fs::create_dir_all(&folder).unwrap();
    let scene = json!({ "layers": [
        { "id": "a", "effects": [{ "effect": "chroma_key" }, { "effect": "blur" }] },
        { "id": "b", "effects": [{ "effect": "chroma_key", "params": { "key": "#ffffff", "keep": 0.5 } }] }
    ] });
    fs::write(folder.join("scene.json"), scene.to_string()).unwrap();
    fs::write(dir.path().join("pack/other.json"), "{}").unwrap();
    let changed = reiwand_upgrade::scenes::run(&[dir.path().join("pack")]).unwrap();
    assert_eq!(changed, [folder.join("scene.json")]);
    let read: Value = serde_json::from_slice(&fs::read(folder.join("scene.json")).unwrap()).unwrap();
    let effects = |i: usize| read["layers"][i]["effects"].clone();
    assert_eq!(effects(0)[0]["params"], json!({ "distance": 0, "keep": 0 }));
    assert!(effects(0)[1].get("params").is_none());
    assert_eq!(effects(1)[0]["params"], json!({ "key": "#ffffff", "distance": 0, "keep": 0.5 }));
    assert!(reiwand_upgrade::scenes::run(&[dir.path().join("pack")]).unwrap().is_empty());
}
