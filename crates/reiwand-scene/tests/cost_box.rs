use reiwand_scene::{Quality, SCHEMA, Scene};
use serde_json::{Value, json};

const WGSL: &str = "@fragment
fn fs_push(in: VertexOut) -> @location(0) vec4<f32> {
    return tap(in.uv);
}";

fn png(lit: [u32; 4]) -> Vec<u8> {
    let image = image::RgbaImage::from_fn(100, 100, |x, y| {
        let inside = x >= lit[0] && x < lit[2] && y >= lit[1] && y < lit[3];
        image::Rgba([255, 255, 255, 255].map(|v| if inside { v } else { 0 }))
    });
    let mut bytes = std::io::Cursor::new(Vec::new());
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    bytes.into_inner()
}

fn scene(effect: Value, mask: Option<Value>, lit: [u32; 4]) -> Scene {
    let mut used = json!({ "effect": "push", "params": { "by": 0.2, "area": "box" } });
    if let Some(m) = mask {
        used["mask"] = m;
    }
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [1000, 500], "fit": "fill" },
        "assets": { "a": { "path": "a.png" }, "box": { "path": "box.png" } },
        "effects": { "push": { "descriptor": "push.json", "wgsl": "push.wgsl" } },
        "layers": [{ "id": "a", "type": "image", "image": "a", "size": [1000, 500], "effects": [used] }] });
    let mut s = Scene::from_json(&text.to_string()).unwrap();
    let (json, picture) = (effect.to_string(), png(lit));
    let mut files = move |path: &str, _: usize| match path {
        "push.json" => Ok(json.clone().into_bytes()),
        "push.wgsl" => Ok(WGSL.as_bytes().to_vec()),
        "box.png" => Ok(picture.clone()),
        other => Err(format!("no {other}")),
    };
    s.adopt(&Default::default(), files.clone()).unwrap();
    s.read_inked(&mut files);
    s
}

fn descriptor(bounded: bool) -> Value {
    let mut d = json!({ "id": "push", "mask_mode": "strength", "cost": {},
        "params": [{ "id": "by", "type": "float", "default": 0.0 }, { "id": "area", "type": "texture", "default": "" }],
        "passes": [{ "entry": "fs_push" }] });
    if bounded {
        d["bounded"] = json!(["area"]);
        d["reach"] = json!("by");
    }
    d
}

fn cost(s: &Scene) -> f32 {
    s.cost(Quality::Q2)
}

#[test]
fn a_small_bounded_effect_costs_less_than_the_same_effect_over_the_whole_layer() {
    let open = scene(descriptor(false), None, [0, 0, 100, 100]);
    let small = scene(descriptor(true), None, [10, 10, 20, 20]);
    let full = scene(descriptor(true), None, [0, 0, 100, 100]);
    assert!(cost(&small) < cost(&open), "{} against {}", cost(&small), cost(&open));
    assert_eq!(cost(&full), cost(&open));
}

#[test]
fn a_fixed_mask_in_strength_mode_costs_by_its_lit_box() {
    let mask = json!({ "texture": "box", "channel": "a", "invert": false, "feather": 0.0, "strength": 1.0 });
    let open = scene(descriptor(false), None, [0, 0, 100, 100]);
    let small = scene(descriptor(false), Some(mask.clone()), [10, 10, 20, 20]);
    let full = scene(descriptor(false), Some(mask), [0, 0, 100, 100]);
    assert!(cost(&small) < cost(&open));
    assert_eq!(cost(&full), cost(&open));
}

#[test]
fn the_box_cost_keeps_a_floor_for_the_fixed_cost_of_the_pass() {
    let empty = scene(descriptor(true), None, [0, 0, 0, 0]);
    let mut plain = empty.clone();
    plain.layers[0].effects.clear();
    assert!(cost(&empty) > cost(&plain), "{} against {}", cost(&empty), cost(&plain));
}

#[test]
fn the_box_grows_by_the_reach_of_the_effect() {
    let near = scene(descriptor(true), None, [40, 40, 50, 50]);
    let mut far = scene(descriptor(true), None, [40, 40, 50, 50]);
    far.layers[0].effects[0]
        .params
        .insert("by".into(), reiwand_scene::Value::Fixed(reiwand_scene::Param::Number(200.0)));
    assert!(cost(&far) > cost(&near), "{} against {}", cost(&far), cost(&near));
}
