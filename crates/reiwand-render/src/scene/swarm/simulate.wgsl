fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = p - i;
    let u = f * f * (3.0 - 2.0 * f);
    let c = vec2<i32>(i);
    var g: array<f32, 4>;
    for (var k = 0; k < 4; k++) {
        let o = vec2<i32>(k & 1, k >> 1u);
        let a = rand(bitcast<u32>((c.x + o.x) & 255), bitcast<u32>((c.y + o.y) & 255)) * TAU;
        g[k] = cos(a) * (f.x - f32(o.x)) + sin(a) * (f.y - f32(o.y));
    }
    return lerp(lerp(g[0], g[1], u.x), lerp(g[2], g[3], u.x), u.y);
}

fn curl(p: vec2<f32>, shift: f32) -> vec2<f32> {
    let q = vec2<f32>(p.x, p.y + shift);
    let e = 0.01;
    let dy = noise(q + vec2<f32>(0.0, e)) - noise(q - vec2<f32>(0.0, e));
    let dx = noise(q - vec2<f32>(e, 0.0)) - noise(q + vec2<f32>(e, 0.0));
    return vec2<f32>(dy, dx) / (2.0 * e);
}

fn up(a: f32, b: f32, t: f32, linear: bool) -> f32 {
    if b > a {
        let x = clamp((t - a) / (b - a), 0.0, 1.0);
        return select(x * x * (3.0 - 2.0 * x), x, linear);
    }
    return select(0.0, 1.0, t >= a);
}

// Bit k of sim.remap.y: force k's window ramps in a straight line instead of easing.
fn window(k: u32, t: f32) -> f32 {
    let w = sim.forces[k * 4u + 3u];
    let linear = ((sim.remap.y >> k) & 1u) == 1u;
    return up(w.x, w.y, t, linear) * (1.0 - up(w.z, w.w, t, linear));
}

fn reach(r: f32, f: vec4<f32>) -> f32 {
    let kind = u32(f.y);
    var x = vec3<f32>(1.0, r / max(r * r, 1.0), 1.0 / max(r * r, 1.0));
    if f.x > 0.0 {
        let near = max(1.0 - r / f.x, 0.0);
        x = vec3<f32>(1.0, near, near * near);
    }
    return x[min(kind, 2u)];
}

fn wobble(p: Particle, k: u32, a: f32) -> vec4<f32> {
    let f = sim.forces[k * 4u + 1u];
    let g = sim.forces[k * 4u + 2u];
    let key = KEY + 64u + 3u * k;
    let u = rand(p.id, key);
    let freq = lerp(f.x, f.y, u);
    let fade = window(k, clamp(a / p.life, 0.0, 1.0));
    // A shared draw takes the frequency's random number for the scale and the phase too.
    let tied = sim.forces[k * 4u].w > 0.5;
    let ra = select(rand(p.id, key + 1u), u, tied);
    let phase = select(lerp(g.x, g.y, rand(p.id, key + 2u)), lerp(g.x, g.y, u) * freq / TAU, tied);
    let amp = lerp(f.z, f.w, ra) * fade;
    let turn = TAU * (freq * a + phase);
    let peak = lerp(f.z, f.w, select(rand(p.id, KEY + 128u + k), u, tied));
    let floor = lerp(1.0, f.z + (peak - f.z) * (sin(turn) + 1.0) * 0.5, fade);
    return vec4<f32>(amp * sin(turn), amp * sin(turn + TAU / 4.0), amp, floor);
}

fn follow(p: ptr<function, Particle>) {
    let o = 4u + sim.links.w + (*p).host.x * EVENT_WORDS;
    if (*p).host.x >= sim.mode.w || host[o + 10u] != (*p).host.y || host[o + 14u] == 0u {
        (*p).age = (*p).life;
        return;
    }
    let h = event((*p).host.x);
    let near = nearness(h.depth);
    if sim.hatch[1].y < 0.5 {
        (*p).pos += (h.pos - h.prev) * near;
    }
    if sim.depth[2].w > 0.5 {
        (*p).depth.x += h.depth - (*p).spare.y;
        (*p).spare.y = h.depth;
    }
    let bits = select(0u, u32(sim.hatch[0].z), sim.hatch[0].w > 0.5);
    if (bits & 1u) != 0u {
        (*p).color = vec4<f32>(h.color.rgb, (*p).color.a);
    }
    if (bits & 2u) != 0u {
        (*p).size = h.size * sim.hatch[1].x * near;
    }
    if (bits & 4u) != 0u {
        (*p).color.a = h.color.a;
    }
    if (bits & 8u) != 0u {
        (*p).vel = h.vel * near;
        if sim.depth[2].w > 0.5 {
            (*p).depth.y = h.vz;
        }
    }
}
