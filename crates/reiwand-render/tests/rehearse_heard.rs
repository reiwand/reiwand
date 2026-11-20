use reiwand_render::{Rehearsal, Renderer, SceneAssets, Track};
use serde_json::{Value, json};
use std::sync::Arc;

fn gpu() -> Option<&'static Renderer> {
    match Renderer::shared() {
        Ok(r) => Some(r),
        Err(e) if std::env::var_os("REIWAND_REQUIRE_GPU").is_none() => {
            eprintln!("skipping GPU test: {e}");
            None
        }
        Err(e) => panic!("GPU required but unavailable: {e}"),
    }
}

fn lamp(opacity: Value) -> reiwand_scene::Scene {
    let mut doc = json!({ "schema": reiwand_scene::SCHEMA, "canvas": { "size": [32, 18], "background": "#000000" },
        "layers": [{ "id": "lamp", "type": "solid", "color": "#ffffff", "opacity": opacity }] });
    reiwand_upgrade::fill::complete(reiwand_upgrade::fill::Format::Scene, &mut doc);
    reiwand_scene::Scene::load(&doc.to_string(), &Default::default()).unwrap()
}

fn kicks() -> Arc<Track> {
    let rate = 48_000;
    let kick = |i: usize| {
        let t = i as f64 / f64::from(rate);
        let since = t % 0.5;
        let wave = (std::f64::consts::TAU * 60.0 * t).sin() * (-since * 12.0).exp();
        if t >= 0.4 { (wave * 30_000.0) as i16 } else { 0 }
    };
    let samples: Vec<i16> = (0..2 * rate as usize).map(kick).collect();
    Arc::new(Track::new(rate, 0, 1, &samples))
}

fn reel(scene: &reiwand_scene::Scene, audio: Option<Arc<Track>>) -> Vec<image::RgbaImage> {
    let wanted = Rehearsal { edge: 32, wall: 0.0, frames: 40, fps: 30, audio, ..Rehearsal::default() };
    let r = Renderer::shared().unwrap();
    r.rehearse(scene, &SceneAssets::default(), &Default::default(), &wanted).unwrap().frames
}

fn brightness(frames: &[image::RgbaImage]) -> Vec<u8> {
    frames.iter().map(|f| f.get_pixel(16, 9).0[0]).collect()
}

#[test]
fn a_track_moves_what_hears_the_sound_and_draws_the_same_every_time() {
    if gpu().is_none() {
        return;
    }
    let scene = lamp(json!({ "audio": "bass", "kind": "relative", "unit": "db" }));
    let quiet = reel(&scene, None);
    assert!(brightness(&quiet).iter().all(|&v| v == 0), "without a track the scene hears silence");
    let heard = reel(&scene, Some(kicks()));
    let lit = brightness(&heard);
    assert!(lit[..12].iter().all(|&v| v == 0), "the track is silent until 0.4 s: {lit:?}");
    assert!(lit.iter().any(|&v| v > 100), "the kicks light the lamp: {lit:?}");
    assert!(lit.windows(2).any(|w| w[1] < w[0]), "the lamp falls between kicks: {lit:?}");
    let again = reel(&scene, Some(kicks()));
    assert!(heard.iter().zip(&again).all(|(a, b)| a.as_raw() == b.as_raw()), "the same bytes twice");
}

#[test]
fn a_track_changes_nothing_in_a_scene_that_does_not_hear() {
    if gpu().is_none() {
        return;
    }
    let scene = lamp(json!(0.5));
    let (quiet, heard) = (reel(&scene, None), reel(&scene, Some(kicks())));
    assert!(quiet.iter().zip(&heard).all(|(a, b)| a.as_raw() == b.as_raw()));
}

#[test]
fn a_late_first_frame_hears_the_track_from_its_start() {
    if gpu().is_none() {
        return;
    }
    let scene = lamp(json!({ "audio": "bass", "kind": "relative", "unit": "db" }));
    let wanted =
        Rehearsal { edge: 32, wall: 0.0, time: 0.5, fps: 30, audio: Some(kicks()), ..Rehearsal::default() };
    let r = Renderer::shared().unwrap();
    let late = r.rehearse(&scene, &SceneAssets::default(), &Default::default(), &wanted).unwrap().frames;
    let from_zero = brightness(&reel(&scene, Some(kicks())))[15];
    assert!(late[0].get_pixel(16, 9).0[0] > 100 && from_zero > 100);
}
