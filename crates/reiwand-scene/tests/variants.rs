use reiwand_scene::{Descriptor, EffectUse, Param};
use serde_json::json;

const WGSL: &str = "override o: u32 = 1u; override z: u32 = 4u / o;
    @fragment fn fs_v(in: VertexOut) -> @location(0) vec4<f32> { return tap(in.uv) * f32(z) * 0.25; }";

fn whole_use(mut v: serde_json::Value) -> serde_json::Value {
    let o = v.as_object_mut().unwrap();
    o.entry("name").or_insert(serde_json::Value::Null);
    o.entry("enabled").or_insert(serde_json::json!(true));
    o.entry("mask").or_insert(serde_json::Value::Null);
    o.entry("optional").or_insert(serde_json::json!(false));
    o.entry("params").or_insert(serde_json::json!({}));
    v
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
