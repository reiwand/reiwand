use crate::ops::{Op, escape, set};
use reiwand_scene::prefill;
use serde_json::Value;

fn instances(doc: &Value, base: &str, layers: &Value, ops: &mut Vec<Op>) {
    for (i, layer) in layers.as_array().into_iter().flatten().enumerate() {
        let here = format!("{base}/{i}");
        if layer["instance"].is_object() {
            let filled = prefill::instance(layer["instance"].clone());
            if filled != layer["instance"] {
                ops.extend(set(doc, &format!("{here}/instance"), filled));
            }
        }
        instances(doc, &format!("{here}/layers"), &layer["layers"], ops);
    }
}

pub fn ops(doc: &Value) -> Vec<Op> {
    let mut ops = Vec::new();
    for (id, system) in doc["scene"]["particles"].as_object().into_iter().flatten() {
        let filled = prefill::completed(system.clone());
        if &filled != system {
            ops.extend(set(doc, &format!("/scene/particles/{}", escape(id)), filled));
        }
    }
    instances(doc, "/scene/layers", &doc["scene"]["layers"], &mut ops);
    ops
}
