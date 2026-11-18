use serde_json::{Value, json};

fn suggested(shader: &str) -> Vec<(&'static str, Value)> {
    match shader {
        "fur" => vec![
            ("layers", json!(12.0)),
            ("length", json!(10.0)),
            ("density", json!(0.5)),
            ("comb_x", json!(0.0)),
            ("comb_y", json!(0.0)),
            ("comb_z", json!(0.0)),
        ],
        "foliage" => vec![("strength", json!(1.0)), ("speed", json!(1.0))],
        "chroma" => vec![("key", json!("#00ff00")), ("threshold", json!(0.4)), ("softness", json!(0.1))],
        _ => Vec::new(),
    }
}

pub(super) fn filled(block: &mut Value, shader: &str) {
    for (id, value) in suggested(shader) {
        if block["params"].get(id).is_none() {
            block["params"][id] = value;
        }
    }
}

pub(crate) fn camera() -> Value {
    json!({
        "fov": 60.0, "near": 0.1, "far": 1000.0, "eye": [0.0, 0.0, 5.0], "center": [0.0, 0.0, 0.0],
        "up": [0.0, 1.0, 0.0], "paths": [], "order": "sequential", "fade": null
    })
}
