use super::{put, render};
use crate::ops::{Op, escape};
use anyhow::{Result, bail, ensure};
use reiwand_core::message::said;
use serde_json::{Value, json};
use std::path::Path;

pub(super) const LEAST: f64 = 0.05;
pub(super) const MOST: f64 = 20.0;

pub(crate) fn from_texture(dir: &Path, doc: &Value, texture: &Value) -> Result<f64> {
    let Some(id) = texture.as_str() else {
        bail!(said("editor.aspect_no_texture", "the render has no picture to take the proportion from", &[]));
    };
    let asset = doc.pointer(&format!("/scene/assets/{}", escape(id)));
    let Some(path) = asset.and_then(|a| a["path"].as_str()) else {
        bail!(said("editor.aspect_no_asset", "there is no picture \"%1\" in the scene", &[&id]));
    };
    let (w, h) = crate::pictures::dimensions(dir.join(path))
        .map_err(|e| said("editor.unreadable_picture", "the picture cannot be read: %1", &[&e]))?;
    let cell = |c: Option<&Value>| c.and_then(|c| serde_json::from_value::<[f64; 2]>(c.clone()).ok());
    let atlas = cell(asset.and_then(|a| a.get("atlas"))).filter(|c| c[0] >= 1.0 && c[1] >= 1.0);
    let first = asset
        .and_then(|a| a["rects"].get(0))
        .and_then(|r| serde_json::from_value::<[f64; 4]>(r.clone()).ok());
    let (cw, ch) = match (first, atlas) {
        (Some(r), _) => (r[2], r[3]),
        (None, Some([rows, cols])) => (f64::from(w) / cols, f64::from(h) / rows),
        _ => (f64::from(w), f64::from(h)),
    };
    ensure!(cw > 0.0 && ch > 0.0, said("editor.aspect_empty", "the picture has an empty frame", &[]));
    Ok(((cw / ch) * 1000.0).round() / 1000.0)
}

pub(super) fn aspect(
    (dir, doc): (&Path, &Value),
    sys: &str,
    layer: Option<&str>,
    (value, taken): (Option<f64>, bool),
) -> Result<Vec<Op>> {
    ensure!(
        value.is_some() != taken,
        said("editor.aspect_one", "give either the proportion or take it from the texture", &[])
    );
    let own = match layer {
        Some(id) => crate::actions::layer(doc, id)?.1["render"]["texture"].clone(),
        None => Value::Null,
    };
    let texture =
        if own.is_null() { doc.pointer(&format!("{sys}/render/texture")).cloned() } else { Some(own) };
    let v = match value {
        Some(v) => v,
        None => from_texture(dir, doc, &texture.unwrap_or(Value::Null))?,
    };
    ensure!(
        (LEAST..=MOST).contains(&v),
        said("editor.aspect_range", "the proportion %1 must be between %2 and %3", &[&v, &LEAST, &MOST])
    );
    render(doc, sys, layer, |d, at| put(d, &format!("{at}/aspect"), Some(json!(v))))
}

fn aspect_of(render: &Value) -> f64 {
    render["aspect"].as_f64().unwrap_or(1.0).clamp(LEAST, MOST)
}

pub(super) fn cover(render: &Value) -> f64 {
    let a = aspect_of(render);
    a.min(a.recip())
}

pub(super) fn sprite(system: &Value, render: &Value) -> Value {
    let aspect = aspect_of(render);
    let end = |i: usize| system["init"]["size"][i].as_f64().unwrap_or(0.0);
    let side = (end(0) + end(1)) / 2.0;
    let (w, h) = (aspect.min(1.0), aspect.recip().min(1.0));
    json!({ "aspect": aspect, "side": side, "width": side * w, "height": side * h, "cover": aspect.min(aspect.recip()),
        "texture": render["texture"] })
}
