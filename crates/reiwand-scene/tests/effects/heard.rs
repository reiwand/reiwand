use super::*;

fn tone(on: bool) -> AudioFrame {
    let mut power = [[0.0; BANDS]; 2];
    if on {
        power[0][3] = 0.5;
        power[1][3] = 0.5;
    }
    AudioFrame { silent: !on, power, ..AudioFrame::default() }
}

#[test]
fn an_effect_with_a_surge_input_reads_above_one_after_silence_and_decays() {
    probe(
        json!({ "id": "probe_surge", "uses": ["audio"],
            "audio": [{ "kind": "surge", "unit": "amplitude", "window": 1, "reference": "band", "hold_silence": false }],
            "passes": [{ "entry": "fs_a" }] }),
        "",
    );
    let mut rt = runtime(json!([{ "effect": "probe_surge" }]));
    for _ in 0..180 {
        rt.set_audio(tone(false));
        rt.advance(1.0 / 60.0);
        rt.frame(VIEW, Quality::Q3);
    }
    let mut seen = Vec::new();
    for _ in 0..240 {
        rt.set_audio(tone(true));
        rt.advance(1.0 / 60.0);
        seen.push(rt.frame(VIEW, Quality::Q3).effects[0].heard[0][3]);
    }
    let peak = seen.iter().cloned().fold(0.0, f32::max);
    assert!(peak > 1.0, "peak {peak}");
    assert!(*seen.last().unwrap() < peak * 0.6);
}

#[test]
fn effect_audio_inputs_state_every_surge_field_and_stay_within_the_cap() {
    let fine = |audio: serde_json::Value| {
        let d = json!({ "id": "probe_bad_audio", "audio": audio, "passes": [{ "entry": "fs_a" }] });
        Descriptor::parse(&d.to_string(), "").unwrap().problems().is_empty()
    };
    assert!(!fine(json!([{ "kind": "surge", "unit": "db" }])));
    assert!(!fine(json!([{ "kind": "relative", "unit": "db", "window": 1 }])));
    let three = json!([{ "kind": "relative", "unit": "db" }, { "kind": "relative", "unit": "db" }, { "kind": "relative", "unit": "db" }]);
    assert!(!fine(three));
    assert!(fine(json!([{ "kind": "relative", "unit": "amplitude" }])));
}
