@group(2) @binding(0) var<uniform> lights: Lights;
@group(2) @binding(1) var normal_map: texture_2d<f32>;
@group(2) @binding(2) var bend_map: texture_2d<f32>;
@group(2) @binding(3) var cast_a: texture_2d<f32>;
@group(2) @binding(4) var cast_b: texture_2d<f32>;
@group(2) @binding(5) var cast_c: texture_2d<f32>;
@group(2) @binding(6) var cast_d: texture_2d<f32>;

fn turned(t: vec2<f32>, turn: vec4<f32>) -> vec2<f32> {
    return t.x * turn.xy - t.y * vec2<f32>(-turn.y, turn.x);
}

fn tilted_normal(t: vec3<f32>, a: vec4<f32>) -> vec3<f32> {
    let ex = vec3<f32>(a.z, 0.0, -a.w);
    let ey = vec3<f32>(0.0, a.x, a.y);
    return normalize(t.x * ex + t.y * ey + t.z * cross(ex, ey));
}

fn lamp_lit(c: vec4<f32>, n: vec3<f32>, p: vec2<f32>) -> vec4<f32> {
    if c.a <= 1e-5 {
        return c;
    }
    let albedo = c.rgb / c.a;
    var light = mix(lights.ambient.rgb, lights.sky.rgb, 0.5 - 0.5 * n.y);
    for (var i = 0u; i < min(u32(lights.head.x), 8u); i++) {
        let b = lights.bulbs[i];
        let r = reach(b, p);
        if r.w <= 0.0 {
            continue;
        }
        let facing = vec3<f32>(n.xy, select(n.z, -n.z, r.z < 0.0));
        let tone = b.color.rgb * cast_from(cast_at(b, r.xyz), cast_a, cast_b, cast_c, cast_d);
        light += tone * max(dot(facing, r.xyz), 0.0) * r.w;
    }
    let rgb = albedo * light + albedo * look.place.w;
    return vec4<f32>(mix(albedo, rgb, look.place.z) * c.a, c.a);
}

@fragment
fn fs_lit_particle(in: Corner) -> @location(0) vec4<f32> {
    var t = vec3<f32>(0.0, 0.0, 1.0);
    if look.light.y > 0.5 {
        let m = textureSampleLevel(normal_map, samp, in.uv * look.crops.xy, 0.0).xyz * 2.0 - 1.0;
        t = vec3<f32>(m.xy, max(m.z, 0.0) + 1e-3);
    }
    if look.tilt[2].x > 0.5 {
        t = tilted_normal(t, in.tilt);
    }
    let n = normalize(vec3<f32>(turned(t.xy, in.turn), t.z));
    return lamp_lit(painted(in), n, in.position.xy + look.place.xy);
}

@fragment
fn fs_bend_particle(in: Corner) -> @location(0) vec4<f32> {
    let a = painted(in).a;
    let t = textureSampleLevel(bend_map, samp, in.uv * look.crops.zw, 0.0).xy * 2.0 - 1.0;
    let shift = turned(t, in.turn) * look.bend.x * select(0.5 * in.turn.z, 1.0, look.bend.y > 0.5);
    return vec4<f32>(shift * a, 0.0, a);
}
