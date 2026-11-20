struct Segment {
    mid: vec2<f32>,
    axis: vec2<f32>,
    long: f32,
    speed: f32,
};

fn hull_corner(v: u32) -> vec2<f32> {
    let lo = look.hull[0];
    let hi = look.hull[1];
    var at = array<vec2<f32>, 8>(
        vec2<f32>(lo.y + hi.w, lo.y),
        vec2<f32>(hi.x, hi.x - hi.w),
        vec2<f32>(lo.z - lo.y, lo.y),
        vec2<f32>(hi.x, hi.z - hi.x),
        vec2<f32>(lo.x, lo.z - lo.x),
        vec2<f32>(hi.z - hi.y, hi.y),
        vec2<f32>(lo.x, lo.x - lo.w),
        vec2<f32>(hi.y + lo.w, hi.y),
    );
    return at[min(v, 7u)];
}

fn segment_end(at: vec3<f32>, h: vec3<f32>) -> vec2<f32> {
    if look.deep.w < 0.5 {
        return at.xy + h.xy;
    }
    let lowest = look.deep.w - look.deep.x + 0.001;
    var e = at + h;
    if e.z < lowest {
        e = at + h * clamp((lowest - at.z) / min(h.z, -1e-9), 0.0, 1.0);
    }
    return look.deep.yz + (e.xy - look.deep.yz) * look.deep.x / (look.deep.x + e.z);
}

fn segment(a: vec4<f32>, d: vec4<f32>, size: f32, k: f32, angle: f32, relative: bool) -> Segment {
    let deep = look.deep.w > 0.5;
    let at = vec3<f32>(a.xy, select(0.0, d.x, deep));
    let v = vec3<f32>(a.zw, select(0.0, d.y, deep));
    let speed = length(v);
    let side = select(1.0, size / max(k, 1e-6), relative);
    let long = side * clamp(look.trail.x * speed, look.shade.z, look.shade.w);
    let own = vec2<f32>(cos(radians(angle)), sin(radians(angle)));
    let dir = select(vec3<f32>(own, 0.0), v / max(speed, 1e-9), speed > 0.0);
    let back = segment_end(at, dir * (-0.5 * long));
    let front = segment_end(at, dir * (0.5 * long));
    let span = front - back;
    let l = length(span);
    let axis = select(own, span / max(l, 1e-9), l > 1e-6);
    return Segment(0.5 * (back + front), axis, select(l, l / max(size, 1e-6), relative), speed);
}
