use serde_json::{Value, json};

#[test]
fn vortex_and_around_gain_the_axis_once() {
    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("scenes-axis");
    let _ = std::fs::create_dir_all(dir.join("a"));
    let file = dir.join("a/scene.json");
    let scene = json!({"layers": [{"forces": [{"kind": "vortex", "center": [0, 0]}, {"kind": "wind"}],
        "emitters": [{"kind": "around", "axis": [1, 0, 0]}]}]});
    std::fs::write(&file, scene.to_string()).unwrap();
    assert_eq!(reiwand_upgrade::scenes::run(&dir).unwrap(), 1);
    let after: Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
    let layer = &after["layers"][0];
    assert_eq!(layer["forces"][0]["axis"], json!([0, 0, 1]));
    assert!(layer["forces"][1].get("axis").is_none());
    assert_eq!(layer["emitters"][0]["axis"], json!([1, 0, 0]));
    assert_eq!(reiwand_upgrade::scenes::run(&dir).unwrap(), 0);
}
