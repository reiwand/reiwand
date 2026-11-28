use super::*;

// Lifts the top edge of the layer by a quarter of its height, in layer coordinates, so it passes
// above the layer's rectangle; the bottom edge stays.
const LIFT: &str = "@vertex fn vs_lift(@builtin(vertex_index) i: u32) -> VertexOut {
    let uv = grid_uv(i);
    let moved = vec2<f32>(uv.x, uv.y - 0.25 * (1.0 - uv.y));
    return VertexOut(grid_position(from_layer(moved)), from_layer(uv));
}
@fragment fn fs_lift(in: VertexOut) -> @location(0) vec4<f32> { return vec4<f32>(0.0, 0.0, 1.0, 1.0); }";

fn lifted(margin: Option<[f32; 4]>) -> Option<Shot> {
    let json = json!({ "id": "lift", "cost": {}, "passes": [{ "entry": "fs_lift", "vertex": "vs_lift", "grid": [1, 1] }] });
    let mut src = json!({ "size": [32, 16] });
    if let Some(m) = margin {
        src["margin"] = json!(m);
    }
    Fx::new("lift", json!({})).input(solid(RED)).own(json, LIFT).src(src).shot()
}

#[test]
fn a_margin_gives_the_effects_room_beyond_the_layer_and_keeps_the_layer_in_place() {
    // The 32×16 layer sits in the middle of the 64×32 canvas: rows 8 to 23, columns 16 to 47.
    let Some(cut) = lifted(None) else { return };
    let roomy = lifted(Some([0.0, 8.0, 0.0, 0.0])).unwrap();
    let blue = |s: &Shot, x: u32, y: u32| s.at(x, y)[2] > 200;
    assert!(!blue(&cut, 32, 6), "without a margin the lifted top is cut at the layer edge: {:?}", cut.at(32, 6));
    assert!(blue(&roomy, 32, 6), "with a top margin of 8 the lifted top is drawn above it: {:?}", roomy.at(32, 6));
    for (x, y) in [(32, 12), (32, 22), (20, 20)] {
        assert!(blue(&cut, x, y) && blue(&roomy, x, y), "the layer itself stays where it was at {x},{y}");
    }
    assert!(!blue(&roomy, 32, 26) && !blue(&roomy, 8, 12), "the bottom edge and the sides do not move");
}
