use reiwand_upgrade::scenes;
use serde_json::{Value, json};

fn scene() -> Value {
    json!({"schema": "reiwand.scene/1", "layers": [
        {"id": "a", "type": "solid"},
        {"id": "g", "type": "group", "layers": [{"id": "b", "type": "solid"}]}
    ]})
}

#[test]
fn every_layer_gets_canvas_and_a_second_run_changes_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("one").join("scene.json");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, scene().to_string()).unwrap();
    std::fs::write(dir.path().join("other.json"), "{\"x\":1}").unwrap();
    let first = scenes::run(&[dir.path().to_path_buf()]).unwrap();
    assert!(first.contains("1 scenes, 3 layers"), "{first}");
    let out: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    assert_eq!(out["layers"][0]["blend_space"], "canvas");
    assert_eq!(out["layers"][1]["layers"][0]["blend_space"], "canvas");
    let again = scenes::run(&[file]).unwrap();
    assert!(again.contains("0 layers"), "{again}");
}

#[test]
fn an_existing_choice_is_kept() {
    let mut s = scene();
    s["layers"][0]["blend_space"] = json!("srgb");
    let (out, added) = scenes::upgrade(&s.to_string()).unwrap().unwrap();
    assert_eq!(added, 2);
    assert!(out.contains("\"srgb\""));
}
