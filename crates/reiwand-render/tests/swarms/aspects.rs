use super::*;

fn lamp(at: [f32; 2], velocity: [f32; 2], render: Value) -> Value {
    json!({ "capacity": 8, "seed": 5, "emitters": [{ "shape": { "point": at }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "velocity": [velocity, velocity], "size": [24, 24],
            "tilt": { "angle": [[0, 0], [0, 0]], "spin": [[0, 0], [0, 0]] }, "color": ["#ffffff", "#ffffff"] },
        "render": render })
}

fn extent(pixels: &[u8], around: [u32; 2]) -> [u32; 2] {
    let lit = |x: u32, y: u32| at(pixels, x, y)[0] > 100;
    let span = |c: u32, max: u32| c.saturating_sub(16)..(c + 16).min(max);
    let across = |y: u32| span(around[0], SIZE.0).filter(|x| lit(*x, y)).count() as u32;
    let down = |x: u32| span(around[1], SIZE.1).filter(|y| lit(x, *y)).count() as u32;
    [across(around[1]), down(around[0])]
}

fn shot(system: Value, around: [u32; 2], settle: f64) -> [[u32; 2]; 2] {
    let layers = json!([{ "id": "s", "type": "particles", "system": "s", "blend": "add" }]);
    let Some(r) = renderer() else { return [[0; 2]; 2] };
    let mut found = [[0; 2]; 2];
    for (k, gpu) in [false, true].into_iter().enumerate() {
        let Some(mut run) = Run::new(r, json!({ "s": system.clone() }), layers.clone(), gpu) else {
            return found;
        };
        run.frame(r, 0.0);
        found[k] = extent(&run.frame(r, settle), around);
    }
    found
}

#[test]
fn a_slim_texture_draws_a_slim_rectangle_at_its_size_on_both_paths() {
    for (aspect, tall) in [(0.25, true), (4.0, false)] {
        for found in
            shot(lamp([40.0, 40.0], [0.0, 0.0], json!({ "aspect": aspect })), [40, 40], 1.0 / 30.0_f64)
        {
            let [long, short] = if tall { [found[1], found[0]] } else { [found[0], found[1]] };
            if long == 0 {
                continue;
            }
            assert!(long.abs_diff(19) <= 3, "{aspect}: {found:?}");
            assert!((short * 4).abs_diff(long) <= 6, "{aspect}: {found:?}");
        }
    }
}

#[test]
fn a_tall_sprite_carried_by_its_motion_lies_along_it_and_stretches_along_its_long_side() {
    let flying = |render: Value| {
        let mut system = lamp([40.0, 40.0], [30.0, 0.0], render);
        system["init"]["size"] = json!([12, 12]);
        system
    };
    let plain = shot(flying(json!({ "aspect": 0.25, "orient": "velocity" })), [41, 40], 1.0 / 30.0_f64);
    let long = shot(
        flying(
            json!({ "aspect": 0.25, "stretch": { "mode": "add", "depth": "flat", "time": 0.5, "min": 8, "max": 12 } }),
        ),
        [41, 40],
        1.0 / 30.0_f64,
    );
    for found in plain {
        assert!(found[0] == 0 || (found[0] > found[1] * 2), "{found:?}");
    }
    for (a, b) in plain.iter().zip(long) {
        assert!(b[0] == 0 || b[0] > a[0] + 5, "{a:?} {b:?}");
    }
}

fn banded() -> image::RgbaImage {
    image::RgbaImage::from_fn(8, 32, |_, y| {
        let (g, b) = if y < 8 { (0, 0) } else { (255, 255) };
        image::Rgba([255, g, b, 255])
    })
}

#[test]
fn the_top_of_a_tall_texture_leads_when_it_follows_its_motion_and_stays_up_otherwise() {
    let Some(r) = renderer() else { return };
    let moving = |velocity: [f32; 2], at: [f32; 2]| {
        let mut s = lamp(at, velocity, json!({ "aspect": 0.25, "texture": "tex", "orient": "velocity" }));
        s["init"]["size"] = json!([24, 24]);
        s
    };
    let layers = json!([{ "id": "s", "type": "particles", "system": "s", "blend": "normal" }]);
    let assets = json!({ "tex": { "path": "tex.png" } });
    let red = |p: &[u8], x: u32, y: u32| at(p, x, y)[2] < 80 && at(p, x, y)[0] > 150;
    let white = |p: &[u8], x: u32, y: u32| at(p, x, y)[2] > 150;
    for gpu in [false, true] {
        for (velocity, ahead, behind) in
            [([30.0, 0.0], [49, 40], [31, 40]), ([0.0, 30.0], [40, 49], [40, 31])]
        {
            let images = vec![("tex", Pixels::new(banded()))];
            let scene = (json!({ "s": moving(velocity, [40.0, 40.0]) }), layers.clone(), assets.clone());
            let Some(mut run) = Run::with(r, scene, images, gpu) else { return };
            run.frame(r, 0.0);
            let p = run.frame(r, 1.0 / 30.0);
            assert!(red(&p, ahead[0], ahead[1]) && white(&p, behind[0], behind[1]), "gpu {gpu} {velocity:?}");
        }
        let mut still = moving([0.0, 0.0], [40.0, 40.0]);
        still["render"]["orient"] = json!("fixed");
        let images = vec![("tex", Pixels::new(banded()))];
        let Some(mut run) =
            Run::with(r, (json!({ "s": still }), layers.clone(), assets.clone()), images, gpu)
        else {
            return;
        };
        run.frame(r, 0.0);
        let p = run.frame(r, 1.0 / 30.0);
        assert!(red(&p, 40, 31) && white(&p, 40, 49), "gpu {gpu}: upright");
    }
}
