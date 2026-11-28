use super::draws;
use crate::particles::motion::rand;
use std::f32::consts::TAU;

const F: [f32; 4] = [0.5, 4.0, 2.0, 30.0];
const G: [f32; 4] = [0.1, 0.9, 0.0, 0.0];

fn correlation(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len() as f32;
    let (ma, mb) = (a.iter().sum::<f32>() / n, b.iter().sum::<f32>() / n);
    let cov: f32 = a.iter().zip(b).map(|(x, y)| (x - ma) * (y - mb)).sum();
    let var = |v: &[f32], m: f32| v.iter().map(|x| (x - m).powi(2)).sum::<f32>();
    cov / (var(a, ma) * var(b, mb)).sqrt()
}

fn particles(shared: bool) -> [Vec<f32>; 4] {
    let mut out: [Vec<f32>; 4] = Default::default();
    for id in 0..4000u32 {
        let d = draws(F, G, shared, |j| rand(7, id, 1000 + j));
        out.iter_mut().zip(d).for_each(|(v, x)| v.push(x));
    }
    out
}

#[test]
fn a_shared_draw_ties_frequency_scale_and_phase_to_one_number() {
    let [freq, scale, phase, peak] = particles(true);
    // The phase is lerp(phase)·freq/2π, a product of two rising lines: rank-perfect, nearly linear.
    assert!(correlation(&freq, &scale) > 0.9999, "{}", correlation(&freq, &scale));
    assert!(correlation(&scale, &peak) > 0.9999);
    assert!(correlation(&freq, &phase) > 0.95, "{}", correlation(&freq, &phase));
    for i in 0..freq.len() {
        let r = (freq[i] - F[0]) / (F[1] - F[0]);
        assert!((scale[i] - (F[2] + r * (F[3] - F[2]))).abs() < 1e-3);
        assert!((phase[i] - (G[0] + r * (G[1] - G[0])) * freq[i] / TAU).abs() < 1e-4);
    }
}

#[test]
fn separate_draws_stay_independent() {
    let [freq, scale, phase, peak] = particles(false);
    for (a, b) in [(&freq, &scale), (&freq, &phase), (&scale, &phase), (&scale, &peak)] {
        assert!(correlation(a, b).abs() < 0.05, "{}", correlation(a, b));
    }
}
