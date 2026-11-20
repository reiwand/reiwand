use super::{
    Burst, Emitter, Flock, Instance, MAX_EMITTERS2, MAX_HATCH, MAX_RATE, Moment, Part, STEP, Spread, System,
};
use crate::value::Value;
use crate::{Layer, Scene, ScriptApi};

const ENDLESS: f32 = f32::INFINITY;

#[derive(Clone, Copy)]
struct Tunes {
    count: f32,
    life: f32,
    free: bool,
}

fn most(v: &Value<f32>) -> f32 {
    match v {
        Value::Fixed(v) => *v,
        Value::Bound(b) => b.clamp.map_or(ENDLESS, |[lo, hi]| lo.max(hi)),
    }
}

fn top(s: &Spread<f32>) -> f32 {
    match s.bound.as_deref() {
        Some([a, b]) => most(a).max(most(b)),
        None => s.ends[0].max(s.ends[1]),
    }
}

fn times(window: f32, every: f32) -> f32 {
    (window / every.max(STEP)).floor() + 1.0
}

fn burst(b: &Burst, window: f32, gap: Option<[f32; 2]>) -> f32 {
    let count = b.count as f32;
    match (b.on, b.every, gap) {
        (Some(_), ..) if count > 0.0 => ENDLESS,
        (Some(_), ..) => 0.0,
        (None, Some(every), Some([_, off])) => count * times(window, every.min(off)),
        (None, Some(every), None) => count * times(window, every),
        (None, None, Some([cycle, _])) => count * times(window, cycle),
        (None, None, None) => count,
    }
}

fn emitted(e: &Emitter, window: f32, count: f32) -> f32 {
    let gap = e.periodic.map(|p| {
        let [on, off] = [p.on[0], p.off[0]].map(|v| v.max(STEP));
        [on + off, off]
    });
    let rate = most(&e.rate).clamp(0.0, MAX_RATE) * count;
    let born = rate * window + e.bursts.iter().map(|b| burst(b, window, gap)).sum::<f32>();
    e.max_per_step.map_or(born, |n| born.min(n as f32 * times(window, STEP)))
}

fn own(s: &System, window: f32, t: Tunes) -> f32 {
    if !t.free || !window.is_finite() {
        return ENDLESS;
    }
    s.emitters.iter().take(MAX_EMITTERS2).map(|e| emitted(e, window, t.count)).sum()
}

fn each(s: &System) -> f32 {
    let once = s.emitters.iter().take(MAX_EMITTERS2).flat_map(|e| e.bursts.iter().filter(|b| b.immediate()));
    let n = once.fold(0u32, |n, b| n.saturating_add(b.count));
    (if n == 0 { 1 } else { n.min(MAX_HATCH) }) as f32
}

fn life(s: &System, t: Tunes) -> f32 {
    top(&s.init.life).max(0.0) * t.life
}

fn born(systems: &[&System], parts: &[Part], k: usize, window: f32, t: Tunes) -> f32 {
    let part = &parts[k];
    let s = systems[part.system];
    let Some(h) = part.host else { return own(s, window, t) };
    let host = systems[parts[h].system];
    let Some(c) = host.children.get(part.link) else { return own(s, window, t) };
    let hosted = match c.on {
        Moment::Static => return own(s, window, t),
        Moment::Follow => ENDLESS,
        Moment::Spawn => born(systems, parts, h, window, t),
        Moment::Death => born(systems, parts, h, window + life(host, t), t),
    };
    if c.probability <= 0.0 { 0.0 } else { hosted * each(s) * c.probability.min(1.0) }
}

fn tunes(scene: &Scene, layer: Option<&Layer>) -> Tunes {
    let i = super::tune::instance(layer);
    let life = i.and_then(|i| i.life.as_ref()).map_or(1.0, |v| most(v).clamp(0.0, 100.0));
    let free = !scene.scripts.iter().any(|s| s.wants(ScriptApi::Particles));
    Tunes { count: i.map_or(1.0, Instance::peak), life, free }
}

pub fn alive(scene: &Scene, layer: Option<&Layer>, s: &System) -> f32 {
    let t = tunes(scene, layer);
    (s.capacity as f32 * t.count).ceil().min(own(s, life(s, t), t))
}

pub fn lives(scene: &Scene, f: &Flock) -> Vec<f32> {
    let systems: Vec<&System> = scene.particles.values().collect();
    let layer = f.layer.as_ref().and_then(|id| scene.layers().find(|l| &l.id == id));
    let t = tunes(scene, layer);
    let room = |s: &System| (s.capacity as f32 * t.count).ceil();
    let alive = |k: usize, p: &Part| {
        let s = systems[p.system];
        room(s).min(born(&systems, &f.parts, k, life(s, t), t))
    };
    f.parts.iter().enumerate().map(|(k, p)| alive(k, p)).collect()
}
