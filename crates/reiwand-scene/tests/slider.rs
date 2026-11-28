use reiwand_scene::*;
use serde_json::json;
use std::collections::BTreeMap;
use std::sync::Arc;

#[test]
fn a_slider_number_drives_a_pair_field_on_both_directions() {
    let layer = json!({ "blend_space": "canvas", "id": "n", "type": "solid", "color": "#ffffff", "size": [10, 10],
        "transform": { "scale": { "prop": "size" } } });
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": [layer] });
    let declared: BTreeMap<String, Property> = reiwand_scene::prefill::properties(
        json!({ "size": { "type": "slider", "min": 0.5, "max": 3, "default": 1 } }),
    );
    let scene = Arc::new(Scene::load(&reiwand_fill::whole(&text.to_string()), &declared).unwrap());
    let mut props = Values::new();
    props.insert("size".into(), json!(2.0));
    let rt = Runtime::new(scene, props);
    let d = &rt.frame([0.0, 0.0, 100.0, 50.0], Quality::Q3).draws[0];
    let (o, x, y) = (d.world.apply([0.0; 2]), d.world.apply([1.0, 0.0]), d.world.apply([0.0, 1.0]));
    assert_eq!(((x[0] - o[0]).abs(), (y[1] - o[1]).abs()), (2.0, 2.0));
}
