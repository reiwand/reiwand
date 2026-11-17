use reiwand_scene::{Blend, Kind, Orient, Render, Scene, Spread, Stack, System};

fn flag(b: bool) -> f32 {
    f32::from(u8::from(b))
}

pub(super) fn strand(r: &Render) -> [f32; 4] {
    let Some(t) = r.trail.as_ref().filter(|t| t.long()) else { return [0.0; 4] };
    [t.subdivide.min(7) as f32, t.uv_scale, t.uv_scroll, flag(t.textured)]
}

pub(super) fn cord(r: &Render) -> [f32; 4] {
    r.rope.map_or([0.0; 4], |c| [c.width, c.uv_scale, c.uv_scroll, flag(r.texture.is_some())])
}

pub(super) fn turn(r: &Render) -> [f32; 4] {
    let mode = match r.orient {
        Some(Orient::Velocity) => 1.0,
        Some(Orient::Fixed) => 2.0,
        _ => 0.0,
    };
    [mode, r.angle.unwrap_or(0.0).to_radians(), 0.0, 0.0]
}

pub(super) fn far(scene: &Scene, id: &str, s: Option<&System>) -> Option<(Stack, f32)> {
    let s = s.filter(|s| s.perspective.is_some())?;
    let drawn = |l: &reiwand_scene::Layer| matches!(&l.kind, Kind::Particles { system, .. } if system == id);
    scene.layers().any(|l| drawn(l) && l.blend == Blend::Normal).then_some(())?;
    let reach = |d: &Option<Spread<f32>>| d.as_ref().map_or(0.0, |d| d.ends[0].abs().max(d.ends[1].abs()));
    let life = s.init.life.ends[0].max(s.init.life.ends[1]);
    Some((Stack::Farthest, (reach(&s.init.depth) + reach(&s.init.depth_speed) * life).max(1.0)))
}

pub(super) fn tilt(s: Option<&System>, r: &Render) -> [[f32; 4]; 3] {
    let [w, h] = r.sides();
    let turned = s.map(|s| s.init.tilt).filter(|t| t.turned());
    let Some(t) = turned else { return [[0.0; 4], [0.0; 4], [0.0, w, h, 0.0]] };
    let row = |[lo, hi]: [[f32; 2]; 2]| [lo[0], lo[1], hi[0], hi[1]].map(f32::to_radians);
    [row(t.angle), row(t.spin), [1.0, w, h, 0.0]]
}
