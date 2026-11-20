use super::kit::{mesh, near, render, renderer, rgb, scene};
use super::parts::colored;
use reiwand_scene::Quality;
use serde_json::json;

fn shot(size: u32) -> Vec<u8> {
    let mut layer = mesh(json!({ "size": [size, size] }));
    layer.as_object_mut().unwrap().remove("texture");
    let glb = colored(0.0, [[1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0]]);
    render(renderer().unwrap(), scene(layer, glb), 0.0, Quality::Q3)
}

#[test]
fn parts_beyond_the_box_of_a_mesh_are_not_cut_at_its_edge() {
    if renderer().is_none() {
        return;
    }
    let wide = shot(32);
    assert!(near(rgb(&wide, 10, 32), [255, 0, 0]), "{:?}", rgb(&wide, 10, 32));
    assert!(near(rgb(&wide, 54, 32), [0, 255, 0]), "{:?}", rgb(&wide, 54, 32));
    assert!(near(rgb(&wide, 3, 32), [0, 0, 0]));
    let fitted = shot(64);
    assert!(near(rgb(&fitted, 10, 32), [255, 0, 0]));
}

#[test]
fn parts_beyond_the_box_of_a_mesh_keep_their_size() {
    if renderer().is_none() {
        return;
    }
    for (box_side, x) in [(32, 25), (32, 26), (48, 25), (48, 26)] {
        let px = shot(box_side);
        assert!(near(rgb(&px, x, 32), [0, 255, 0]), "{box_side} {x}: {:?}", rgb(&px, x, 32));
        assert!(near(rgb(&px, 22, 32), [255, 0, 0]), "{box_side}: {:?}", rgb(&px, 22, 32));
    }
}
