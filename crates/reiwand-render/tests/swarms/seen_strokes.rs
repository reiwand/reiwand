use super::*;

fn coming(at: [f32; 2], depth: &str) -> Value {
    json!({ "capacity": 8, "seed": 5, "perspective": { "focal": 100 },
        "emitters": [{ "shape": { "point": at }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "speed": [0, 0], "size": [10, 10], "depth": [0, 0], "depth_speed": [-100, -100],
            "color": ["#ffffff", "#ffffff"] },
        "render": { "stretch": { "mode": "relative", "depth": depth, "time": 0.05, "min": 0, "max": 1000 } } })
}

pub(super) struct Stroke {
    pub(super) count: usize,
    pub(super) wide: u32,
    pub(super) high: u32,
    pub(super) angle: f64,
}

pub(super) fn measure(pixels: &[u8]) -> Stroke {
    let lit: Vec<(f64, f64)> = (0..SIZE.1)
        .flat_map(|y| (0..SIZE.0).map(move |x| (x, y)))
        .filter(|(x, y)| at(pixels, *x, *y)[0] > 100)
        .map(|(x, y)| (f64::from(x), f64::from(y)))
        .collect();
    let n = lit.len().max(1) as f64;
    let (mx, my) = (lit.iter().map(|p| p.0).sum::<f64>() / n, lit.iter().map(|p| p.1).sum::<f64>() / n);
    let (mut sxx, mut syy, mut sxy) = (0.0, 0.0, 0.0);
    for (x, y) in &lit {
        (sxx, syy, sxy) = (sxx + (x - mx).powi(2), syy + (y - my).powi(2), sxy + (x - mx) * (y - my));
    }
    let span = |f: fn(&(f64, f64)) -> f64| {
        let v: Vec<f64> = lit.iter().map(f).collect();
        v.iter().cloned().fold(f64::MIN, f64::max) - v.iter().cloned().fold(f64::MAX, f64::min) + 1.0
    };
    let wide = if lit.is_empty() { 0 } else { span(|p| p.0) as u32 };
    let high = if lit.is_empty() { 0 } else { span(|p| p.1) as u32 };
    Stroke { count: lit.len(), wide, high, angle: 0.5 * (2.0 * sxy).atan2(sxx - syy).to_degrees() }
}

fn shot(r: &Renderer, at: [f32; 2], depth: &str, gpu: bool) -> Option<Stroke> {
    let systems = json!({ "s": coming(at, depth) });
    let layers =
        json!([{ "blend_space": "canvas", "id": "s", "type": "particles", "system": "s", "blend": "add" }]);
    let mut run = Run::new(r, systems, layers, gpu)?;
    run.frame(r, 0.0);
    let pixels = (0..6).map(|_| run.frame(r, 1.0 / 30.0)).last().unwrap();
    Some(measure(&pixels))
}

#[test]
fn depth_motion_strokes_point_away_from_the_centre_and_grow_with_distance() {
    let Some(r) = renderer() else { return };
    for gpu in [false, true] {
        let Some(near) = shot(r, [100.0, 40.0], "motion", gpu) else { return };
        let far = shot(r, [130.0, 40.0], "motion", gpu).unwrap();
        let below = shot(r, [80.0, 60.0], "motion", gpu).unwrap();
        let slant = shot(r, [100.0, 60.0], "motion", gpu).unwrap();
        assert!(near.wide > 14 && near.high < 14, "gpu {gpu}: {} x {}", near.wide, near.high);
        assert!(far.wide > near.wide + 12, "gpu {gpu}: {} against {}", far.wide, near.wide);
        assert!(below.high > 14 && below.wide < 14, "gpu {gpu}: {} x {}", below.wide, below.high);
        assert!((slant.angle - 45.0).abs() < 5.0, "gpu {gpu}: {}", slant.angle);
        for flat in ["flat", "projected"] {
            let none = shot(r, [100.0, 40.0], flat, gpu).unwrap();
            assert_eq!(none.count, 0, "gpu {gpu}: {flat} draws nothing {} x {}", none.wide, none.high);
        }
    }
}
