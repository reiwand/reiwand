use reiwand_editor::completing::complete_all;
use reiwand_editor::ops::{Op, apply};
use serde_json::{Value, json};

fn doc(layers: Value, camera: Value) -> Value {
    json!({ "scene": { "layers": layers, "camera3d": camera } })
}

fn camera_after(mut doc: Value, ops: Vec<Op>) -> Value {
    let done = complete_all(&doc, ops);
    apply(&mut doc, &done).unwrap();
    doc["scene"]["camera3d"].clone()
}

#[test]
fn a_group_turned_3d_writes_the_whole_camera() {
    let before = doc(json!([{ "type": "group", "space": "2d", "layers": [] }]), Value::Null);
    let ops = vec![Op::Replace { path: "/scene/layers/0/space".into(), value: json!("3d") }];
    let camera = camera_after(before, ops);
    assert!(camera["center"].is_array());
}

#[test]
fn clearing_the_camera_of_a_3d_scene_writes_the_suggested_one() {
    let before = doc(json!([{ "type": "mesh" }]), json!({ "center": [9, 9, 9] }));
    let camera = camera_after(before.clone(), vec![Op::Remove { path: "/scene/camera3d".into() }]);
    assert!(camera["center"].is_array());
    assert_ne!(camera["center"], json!([9, 9, 9]));
    let flat = doc(json!([{ "type": "group", "space": "2d", "layers": [] }]), json!({ "center": [1, 1, 1] }));
    let kept = camera_after(flat, vec![Op::Remove { path: "/scene/camera3d".into() }]);
    assert!(kept.is_null());
}
