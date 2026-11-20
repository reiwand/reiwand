use serde_json::{Value, json};

fn upgraded(name: &str, scene: Value) -> Value {
    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::create_dir_all(dir.join("a"));
    let file = dir.join("a/scene.json");
    std::fs::write(&file, scene.to_string()).unwrap();
    assert_eq!(reiwand_upgrade::scenes::run(&dir).unwrap(), 1);
    let after: Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
    assert_eq!(reiwand_upgrade::scenes::run(&dir).unwrap(), 0);
    after
}

#[test]
fn vortex_and_around_gain_the_axis_once() {
    let scene = json!({"particles": {"s": {
        "forces": [{"vortex": {"center": [0, 0], "strength": 1}}, {"drag": 1}],
        "emitters": [{"shape": {"around": {"axis": [1, 0, 0], "count": 4}}}]}}});
    let s = &upgraded("scenes-axis", scene)["particles"]["s"];
    assert_eq!(s["forces"][0]["vortex"]["axis"], json!([0, 0, 1]));
    assert_eq!(s["emitters"][0]["shape"]["around"]["axis"], json!([1, 0, 0]));
}

#[test]
fn oscillate_and_stretch_keep_today_behaviour() {
    let scene = json!({"particles": {"s": {
        "forces": [{"oscillate": {"what": "alpha", "freq": [1, 2], "scale": [0.2, 0.5]}}],
        "render": {"stretch": {"time": 0.1, "max": 40}}}}});
    let s = &upgraded("scenes-wave", scene)["particles"]["s"];
    assert_eq!(s["forces"][0]["oscillate"]["from"], "top");
    assert_eq!(
        s["render"]["stretch"],
        json!({"mode": "add", "depth": "flat", "time": 0.1, "min": 0, "max": 40})
    );
}

#[test]
fn a_stretch_with_a_mode_gains_the_flat_depth() {
    let scene = json!({"particles": {"s": {
        "render": {"stretch": {"mode": "relative", "time": 0.1, "min": 0, "max": 4}}}}});
    let s = &upgraded("scenes-depth", scene)["particles"]["s"];
    assert_eq!(s["render"]["stretch"]["depth"], "flat");
    assert_eq!(s["render"]["stretch"]["mode"], "relative");
}

#[test]
fn every_particle_system_gains_explicit_zero_tilt() {
    let scene = json!({"particles": {
        "bare": {"capacity": 10, "emitters": []},
        "kept": {"capacity": 10, "emitters": [], "init": {"life": [1, 2],
            "tilt": {"angle": [[5, 5], [9, 9]], "spin": [[0, 0], [0, 0]]}}},
        "plain": {"capacity": 10, "emitters": [], "init": {"life": [1, 2]}}}});
    let p = &upgraded("scenes-tilt", scene)["particles"];
    let zeros = json!({"angle": [[0.0, 0.0], [0.0, 0.0]], "spin": [[0.0, 0.0], [0.0, 0.0]]});
    assert_eq!(p["bare"]["init"]["tilt"], zeros);
    assert_eq!(p["plain"]["init"]["tilt"], zeros);
    assert_eq!(p["plain"]["init"]["life"], json!([1, 2, 1]));
    assert_eq!(p["kept"]["init"]["tilt"]["angle"], json!([[5, 5], [9, 9]]));
}

#[test]
fn every_particle_render_gains_the_square_aspect() {
    let scene = json!({"particles": {
        "bare": {"capacity": 4, "emitters": []},
        "drawn": {"capacity": 4, "emitters": [], "render": {"texture": "a", "aspect": 0.25}},
        "plain": {"capacity": 4, "emitters": [], "render": {"fps": 8}}}});
    let p = &upgraded("scenes-aspect", scene)["particles"];
    assert_eq!(p["bare"]["render"]["aspect"], 1.0);
    assert_eq!(p["plain"]["render"]["aspect"], 1.0);
    assert_eq!(p["plain"]["render"]["fps"], 8);
    assert_eq!(p["drawn"]["render"]["aspect"], 0.25);
}

#[test]
fn every_layer_and_template_gains_the_canvas_blend_space() {
    let scene = json!({"layers": [
        {"id": "a", "type": "solid"},
        {"id": "g", "type": "group", "blend_space": "srgb", "layers": [{"id": "b", "type": "solid"}]}],
        "templates": {"t": {"type": "group", "layers": [{"id": "c", "type": "solid"}]}}});
    let s = upgraded("scenes-space", scene);
    assert_eq!(s["layers"][0]["blend_space"], "canvas");
    assert_eq!(s["layers"][1]["blend_space"], "srgb");
    assert_eq!(s["layers"][1]["layers"][0]["blend_space"], "canvas");
    assert_eq!(s["templates"]["t"]["blend_space"], "canvas");
    assert_eq!(s["templates"]["t"]["layers"][0]["blend_space"], "canvas");
}

#[test]
fn chroma_key_gains_distance_and_keep() {
    let scene = json!({"layers": [{"blend_space": "canvas", "effects": [
        {"effect": "chroma_key", "params": {"keep": 1}}, {"effect": "chroma_key"}, {"effect": "blur"}]}]});
    let e = &upgraded("scenes-chroma", scene)["layers"][0]["effects"];
    assert_eq!(e[0]["params"]["keep"], json!(1));
    assert_eq!(e[0]["params"]["distance"].as_f64(), Some(0.0));
    assert_eq!(e[1]["params"]["keep"].as_f64(), Some(0.0));
    assert_eq!(e[1]["params"]["key"], json!("#00ff00"));
    assert_eq!(e[2]["params"]["radius"].as_f64(), Some(8.0));
}

#[test]
fn depth_parallax_gains_the_circle_reach() {
    let scene = json!({"layers": [{"blend_space": "canvas", "effects": [
        {"effect": "depth_parallax", "params": {"strength": 9}}, {"effect": "depth_parallax", "params": {"reach": 1}}]}]});
    let e = &upgraded("scenes-reach", scene)["layers"][0]["effects"];
    assert_eq!(e[0]["params"]["reach"].as_f64(), Some(0.0));
    assert_eq!(e[0]["params"]["strength"], json!(9));
    assert_eq!(e[1]["params"]["reach"], json!(1));
}

#[test]
fn the_audio_block_gains_the_gain_release() {
    let s = upgraded("scenes-audio", json!({"audio": {"attack": 0.05, "gain": "auto"}}));
    assert_eq!(s["audio"]["gain_release"], 5);
    let kept = upgraded("scenes-audio-kept", json!({"audio": {"gain_release": 2}, "layers": [{"id": "a"}]}));
    assert_eq!(kept["audio"]["gain_release"], 2);
}

#[test]
fn the_scene_wide_gain_becomes_the_kind_of_each_binding() {
    let layers = json!([{"id": "a", "type": "solid", "opacity": {"audio": "bass"}, "init": {"turbulent": {"audio": 3}}}]);
    let fixed = upgraded("scenes-gain-fixed", json!({"audio": {"gain": "fixed"}, "layers": layers}));
    assert!(fixed["audio"].get("gain").is_none());
    assert_eq!(fixed["layers"][0]["opacity"]["kind"], "absolute");
    assert_eq!(fixed["layers"][0]["opacity"]["unit"], "db");
    let auto = upgraded("scenes-gain-auto", json!({"audio": {"gain": "auto"}, "layers": layers}));
    assert_eq!(auto["layers"][0]["opacity"]["kind"], "relative");
    assert!(auto["audio"].get("gain").is_none());
}

#[test]
fn a_scene_needing_every_step_gets_all_of_them_once() {
    let scene = json!({
        "audio": {"attack": 0.05},
        "layers": [{"id": "a", "type": "particles", "effects": [{"effect": "chroma_key"}]}],
        "particles": {"s": {"capacity": 4,
            "forces": [{"vortex": {"center": [0, 0], "strength": 1}}, {"oscillate": {"what": "alpha"}}],
            "emitters": [{"shape": {"around": {"count": 4}}}],
            "render": {"stretch": {"time": 0.1}}}}});
    let s = upgraded("scenes-all", scene);
    let p = &s["particles"]["s"];
    assert_eq!(s["audio"]["gain_release"], 5);
    assert_eq!(s["layers"][0]["blend_space"], "canvas");
    assert_eq!(s["layers"][0]["effects"][0]["params"]["keep"].as_f64(), Some(0.0));
    assert_eq!(p["forces"][0]["vortex"]["axis"], json!([0, 0, 1]));
    assert_eq!(p["forces"][1]["oscillate"]["from"], "top");
    assert_eq!(p["emitters"][0]["shape"]["around"]["axis"], json!([0, 0, 1]));
    assert_eq!(p["render"]["stretch"]["mode"], "add");
    assert_eq!(p["render"]["stretch"]["depth"], "flat");
    assert_eq!(p["render"]["aspect"], 1.0);
    assert_eq!(p["init"]["tilt"]["angle"], json!([[0.0, 0.0], [0.0, 0.0]]));
}

#[test]
fn every_refraction_gains_its_size_units() {
    let scene = json!({"particles": {
        "rain": {"capacity": 4, "emitters": [], "render": {"refract": {"normal": "drop", "amount": 0.4}}},
        "far": {"capacity": 4, "emitters": [],
            "render": {"refract": {"normal": "drop", "units": "pixels", "shift": 30}}}}});
    let p = &upgraded("scenes-refract-units", scene)["particles"];
    assert_eq!(p["rain"]["render"]["refract"]["units"], "size");
    assert_eq!(p["rain"]["render"]["refract"]["amount"], 0.4);
    assert_eq!(p["far"]["render"]["refract"]["units"], "pixels");
    assert_eq!(p["rain"]["render"]["refract"]["color"], "none");
    assert_eq!(p["far"]["render"]["refract"]["color"], "none");
}

#[test]
fn a_manifest_next_to_the_scene_is_completed() {
    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("scenes-manifest");
    let _ = std::fs::create_dir_all(dir.join("a"));
    let file = dir.join("a/manifest.json");
    std::fs::write(
        &file,
        json!({"schema": "reiwand.wpak/2", "properties": {"speed": {"type": "bool", "default": true}}})
            .to_string(),
    )
    .unwrap();
    assert_eq!(reiwand_upgrade::scenes::run(&dir).unwrap(), 1);
    let after: Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
    assert!(after["properties"]["speed"].get("help").is_none());
    assert_eq!(reiwand_upgrade::scenes::run(&dir).unwrap(), 0);
}

#[test]
fn the_canvas_glow_gains_its_five_capabilities_at_today_values() {
    let glow = json!({"canvas": {"size": [8, 8], "bloom": {"threshold": 0.8, "strength": 1, "radius": 16,
        "tint": "#ffffff", "hdr": {"threshold": 2, "strength": 1}}},
        "layers": [{"effects": [{"effect": "bloom", "params": {"threshold": 0.5, "intensity": 1, "radius": 8, "knee": 0.5, "tint": "#ffffff"}}]}]});
    let after = upgraded("scenes-glow", glow);
    let bloom = &after["canvas"]["bloom"];
    assert_eq!((&bloom["saturation"], &bloom["knee"]), (&json!(1.0), &json!(0.5)));
    assert_eq!(bloom["radius_unit"], "canvas");
    assert_eq!(bloom["hdr"]["levels"], 1);
    let params = &after["layers"][0]["effects"][0]["params"];
    assert_eq!(
        (&params["saturation"], &params["capped"], &params["levels"]),
        (&json!(1.0), &json!(false), &json!(1))
    );
    assert_eq!(params["unit"], 0);
}
