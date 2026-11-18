use serde_json::{Value, json};

pub const TEMPLATES: [&str; 5] = ["color", "distort", "passes", "material", "particle"];

const COLOR: &str = include_str!("starters/color.wgsl");

const DISTORT: &str = include_str!("starters/distort.wgsl");

const PASSES: &str = include_str!("starters/passes.wgsl");

const MATERIAL: &str = include_str!("starters/material.wgsl");

const PARTICLE: &str = include_str!("starters/particle.wgsl");

fn param(id: &str, kind: &str, prefill: Value, range: [Value; 2], unit: Value) -> Value {
    json!({ "id": id, "type": kind, "prefill": prefill, "min": range[0], "max": range[1], "uses": null,
        "unit": unit, "tools": [], "label": {}, "help": {}, "values": [], "channels": null, "paint": null,
        "yields_to": null, "waits_for": null, "tool": null })
}

fn pass(entry: &str, scale: f64, inputs: Value) -> Value {
    let mut made = super::whole::pass(entry.to_owned());
    made["scale"] = json!(scale);
    made["inputs"] = inputs;
    made
}

fn plain(id: &str, kind: &str, prefill: Value) -> Value {
    param(id, kind, prefill, [Value::Null, Value::Null], Value::Null)
}

fn float(id: &str, default: f64, range: [f64; 2]) -> Value {
    param(id, "float", json!(default), [json!(range[0]), json!(range[1])], Value::Null)
}

pub(super) fn completed(mut descriptor: Value) -> Value {
    let blank = plain("", "float", Value::Null);
    if let Some(d) = descriptor.as_object_mut() {
        for (key, value) in whole() {
            d.entry(key).or_insert(value);
        }
    }
    let params = descriptor["params"].as_array_mut().into_iter().flatten();
    for param in params.filter_map(Value::as_object_mut) {
        for (key, value) in blank.as_object().into_iter().flatten() {
            param.entry(key.clone()).or_insert(value.clone());
        }
    }
    let passes = descriptor["passes"].as_array_mut().into_iter().flatten();
    for item in passes.filter_map(Value::as_object_mut) {
        for (key, value) in pass("", 1.0, json!(["previous"])).as_object().into_iter().flatten() {
            item.entry(key.clone()).or_insert(value.clone());
        }
    }
    descriptor
}

pub fn starter(name: &str, template: &str) -> Option<(Value, String)> {
    let (descriptor, wgsl) = match template {
        "color" => (
            json!({ "category": "color", "fuse": "color", "cost": { "taps": 1, "passes": 1, "scale": 1 },
                "params": [float("amount", 1.0, [0.0, 1.0]), plain("color", "color", json!("#ff8800"))] }),
            COLOR,
        ),
        "distort" => (
            json!({ "category": "distort", "fuse": "uv", "cost": { "taps": 1, "passes": 1, "scale": 1 },
                "params": [plain("center", "position", json!([0.5, 0.5])),
                    float("strength", 0.3, [-1.0, 1.0]), float("radius", 0.25, [0.01, 1.0])] }),
            DISTORT,
        ),
        "passes" => (
            json!({ "category": "blur", "cost": { "taps": 9, "passes": 2, "scale": 1 },
                "params": [float("length", 8.0, [0.0, 64.0]),
                    param("angle", "float", json!(0), [json!(-180), json!(180)], json!("degrees"))],
                "passes": [pass("fs_NAME_soft", 0.5, json!(["previous"])),
                    pass("fs_NAME", 1.0, json!(["pass:0", "input"]))] }),
            PASSES,
        ),
        "material" => (
            json!({ "stage": "material", "cost": { "taps": 1, "passes": 1, "scale": 1 },
                "params": [plain("tint", "color", json!("#ffffff")), float("glow", 0.0, [0.0, 4.0]),
                    plain("picture", "texture", json!("white"))] }),
            MATERIAL,
        ),
        "particle" => (
            json!({ "stage": "particle", "cost": { "taps": 1, "passes": 1, "scale": 1 }, "params": [float("pulse", 0.3, [0.0, 1.0])] }),
            PARTICLE,
        ),
        _ => return None,
    };
    let mut descriptor = descriptor;
    descriptor["id"] = json!(name);
    for (key, value) in whole() {
        if let Some(d) = descriptor.as_object_mut() {
            d.entry(key).or_insert(value);
        }
    }
    let text = serde_json::to_string(&descriptor).unwrap_or_default().replace("NAME", name);
    Some((serde_json::from_str(&text).unwrap_or(descriptor), wgsl.replace("NAME", name)))
}

fn whole() -> [(&'static str, Value); 21] {
    let none = Value::Null;
    [
        ("name", json!({})),
        ("category", none.clone()),
        ("description", none.clone()),
        ("uses", json!([])),
        ("audio", json!([])),
        ("mask_mode", json!("mix")),
        ("passes", json!([])),
        ("fuse", none.clone()),
        ("reach", none.clone()),
        ("bounded", json!([])),
        ("min_fps", none.clone()),
        ("velocity", none.clone()),
        ("state", none.clone()),
        ("solve", none.clone()),
        ("pace", none.clone()),
        ("quality", json!({})),
        ("targets", json!({})),
        ("options", json!([])),
        ("stage", json!("layer")),
        ("compose", json!(false)),
        ("lighter", none),
    ]
}

pub(crate) fn lighter(
    dir: &std::path::Path,
    doc: &Value,
    name: &str,
    path: &str,
) -> anyhow::Result<Vec<crate::ops::Op>> {
    let code = std::fs::read_to_string(dir.join(path)).unwrap_or_default();
    if code.contains(&format!("fn fs_{name}_lite(")) {
        return Ok(Vec::new());
    }
    let more = include_str!("starters/lighter.wgsl").replace("NAME", name);
    crate::scripts::save(dir, doc, path, &format!("{}\n{more}", code.trim_end()))
}
