use super::*;

fn dots(r: &Renderer, size: [u32; 2], bloom: Value, list: &[(u32, [u8; 3], u32)]) -> Vec<u8> {
    let layers: Vec<Value> = list
        .iter()
        .enumerate()
        .map(|(i, (x, c, side))| {
            let color = format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2]);
            json!({ "blend_space": "canvas", "id": format!("d{i}"), "type": "solid", "color": color,
                "size": [side, side], "transform": { "position": [x, size[1] / 2] } })
        })
        .collect();
    let canvas = json!({ "size": size, "background": "#000000", "bloom": bloom });
    Shot::new(r, canvas, json!(layers), &[]).pixels(r)
}

fn glow(extra: Value) -> Value {
    let mut base = json!({ "threshold": 0.5, "strength": 2, "radius": 8 });
    base.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
    base
}

#[test]
fn saturation_two_strengthens_the_red_halo_against_grey() {
    let Some(r) = renderer() else { return };
    let red = [(50, [255, 64, 32], 6)];
    let plain = at(&dots(r, [100, 50], glow(json!({})), &red), 56, 25);
    let vivid = at(&dots(r, [100, 50], glow(json!({ "saturation": 2 })), &red), 56, 25);
    let grey = at(&dots(r, [100, 50], glow(json!({ "saturation": 0 })), &red), 56, 25);
    let share = |p: [u8; 3]| f32::from(p[0]) / (f32::from(p[1]) + f32::from(p[2]) + 1.0);
    assert!(share(vivid) > share(plain) && share(plain) > share(grey), "{vivid:?} {plain:?} {grey:?}");
    assert!(grey[0].abs_diff(grey[1]) <= 2, "{grey:?}");
}

#[test]
fn a_limit_makes_a_white_and_a_grey_halo_equal() {
    let Some(r) = renderer() else { return };
    let pair = [(25, [255; 3], 6), (75, [204; 3], 6)];
    let free = dots(r, [100, 50], glow(json!({})), &pair);
    assert!(at(&free, 31, 25)[0] > at(&free, 81, 25)[0] + 3);
    let capped = dots(r, [100, 50], glow(json!({ "limit": 0.2 })), &pair);
    let (white, grey) = (at(&capped, 31, 25), at(&capped, 81, 25));
    assert!(white[0] > 5 && white[0].abs_diff(grey[0]) <= 5, "{white:?} {grey:?}");
}

#[test]
fn a_hard_knee_leaves_what_lies_under_the_threshold_untouched() {
    let Some(r) = renderer() else { return };
    let under = [(50, [200; 3], 6)];
    let hard = dots(r, [100, 50], glow(json!({ "threshold": 0.65, "strength": 8, "knee": 0 })), &under);
    check(&hard, &[((54, 25), [0; 3]), ((56, 25), [0; 3])]);
    let soft = dots(r, [100, 50], glow(json!({ "threshold": 0.65, "strength": 8, "knee": 0.5 })), &under);
    assert!(at(&soft, 54, 25)[0] > 0);
}

#[test]
fn more_levels_reach_farther_and_one_level_ignores_the_scatter() {
    let Some(r) = renderer() else { return };
    let one = [(50, [255; 3], 6)];
    let hdr = |levels: u32, scatter: f32| {
        let hdr =
            json!({ "threshold": 0.5, "strength": 4, "knee": 0.5, "levels": levels, "scatter": scatter });
        glow(json!({ "strength": 0, "hdr": hdr }))
    };
    let narrow = dots(r, [100, 50], hdr(1, 0.0), &one);
    let same = dots(r, [100, 50], hdr(1, 2.0), &one);
    assert_eq!(narrow, same);
    let wide = dots(r, [100, 50], hdr(4, 1.0), &one);
    let far = |p: &[u8]| at(p, 78, 25)[0];
    assert!(far(&wide) > far(&narrow) + 2, "{} {}", far(&wide), far(&narrow));
    let none = dots(r, [100, 50], hdr(4, 0.0), &one);
    assert_eq!(none, narrow);
}

#[test]
fn a_radius_in_screen_pixels_keeps_its_width_on_a_canvas_twice_the_screen() {
    let Some(r) = renderer() else { return };
    let small = dots(r, [100, 50], glow(json!({ "radius": 8 })), &[(50, [255; 3], 6)]);
    let big_screen = glow(json!({ "radius": 8, "radius_unit": "screen" }));
    let wide = dots(r, [200, 100], big_screen, &[(100, [255; 3], 12)]);
    let in_canvas = dots(r, [200, 100], glow(json!({ "radius": 8 })), &[(100, [255; 3], 12)]);
    for x in [54, 56, 58, 62] {
        assert!(at(&small, x, 25)[0].abs_diff(at(&wide, x, 25)[0]) <= 4, "{x}");
    }
    let reach = |p: &[u8]| (54..80).map(|x| u32::from(at(p, x, 25)[0])).sum::<u32>();
    assert!(reach(&in_canvas) < reach(&wide));
}
