use serde_json::{Value, json};

fn upgraded(name: &str, scene: Value) -> Value {
    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::create_dir_all(dir.join("a"));
    let file = dir.join("a/scene.json");
    std::fs::write(&file, scene.to_string()).unwrap();
    assert_eq!(reiwand_upgrade::scenes::run(&dir).unwrap(), 1);
    let after: Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
    assert_eq!(reiwand_upgrade::scenes::run(&dir).unwrap(), 0);
    after
}

#[test]
fn vortex_and_around_gain_the_axis_once() {
    let scene = json!({"particles": {"s": {
        "forces": [{"vortex": {"center": [0, 0], "strength": 1}}, {"drag": 1}],
        "emitters": [{"shape": {"around": {"axis": [1, 0, 0], "count": 4}}}]}}});
    let s = &upgraded("scenes-axis", scene)["particles"]["s"];
    assert_eq!(s["forces"][0]["vortex"]["axis"], json!([0, 0, 1]));
    assert_eq!(s["emitters"][0]["shape"]["around"]["axis"], json!([1, 0, 0]));
}

#[test]
fn oscillate_and_stretch_keep_today_behaviour() {
    let scene = json!({"particles": {"s": {
        "forces": [{"oscillate": {"what": "alpha", "freq": [1, 2], "scale": [0.2, 0.5]}}],
        "render": {"stretch": {"time": 0.1, "max": 40}}}}});
    let s = &upgraded("scenes-wave", scene)["particles"]["s"];
    assert_eq!(s["forces"][0]["oscillate"]["from"], "top");
    assert_eq!(s["render"]["stretch"], json!({"mode": "add", "time": 0.1, "min": 0, "max": 40}));
}
