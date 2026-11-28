use reiwand_core::model::{OutputState, SceneInputs};
use serde_json::{Value, json};

pub fn output(fields: Value) -> OutputState {
    let mut state = json!({
        "width": 1, "height": 1, "image": null, "fit": "fill", "background": "", "kind": "image",
        "playing": true, "active": true, "covered": false, "still_reason": null, "clients": 0, "error": null,
        "item": null, "preset": null, "playlist": null, "credit": null, "edited": false, "screen": null,
        "mm": null, "follows": null, "parallax": false, "still": null, "frames": 0, "page_frames": 0,
        "refresh": 0, "fps": 0, "buffers": "", "sound": "", "shown_fit": "fill", "flashes": "",
        "left_out": [], "page_errors": [], "blank": false, "previous": false, "released": false
    });
    let rest = json!({
        "particles": "", "quality": "", "quality_reason": "", "quality_choice": null, "quality_cost": 0.0,
        "quality_budget": 0.0, "quality_auto": "", "quality_frame_ms": null, "quality_period_ms": null,
        "quality_step": "", "quality_pace": 0, "decode": "", "drafted": false, "sandbox": "", "helper": null,
        "accent": null, "tones": null, "filter": null,
        "hdr": hdr(json!({})), "transition": {"choice": "", "own": null, "plays": "", "why": ""}
    });
    state.as_object_mut().unwrap().extend(rest.as_object().unwrap().clone());
    state.as_object_mut().unwrap().extend(fields.as_object().unwrap().clone());
    serde_json::from_value(state).unwrap()
}

pub fn hdr(fields: Value) -> Value {
    let mut state = json!({
        "choice": "", "own": null, "active": false, "offered": false, "display": false, "peak_nits": null,
        "reference_nits": null, "reason": "", "encoding": "", "max_cll": null, "max_fall": null
    });
    state.as_object_mut().unwrap().extend(fields.as_object().unwrap().clone());
    state
}

pub fn inputs(fields: Value) -> SceneInputs {
    let mut shown = json!({
        "scene": "", "audio": false, "pointer": false, "clicks": false, "network": false, "rgb": false,
        "media": false, "title": "", "key": "", "granted": [], "without": [], "undecided": []
    });
    shown.as_object_mut().unwrap().extend(fields.as_object().unwrap().clone());
    serde_json::from_value(shown).unwrap()
}
