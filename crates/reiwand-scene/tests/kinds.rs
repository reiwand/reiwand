use reiwand_scene::*;
use serde_json::{Value as Json, json};
use std::collections::BTreeMap;
use std::sync::Arc;

const STEP: f32 = 1.0 / 60.0;

fn ticks(seconds: f32) -> usize {
    (seconds / STEP).round() as usize
}

fn binding(extra: Json) -> Binding<f32> {
    let mut b = json!({ "audio": 40, "kind": "surge", "unit": "amplitude", "window": 1, "reference": "band", "hold_silence": false });
    b.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
    serde_json::from_value(b).unwrap()
}

fn tone(bands: &[(usize, f32)]) -> AudioFrame {
    let mut power = [[0.0; BANDS]; 2];
    for (b, p) in bands {
        power[0][*b] = *p;
        power[1][*b] = *p;
    }
    AudioFrame { silent: bands.is_empty(), power, ..AudioFrame::default() }
}

fn run(b: &Binding<f32>, frames: &[(AudioFrame, f32)]) -> Vec<f32> {
    let key = usual_key(b);
    let mut usual = Usual::at(&frames[0].0);
    let mut out = Vec::new();
    for (frame, seconds) in frames {
        for _ in 0..ticks(*seconds) {
            usual.advance(frame, STEP, key);
            out.push(read(frame, Some(&usual), b, b.audio.unwrap()));
        }
    }
    out
}

#[test]
fn a_surge_rises_far_above_one_when_a_tone_starts_after_silence_and_settles_to_one() {
    let b = binding(json!({ "window": 1 }));
    let out = run(&b, &[(tone(&[]), 2.0), (tone(&[(40, 0.1)]), 6.0)]);
    let start = ticks(2.0);
    assert!(out[start] > 50.0, "{}", out[start]);
    let late = out[out.len() - 1];
    assert!((late - 1.0).abs() < 0.05, "{late}");
    let slow = run(&binding(json!({ "window": 8 })), &[(tone(&[]), 2.0), (tone(&[(40, 0.1)]), 3.0)]);
    assert!(slow[slow.len() - 1] > out[ticks(5.0)] * 1.5 && slow[slow.len() - 1] > 1.3);
}

#[test]
fn a_common_reference_drops_a_band_when_a_louder_one_enters_and_returns_after() {
    let b = binding(json!({ "reference": "all", "window": 0.5, "audio": 30 }));
    let frames = [(tone(&[(30, 1.0)]), 3.0), (tone(&[(30, 1.0), (50, 6.0)]), 3.0), (tone(&[(30, 1.0)]), 4.0)];
    let out = run(&b, &frames);
    let at = |s: f32| out[ticks(s) - 1];
    assert!((at(3.0) - 1.0).abs() < 0.02 && (0.33..0.45).contains(&at(6.0)), "{} {}", at(3.0), at(6.0));
    assert!(at(10.0) > 0.95, "{}", at(10.0));
    let alone = run(&binding(json!({ "reference": "band", "window": 0.5, "audio": 30 })), &frames);
    assert!((alone[ticks(6.0) - 1] - 1.0).abs() < 0.02);
}

#[test]
fn holding_the_silence_keeps_the_reference_so_a_short_pause_does_not_recharge_the_burst() {
    let frames = [(tone(&[(40, 0.1)]), 2.0), (tone(&[]), 3.0), (tone(&[(40, 0.1)]), 1.0)];
    let burst = |hold: bool| {
        let mut b = binding(json!({ "hold_silence": hold }));
        b.window = Some(0.5);
        let out = run(&b, &frames);
        out[ticks(5.0)]
    };
    assert!(burst(false) > 20.0, "{}", burst(false));
    assert!((burst(true) - 1.0).abs() < 0.1, "{}", burst(true));
}

fn dip(unit: &str) -> (f32, f32) {
    let layer = json!({ "blend_space": "canvas", "id": "l0", "type": "solid", "color": "#ffffff",
        "opacity": { "audio": 4, "kind": "relative", "unit": unit } });
    let audio = json!({ "attack": 0.001, "release": 0.01, "gain_release": 0.4 });
    let text =
        json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": [layer], "audio": audio });
    let scene = Scene::load(&reiwand_upgrade::fill::whole(&text.to_string()), &BTreeMap::new())
        .unwrap_or_else(|e| panic!("{e}"));
    let mut rt = Runtime::new(Arc::new(scene), Values::new());
    let mut seen = Vec::new();
    for tick in 0..60 {
        let power = if tick == 0 { 1.0 } else { 0.25 };
        rt.advance(f64::from(STEP));
        rt.set_audio(AudioFrame { silent: false, power: [[power; BANDS]; 2], ..AudioFrame::default() });
        seen.push(rt.frame([0.0, 0.0, 100.0, 50.0], Quality::Q3).draws.first().map_or(0.0, |d| d.opacity));
    }
    (seen[0], seen[4])
}

#[test]
fn the_amplitude_scale_shows_a_deep_dip_after_a_hit_where_the_db_scale_shows_a_small_one() {
    let (hit, db) = dip("db");
    let (_, amplitude) = dip("amplitude");
    assert!(hit > 0.95 && db > 0.75 * hit, "{hit} {db}");
    assert!(amplitude < 0.7 * hit, "{amplitude}");
}

#[test]
fn absolute_levels_ignore_the_reference() {
    let mut frame = tone(&[(40, 0.01)]);
    frame.silent = false;
    frame.raw = [[1.0; BANDS]; 2];
    frame.loud = [[0.5; BANDS]; 2];
    Hearing::default().kinds(&AudioFrame::default(), &mut frame, 1.0);
    let at = |kind: &str, unit: &str| {
        read(&frame, None, &binding(json!({ "kind": kind, "unit": unit })), Band::Index(40))
    };
    assert!((at("absolute", "db") - 0.5).abs() < 0.01 && (at("absolute", "amplitude") - 0.1).abs() < 0.01);
    assert!(at("relative", "amplitude") > 0.9);
}

#[test]
fn kind_unit_and_the_surge_fields_are_required() {
    let load = |b: Json| {
        let layer =
            json!({ "blend_space": "canvas", "id": "l0", "type": "solid", "color": "#ffffff", "opacity": b });
        let text = json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": [layer] });
        Scene::load(&reiwand_upgrade::fill::whole(&text.to_string()), &BTreeMap::new()).is_ok()
    };
    assert!(load(json!({ "audio": 4, "kind": "relative", "unit": "db" })));
    assert!(!load(json!({ "audio": 4, "kind": "surge", "unit": "db", "window": 1 })));
    assert!(load(
        json!({ "audio": 4, "kind": "surge", "unit": "db", "window": 1, "reference": "all", "hold_silence": true })
    ));
    assert!(!load(json!({ "audio": 4, "kind": "relative", "unit": "db", "window": 1 })));
    assert!(!load(
        json!({ "audio": 4, "kind": "surge", "unit": "db", "window": 0.01, "reference": "all", "hold_silence": true })
    ));
}
