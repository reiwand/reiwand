use reiwand_scene::{SCHEMA, Scene};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn glow() -> Value {
    json!({
        "threshold": 0.8, "strength": 1.0, "radius": 16.0, "radius_unit": "canvas", "knee": 0.5,
        "saturation": 1.0, "limit": null, "tint": "#ffffff",
        "hdr": { "threshold": 1.5, "strength": 0.5, "knee": 0.5, "levels": 4, "scatter": 1.0 }
    })
}

fn scene(bloom: &Value) -> Value {
    let base = json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": [] });
    let mut filled: Value = serde_json::from_str(&reiwand_fill::whole(&base.to_string())).unwrap();
    filled["canvas"]["bloom"] = bloom.clone();
    filled
}

fn load(scene: &Value) -> Result<Scene, String> {
    Scene::load(&scene.to_string(), &BTreeMap::new()).map_err(|e| e.to_string())
}

fn with(path: &[&str], value: Option<Value>) -> Value {
    let mut bloom = glow();
    let mut at = &mut bloom;
    for key in &path[..path.len() - 1] {
        at = &mut at[*key];
    }
    let last = path[path.len() - 1];
    match value {
        Some(v) => at[last] = v,
        None => {
            at.as_object_mut().unwrap().remove(last);
        }
    }
    scene(&bloom)
}

#[test]
fn a_full_glow_loads_and_a_null_limit_is_explicit() {
    load(&scene(&glow())).unwrap();
}

#[test]
fn a_glow_missing_any_new_field_is_refused_naming_it() {
    let fields: [&[&str]; 8] = [
        &["saturation"],
        &["limit"],
        &["knee"],
        &["radius_unit"],
        &["hdr", "knee"],
        &["hdr", "levels"],
        &["hdr", "scatter"],
        &["hdr", "strength"],
    ];
    for path in fields {
        let error = load(&with(path, None)).unwrap_err();
        assert!(error.contains(path[path.len() - 1]), "{path:?}: {error}");
    }
}

#[test]
fn glow_values_out_of_range_are_refused() {
    let cases: [(&[&str], Value); 7] = [
        (&["saturation"], json!(9.0)),
        (&["knee"], json!(2.0)),
        (&["limit"], json!(99.0)),
        (&["radius_unit"], json!("inch")),
        (&["hdr", "knee"], json!(-1.0)),
        (&["hdr", "levels"], json!(20)),
        (&["hdr", "scatter"], json!(5.0)),
    ];
    for (path, value) in cases {
        let error = load(&with(path, Some(value.clone()))).unwrap_err();
        assert!(!error.is_empty(), "{path:?} {value}");
    }
}
