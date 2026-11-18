use reiwand_scene::{Descriptor, EffectUse, Param};
use serde_json::json;

const WGSL: &str = "override o: u32 = 1u; override z: u32 = 4u / o;
    @fragment fn fs_v(in: VertexOut) -> @location(0) vec4<f32> { return tap(in.uv) * f32(z) * 0.25; }";

fn whole_use(v: serde_json::Value) -> serde_json::Value {
    reiwand_scene::prefill::of("EffectUse", v)
}

fn used(params: serde_json::Value) -> EffectUse {
    serde_json::from_value(whole_use(json!({ "effect": "vv", "params": params }))).unwrap()
}

#[test]
fn values_outside_the_prebuilt_variants_draw_a_prebuilt_one() {
    let json = json!({ "id": "vv", "cost": {}, "options": [{ "id": "o", "values": [1, 0], "prefill": 1 }],
        "passes": [{ "entry": "fs_v" }] });
    let d = Descriptor::packaged("vv", &reiwand_upgrade::descriptors::completed(&json.to_string()), WGSL)
        .unwrap();
    let zero = |_: &str| Some(Param::Number(0.0));
    let mapped = used(json!({ "o": { "prop": "p", "map": { "a": 1 } } }));
    assert_eq!(d.variants(&mapped).unwrap(), [0]);
    assert_eq!(d.variant(&mapped, zero), 0, "an unmapped property value drew a variant never built");
    let both = used(json!({ "o": { "prop": "p", "map": { "a": 1, "b": 0 } } }));
    assert_eq!((d.variants(&both).unwrap(), d.variant(&both, zero)), (vec![0, 1], 1));
    let fixed = used(json!({ "o": 0 }));
    let five = |_: &str| Some(Param::Number(5.0));
    assert_eq!((d.variants(&fixed).unwrap(), d.variant(&fixed, five)), (vec![1], 1));
    let free = used(json!({ "o": { "prop": "p" } }));
    assert_eq!(
        (d.variants(&free).unwrap(), d.variant(&free, zero), d.variant(&free, five)),
        (vec![0, 1], 1, 0)
    );
}

#[test]
fn an_option_that_waits_for_a_texture_accepts_null_and_follows_the_binding() {
    let json = json!({ "id": "vv", "cost": {},
        "params": [{ "id": "paint", "type": "texture", "prefill": "" }],
        "options": [{ "id": "o", "values": [0, 1], "prefill": 0, "when_bound": "paint" }],
        "passes": [{ "entry": "fs_v" }] });
    let d = Descriptor::packaged("vv", &reiwand_upgrade::descriptors::completed(&json.to_string()), WGSL)
        .unwrap();
    let unbound = used(json!({ "o": null, "paint": "" }));
    assert_eq!((d.variants(&unbound).unwrap(), d.variant(&unbound, |_| None)), (vec![0], 0));
    let layered = used(json!({ "o": null, "paint": { "layer": "sky" } }));
    assert_eq!((d.variants(&layered).unwrap(), d.variant(&layered, |_| None)), (vec![1 << 0], 1));
    let named = used(json!({ "o": null, "paint": "a" }));
    assert_eq!(d.variants(&named).unwrap(), vec![1]);
}

#[test]
fn an_option_without_a_binding_refuses_null() {
    let doc = json!({ "schema": reiwand_scene::SCHEMA, "canvas": { "size": [8, 8] },
        "effects": { "vv": { "descriptor": "fx/vv.json", "wgsl": "fx/vv.wgsl" } },
        "layers": [{ "blend_space": "canvas", "id": "l", "type": "solid", "color": "#000000",
            "effects": [{ "effect": "vv", "params": { "o": null } }] }] });
    let mut scene = reiwand_scene::Scene::from_json(&reiwand_upgrade::fill::whole(&doc.to_string())).unwrap();
    let descriptor = json!({ "id": "vv", "cost": {}, "options": [{ "id": "o", "values": [1, 0], "prefill": 1 }],
        "passes": [{ "entry": "fs_v" }] });
    let files = |path: &str, _: usize| -> Result<Vec<u8>, String> {
        Ok(match path.ends_with("json") {
            true => reiwand_upgrade::descriptors::completed(&descriptor.to_string()),
            false => WGSL.to_owned(),
        }
        .into_bytes())
    };
    let fault = scene.adopt(&Default::default(), files).unwrap_err().to_string();
    assert!(fault.contains("/params/o"), "{fault}");
}
