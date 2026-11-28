// 3D simplex noise of the `noise_field` force: the same numbers as `simplex.rs` of the scene crate
// (one octave, within -1..1; the last corner takes its gradient from perm[i+1+perm[j+1+perm[k]]]).
var<private> PERM: array<u32, 256> = array<u32, 256>(
    151u, 160u, 137u, 91u, 90u, 15u, 131u, 13u, 201u, 95u, 96u, 53u, 194u, 233u, 7u, 225u,
    140u, 36u, 103u, 30u, 69u, 142u, 8u, 99u, 37u, 240u, 21u, 10u, 23u, 190u, 6u, 148u,
    247u, 120u, 234u, 75u, 0u, 26u, 197u, 62u, 94u, 252u, 219u, 203u, 117u, 35u, 11u, 32u,
    57u, 177u, 33u, 88u, 237u, 149u, 56u, 87u, 174u, 20u, 125u, 136u, 171u, 168u, 68u, 175u,
    74u, 165u, 71u, 134u, 139u, 48u, 27u, 166u, 77u, 146u, 158u, 231u, 83u, 111u, 229u, 122u,
    60u, 211u, 133u, 230u, 220u, 105u, 92u, 41u, 55u, 46u, 245u, 40u, 244u, 102u, 143u, 54u,
    65u, 25u, 63u, 161u, 1u, 216u, 80u, 73u, 209u, 76u, 132u, 187u, 208u, 89u, 18u, 169u,
    200u, 196u, 135u, 130u, 116u, 188u, 159u, 86u, 164u, 100u, 109u, 198u, 173u, 186u, 3u, 64u,
    52u, 217u, 226u, 250u, 124u, 123u, 5u, 202u, 38u, 147u, 118u, 126u, 255u, 82u, 85u, 212u,
    207u, 206u, 59u, 227u, 47u, 16u, 58u, 17u, 182u, 189u, 28u, 42u, 223u, 183u, 170u, 213u,
    119u, 248u, 152u, 2u, 44u, 154u, 163u, 70u, 221u, 153u, 101u, 155u, 167u, 43u, 172u, 9u,
    129u, 22u, 39u, 253u, 19u, 98u, 108u, 110u, 79u, 113u, 224u, 232u, 178u, 185u, 112u, 104u,
    218u, 246u, 97u, 228u, 251u, 34u, 242u, 193u, 238u, 210u, 144u, 12u, 191u, 179u, 162u, 241u,
    81u, 51u, 145u, 235u, 249u, 14u, 239u, 107u, 49u, 192u, 214u, 31u, 181u, 199u, 106u, 157u,
    184u, 84u, 204u, 176u, 115u, 121u, 50u, 45u, 127u, 4u, 150u, 254u, 138u, 236u, 205u, 93u,
    222u, 114u, 67u, 29u, 24u, 72u, 243u, 141u, 128u, 195u, 78u, 66u, 215u, 61u, 156u, 180u,
);

fn grad3(h: u32, d: vec3<f32>) -> f32 {
    switch h {
        case 0u: { return d.x + d.y; }
        case 1u: { return -d.x + d.y; }
        case 2u: { return d.x - d.y; }
        case 3u: { return -d.x - d.y; }
        case 4u: { return d.x + d.z; }
        case 5u: { return -d.x + d.z; }
        case 6u: { return d.x - d.z; }
        case 7u: { return -d.x - d.z; }
        case 8u: { return d.y + d.z; }
        case 9u: { return -d.y + d.z; }
        case 10u: { return d.y - d.z; }
        default: { return -d.y - d.z; }
    }
}

fn perm(v: i32) -> i32 {
    return i32(PERM[u32(v & 255)]);
}

fn corner(cell: vec3<i32>, o: vec3<i32>, d0: vec3<f32>, k: f32) -> f32 {
    let d = d0 - vec3<f32>(o) + k * (1.0 / 6.0);
    let w = 0.6 - dot(d, d);
    if w <= 0.0 {
        return 0.0;
    }
    var c = cell + o;
    if all(o == vec3<i32>(1)) {
        c.z = cell.z;
    }
    let h = u32(perm(c.x + perm(c.y + perm(c.z))) % 12);
    return w * w * w * w * grad3(h, d);
}

fn simplex(p: vec3<f32>) -> f32 {
    let s = (p.x + p.y + p.z) * (1.0 / 3.0);
    let cell = vec3<i32>(floor(p + s));
    let t = f32(cell.x + cell.y + cell.z) * (1.0 / 6.0);
    let d0 = p - (vec3<f32>(cell) - t);
    var o1 = vec3<i32>(0, 1, 0);
    var o2 = vec3<i32>(1, 1, 0);
    if d0.x >= d0.y {
        if d0.y >= d0.z {
            o1 = vec3<i32>(1, 0, 0);
        } else if d0.x >= d0.z {
            o1 = vec3<i32>(1, 0, 0);
            o2 = vec3<i32>(1, 0, 1);
        } else {
            o1 = vec3<i32>(0, 0, 1);
            o2 = vec3<i32>(1, 0, 1);
        }
    } else if d0.y < d0.z {
        o1 = vec3<i32>(0, 0, 1);
        o2 = vec3<i32>(0, 1, 1);
    } else if d0.x < d0.z {
        o2 = vec3<i32>(0, 1, 1);
    }
    let sum = corner(cell, vec3<i32>(0), d0, 0.0) + corner(cell, o1, d0, 1.0) + corner(cell, o2, d0, 2.0)
        + corner(cell, vec3<i32>(1), d0, 3.0);
    return 32.0 * sum;
}

fn pushed(p: Particle, f: vec4<f32>, a: vec4<f32>, b: vec4<f32>, z: f32) -> vec3<f32> {
    let strength = lerp(f.y, f.z, p.rand);
    let q = (vec3<f32>(p.pos, z) + lerp(a.x, a.y, p.rand)) * f.w + a.w;
    let n = vec3<f32>(simplex(q), simplex(q.zxy), simplex(q.yzx));
    return strength * b.xyz * n;
}
