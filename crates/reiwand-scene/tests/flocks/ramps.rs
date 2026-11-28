use super::{Fly, still};
use serde_json::{Value as Json, json};

// A size pulse that rises from a floor of 0: its size is 1 − fade, so the size reads the window.
fn faded(ramp: Option<&str>, seconds: f64) -> f32 {
    let mut force = json!({ "oscillate": { "what": "size", "from": "floor", "freq": [1, 1], "scale": [0, 0], "phase": [0, 0], "mask": [1, 1] },
        "window": [0, 0.4, 1, 1] });
    if let Some(r) = ramp {
        force["ramp"] = Json::from(r);
    }
    let mut fly = Fly::of(json!({ "a": still(json!({ "forces": [force] })) }));
    fly.run(seconds);
    let mut sizes: Vec<f32> = fly.alive(0).iter().map(|p| p.look[1]).collect();
    sizes.sort_by(f32::total_cmp);
    1.0 - sizes[sizes.len() / 2]
}

#[test]
fn a_linear_window_ramps_in_a_straight_line() {
    // The ramp fills the first 40% of a 10 s life: a quarter of it is 1 s, half of it 2 s.
    assert!((faded(Some("linear"), 1.0) - 0.25).abs() < 0.02, "{}", faded(Some("linear"), 1.0));
    assert!((faded(Some("linear"), 2.0) - 0.5).abs() < 0.02, "{}", faded(Some("linear"), 2.0));
    assert!((faded(Some("linear"), 3.0) - 0.75).abs() < 0.02, "{}", faded(Some("linear"), 3.0));
}

#[test]
fn without_ramp_or_with_smooth_the_window_eases_as_before() {
    let eased = |x: f32| x * x * (3.0 - 2.0 * x);
    for ramp in [None, Some("smooth")] {
        assert!((faded(ramp, 1.0) - eased(0.25)).abs() < 0.02, "{ramp:?}: {}", faded(ramp, 1.0));
        assert!((faded(ramp, 2.0) - 0.5).abs() < 0.02, "{ramp:?}");
    }
}
