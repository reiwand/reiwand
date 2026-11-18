use serde_json::{Map, Value, json};

pub(super) fn value(def: &str, name: &str) -> Option<Value> {
    Some(match (def, name) {
        ("Layer", "blend_space") => json!("canvas"),
        ("Oscillate", "from") => json!("top"),
        ("Stretch", "mode") => json!("add"),
        ("Stretch", "depth") => json!("flat"),
        ("Stretch", "time") => json!(0),
        ("Stretch", "min") => json!(0),
        ("Stretch", "max") => json!(10000),
        ("Hearing", "gain_release") => json!(5),
        ("vortex" | "around", "axis") => json!([0, 0, 1]),
        _ => return None,
    })
}

pub(super) fn corrected(def: &str, name: &str) -> Option<Value> {
    Some(match (def, name) {
        ("Layer", "parallax") => json!([0, 0]),
        ("Glide", "offset") => json!([0, 0]),
        ("BoneCollider", "offset") => json!([0, 0, 0]),
        ("System", "space") => json!("local"),
        ("Manifest", "description") => json!(""),
        ("Canvas", "clear") => json!(true),
        ("Clock", "step") => json!(60),
        ("Asset", "filter") => json!("linear"),
        ("Asset", "wrap") => json!("clamp"),
        ("Layer", "solid") => json!(false),
        ("Property", "mode") => json!("one"),
        ("Property", "max_files") => json!(1000),
        _ => return None,
    })
}

pub(super) fn follows(def: &str, name: &str, map: &Map<String, Value>, root: &Value) -> Option<Value> {
    let package = root["title"] == "Manifest";
    let kind = map.get("type").and_then(Value::as_str).unwrap_or("");
    Some(match (def, name) {
        ("Transform", "origin") => json!("anchor"),
        ("Camera", "center") => json!("canvas"),
        ("Playback", "end")
            if !package && !["speed", "volume", "max_fps", "muted"].iter().any(|k| map.contains_key(*k)) =>
        {
            json!("end")
        }
        ("Layer", "size") if matches!(kind, "image" | "video") => json!("source"),
        ("Layer", "size") if matches!(kind, "solid" | "adjustment" | "mesh" | "model") => json!("canvas"),
        ("Layer", "color")
            if matches!(kind, "group" | "image" | "video" | "particles" | "text" | "spectrum" | "mesh") =>
        {
            json!("#ffffff")
        }
        ("Emitter", "delay") | ("Burst", "at") | ("Child", "angle") => json!(0),
        ("Trail", "subdivide") | ("Trail", "uv_scroll") | ("Hsv", "steps") => json!(0),
        ("Trail", "uv_scale") => json!(1),
        ("Mask", "channel") => {
            json!(if map.get("texture").is_some_and(|t| !t.is_null()) { "luminance" } else { "a" })
        }
        ("Mask", "strength") => json!(1),
        ("Shake", "enabled") => json!(true),
        ("Shake", "roughness") => json!(0.5),
        ("Layer", "space") if kind == "group" => json!("2d"),
        ("Layer", "cone") if kind == "light" => json!([30, 60]),
        ("Layer", "length") if kind == "light" => json!(256),
        ("Layer", "shadow_bias") if kind == "light" => json!(0.2),
        ("Layer", "padding") if kind == "text" => json!(0),
        ("Layer", "valign") if kind == "text" => json!("center"),
        ("Layer", "mode") if kind == "sound" => json!("sequence"),
        ("Layer", "render") if kind == "text" => json!("auto"),
        ("Material", "shader") => json!("pbr"),
        ("Scene", "camera3d") if solid(map.get("layers")) => camera(),
        (d, "of") if d.starts_with("Binding_for_") && indexed(map) => json!(64),
        (d, "loop") if d.starts_with("Binding_for_") && keyed(map) => json!("once"),
        ("Layer", "playback") if kind == "video" => json!({
            "rate": 1, "playing": true, "loop": true, "start": 0, "end": "end", "frame": null, "own_clock": false
        }),
        _ => return None,
    })
}

pub(super) fn contextual(def: &str, name: &str) -> bool {
    matches!((def, name), ("Layer", "blend_space")) || heard_value(def, name).is_some()
}

pub(super) fn heard_value(def: &str, name: &str) -> Option<Value> {
    if !(def == "Remap" || def == "Turbulent" || def.starts_with("Binding_for_")) {
        return None;
    }
    match name {
        "kind" => Some(json!("relative")),
        "unit" => Some(json!("db")),
        _ => None,
    }
}

pub(super) fn heard(def: &str, map: &Map<String, Value>) -> Vec<(&'static str, Value)> {
    let listens = match def {
        "Remap" => map.get("input").and_then(Value::as_str).is_some_and(|i| i.starts_with("audio:")),
        "Turbulent" => map.get("audio").is_some_and(|a| !a.is_null()),
        _ => {
            def.starts_with("Binding_for_")
                && map.get("audio").is_some_and(|a| !a.is_object() && !a.is_null())
        }
    };
    if !listens {
        return Vec::new();
    }
    ["kind", "unit"]
        .into_iter()
        .filter(|name| map.get(*name).is_none_or(Value::is_null))
        .filter_map(|name| Some((name, heard_value(def, name)?)))
        .collect()
}

pub(super) fn tag(ctx: &str, map: &Map<String, Value>) -> Option<(&'static str, Value)> {
    (ctx == "refract" && map.contains_key("normal") && !map.contains_key("units"))
        .then(|| ("units", json!("size")))
}

fn solid(layers: Option<&Value>) -> bool {
    layers.and_then(Value::as_array).is_some_and(|list| {
        list.iter().any(|l| {
            matches!(l.get("type").and_then(Value::as_str), Some("mesh" | "model"))
                || (l.get("type").and_then(Value::as_str) == Some("group")
                    && l.get("space").and_then(Value::as_str) == Some("3d"))
                || solid(l.get("layers"))
        })
    })
}

fn camera() -> Value {
    json!({
        "fov": 60, "near": 0.1, "far": 1000, "eye": [0, 0, 5], "center": [0, 0, 0],
        "up": [0, 1, 0], "paths": [], "order": "sequential", "fade": null
    })
}

fn indexed(map: &Map<String, Value>) -> bool {
    map.get("audio").is_some_and(|a| a.is_number() || a.is_array())
}

fn keyed(map: &Map<String, Value>) -> bool {
    map.get("keys").is_some_and(|k| !k.is_null()) && map.get("timeline").is_none_or(Value::is_null)
}

pub(super) fn looks(def: &str, map: &mut Map<String, Value>, report: &mut super::Report) {
    if def != "Material" {
        return;
    }
    let wanted: &[(&str, Value)] = match map.get("shader").and_then(Value::as_str) {
        Some("fur") => &[
            ("layers", json!(12)),
            ("length", json!(10)),
            ("density", json!(0.5)),
            ("comb_x", json!(0)),
            ("comb_y", json!(0)),
            ("comb_z", json!(0)),
        ],
        Some("foliage") => &[("strength", json!(1)), ("speed", json!(1))],
        Some("chroma") => &[("key", json!("#00ff00")), ("threshold", json!(0.4)), ("softness", json!(0.1))],
        _ => return,
    };
    let params = map.entry("params").or_insert_with(|| Value::Object(Map::new()));
    let Some(params) = params.as_object_mut() else { return };
    for (id, value) in wanted {
        if !params.contains_key(*id) {
            params.insert((*id).to_owned(), value.clone());
            report.note("Material.params", id);
        }
    }
}
