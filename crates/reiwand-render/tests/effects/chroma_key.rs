use super::*;

const GREY: [u8; 3] = [128; 3];
const WHITE: [u8; 3] = [255; 3];

fn key(key: &str, distance: u8, keep: f32) -> serde_json::Value {
    json!({ "key": key, "tolerance": 0.1, "softness": 0.08, "spill": 0.5, "distance": distance, "keep": keep })
}

#[test]
fn chroma_key_removes_the_key_color() {
    let green = [0, 255, 0];
    let Some(out) = Fx::new("chroma_key", key("#00ff00", 0, 0.0)).input(halves(green, RED)).shot() else {
        return;
    };
    out.check(&[((10, 16), [0; 3]), ((54, 16), RED)]);
    let blue = Fx::new("chroma_key", key("#0000ff", 0, 0.0)).input(halves(green, BLUE));
    let Some(out) = blue.shot() else { return };
    out.check(&[((10, 16), green), ((54, 16), [0; 3])]);
}

#[test]
fn full_color_distance_cuts_white_and_keeps_grey() {
    let Some(out) = Fx::new("chroma_key", key("#ffffff", 1, 0.0)).input(halves(GREY, WHITE)).shot() else {
        return;
    };
    out.check(&[((10, 16), GREY), ((54, 16), [0; 3])]);
}

#[test]
fn chroma_only_distance_cuts_every_neutral_color() {
    let Some(out) = Fx::new("chroma_key", key("#ffffff", 0, 0.0)).input(halves(GREY, WHITE)).shot() else {
        return;
    };
    out.check(&[((10, 16), [0; 3]), ((54, 16), [0; 3])]);
}

#[test]
fn full_color_distance_cuts_black_and_keeps_grey() {
    let Some(out) = Fx::new("chroma_key", key("#000000", 1, 0.0)).input(halves(GREY, [0; 3])).shot() else {
        return;
    };
    out.check(&[((10, 16), GREY)]);
}

#[test]
fn keep_leaves_opacity_on_the_cut_color() {
    let Some(out) = Fx::new("chroma_key", key("#ffffff", 1, 1.0)).input(halves(GREY, WHITE)).shot() else {
        return;
    };
    out.check(&[((10, 16), GREY), ((54, 16), WHITE)]);
}
