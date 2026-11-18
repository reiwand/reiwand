use crate::{meter_gain, meter_kind};
use reiwand_core::message::said;
use reiwand_scene::{
    AudioKind, AudioUnit, BANDS, Band, Binding, Level, Reduce, SPLITS, center, read, usual_key,
};
use serde_json::{Value, json};

const DISPLAYS: [usize; 3] = [16, 32, 64];

pub(crate) fn channel(reading: &Value, key: &str, side: usize) -> [f32; BANDS] {
    let list = reading[key][side].as_array();
    std::array::from_fn(|i| list.and_then(|l| l.get(i)).and_then(Value::as_f64).unwrap_or(0.0) as f32)
}

fn fold(levels: &[f32; BANDS], of: usize, reduce: Reduce) -> Vec<f32> {
    let step = BANDS / of;
    levels
        .chunks(step)
        .map(|c| match reduce {
            Reduce::Max => c.iter().copied().fold(0.0, f32::max),
            Reduce::Mean => c.iter().sum::<f32>() / c.len() as f32,
        })
        .collect()
}

fn span(source: &Binding<f32>) -> Option<(usize, usize)> {
    let of = source.of.unwrap_or(BANDS as u32).clamp(1, BANDS as u32) as usize;
    let group = |lo: u32, hi: u32| {
        let a = (lo as usize * BANDS / of).min(BANDS - 1);
        (a, ((hi.max(lo) as usize + 1) * BANDS / of).clamp(a + 1, BANDS))
    };
    match source.audio? {
        Band::Index(i) => Some(group(i, i)),
        Band::Range([lo, hi]) => Some(group(lo, hi)),
        Band::Named(Level::Bass) => Some((0, below(SPLITS[0]))),
        Band::Named(Level::Mid) => Some((below(SPLITS[0]), below(SPLITS[1]))),
        Band::Named(Level::Treble) => Some((below(SPLITS[1]), BANDS)),
        Band::Named(_) => None,
    }
}

fn below(limit: f32) -> usize {
    (0..BANDS).filter(|b| center(*b) < limit).count()
}

pub fn query(doc: &Value, q: &Value) -> Value {
    let of = q["of"].as_u64().map_or(BANDS, |n| n as usize);
    if !DISPLAYS.contains(&of) {
        return json!({ "error": said("editor.meter_of", "bands are counted out of 16, 32 or 64", &[]).to_string() });
    }
    let mut given = q["source"].clone();
    if let Some((root, _)) = crate::schema::root_of("/scene") {
        crate::schema::fill(root, &json!({ "$ref": "#/$defs/Binding_for_float" }), &mut given);
    }
    let source: Binding<f32> = match serde_json::from_value(given) {
        Ok(source) => source,
        Err(e) => return json!({ "error": e.to_string() }),
    };
    let reading = &q["reading"];
    let listening = reading["listening"] == true && reading["sound"] != false;
    let mut out = json!({
        "granted": reading["granted"] != false,
        "listening": listening,
        "of": of,
        "marked": span(&source).map(|(a, z)| [a * of / BANDS, (z * of).div_ceil(BANDS) - 1]),
    });
    let audio = &doc["scene"]["audio"];
    let release = audio["gain_release"].as_f64().filter(|_| audio.is_object());
    if !listening {
        meter_gain::reset();
        return out;
    }
    let frame = meter_kind::frame(reading, release.map(|r| r as f32));
    let (kind, unit) = (source.kind.unwrap_or(AudioKind::Relative), source.unit.unwrap_or(AudioUnit::Db));
    let usual = (kind == AudioKind::Surge).then(|| meter_gain::usual(&frame, usual_key(&source)));
    let side = meter_kind::side(&frame, usual.as_ref(), &source, kind, unit);
    let named = |level| match (&usual, level) {
        (Some(_), Level::Beat) => f32::from(frame.beat),
        (Some(u), level) => meter_kind::surged(&frame, u, level, unit),
        (None, level) => frame.named(kind, unit, level),
    };
    let value = source.audio.map_or(0.0, |band| read(&frame, usual.as_ref(), &source, band));
    out["bands"] = json!(fold(&side, of, source.reduce.unwrap_or_default()));
    out["raw"] = json!(value);
    out["value"] = json!(source.modify(value));
    out["levels"] = json!({
        "bass": named(Level::Bass), "mid": named(Level::Mid), "treble": named(Level::Treble),
        "level": named(Level::Level), "beat": named(Level::Beat),
    });
    out["silent"] = json!(reading["silent"] == true);
    out
}
