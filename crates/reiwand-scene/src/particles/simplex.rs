//! 3D simplex noise over the standard table of 256 permutations: one octave, skewed by 1/3 and
//! unskewed by 1/6, each corner weighed by (0.6 − r²)⁴ over the 12 edge gradients, times 32, so it
//! stays within −1..1. The gradient of a corner at cell offset (a, b, c) is
//! `perm[i + a + perm[j + b + perm[k + c]]] % 12`, except the last corner (1, 1, 1), which takes
//! `perm[i + 1 + perm[j + 1 + perm[k]]] % 12`. `simplex.wgsl` of the renderer computes the same
//! numbers on the GPU.

const PERM: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99,
    37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32,
    57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27,
    166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244,
    102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130,
    116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119,
    248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110,
    79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179,
    162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176,
    115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78,
    66, 215, 61, 156, 180,
];

const GRAD: [[f32; 3]; 12] = [
    [1.0, 1.0, 0.0],
    [-1.0, 1.0, 0.0],
    [1.0, -1.0, 0.0],
    [-1.0, -1.0, 0.0],
    [1.0, 0.0, 1.0],
    [-1.0, 0.0, 1.0],
    [1.0, 0.0, -1.0],
    [-1.0, 0.0, -1.0],
    [0.0, 1.0, 1.0],
    [0.0, -1.0, 1.0],
    [0.0, 1.0, -1.0],
    [0.0, -1.0, -1.0],
];

const F3: f32 = 1.0 / 3.0;
const G3: f32 = 1.0 / 6.0;

/// The noise adds the same value again 128 units further along all three axes at once.
pub const SIMPLEX_PERIOD: f64 = 128.0;

fn order([x, y, z]: [f32; 3]) -> ([i32; 3], [i32; 3]) {
    match () {
        _ if x >= y && y >= z => ([1, 0, 0], [1, 1, 0]),
        _ if x >= y && x >= z => ([1, 0, 0], [1, 0, 1]),
        _ if x >= y => ([0, 0, 1], [1, 0, 1]),
        _ if y < z => ([0, 0, 1], [0, 1, 1]),
        _ if x < z => ([0, 1, 0], [0, 1, 1]),
        _ => ([0, 1, 0], [1, 1, 0]),
    }
}

fn hashed(c: [i32; 3]) -> usize {
    let p = |v: i32| i32::from(PERM[(v & 255) as usize]);
    (p(c[0] + p(c[1] + p(c[2]))) % 12) as usize
}

pub fn simplex([x, y, z]: [f32; 3]) -> f32 {
    let s = (x + y + z) * F3;
    let cell = [x + s, y + s, z + s].map(|v| v.floor() as i32);
    let t = (cell[0] + cell[1] + cell[2]) as f32 * G3;
    let d0 = [x - (cell[0] as f32 - t), y - (cell[1] as f32 - t), z - (cell[2] as f32 - t)];
    let (o1, o2) = order(d0);
    let mut sum = 0.0;
    for (o, k) in [([0; 3], 0.0), (o1, 1.0), (o2, 2.0), ([1; 3], 3.0)] {
        let d = [0, 1, 2].map(|a| d0[a] - o[a] as f32 + k * G3);
        let w = 0.6 - d[0] * d[0] - d[1] * d[1] - d[2] * d[2];
        if w > 0.0 {
            let mut c = [0, 1, 2].map(|a| cell[a] + o[a]);
            if o == [1; 3] {
                c[2] = cell[2];
            }
            let g = GRAD[hashed(c)];
            sum += w * w * w * w * (g[0] * d[0] + g[1] * d[1] + g[2] * d[2]);
        }
    }
    32.0 * sum
}

/// The three components of the noise force at `p`: N(x, y, z), N(z, x, y) and N(y, z, x).
pub fn simplex_field([x, y, z]: [f32; 3]) -> [f32; 3] {
    [simplex([x, y, z]), simplex([z, x, y]), simplex([y, z, x])]
}

/// The acceleration of the `noise_field` force packed as `[17, strength, scale]`, `[phase, drift,
/// moved]` and `[mask]` on a particle at depth `z`.
pub(super) fn pushed(p: &super::Particle, [f, a, b]: [[f32; 4]; 3], z: f32) -> [f32; 3] {
    let lerp = super::motion::lerp;
    let (strength, phase) = (lerp(f[1], f[2], p.rand), lerp(a[0], a[1], p.rand));
    let n = simplex_field([p.pos[0], p.pos[1], z].map(|v| (v + phase) * f[3] + a[3]));
    [0, 1, 2].map(|i| strength * b[i] * n[i])
}
