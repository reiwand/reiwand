use super::{Shot, at, files, renderer};
use reiwand_scene::{Quality, SCHEMA, Scene};
use serde_json::{Value, json};

fn take(color: &str, tint: &str, back: &str, quality: Quality) -> Option<image::RgbaImage> {
    let refract = json!({ "normal": "bump", "units": "size", "amount": 1, "color": tint });
    let system = json!({ "capacity": 4, "seed": 2, "emitters": [{ "shape": { "point": [80, 45] }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "speed": [0, 0], "size": [40, 40], "color": [color, color] },
        "render": { "refract": refract } });
    shot(system, back, [160, 90], quality)
}

fn glass(color: &str, tint: &str, amount: u32, quality: Quality) -> Option<image::RgbaImage> {
    let refract = json!({ "normal": "bump", "units": "size", "amount": amount, "color": tint });
    let system = json!({ "capacity": 4, "seed": 2, "emitters": [{ "shape": { "point": [80, 45] }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "speed": [0, 0], "size": [40, 40], "color": [color, color] },
        "render": { "refract": refract } });
    shot(system, "mask", [160, 90], quality)
}

fn simulated(gpu: bool, color: &str, tint: &str) -> Option<image::RgbaImage> {
    let refract = json!({ "normal": "bump", "units": "size", "amount": 1, "color": tint });
    let system = json!({ "capacity": 4, "seed": 2, "emitters": [{ "shape": { "point": [80, 45] }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "speed": [0, 0], "size": [40, 40], "color": [color, color] },
        "render": { "refract": refract } });
    let r = renderer()?;
    let assets = json!({ "wall": { "path": "wall.png" }, "bump": { "path": "bump.png", "linear": true } });
    let back = json!({ "blend_space": "canvas", "id": "back", "type": "image", "image": "wall", "size": [160, 90],
        "transform": { "position": [80, 45] } });
    let layers = [back, json!({ "blend_space": "canvas", "id": "p", "type": "particles", "system": "s" })];
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [160, 90], "background": "#000000" }, "assets": assets,
        "layers": layers, "particles": { "s": system } });
    let scene = Scene::load(&crate::gpu::whole(&text.to_string()), &Default::default()).unwrap();
    let mut shot = Shot::new(r, scene, &files([255, 128, 128, 255]));
    if !shot.view.set_gpu_particles(r, gpu) {
        return None;
    }
    shot.pixels(r, Quality::Q3);
    shot.runtime.advance(1.0 / 30.0);
    Some(shot.pixels(r, Quality::Q3))
}

fn shot(system: Value, back: &str, size: [u32; 2], quality: Quality) -> Option<image::RgbaImage> {
    let r = renderer()?;
    let [w, h] = size;
    let layers = [
        json!({ "blend_space": "canvas", "id": "back", "type": "image", "image": back, "size": [w, h],
            "transform": { "position": [w / 2, h / 2] } }),
        json!({ "blend_space": "canvas", "id": "p", "type": "particles", "system": "s" }),
    ];
    let assets = json!({ "wall": { "path": "wall.png" }, "bump": { "path": "bump.png", "linear": true },
        "mask": { "path": "mask.png", "linear": true } });
    let text = json!({ "schema": SCHEMA, "canvas": { "size": size, "background": "#000000" }, "assets": assets,
        "layers": layers, "particles": { "s": system } });
    let scene = Scene::load(&crate::gpu::whole(&text.to_string()), &Default::default()).unwrap();
    let mut shot = Shot::new(r, scene, &files([255, 128, 128, 255]));
    shot.sized(r, quality, (w, h));
    shot.runtime.advance(1.0 / 30.0);
    Some(shot.sized(r, quality, (w, h)))
}

fn rgb(image: &image::RgbaImage, x: u32, y: u32) -> [u8; 3] {
    let [b, g, r, _] = at(image, x, y);
    [r, g, b]
}

fn close(a: &image::RgbaImage, b: &image::RgbaImage) -> bool {
    a.pixels().zip(b.pixels()).all(|(p, q)| (0..4).all(|k| p.0[k].abs_diff(q.0[k]) <= 8))
}

#[test]
fn a_multiplied_refraction_tints_the_bent_background_and_an_added_one_lights_it() {
    let Some(none) = take("#ff0000", "none", "wall", Quality::Q3) else { return };
    let (Some(multiply), Some(add)) =
        (take("#ff0000", "multiply", "wall", Quality::Q3), take("#0000ff", "add", "wall", Quality::Q3))
    else {
        return;
    };
    let [r, g, b] = rgb(&none, 80, 45);
    assert!(r > 150 && g > 150 && b > 150, "without color the sprite is only a bend: {:?}", [r, g, b]);
    let [r, g, b] = rgb(&multiply, 80, 45);
    assert!(r > 150 && g < 30 && b < 30, "multiply tints the background red: {:?}", [r, g, b]);
    let [r, g, b] = rgb(&add, 80, 45);
    assert!(r > 150 && g > 150 && b == 255, "add lights the background blue: {:?}", [r, g, b]);
    assert!(at(&multiply, 10, 10) == at(&none, 10, 10) && at(&add, 10, 10) == at(&none, 10, 10));
    let Some(low) = take("#ff0000", "multiply", "wall", Quality::Q1) else { return };
    assert!(rgb(&low, 80, 45)[1] > 150, "Q1 leaves the sprites out: {:?}", rgb(&low, 80, 45));
}

#[test]
fn both_simulations_tint_a_refraction_alike() {
    let (Some(cpu), Some(gpu)) =
        (simulated(false, "#ff0000", "multiply"), simulated(true, "#ff0000", "multiply"))
    else {
        return;
    };
    assert!(close(&cpu, &gpu), "the CPU and GPU paths tint differently");
    let [r, g, _] = rgb(&gpu, 80, 45);
    assert!(r > 150 && g < 30, "{:?}", [r, g]);
}

#[test]
fn a_colored_refraction_still_bends_the_background() {
    let Some(none) = take("#ffffff", "none", "mask", Quality::Q3) else { return };
    let (Some(white), Some(black)) =
        (take("#ffffff", "multiply", "mask", Quality::Q3), take("#000000", "add", "mask", Quality::Q3))
    else {
        return;
    };
    assert!(at(&none, 75, 45)[0] > 200, "the bend brings the right half: {:?}", at(&none, 75, 45));
    assert!(close(&none, &white), "a white multiply leaves the bent picture as it is");
    assert!(close(&none, &black), "a black add leaves the bent picture as it is");
    let Some(half) = take("#ff0000", "multiply", "mask", Quality::Q2) else { return };
    let [r, g, _] = rgb(&half, 75, 45);
    assert!(r > 200 && g < 30, "Q2 bends at half size and tints: {:?}", [r, g]);
}

#[test]
fn a_shift_in_pixels_reaches_past_256() {
    let system = |shift: u32| {
        json!({ "capacity": 4, "seed": 2, "emitters": [{ "shape": { "point": [120, 45] }, "bursts": [{ "count": 1 }] }],
            "init": { "life": [30, 30], "speed": [0, 0], "size": [40, 40], "color": ["#ffffff", "#ffffff"] },
            "render": { "refract": { "normal": "bump", "units": "pixels", "shift": shift, "color": "none" } } })
    };
    let Some(short) = shot(system(250), "mask", [800, 90], Quality::Q3) else { return };
    let Some(long) = shot(system(320), "mask", [800, 90], Quality::Q3) else { return };
    assert!(
        at(&short, 120, 45)[0] < 30,
        "250 px from 120 still reads the dark half: {:?}",
        at(&short, 120, 45)
    );
    assert!(at(&long, 120, 45)[0] > 200, "320 px from 120 reaches the light half: {:?}", at(&long, 120, 45));
}

#[test]
fn a_light_refraction_adds_the_bent_background_over_the_unbent_one() {
    let Some(unbent) = glass("#ff0000", "none", 0, Quality::Q3) else { return };
    let tints = ["none", "add", "light"].map(|tint| glass("#ff0000", tint, 1, Quality::Q3));
    let ([Some(none), Some(add), Some(light)], Some(dark)) =
        (tints, glass("#000000", "light", 1, Quality::Q3))
    else {
        return;
    };
    assert!(close(&dark, &unbent), "a black light leaves the unbent background as it is");
    assert!(rgb(&unbent, 75, 45)[0] < 30, "the dark half lies under the sprite: {:?}", rgb(&unbent, 75, 45));
    let [r, g, b] = rgb(&light, 75, 45);
    assert!(r > 200 && g < 30 && b < 30, "the bent white adds red light over the dark half: {:?}", [r, g, b]);
    assert!(rgb(&none, 75, 45)[1] > 200 && rgb(&add, 75, 45)[1] > 200, "none and add replace the dark half");
    assert!(at(&light, 10, 10) == at(&unbent, 10, 10) && at(&light, 150, 10) == at(&unbent, 150, 10));
    let Some(half) = glass("#ff0000", "light", 1, Quality::Q2) else { return };
    let [r, g, _] = rgb(&half, 75, 45);
    assert!(r > 200 && g < 30, "Q2 bends at half size and adds the light: {:?}", [r, g]);
    let Some(low) = glass("#ff0000", "light", 1, Quality::Q1) else { return };
    assert!(close(&low, &unbent), "Q1 leaves the sprites out");
}
