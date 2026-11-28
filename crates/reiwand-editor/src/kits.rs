use anyhow::{Result, bail};
use reiwand_core::message::said;
use serde_json::{Value, json};

pub const KITS: [&str; 4] = ["snow", "sparks", "rain", "dust"];

pub fn size(doc: &Value) -> [f64; 2] {
    let side = |i: usize| doc.pointer(&format!("/scene/canvas/size/{i}")).and_then(Value::as_f64);
    let fallback = reiwand_scene::prefill::canvas();
    [side(0).unwrap_or(fallback[0]), side(1).unwrap_or(fallback[1])]
}

fn fall(h: f64) -> Value {
    json!([{"bounds": {"plane": {"point": [0, h + 60.0], "normal": [0, -1]}}, "kill": true}])
}

pub fn kit(name: &str, [w, h]: [f64; 2]) -> Result<Value> {
    Ok(reiwand_scene::prefill::completed(match name {
        "snow" => json!({"capacity": 3000, "prewarm": 14,
            "emitters": [{"shape": {"box": {"from": [-100, -50], "to": [w + 100.0, -20]}}, "rate": 90}],
            "init": {"life": [12, 18, 1], "speed": [40, 80, 1], "angle": [80, 100, 1], "size": [3, 8, 1],
                "color": ["#ffffff", "#dce6f4", 1], "tilt": {"angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]]}},
            "forces": [{"gravity": [8, 14]}, {"turbulence": {"strength": 16, "scale": 0.004}}, {"drag": 0.2}],
            "collide": fall(h),
            "render": {"aspect": 1, "color_over_life": [[0, "#ffffff00"], [0.08, "#ffffffff"], [0.9, "#ffffffff"], [1, "#ffffff00"]]}}),
        "sparks" => json!({"capacity": 1500, "prewarm": 3,
            "emitters": [{"shape": {"box": {"from": [w * 0.35, h - 20.0], "to": [w * 0.65, h]}}, "rate": 60}],
            "init": {"life": [1.5, 3, 1], "speed": [180, 420, 1], "angle": [240, 300, 1], "size": [2, 5, 1],
                "color": ["#ffd27a", "#ff7a1a", 1], "tilt": {"angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]]}},
            "forces": [{"gravity": [0, 160]}, {"turbulence": {"strength": 60, "scale": 0.006}}, {"drag": 0.3}],
            "render": {"aspect": 1, "color_over_life": [[0, "#ffffffff"], [0.5, "#ff9a30ff"], [1, "#ff300000"]]}}),
        "rain" => json!({"capacity": 4000, "prewarm": 3,
            "emitters": [{"shape": {"box": {"from": [-200, -60], "to": [w + 200.0, -20]}}, "rate": 400}],
            "init": {"life": [1.2, 1.8, 1], "speed": [900, 1200, 1], "angle": [78, 84, 1], "size": [2, 3, 1],
                "color": ["#a8c4e8", "#7fa3d1", 1], "tilt": {"angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]]}},
            "collide": fall(h),
            "render": {"aspect": 1, "stretch": {"mode": "add", "depth": "flat", "time": 0.04, "min": 0, "max": 80}}}),
        "dust" => json!({"capacity": 800, "prewarm": 12,
            "emitters": [{"shape": {"box": {"from": [0, 0], "to": [w, h]}}, "rate": 20}],
            "init": {"life": [8, 14, 1], "speed": [3, 12, 1], "angle": [0, 360, 1], "size": [2, 5, 1], "alpha": [0.3, 0.7, 1],
                "color": ["#ffffff", "#ffe8c8", 1], "tilt": {"angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]]}},
            "forces": [{"turbulence": {"strength": 10, "scale": 0.002}}],
            "render": {"aspect": 1, "color_over_life": [[0, "#ffffff00"], [0.2, "#ffffffff"], [0.8, "#ffffffff"], [1, "#ffffff00"]]}}),
        other => bail!(said(
            "editor.no_kit",
            "%1 is not a starting kit; use one of %2",
            &[&other, &(KITS.join(", "))]
        )),
    }))
}
