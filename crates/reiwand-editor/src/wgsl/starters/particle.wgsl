fn particle(p: Sprite) -> Sprite {
    var o = p;
    o.size = p.size * (1.0 + param(0u).x * sin(frame.time * 6.0 + p.age));
    return o;
}

fn pixel(p: Pixel) -> vec4<f32> {
    let lit = min(p.sample.rgb * p.color.rgb * p.brightness, vec3<f32>(1.0));
    return vec4<f32>(lit, p.sample.a * p.color.a);
}
