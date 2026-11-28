use serde_json::Value;

fn suggested(shader: &str) -> Vec<(String, Value)> {
    let row = match shader {
        "fur" | "foliage" | "chroma" => reiwand_scene::prefill::offered_row(&format!("shader.{shader}")),
        _ => return Vec::new(),
    };
    row.as_object().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()).unwrap_or_default()
}

pub(super) fn filled(block: &mut Value, shader: &str) {
    for (id, value) in suggested(shader) {
        if block["params"].get(&id).is_none() {
            block["params"][id.as_str()] = value;
        }
    }
}

pub(crate) fn camera() -> Value {
    reiwand_scene::prefill::offered_row("camera3d")
}
