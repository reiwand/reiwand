use super::effects::put;
use super::refused;
use crate::action::CanvasEdit as C;
use crate::ops::{Op, set};
use anyhow::{Result, bail, ensure};
use reiwand_core::message::said;
use serde_json::{Map, Value, json};
use std::path::Path;

fn tidy(v: Value) -> Value {
    match v {
        Value::Number(n) if n.is_f64() => {
            let short = n.as_f64().and_then(|f| format!("{}", f as f32).parse::<f64>().ok());
            short.map_or(Value::Number(n), |f| json!(f))
        }
        Value::Object(m) => Value::Object(m.into_iter().map(|(k, v)| (k, tidy(v))).collect()),
        other => other,
    }
}

fn merged<T: serde::Serialize + Default>(doc: &Value, at: &str) -> Map<String, Value> {
    let base = serde_json::to_value(T::default())
        .map(tidy)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default();
    let mut next = base;
    doc.pointer(at)
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .for_each(|(k, v)| drop(next.insert(k.clone(), v.clone())));
    next
}

pub(crate) fn canvas(dir: &Path, doc: &Value, e: &C) -> Result<Vec<Op>> {
    let ops = match e {
        C::Camera { .. } | C::CameraGrip { .. } | C::CameraView { .. } => {
            return super::lens::edit(dir, doc, e);
        }
        C::Probe { pointer, tilt, off } => super::glide::probe(doc, *pointer, *tilt, *off)?,
        C::Size { size } => {
            let fits = size.iter().all(|v| (1..=reiwand_scene::MAX_SIZE).contains(v));
            ensure!(fits, said("editor.canvas_size", "a side of the canvas is 1 to 16384 pixels", &[]));
            set(doc, "/scene/canvas/size", json!(size))
        }
        C::Clock(c) => {
            let mut ops = Vec::new();
            if let Some(t) = c.poster {
                let max = f64::from(reiwand_scene::MAX_POSTER_TIME);
                ensure!(
                    (0.0..=max).contains(&t),
                    said("editor.poster_range", "the still frame is from 0 to %1 seconds", &[&max])
                );
                ops.extend(set(doc, "/scene/clock/poster_time", json!((t * 1000.0).round() / 1000.0)));
            }
            ensure!(
                !(c.fresh && c.seed.is_some()),
                said("editor.seed_or_fresh", "give a seed or ask for a new one each start, not both", &[])
            );
            let seed = if c.fresh { Some(Value::Null) } else { c.seed.map(|s| json!(s)) };
            if let Some(seed) = seed {
                let mut staged = doc.clone();
                crate::ops::apply(&mut staged, &ops)?;
                ops.extend(set(&staged, "/scene/clock/seed", seed));
            }
            ops
        }
        C::Bloom { off: true, .. } => put(doc, "/scene/canvas/bloom", None),
        C::Bloom { threshold, strength, radius, radius_unit, knee, saturation, limit, tint, hdr, .. } => {
            let mut next = merged::<reiwand_scene::Bloom>(doc, "/scene/canvas/bloom");
            let numbers = [
                ("threshold", threshold),
                ("strength", strength),
                ("radius", radius),
                ("knee", knee),
                ("saturation", saturation),
            ];
            numbers.iter().for_each(|(k, v)| v.iter().for_each(|v| drop(next.insert((*k).into(), json!(v)))));
            tint.iter().for_each(|t| drop(next.insert("tint".into(), json!(t))));
            radius_unit.iter().for_each(|u| drop(next.insert("radius_unit".into(), json!(u))));
            limit.iter().for_each(|l| drop(next.insert("limit".into(), none_or_number(l))));
            if let Some(h) = hdr {
                let current = next.get("hdr").cloned();
                match glare(h, current) {
                    Some(glare) => drop(next.insert("hdr".into(), glare)),
                    None if h.is_null() || h == "none" => drop(next.remove("hdr")),
                    None => bail!(said(
                        "editor.hdr_object",
                        "the glare of a glow is none or an object with threshold and strength, and optionally knee, levels and scatter",
                        &[]
                    )),
                }
            }
            put(doc, "/scene/canvas/bloom", Some(Value::Object(next)))
        }
        C::Parallax { off: true, .. } => put(doc, "/scene/canvas/parallax", None),
        C::Parallax { amount, delay, pointer, offset, gyro, .. } => {
            let mut next = merged::<reiwand_scene::Glide>(doc, "/scene/canvas/parallax");
            let numbers = [("amount", amount), ("delay", delay), ("pointer", pointer)];
            numbers.iter().for_each(|(k, v)| v.iter().for_each(|v| drop(next.insert((*k).into(), json!(v)))));
            offset
                .iter()
                .for_each(|o| drop(next.insert("offset".into(), json!(o.map(|v| v.clamp(-1.0, 1.0))))));
            gyro.iter().for_each(|g| drop(next.insert("gyro".into(), json!(g))));
            put(doc, "/scene/canvas/parallax", Some(Value::Object(next)))
        }
    };
    refused(dir, doc, ops)
}

fn none_or_number(v: &Value) -> Value {
    if v == "none" { Value::Null } else { v.clone() }
}

fn glare(given: &Value, current: Option<Value>) -> Option<Value> {
    let given = given.as_object()?;
    let mut out = match current {
        Some(Value::Object(old)) => old,
        _ => serde_json::Map::from_iter([
            ("knee".into(), json!(0.5)),
            ("levels".into(), json!(1)),
            ("scatter".into(), json!(0.5)),
        ]),
    };
    given.iter().for_each(|(k, v)| drop(out.insert(k.clone(), v.clone())));
    (out.contains_key("threshold") && out.contains_key("strength")).then_some(Value::Object(out))
}
