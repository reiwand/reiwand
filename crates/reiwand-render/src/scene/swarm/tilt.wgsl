fn tilt_unit(seed: u32, key: u32) -> f32 {
    let s = (seed ^ (key * 2654435761u)) * 747796405u + 2891336453u;
    let w = ((s >> ((s >> 28u) + 4u)) ^ s) * 277803737u;
    return f32(((w >> 22u) ^ w) >> 8u) / 16777216.0;
}

fn tilt_fold(a: f32) -> vec2<f32> {
    let c = cos(a);
    return vec2<f32>(abs(c), sin(a) * select(1.0, -1.0, c < 0.0));
}

fn tilt_of(seed: u32, age: f32) -> vec4<f32> {
    let u = vec4<f32>(tilt_unit(seed, 1u), tilt_unit(seed, 2u), tilt_unit(seed, 3u), tilt_unit(seed, 4u));
    let angle = mix(look.tilt[0].xy, look.tilt[0].zw, u.xy) + mix(look.tilt[1].xy, look.tilt[1].zw, u.zw) * age;
    return vec4<f32>(tilt_fold(angle.x), tilt_fold(angle.y));
}
