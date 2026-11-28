use reiwand_audio::{Played, Track};
use reiwand_scene::{Binding, Usual, read, usual_key};
use serde_json::{Value as Json, json};
use std::f64::consts::TAU;
use std::sync::Arc;

const FPS: f64 = 60.0;
const RATE: u32 = 48_000;
const PARTS: [(&str, f64); 6] =
    [("silence", 3.0), ("tone", 4.0), ("pause", 2.0), ("sweep", 6.0), ("after", 3.0), ("kicks", 3.0)];

fn binding(extra: Json) -> Binding<f32> {
    let mut b = json!({ "audio": [0, 15], "of": 16, "reduce": "max", "kind": "surge", "unit": "amplitude",
        "window": 2, "reference": "band", "hold_silence": false, "bounds": [0.5, 1.5] });
    b.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
    serde_json::from_value(reiwand_scene::prefill::of("Binding_for_float", b)).unwrap()
}

fn wave(part: &str, t: f64) -> f64 {
    let rise = 100f64.ln() / 6.0;
    match part {
        "tone" => 0.5 * (TAU * 1000.0 * t).sin(),
        "sweep" => 0.5 * (TAU * 100.0 * ((rise * t).exp() - 1.0) / rise).sin(),
        "kicks" => (TAU * 60.0 * t).sin() * (-(t % 0.5) * 12.0).exp(),
        _ => 0.0,
    }
}

fn track() -> Arc<Track> {
    let samples: Vec<i16> = PARTS
        .iter()
        .flat_map(|&(part, seconds)| {
            (0..(seconds * f64::from(RATE)) as usize)
                .map(move |i| (wave(part, i as f64 / f64::from(RATE)) * 30_000.0) as i16)
        })
        .collect();
    Arc::new(Track::new(RATE, 0, 1, &samples))
}

fn heard(b: &Binding<f32>) -> Vec<(&'static str, Vec<f32>)> {
    let mut played = Played::new(track(), 0.0, 1.0 / FPS);
    let (key, [lo, hi]) = (usual_key(b), b.bounds.unwrap());
    let (mut usual, mut tick, mut out) = (None::<Usual>, 0usize, Vec::new());
    for (part, seconds) in PARTS {
        let mut values = Vec::new();
        for _ in 0..(seconds * FPS).round() as usize {
            let frame = played.at(tick as f64 / FPS).clone();
            tick += 1;
            let usual = usual.get_or_insert_with(|| Usual::at(&frame));
            usual.advance(&frame, (1.0 / FPS) as f32, key);
            values.push(((read(&frame, Some(usual), b, b.audio.unwrap()) - lo) / (hi - lo)).clamp(0.0, 1.0));
        }
        out.push((part, values));
    }
    out
}

fn mean(v: &[f32]) -> f32 {
    v.iter().sum::<f32>() / v.len() as f32
}

fn part<'a>(heard: &'a [(&str, Vec<f32>)], name: &str) -> &'a [f32] {
    &heard.iter().find(|(p, _)| *p == name).unwrap().1
}

#[test]
fn a_surge_of_a_band_without_sound_is_zero_and_starts_full_after_silence() {
    let all = heard(&binding(json!({})));
    assert!(part(&all, "silence").iter().all(|&v| v == 0.0), "no sound, no level");
    let tone = part(&all, "tone");
    assert!(mean(&tone[..30]) > 0.95, "full after the silence: {}", mean(&tone[..30]));
    assert!(tone[tone.len() - 1] < 0.6, "back to half as the tone goes on: {}", tone[tone.len() - 1]);
    assert!(mean(part(&all, "pause")) < 0.02 && mean(part(&all, "after")) < 0.02);
    assert!(mean(part(&all, "sweep")) > 0.9);
    assert!(part(&all, "kicks").iter().copied().fold(0.0, f32::max) > 0.9, "each kick lights it");
    let high = heard(&binding(json!({ "audio": [14, 15] })));
    assert!(part(&high, "tone").iter().all(|&v| v == 0.0), "a band the tone never reaches stays at 0");
    assert!(part(&high, "silence").iter().chain(part(&high, "pause")).all(|&v| v == 0.0));
}

#[test]
fn a_kick_after_quiet_stands_out_and_a_steady_tone_reads_as_usual() {
    let low = heard(&binding(json!({ "audio": [0, 1], "bounds": [0.0, 2.0] })));
    let kicks = part(&low, "kicks");
    assert!(kicks.iter().copied().fold(0.0, f32::max) > 0.9, "a kick after the quiet is far above usual");
    let tone = heard(&binding(json!({ "audio": [8, 9], "bounds": [0.0, 2.0], "window": 0.5 })));
    let late = &part(&tone, "tone")[180..];
    assert!(late.iter().all(|&v| (v - 0.5).abs() < 0.05), "a steady tone is 1, as usual: {}", late[0]);
}
