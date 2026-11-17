use reiwand_scene::*;
use serde_json::{Value as Json, json};

const STEP: f32 = 1.0 / 60.0;

fn listen(extra: Json) -> Listen {
    let mut l = json!({ "kind": "surge", "unit": "amplitude", "window": 1, "reference": "band", "hold_silence": false });
    l.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
    serde_json::from_value(l).unwrap()
}

fn tone(on: bool) -> AudioFrame {
    let mut power = [[0.0; BANDS]; 2];
    if on {
        power[0][3] = 0.5;
        power[1][3] = 0.5;
    }
    AudioFrame { silent: !on, power, ..AudioFrame::default() }
}

fn play(l: &Listen, frames: &[(bool, usize)]) -> Vec<f32> {
    let key = usual_key(&l.binding());
    let mut usual = Usual::at(&tone(false));
    let mut out = Vec::new();
    for (on, count) in frames {
        let frame = tone(*on);
        for _ in 0..*count {
            usual.advance(&frame, STEP, key);
            out.push(l.bands(&frame, Some(&usual))[3]);
        }
    }
    out
}

#[test]
fn a_surge_listen_pops_after_silence_and_decays() {
    let l = listen(json!({}));
    let out = play(&l, &[(false, 180), (true, 6), (true, 240)]);
    let peak = out.iter().cloned().fold(0.0, f32::max);
    assert!(peak > 1.0, "peak {peak}");
    assert!(*out.last().unwrap() < peak * 0.6, "last {}", out.last().unwrap());
}

#[test]
fn surge_fields_are_required_and_only_for_surge() {
    let missing: Listen = serde_json::from_value(json!({ "kind": "surge", "unit": "db" })).unwrap();
    assert_eq!(missing.problems("/x").len(), 3);
    let stray: Listen =
        serde_json::from_value(json!({ "kind": "relative", "unit": "db", "window": 1 })).unwrap();
    assert!(matches!(stray.problems("/x")[0], Issue::Type(_)));
    assert!(listen(json!({})).problems("/x").is_empty());
    assert!(!listen(json!({ "window": 90 })).problems("/x").is_empty());
}

#[test]
fn unknown_listen_fields_are_refused() {
    assert!(
        serde_json::from_value::<Listen>(json!({ "kind": "relative", "unit": "db", "gain": 2 })).is_err()
    );
}

#[test]
fn the_runtime_fills_one_slot_per_listen_and_surges_after_silence() {
    let doc = json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] },
        "listens": [{ "kind": "relative", "unit": "amplitude" },
                    { "kind": "surge", "unit": "amplitude", "window": 1, "reference": "band", "hold_silence": false }] });
    let scene = Scene::from_json(&doc.to_string()).unwrap();
    let mut rt = Runtime::new(std::sync::Arc::new(scene), Values::new());
    for _ in 0..180 {
        rt.set_audio(tone(false));
        rt.advance(STEP as f64);
    }
    let mut peak = 0.0f32;
    for _ in 0..6 {
        rt.set_audio(tone(true));
        rt.advance(STEP as f64);
        peak = peak.max(rt.heard()[1][3]);
    }
    assert_eq!(rt.heard().len(), 2);
    assert!(peak > 1.0, "peak {peak}");
}
