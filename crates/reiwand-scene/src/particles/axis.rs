pub fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn ring_point(axis: [f32; 3], angle: f32) -> [f32; 3] {
    let helper = if axis[0].abs() <= axis[1].abs() { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let k = dot(axis, helper);
    let flat = [helper[0] - axis[0] * k, helper[1] - axis[1] * k, helper[2] - axis[2] * k];
    let l = dot(flat, flat).sqrt();
    let first = flat.map(|v| v / l);
    let second = cross(axis, first);
    let (s, c) = angle.sin_cos();
    [0, 1, 2].map(|i| first[i] * c + second[i] * s)
}

pub fn toward_axis(axis: [f32; 3], d: [f32; 3]) -> ([f32; 3], f32) {
    let k = dot(axis, d);
    let flat = [d[0] - axis[0] * k, d[1] - axis[1] * k, d[2] - axis[2] * k];
    let r = dot(flat, flat).sqrt().max(1e-3);
    (flat.map(|v| v / r), r)
}
