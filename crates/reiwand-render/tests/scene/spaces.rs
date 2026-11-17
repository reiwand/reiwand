use super::*;

fn white(space: &str, blend: &str) -> Value {
    json!({ "id": "w", "type": "solid", "color": "#ffffff", "opacity": 0.5, "blend": blend, "blend_space": space })
}

fn over(background: &str, canvas_space: &str, layer: Value) -> [u8; 3] {
    let Some(r) = renderer() else { return [0; 3] };
    let canvas = json!({ "size": [100, 50], "background": background, "blend_space": canvas_space });
    at(&Shot::new(r, canvas, json!([layer]), &[]).pixels(r), 50, 25)
}

fn skip() -> bool {
    renderer().is_none()
}

#[test]
fn a_layer_in_encoded_values_mixes_like_an_editor_on_a_linear_canvas() {
    if skip() {
        return;
    }
    for (back, linear, encoded) in [("#000000", 188, 128), ("#808080", 205, 192)] {
        assert!(near(over(back, "linear", white("canvas", "normal")), [linear; 3], 3), "{back} canvas");
        assert!(near(over(back, "linear", white("linear", "normal")), [linear; 3], 3), "{back} linear");
        assert!(near(over(back, "linear", white("srgb", "normal")), [encoded; 3], 3), "{back} srgb");
    }
}

#[test]
fn a_layer_in_linear_light_stays_linear_on_an_encoded_canvas() {
    if skip() {
        return;
    }
    assert!(near(over("#000000", "srgb", white("canvas", "normal")), [128; 3], 3));
    assert!(near(over("#000000", "srgb", white("srgb", "normal")), [128; 3], 3));
    assert!(near(over("#000000", "srgb", white("linear", "normal")), [188; 3], 3));
}

#[test]
fn every_blend_mode_follows_the_layer_space() {
    if skip() {
        return;
    }
    assert!(near(over("#808080", "linear", white("canvas", "add")), [219; 3], 3));
    assert!(near(over("#808080", "linear", white("srgb", "add")), [255; 3], 3));
    assert!(near(over("#808080", "linear", white("srgb", "screen")), [192; 3], 3));
    assert!(near(over("#808080", "linear", white("srgb", "multiply")), [128; 3], 3));
    assert!(near(over("#808080", "linear", white("linear", "multiply")), [128; 3], 3));
}

#[test]
fn a_group_composites_as_a_whole_in_its_own_space() {
    let Some(r) = renderer() else { return };
    let inner = json!({ "id": "w", "type": "solid", "color": "#ffffff", "blend_space": "canvas" });
    let group = |space: &str| json!([{ "id": "g", "type": "group", "opacity": 0.5, "blend_space": space, "layers": [inner] }]);
    let canvas = json!({ "size": [100, 50], "background": "#000000", "blend_space": "linear" });
    let shot = |space: &str| at(&Shot::new(r, canvas.clone(), group(space), &[]).pixels(r), 50, 25);
    assert!(near(shot("canvas"), [188; 3], 3) && near(shot("srgb"), [128; 3], 3));
}

#[test]
fn particles_mix_in_the_space_of_their_layer() {
    let Some(r) = renderer() else { return };
    let systems = json!({ "dots": {
        "capacity": 2,
        "emitters": [{ "shape": { "point": [50, 25] }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [10, 10], "speed": [0, 0], "size": [40, 40], "color": ["#ffffff", "#ffffff"] }
    } });
    let shot = |space: &str| {
        let layer =
            json!({ "id": "p", "type": "particles", "system": "dots", "opacity": 0.5, "blend_space": space });
        let text = json!({ "canvas": canvas("#000000"), "layers": [layer], "particles": systems });
        at(&Shot::with(r, text, &[]).pixels(r), 50, 25)
    };
    assert!(near(shot("canvas"), [188; 3], 4) && near(shot("srgb"), [128; 3], 4));
}

#[test]
fn an_adjustment_fades_into_the_backdrop_in_its_space() {
    let Some(r) = renderer() else { return };
    let layers = |space: &str| {
        json!([
            { "id": "back", "type": "solid", "color": "#ffffff", "blend_space": "canvas" },
            { "id": "dark", "type": "adjustment", "opacity": 0.5, "blend_space": space,
              "effects": [{ "effect": "tint", "params": { "color": "#000000" } }] }
        ])
    };
    let canvas = json!({ "size": [100, 50], "background": "#000000", "blend_space": "linear" });
    let shot = |space: &str| at(&Shot::new(r, canvas.clone(), layers(space), &[]).pixels(r), 50, 25);
    assert!(near(shot("canvas"), [188; 3], 4) && near(shot("srgb"), [128; 3], 4));
}
