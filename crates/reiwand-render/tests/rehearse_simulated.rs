use crate::gpu::renderer;
use reiwand_render::{Rehearsal, SceneAssets};
use serde_json::json;

fn embers() -> reiwand_scene::Scene {
    let system = json!({ "capacity": 300, "seed": 4,
        "emitters": [{ "shape": { "box": { "from": [20, 60], "to": [140, 80] } }, "rate": 40 }],
        "init": { "life": [3, 5], "speed": [5, 20], "angle": [260, 280], "size": [3, 5], "color": ["#ffa040", "#ff6020"] },
        "forces": [{ "noise_field": { "strength": { "min": 20, "max": 80 }, "phase": { "min": 0, "max": 300 },
            "scale": 0.03, "drift": 6, "mask": [1, 1, 0] } }, { "drag": 0.5 }] });
    let layer =
        json!({ "blend_space": "canvas", "id": "p", "type": "particles", "system": "s", "blend": "add" });
    let doc = json!({ "schema": reiwand_scene::SCHEMA, "canvas": { "size": [160, 90], "background": "#000000" },
        "clock": { "seed": 9 }, "layers": [layer], "particles": { "s": system } });
    reiwand_scene::Scene::load(&crate::gpu::whole(&doc.to_string()), &Default::default()).unwrap()
}

#[test]
fn a_simulated_render_counts_the_frames_it_plays_ahead() {
    let at = |time: f64, fps: u32| Rehearsal { time, fps, simulate: true, ..Rehearsal::default() };
    assert_eq!((at(2.0, 30).ahead(), at(2.0, 30).moment(60)), (60, 2.0));
    assert_eq!((at(0.0, 30).ahead(), Rehearsal { time: 2.0, ..Rehearsal::default() }.ahead()), (0, 0));
    let odd = at(0.05, 30);
    assert_eq!(odd.ahead(), 2);
    assert_eq!(
        [odd.moment(0), odd.moment(1), odd.moment(2), odd.moment(3)],
        [0.0, 1.0 / 30.0, 0.05, 0.05 + 1.0 / 30.0]
    );
    assert_eq!(at(Rehearsal::MAX_SIMULATE + 50.0, 30).start(), Rehearsal::MAX_SIMULATE);
}

#[test]
fn a_simulated_render_at_t_draws_what_a_live_run_frame_by_frame_draws_at_t() {
    let Some(r) = renderer() else { return };
    let scene = embers();
    let shot = |wanted: Rehearsal| {
        r.rehearse(&scene, &SceneAssets::default(), &Default::default(), &wanted).unwrap()
    };
    let base = Rehearsal { edge: 160, wall: 0.0, fps: 30, ..Rehearsal::default() };
    let simulated = shot(Rehearsal { time: 2.0, simulate: true, ..base.clone() });
    let live = shot(Rehearsal { frames: 63, ..base.clone() });
    assert_eq!(simulated.frames.len(), 1);
    assert!(simulated.frames[0].pixels().any(|p| p.0[0] > 40), "the embers show");
    assert_eq!(simulated.frames[0], live.frames[60], "the same frames give the same picture");
    let again = shot(Rehearsal { time: 2.0, simulate: true, ..base.clone() });
    assert_eq!(again.frames[0], simulated.frames[0], "a simulated render is the same every time");
    let clip = shot(Rehearsal { time: 2.0, simulate: true, frames: 3, ..base.clone() });
    assert_eq!(clip.frames[..], live.frames[60..], "a simulated clip goes on as the live run");
}
