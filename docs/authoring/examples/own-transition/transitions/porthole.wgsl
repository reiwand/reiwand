fn transition(c: Crossing) -> Crossing {
    var o = c;
    let edge = max(param(0u).x, 0.001);
    let tint = param(1u).rgb;
    let aspect = vec2<f32>(c.resolution.x / max(c.resolution.y, 1.0), 1.0);
    let d = length((c.uv - 0.5) * aspect) / 0.9;
    let r = c.progress * (1.0 + edge);
    let k = smoothstep(r - edge, r, d);
    let ring = (1.0 - abs(k * 2.0 - 1.0)) * (1.0 - c.progress);
    o.color = vec4<f32>(mix(after(c.uv).rgb, before(c.uv).rgb, k) + tint * ring * 0.6, 1.0);
    return o;
}
