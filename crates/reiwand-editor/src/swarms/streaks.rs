use super::parsed;
use anyhow::Result;
use serde_json::{Value, json};
use std::f64::consts::TAU;

const SAMPLES: usize = 48;

fn mean(range: &Value) -> f64 {
    let end = |i: usize| range[i].as_f64().unwrap_or(0.0);
    (end(0) + end(1)) / 2.0
}

fn drop(system: &Value, s: &Value, speed: f64, z: f64) -> Value {
    let focal = system["perspective"]["focal"].as_f64().unwrap_or(0.0);
    let k = if focal > 0.0 && focal + z > 1.0 { focal / (focal + z) } else { 1.0 };
    let num = |key: &str| s[key].as_f64().unwrap_or(0.0);
    let side = mean(&system["init"]["size"]) * k;
    let near = if s["depth"] == "projected" { k } else { 1.0 };
    let reach = (num("time") * speed * near).clamp(num("min") * near, num("max").max(num("min")) * near);
    let long = if s["mode"] == "relative" {
        side * (num("time") * speed).clamp(num("min"), num("max").max(num("min")))
    } else {
        side + reach
    };
    json!({ "z": z, "k": k, "side": side, "long": long })
}

pub(super) fn stretch(system: &Value, render: &Value) -> Value {
    let Some(s) = render.get("stretch").filter(|s| s.is_object()) else { return Value::Null };
    let (speed, side) = (mean(&system["init"]["speed"]), mean(&system["init"]["size"]));
    let num = |k: &str| s[k].as_f64().unwrap_or(0.0);
    let reach = (num("time") * speed).clamp(num("min"), num("max").max(num("min")));
    let long = if s["mode"] == "relative" { side * reach } else { side + reach };
    let depth = &system["init"]["depth"];
    let (z0, z1) = (depth[0].as_f64().unwrap_or(0.0), depth[1].as_f64().unwrap_or(0.0));
    let drops: Vec<Value> = if system["perspective"].is_object() {
        [z0.min(z1), z0.max(z1)].map(|z| drop(system, s, speed, z)).into()
    } else {
        Vec::new()
    };
    json!({ "mode": s["mode"], "depth": s["depth"], "time": num("time"), "min": num("min"), "max": num("max"),
        "speed": speed, "side": side, "long": long,
        "across": side * super::proportion::cover(render),
        "drops": drops })
}

fn wave(what: &str, from: &str, scale: [f64; 2], u: f64, turn: f64) -> f64 {
    let w = (turn.sin() + 1.0) / 2.0;
    match (what, from) {
        ("size", "top") => 1.0 + (scale[0] + u * (scale[1] - scale[0])) * turn.sin(),
        (_, "top") => 1.0 - (scale[0] + u * (scale[1] - scale[0])) * w,
        _ => scale[0] + u * (scale[1] - scale[0]) * w,
    }
}

pub(super) fn waves(system: &Value) -> Vec<Value> {
    let list = system["forces"].as_array().into_iter().flatten().enumerate();
    list.filter_map(|(index, f)| {
        let o = f.get("oscillate").filter(|o| o["what"] != "position")?;
        let (what, from) = (o["what"].as_str()?, o["from"].as_str()?);
        let scale = [o["scale"][0].as_f64()?, o["scale"][1].as_f64()?];
        let curve = |u: f64| -> Vec<f64> {
            (0..=SAMPLES).map(|i| wave(what, from, scale, u, TAU * i as f64 / SAMPLES as f64)).collect()
        };
        Some(json!({ "index": index, "what": what, "from": from, "scale": scale,
            "curves": [curve(0.0), curve(0.5), curve(1.0)] }))
    })
    .collect()
}

pub(super) fn checked(stretch: Option<&Value>) -> Result<()> {
    stretch
        .filter(|s| s.as_str() != Some("none"))
        .map_or(Ok(()), |s| parsed::<reiwand_scene::Stretch>(s, "stretch"))
}
