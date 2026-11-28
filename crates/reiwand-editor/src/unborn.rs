//! Whether the canvas held at second 0 shows nothing yet: every layer that would draw is a particle system that
//! starts empty (no prewarm), so the paused preview is the bare background until the scene plays.
use crate::layers::walk;
use serde_json::Value;

const UNDRAWN: [&str; 4] = ["group", "group3d", "light", "sound"];

pub(crate) fn unborn(doc: &Value) -> bool {
    let mut drawn = false;
    for (_, layer) in walk(doc) {
        let kind = layer["type"].as_str().unwrap_or_default();
        if UNDRAWN.contains(&kind) || layer["visible"] == false {
            continue;
        }
        let system = &doc["scene"]["particles"][layer["system"].as_str().unwrap_or_default()];
        if kind != "particles" || system["prewarm"].as_f64().unwrap_or(0.0) > 0.0 {
            return false;
        }
        drawn = true;
    }
    drawn
}

#[cfg(test)]
mod tests {
    use super::unborn;
    use serde_json::json;

    #[test]
    fn only_particles_without_prewarm_leave_the_start_empty() {
        let doc = |layers, prewarm| json!({"scene": {"layers": layers, "particles": {"snow": {"prewarm": prewarm}}}});
        let snow = json!({"type": "particles", "system": "snow"});
        let sky = json!({"type": "solid"});
        let hidden = json!({"type": "solid", "visible": false});
        let group = json!({"type": "group", "layers": [snow.clone(), {"type": "sound"}]});
        assert!(unborn(&doc(json!([snow.clone(), hidden]), json!(0))));
        assert!(unborn(&doc(json!([group]), json!(0))));
        assert!(!unborn(&doc(json!([snow.clone()]), json!(14))), "a prewarmed system shows at once");
        assert!(!unborn(&doc(json!([snow, sky]), json!(0))), "another layer draws at once");
        assert!(!unborn(&doc(json!([]), json!(0))), "an empty scene has its own invitation");
    }
}
