use super::*;

fn filmed(layers: serde_json::Value) -> Runtime {
    runtime(
        &json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": layers,
            "assets": { "clip": { "path": "clip.webm" } } })
        .to_string(),
    )
}

fn video(id: &str, playback: serde_json::Value) -> serde_json::Value {
    json!({ "blend_space": "canvas", "id": id, "type": "video", "video": "clip", "size": [20, 20], "playback": playback })
}

pub(super) fn shown(rt: &Runtime) -> Vec<String> {
    let frame = rt.frame(VIEW, Quality::Q3);
    let assets = frame.draws.iter().filter_map(|d| match &d.content {
        Content::Video { asset, .. } => Some(asset.to_string()),
        _ => None,
    });
    assets.collect()
}

#[test]
fn a_folder_video_that_does_not_loop_ends_once_its_length_is_known() {
    let mut rt = filmed(json!([video("a", json!({ "loop": false }))]));
    let file = "wpak-user://album/abc".to_string();
    let set = LayerOverride { file: Some(file.clone()), ..Default::default() };
    rt.set_script(ScriptOutputs { layers: [("a".to_string(), set)].into(), ..Default::default() });
    assert!(rt.plays_once("a") && !rt.plays_once("missing"));
    rt.set_length(&file, 1.0);
    rt.set_length("nothing", f32::NAN);
    rt.frame(VIEW, Quality::Q3);
    rt.advance(1.5);
    rt.frame(VIEW, Quality::Q3);
    assert_eq!(rt.take_ended(), ["a"], "the layer tells scripts its video ended");
    let mut courses = Vec::new();
    rt.fill_videos(&mut courses);
    assert!(courses.iter().any(|c| c.id == "a" && c.duration == 1.0 && !c.playing), "{courses:?}");
}

#[test]
fn a_video_a_file_property_picks_keeps_a_clock_of_its_own() {
    let layers = json!([{ "blend_space": "canvas", "id": "a", "type": "video", "video": { "prop": "pick" }, "size": [20, 20] },
        { "blend_space": "canvas", "id": "b", "type": "video", "video": { "prop": "pick" }, "size": [20, 20], "playback": { "own_clock": true, "rate": 2 } }]);
    let doc = json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": layers });
    let props: BTreeMap<String, Property> =
        reiwand_scene::prefill::properties(json!({ "pick": { "type": "file", "accept": ["video"] } }));
    let scene = Scene::load(&reiwand_upgrade::fill::whole(&doc.to_string()), &props).unwrap();
    assert!(scene.validate(&props).is_empty(), "{:?}", scene.validate(&props));
    let mut rt = Runtime::new(Arc::new(scene), effective(&props, &[]));
    assert_eq!(rt.scene.lanes(), [("pick@b".to_string(), "pick".to_string())]);
    rt.set_props(serde_json::from_value(json!({ "pick": "/videos/sea.webm" })).unwrap());
    assert_eq!(shown(&rt), ["/videos/sea.webm", "pick@b"], "the second layer plays its own clock");
    rt.frame(VIEW, Quality::Q3);
    rt.advance(0.5);
    let at = |rt: &Runtime, key: &str| rt.tapes().iter().find(|t| t.asset == key).map(|t| t.at);
    assert!(at(&rt, "pick@b").is_some_and(|t| (t - 1.0).abs() < 0.05), "{:?}", rt.tapes());
}

#[test]
fn own_clocks_need_a_file_property_that_may_hold_a_video() {
    let layer = json!({ "blend_space": "canvas", "id": "b", "type": "video", "video": { "prop": "pick" }, "playback": { "own_clock": true } });
    let doc = json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] },
        "assets": { "clip": { "path": "clip.webm" }, "sky": { "path": "sky.png" } }, "layers": [layer] });
    let said = |pick: serde_json::Value| {
        let props: BTreeMap<String, Property> = reiwand_scene::prefill::properties(json!({ "pick": pick }));
        let scene = Scene::from_json(&reiwand_upgrade::fill::whole(&doc.to_string())).unwrap();
        scene
            .validate(&props)
            .iter()
            .map(ToString::to_string)
            .any(|i| i.contains("/layers/0/playback/own_clock"))
    };
    assert!(said(json!({ "type": "file" })), "an image property picks no video");
    assert!(said(json!({ "type": "file", "accept": ["image"], "default_asset": "sky" })));
    assert!(!said(json!({ "type": "file", "accept": ["image", "video"] })));
    assert!(!said(json!({ "type": "file", "accept": ["image"], "default_asset": "clip" })));
}
