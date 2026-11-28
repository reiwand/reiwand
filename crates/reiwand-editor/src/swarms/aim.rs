use crate::ops::{Op, set};
use anyhow::{Result, ensure};
use reiwand_core::message::said;
use serde_json::{Value, json};

pub(super) fn unit(tilt: f64, turn: f64) -> [f64; 3] {
    let (t, u) = (tilt.to_radians(), turn.to_radians());
    [t.sin() * u.cos(), t.sin() * u.sin(), t.cos()].map(|v| (v * 1e5).round() / 1e5)
}

pub(super) fn angles(axis: &Value) -> [f64; 2] {
    let at = |i: usize| axis.get(i).and_then(Value::as_f64).unwrap_or(0.0);
    [at(2).clamp(-1.0, 1.0).acos().to_degrees(), at(1).atan2(at(0)).to_degrees()]
}

pub(super) fn aim(doc: &Value, sys: &str, target: &str, tilt: f64, turn: f64) -> Result<Vec<Op>> {
    let (kind, index) = target.split_once(':').unwrap_or((target, "0"));
    let index: usize = index.parse().unwrap_or(usize::MAX);
    let at = match kind {
        "force" => format!("{sys}/forces/{index}/vortex/axis"),
        "emitter" => format!("{sys}/emitters/{index}/shape/around/axis"),
        _ => String::new(),
    };
    ensure!(
        doc.pointer(&at).is_some(),
        said(
            "editor.no_axis",
            "%1 has no axis: aim a vortex force (force:N) or an around emitter (emitter:N)",
            &[&target]
        )
    );
    Ok(set(doc, &at, json!(unit(tilt, turn))))
}
