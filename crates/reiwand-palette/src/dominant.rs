use crate::oklab::{lab, rgb};
use crate::tone::Rgb;

const ROUNDS: usize = 12;
const CLEAR: u8 = 128;

type Lab = [f64; 3];

fn far(a: Lab, b: Lab) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}

fn nearest(centres: &[Lab], p: Lab) -> usize {
    let mut best = (0, f64::INFINITY);
    for (i, c) in centres.iter().enumerate() {
        let d = far(*c, p);
        if d < best.1 {
            best = (i, d);
        }
    }
    best.0
}

fn seeds(points: &[Lab]) -> Vec<Lab> {
    let n = points.len() as f64;
    let mean = points.iter().fold([0.0; 3], |s, p| [s[0] + p[0] / n, s[1] + p[1] / n, s[2] + p[2] / n]);
    let mut centres = vec![points[nearest(points, mean)]];
    while centres.len() < 3 {
        let gap = |p: &Lab| centres.iter().map(|c| far(*c, *p)).fold(f64::INFINITY, f64::min);
        let next = points.iter().copied().max_by(|a, b| gap(a).total_cmp(&gap(b)));
        centres.push(next.unwrap_or(mean));
    }
    centres
}

pub fn dominant(pixels: &[u8], channels: usize) -> Option<[Rgb; 3]> {
    if !(channels == 3 || channels == 4) {
        return None;
    }
    let seen = pixels.chunks_exact(channels).filter(|p| channels == 3 || p[3] >= CLEAR);
    let points: Vec<Lab> = seen.map(|p| lab([p[0], p[1], p[2]])).collect();
    if points.is_empty() {
        return None;
    }
    let mut centres = seeds(&points);
    let mut counts = [0usize; 3];
    for _ in 0..ROUNDS {
        let mut sums = [[0.0f64; 3]; 3];
        counts = [0; 3];
        for p in &points {
            let i = nearest(&centres, *p);
            counts[i] += 1;
            (0..3).for_each(|k| sums[i][k] += p[k]);
        }
        let moved: Vec<Lab> = (0..3)
            .map(|i| match counts[i] {
                0 => centres[i],
                n => sums[i].map(|s| s / n as f64),
            })
            .collect();
        let still = moved.iter().zip(&centres).all(|(a, b)| far(*a, *b) < 1e-8);
        centres = moved;
        if still {
            break;
        }
    }
    let mut order = [0, 1, 2];
    order.sort_by(|a, b| counts[*b].cmp(&counts[*a]).then(a.cmp(b)));
    Some(order.map(|i| {
        let [l, a, b] = centres[i];
        Rgb(rgb(l, a.hypot(b), b.atan2(a)))
    }))
}
