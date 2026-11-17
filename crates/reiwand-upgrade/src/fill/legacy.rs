use serde_json::{Value, json};

pub(super) fn value(def: &str, name: &str) -> Option<Value> {
    Some(match (def, name) {
        ("Layer", "blend_space") => json!("canvas"),
        ("Oscillate", "from") => json!("top"),
        ("Stretch", "mode") => json!("add"),
        ("Stretch", "depth") => json!("flat"),
        ("Stretch", "time") => json!(0),
        ("Stretch", "min") => json!(0),
        ("Stretch", "max") => json!(10000),
        ("Hearing", "gain_release") => json!(5),
        ("vortex" | "around", "axis") => json!([0, 0, 1]),
        _ => return None,
    })
}

pub(super) fn corrected(def: &str, name: &str) -> Option<Value> {
    Some(match (def, name) {
        ("Layer", "parallax") => json!([0, 0]),
        ("Glide", "offset") => json!([0, 0]),
        ("System", "space") => json!("local"),
        ("Manifest", "description") => json!(""),
        ("Canvas", "clear") => json!(true),
        ("Layer", "solid") => json!(false),
        ("Property", "mode") => json!("one"),
        ("Property", "max_files") => json!(1000),
        _ => return None,
    })
}

pub(super) fn contextual(def: &str, name: &str) -> bool {
    matches!((def, name), ("Layer", "blend_space"))
}
