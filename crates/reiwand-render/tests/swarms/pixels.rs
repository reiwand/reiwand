use super::drawn::{dot, seen_all};
use super::*;

const CLIP: &str = "fn particle(p: Sprite) -> Sprite {\n    return p;\n}\nfn pixel(p: Pixel) -> vec4<f32> {\n    let lit = min(p.sample.rgb * p.color.rgb * p.brightness, vec3<f32>(1.0));\n    return vec4<f32>(lit, p.sample.a * p.color.a);\n}\n";

const FILES: [(&str, &str); 2] = [
    ("effects/clip.json", r#"{ "id": "clip", "stage": "particle", "cost": {} }"#),
    ("effects/clip.wgsl", CLIP),
];

fn bright(color: &str, alpha: f32, glow: f32, shader: bool) -> Value {
    let mut render = json!({ "brightness": glow });
    if shader {
        render["shader"] = json!("clip");
    }
    let init = json!({ "life": [30, 30], "speed": [0, 0], "size": [24, 24], "color": [color, color], "alpha": [alpha, alpha] });
    let mut s = dot([80.0, 40.0], json!({ "render": render }));
    s["init"] = init;
    s
}

fn shot(r: &Renderer, system: Value, add: bool, gpu: bool) -> Option<[u8; 3]> {
    let blend = if add { "add" } else { "normal" };
    let layer =
        json!({ "blend_space": "canvas", "id": "p", "type": "particles", "system": "s", "blend": blend });
    let (p, left) = seen_all(r, (json!({ "s": system }), vec![layer]), (&FILES, "srgb"), gpu)?;
    assert!(left.is_empty(), "{left:?}");
    Some(at(&p, 80, 40))
}

#[test]
fn per_pixel_clipping_keeps_a_translucent_core_at_its_alpha() {
    let Some(r) = renderer() else { return };
    for gpu in [false, true] {
        for glow in [1.0, 1.3, 2.5] {
            for add in [false, true] {
                let Some(clipped) = shot(r, bright("#ffffff", 0.5, glow, true), add, gpu) else { return };
                let plain = shot(r, bright("#ffffff", 0.5, glow, false), add, gpu).unwrap();
                assert!(clipped[0].abs_diff(127) <= 3, "gpu {gpu} {glow} {add}: {clipped:?}");
                if glow > 2.0 {
                    assert!(plain[0] > 250, "gpu {gpu} {add}: without the function {plain:?}");
                }
            }
        }
    }
}

#[test]
fn per_pixel_clipping_lifts_the_other_channels_up_to_the_first() {
    let Some(r) = renderer() else { return };
    for gpu in [false, true] {
        let Some(base) = shot(r, bright("#ff8f66", 0.886, 1.0, true), true, gpu) else { return };
        let high = shot(r, bright("#ff8f66", 0.886, 2.5, true), true, gpu).unwrap();
        assert!(base[0].abs_diff(226) <= 3 && base[1] < base[0] && base[2] < base[1], "gpu {gpu}: {base:?}");
        assert!(high.iter().all(|v| v.abs_diff(226) <= 4), "gpu {gpu}: {high:?}");
    }
}

#[test]
fn two_systems_share_one_per_pixel_effect() {
    let Some(r) = renderer() else { return };
    let mut left = bright("#ffffff", 0.5, 2.5, true);
    let mut right = bright("#ffffff", 0.5, 1.3, true);
    left["emitters"][0]["shape"] = json!({ "point": [40, 40] });
    right["emitters"][0]["shape"] = json!({ "point": [120, 40] });
    let layers =
        ["a", "b"].map(|s| json!({ "blend_space": "canvas", "id": s, "type": "particles", "system": s }));
    for gpu in [false, true] {
        let Some((p, left_out)) =
            seen_all(r, (json!({ "a": left, "b": right }), layers.to_vec()), (&FILES, "srgb"), gpu)
        else {
            return;
        };
        assert!(left_out.is_empty(), "{left_out:?}");
        for x in [40, 120] {
            assert!(at(&p, x, 40)[0].abs_diff(127) <= 3, "gpu {gpu} at {x}: {:?}", at(&p, x, 40));
        }
    }
}
