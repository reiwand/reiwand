use crate::look::{Bloom, RadiusUnit};
use crate::value::{Param, Value};
use crate::{Canvas, EffectUse, Kind, Layer, Quality};
use serde_json::{Value as Json, json};

pub(crate) type Glows = [Option<Box<Layer>>; 2];

fn json(v: &Value<f32>) -> Json {
    serde_json::to_value(v).unwrap_or(Json::Null)
}

fn param(v: Json) -> Option<Value<Param>> {
    serde_json::from_value(v).ok()
}

struct Scales {
    threshold: Json,
    strength: Json,
    knee: Json,
    levels: u32,
    scatter: f32,
}

fn layer(b: &Bloom, scales: Scales) -> Box<Layer> {
    let radius = serde_json::to_value(&b.radius).unwrap_or(json!(16));
    let tint = serde_json::to_value(&b.tint).unwrap_or(json!("#ffffff"));
    let screen = b.radius_unit == RadiusUnit::Screen;
    let (capped, limit) = match &b.limit {
        Some(v) => (true, json(v)),
        None => (false, json!(0.0)),
    };
    let params = [
        ("threshold", scales.threshold),
        ("intensity", scales.strength),
        ("radius", radius),
        ("knee", scales.knee),
        ("tint", tint),
        ("saturation", json(&b.saturation)),
        ("capped", json!(capped)),
        ("limit", limit),
        ("levels", json!(scales.levels)),
        ("scatter", json!(scales.scatter)),
        ("unit", json!(u8::from(screen))),
    ];
    let params = params.into_iter().filter_map(|(k, v)| Some((k.to_owned(), param(v)?))).collect();
    let (effect, name, enabled, mask, optional) = ("bloom".into(), None, Value::Fixed(true), None, false);
    let mut layer = Layer::new("", Kind::Adjustment { size: crate::Ctx::word(), redraw: false, rgb: false });
    layer.effects = vec![EffectUse { effect, name, enabled, mask, params, optional }];
    Box::new(layer)
}

pub(crate) fn glows(canvas: &Canvas) -> Glows {
    let Some(b) = &canvas.bloom else { return [None, None] };
    let hdr = b.hdr.map(|h| {
        let scales = Scales {
            threshold: json!(h.threshold),
            strength: json!(h.strength),
            knee: json!(h.knee),
            levels: h.levels,
            scatter: h.scatter,
        };
        layer(b, scales)
    });
    let base = Scales {
        threshold: json(&b.threshold),
        strength: json(&b.strength),
        knee: json(&b.knee),
        levels: 1,
        scatter: 0.0,
    };
    [Some(layer(b, base)), hdr]
}

pub(crate) fn pick(glows: &Glows, quality: Quality, battery: bool) -> Option<&Layer> {
    let hdr = quality == Quality::Q3 || (quality == Quality::Q2 && !battery);
    let [base, high] = glows;
    high.as_deref().filter(|_| hdr).or(base.as_deref())
}

pub(crate) fn lend(lent: &mut Glows, canvas: &Canvas, wanted: bool) -> bool {
    if canvas.bloom.is_some() || wanted == lent[0].is_some() {
        return false;
    }
    *lent = match wanted {
        true => glows(&Canvas { bloom: Some(Bloom::default()), ..canvas.clone() }),
        false => [None, None],
    };
    true
}
