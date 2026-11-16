struct Crossing {
    uv: vec2<f32>,
    resolution: vec2<f32>,
    progress: f32,
    quality: f32,
    color: vec4<f32>,
};

struct ReiwandPassage {
    slot: array<vec4<f32>, 24>,
    size: vec2<f32>,
    progress: f32,
    quality: f32,
};

struct ReiwandCrossed {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(0) @binding(0) var<uniform> reiwand_passage: ReiwandPassage;
@group(0) @binding(1) var reiwand_samp: sampler;
@group(0) @binding(2) var reiwand_before: texture_2d<f32>;
@group(0) @binding(3) var reiwand_after: texture_2d<f32>;

const TAU: f32 = 6.2831853;
const LUMA: vec3<f32> = vec3<f32>(0.2126, 0.7152, 0.0722);

fn param(i: u32) -> vec4<f32> {
    return reiwand_passage.slot[min(i, 23u)];
}

fn before(uv: vec2<f32>) -> vec4<f32> {
    return textureSampleLevel(reiwand_before, reiwand_samp, uv, 0.0);
}

fn after(uv: vec2<f32>) -> vec4<f32> {
    return textureSampleLevel(reiwand_after, reiwand_samp, uv, 0.0);
}

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> ReiwandCrossed {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return ReiwandCrossed(vec4<f32>(p.x * 2.0 - 1.0, 1.0 - p.y * 2.0, 0.0, 1.0), p);
}

@fragment
fn reiwand_fs(in: ReiwandCrossed) -> @location(0) vec4<f32> {
    let t = reiwand_passage.progress;
    let mixed = mix(before(in.uv), after(in.uv), t);
    let c = transition(Crossing(in.uv, reiwand_passage.size, t, reiwand_passage.quality, mixed));
    return vec4<f32>(clamp(c.color.rgb, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
