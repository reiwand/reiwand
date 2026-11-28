use reiwand_scene::{SCHEMA, Scene};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn filled(scene: Value) -> Value {
    serde_json::from_str(&reiwand_fill::whole(&scene.to_string())).unwrap()
}

fn load(scene: &Value) -> Result<Scene, String> {
    Scene::load(&scene.to_string(), &BTreeMap::new()).map_err(|e| e.to_string())
}

fn solid(opacity: Value) -> Value {
    let layer = json!({ "blend_space": "canvas", "id": "l0", "type": "solid", "color": "#ffffff", "opacity": opacity });
    json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": [layer] })
}

#[test]
fn an_indexed_band_writes_its_band_count() {
    let scene = filled(solid(json!({ "audio": 4, "kind": "relative", "unit": "db" })));
    assert_eq!(scene["layers"][0]["opacity"]["of"], json!(64));
    load(&scene).unwrap();
    let mut bare = scene.clone();
    bare["layers"][0]["opacity"]["of"] = Value::Null;
    assert!(load(&bare).unwrap_err().contains("/layers/0/opacity/of"));
}

#[test]
fn keys_without_a_timeline_write_their_loop() {
    let scene = filled(solid(json!({ "keys": [[0, 0], [1, 1]] })));
    assert_eq!(scene["layers"][0]["opacity"]["loop"], json!("once"));
    load(&scene).unwrap();
    let mut bare = scene.clone();
    bare["layers"][0]["opacity"]["loop"] = Value::Null;
    assert!(load(&bare).unwrap_err().contains("/layers/0/opacity/loop"));
}

#[test]
fn a_scene_with_3d_content_writes_its_camera() {
    let group = json!({ "blend_space": "canvas", "id": "g", "type": "group", "space": "3d", "layers": [] });
    let mut scene = filled(json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": [group] }));
    assert_eq!(scene["camera3d"]["eye"], json!([0, 0, 5]));
    load(&scene).unwrap();
    scene["camera3d"] = Value::Null;
    assert!(load(&scene).unwrap_err().contains("/camera3d"));
    let flat = filled(solid(json!(1.0)));
    assert!(flat["camera3d"].is_null());
    load(&flat).unwrap();
}

#[test]
fn a_material_writes_its_shader_and_the_constants_of_its_look() {
    let materials = json!({ "skin": { "shader": "fur" }, "key": { "shader": "chroma" }, "plain": {} });
    let layer = json!({ "blend_space": "canvas", "id": "m", "type": "model", "model": "toy", "materials": materials });
    let scene = filled(json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": [layer] }));
    let m = &scene["layers"][0]["materials"];
    assert_eq!(m["plain"]["shader"], json!("pbr"));
    assert_eq!(m["skin"]["params"]["layers"], json!(12));
    assert_eq!(m["skin"]["params"]["density"], json!(0.5));
    assert_eq!(m["key"]["params"]["key"], json!("#00ff00"));
    assert_eq!(m["key"]["params"]["softness"], json!(0.1));
}
