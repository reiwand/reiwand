use crate::gpu::renderer;
use image::{Rgba, RgbaImage};
use reiwand_render::{Films, Rehearsal, SceneAssets, Trace};
use serde_json::{Value, json};
use std::sync::Arc;

fn scene(layers: Value, timelines: Value) -> reiwand_scene::Scene {
    let doc = json!({ "schema": reiwand_scene::SCHEMA, "canvas": { "size": [160, 90], "background": "#000000" },
        "assets": { "film": { "path": "assets/film.webm" } }, "layers": layers, "timelines": timelines });
    reiwand_scene::Scene::load(&crate::gpu::whole(&doc.to_string()), &Default::default()).unwrap()
}

/// A video one second long: red in its first half, blue in its second.
#[derive(Debug)]
struct Halves;

impl Films for Halves {
    fn length(&self, path: &str) -> Option<f32> {
        (path == "assets/film.webm").then_some(1.0)
    }

    fn frame(&self, _: &str, at: f64) -> Result<RgbaImage, String> {
        let color = if at < 0.5 { Rgba([255, 0, 0, 255]) } else { Rgba([0, 0, 255, 255]) };
        Ok(RgbaImage::from_pixel(16, 9, color))
    }
}

#[test]
fn renders_draw_the_frame_of_a_video_where_its_clock_is_and_name_the_layer_they_cannot_decode() {
    let Some(r) = renderer() else { return };
    let film =
        json!({ "blend_space": "canvas", "id": "film", "type": "video", "video": "film", "size": [160, 90] });
    let scene = scene(json!([film]), json!([]));
    let shot = |time: f64, films: Option<Arc<dyn Films>>| {
        let assets = SceneAssets { films, ..Default::default() };
        let wanted = Rehearsal { edge: 160, time, ..Rehearsal::default() };
        r.rehearse(&scene, &assets, &Default::default(), &wanted).unwrap()
    };
    let middle = |reel: &reiwand_render::Reel| reel.frames[0].get_pixel(80, 45).0;
    assert_eq!(middle(&shot(0.2, Some(Arc::new(Halves)))), [255, 0, 0, 255]);
    assert_eq!(middle(&shot(0.7, Some(Arc::new(Halves)))), [0, 0, 255, 255]);
    assert_eq!(middle(&shot(1.2, Some(Arc::new(Halves)))), [255, 0, 0, 255], "the video loops at its end");
    let blind = shot(0.2, None);
    assert_eq!(middle(&blind), [0, 0, 0, 255]);
    assert!(blind.left_out.iter().any(|l| l.contains("video layer film")), "{:?}", blind.left_out);
}

#[test]
fn a_made_up_click_on_a_layer_fires_its_click_trigger() {
    let Some(r) = renderer() else { return };
    let button = json!({ "blend_space": "canvas", "id": "button", "type": "solid", "color": "#2040c0", "size": [40, 20], "solid": true, "transform": { "position": [80, 45] } });
    let lamp = json!({ "blend_space": "canvas", "id": "lamp", "type": "solid", "color": "#ffffff", "size": [20, 20],
        "opacity": { "timeline": "light", "keys": [[0, 0], [0.1, 1]] }, "transform": { "position": [140, 20] } });
    let timelines = json!([{ "id": "light", "duration": 0.1, "autoplay": false, "trigger": "click:button" }]);
    let scene = scene(json!([button, lamp]), timelines);
    let lamp_after = |at: [f32; 2], click: bool| {
        let pointer = Some(Trace { from: at, to: at, click });
        let wanted = Rehearsal { edge: 160, frames: 12, fps: 30, pointer, ..Rehearsal::default() };
        let reel = r.rehearse(&scene, &SceneAssets::default(), &Default::default(), &wanted).unwrap();
        reel.frames.last().unwrap().get_pixel(140, 20).0[0]
    };
    assert_eq!(lamp_after([0.5, 0.5], true), 255, "a click on the button");
    assert_eq!(lamp_after([0.5, 0.5], false), 0, "a pointer that only rests on the button");
    assert_eq!(lamp_after([0.1, 0.1], true), 0, "a click beside the button");
}

#[test]
fn the_glow_of_a_canvas_that_is_not_cleared_does_not_pile_up_in_what_it_keeps() {
    let Some(r) = renderer() else { return };
    let bloom = json!({ "threshold": 0.5, "strength": 2, "radius": 16, "radius_unit": "canvas", "tint": "#ffffff",
        "knee": 0.5, "saturation": 1, "limit": null });
    let path = json!({ "keys": [[0, [40, 30]], [2, [120, 30]]], "loop": "pingpong" });
    let dot = json!({ "blend_space": "canvas", "id": "dot", "type": "solid", "color": "#ffffff", "size": [6, 6],
        "transform": { "position": path } });
    let doc = json!({ "schema": reiwand_scene::SCHEMA,
        "canvas": { "size": [160, 90], "background": "#000000", "clear": false, "bloom": bloom }, "layers": [dot] });
    let scene =
        reiwand_scene::Scene::load(&crate::gpu::whole(&doc.to_string()), &Default::default()).unwrap();
    let wanted = Rehearsal { edge: 160, frames: 90, fps: 30, ..Rehearsal::default() };
    let reel = r.rehearse(&scene, &SceneAssets::default(), &Default::default(), &wanted).unwrap();
    let last = reel.frames.last().unwrap();
    assert!(last.get_pixel(80, 30).0[0] > 200, "the trail of the dot is kept");
    for (x, y) in [(80, 75), (10, 80), (150, 80)] {
        assert!(last.get_pixel(x, y).0[0] < 24, "the glow piled up at {x},{y}: {:?}", last.get_pixel(x, y));
    }
}

#[test]
fn a_trail_on_a_group_keeps_fading_copies_of_its_moving_children() {
    let Some(r) = renderer() else { return };
    let path = json!({ "keys": [[0, [20, 45]], [1, [140, 45]]], "loop": "once" });
    let dot = json!({ "blend_space": "canvas", "id": "dot", "type": "solid", "color": "#ffffff", "size": [8, 8],
        "transform": { "position": path } });
    let trail = json!([{ "effect": "trail", "params": { "persistence": 0.9 } }]);
    let shot = |layer: serde_json::Value| {
        let doc = json!({ "schema": reiwand_scene::SCHEMA,
            "canvas": { "size": [160, 90], "background": "#000000" }, "layers": [layer] });
        let scene =
            reiwand_scene::Scene::load(&crate::gpu::whole(&doc.to_string()), &Default::default()).unwrap();
        let wanted = Rehearsal { edge: 160, frames: 20, fps: 30, ..Rehearsal::default() };
        let reel = r.rehearse(&scene, &SceneAssets::default(), &Default::default(), &wanted).unwrap();
        reel.frames.last().unwrap().clone()
    };
    let mut alone = dot.clone();
    alone["effects"] = trail.clone();
    let group =
        json!({ "blend_space": "canvas", "id": "g", "type": "group", "layers": [dot], "effects": trail });
    let (alone, grouped) = (shot(alone), shot(group));
    let behind = |i: &RgbaImage| i.get_pixel(60, 45).0[0];
    assert_eq!(behind(&alone), 0, "a layer that moves as a whole keeps its own picture");
    assert!(behind(&grouped) > 20, "the trail of the group: {}", behind(&grouped));
}

#[test]
fn a_trail_over_opaque_content_keeps_the_newest_picture_whole() {
    let Some(r) = renderer() else { return };
    let path = json!({ "keys": [[0, [20, 45]], [1, [140, 45]]], "loop": "once" });
    let ground = json!({ "blend_space": "canvas", "id": "ground", "type": "solid", "color": "#202020", "size": [160, 90],
        "transform": { "position": [80, 45] } });
    let dot = json!({ "blend_space": "canvas", "id": "dot", "type": "solid", "color": "#ffffff", "size": [8, 8],
        "transform": { "position": path } });
    let group = json!({ "blend_space": "canvas", "id": "g", "type": "group", "layers": [ground, dot],
        "effects": [{ "effect": "trail", "params": { "persistence": 0.9 } }] });
    let doc = json!({ "schema": reiwand_scene::SCHEMA, "canvas": { "size": [160, 90], "background": "#000000" },
        "layers": [group] });
    let scene =
        reiwand_scene::Scene::load(&crate::gpu::whole(&doc.to_string()), &Default::default()).unwrap();
    let wanted = Rehearsal { edge: 160, frames: 20, fps: 30, ..Rehearsal::default() };
    let reel = r.rehearse(&scene, &SceneAssets::default(), &Default::default(), &wanted).unwrap();
    let last = reel.frames.last().unwrap();
    let brightest = (0..160).map(|x| last.get_pixel(x, 45).0[0]).max().unwrap();
    assert!(brightest > 240, "the dot where it is now is dimmed: {brightest}");
    assert!(last.get_pixel(60, 45).0[0] > 0x30, "no trail behind the dot over the ground");
    assert!(last.get_pixel(80, 10).0[0] <= 0x22, "the ground away from the dot changed");
}

#[test]
fn the_mask_of_an_adjustment_layer_limits_where_its_effects_change_the_picture() {
    let Some(r) = renderer() else { return };
    let red = json!({ "blend_space": "canvas", "id": "red", "type": "solid", "color": "#ff0000", "size": [160, 90] });
    let shape = json!({ "shape": { "rect": { "size": [80, 90], "radius": 0 } } });
    let tint = json!({ "blend_space": "canvas", "id": "tint", "type": "adjustment", "mask": shape,
        "effects": [{ "effect": "color_adjust", "params": { "saturation": 0 } }] });
    let doc = json!({ "schema": reiwand_scene::SCHEMA, "canvas": { "size": [160, 90], "background": "#000000" },
        "layers": [red, tint] });
    let scene =
        reiwand_scene::Scene::load(&crate::gpu::whole(&doc.to_string()), &Default::default()).unwrap();
    let wanted = Rehearsal { edge: 160, ..Rehearsal::default() };
    let shot =
        r.rehearse(&scene, &SceneAssets::default(), &Default::default(), &wanted).unwrap().frames.remove(0);
    let [inside, outside] = [shot.get_pixel(80, 45).0, shot.get_pixel(20, 45).0];
    assert!(inside[0].abs_diff(inside[1]) < 8, "grey inside the mask: {inside:?}");
    assert_eq!(outside, [255, 0, 0, 255], "red outside the mask");
}
