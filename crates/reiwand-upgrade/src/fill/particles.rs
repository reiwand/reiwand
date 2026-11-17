use serde_json::{Map, Value, json};

pub(super) fn value(def: &str, ctx: &str, name: &str) -> Option<Value> {
    Some(match (def, ctx, name) {
        ("Child", _, "angle") | ("ControlPoint", _, "angle") => json!(0),
        ("Child", _, "offset") | ("Area", _, "inner") => json!([0, 0]),
        ("Emitter", _, "delay") | ("Burst", _, "at") | ("Hsv", _, "steps") => json!(0),
        ("Trail", _, "subdivide") | ("Trail", _, "uv_scroll") => json!(0),
        ("Trail", _, "uv_scale") => json!(1),
        ("Oscillate", _, "mask") => json!([1, 1]),
        (_, "circle", "scale") => json!([1, 1]),
        (_, "circle", "sign") => json!([0, 0]),
        (_, "sphere", "sign") => json!([0, 0, 0]),
        (_, "turbulence", "speed") => json!(0),
        (_, "turbulence", "time_scale") => json!(1),
        (_, "turbulence", "mask") => json!([1, 1]),
        (_, "attractor", "radius") | (_, "vortex", "radius") | (_, "orbit", "radius") => json!(0),
        (_, "attractor", "falloff") => json!("inverse_square"),
        (_, "vortex", "falloff") => json!("linear"),
        (_, "orbit", "falloff") => json!("constant"),
        (_, "vortex", "inner") => json!(0),
        _ => return None,
    })
}

pub(super) fn before(ctx: &str, map: &mut Map<String, Value>) {
    if ctx == "forces" && !map.contains_key("window") {
        map.insert("window".into(), json!([0, 0, 1, 1]));
    }
    if ctx == "vortex"
        && !map.contains_key("outer")
        && let Some(inner) = map.get("inner").filter(|v| !v.is_null()).cloned()
    {
        map.insert("outer".into(), inner);
    }
}

pub(super) fn spread(def: &str, v: &mut Value) {
    if def.starts_with("Spread_for_")
        && let Value::Array(list) = v
        && list.len() == 2
    {
        list.push(json!(1));
    }
}
