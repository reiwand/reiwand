use super::{Base, Collider, FULL_WINDOW, Falloff, Force, Operator, Surface, System, Wobble};
use crate::validate::{Check, Issue};

impl Check<'_> {
    pub(super) fn operator(&mut self, q: &str, o: &Operator, s: &System) {
        let w = o.window;
        self.uses(w != FULL_WINDOW, "particles2");
        self.uses(o.ramp.is_some(), "window_ramp");
        w.iter().enumerate().for_each(|(k, v)| self.range(&format!("{q}/window/{k}"), *v, 0.0, 1.0));
        if w.windows(2).any(|p| p[0] > p[1]) {
            self.issues.push(Issue::Type(format!("{q}/window")));
        }
        self.force2(q, &o.force, s);
    }

    pub(super) fn collider2(&mut self, q: &str, c: &Collider) {
        self.uses(c.stick, "particle_collisions");
        match &c.bounds {
            Surface::Circle { inside, .. } => self.uses(*inside, "particle_collisions"),
            Surface::Box { from, to, .. } | Surface::Segment { from, to } => {
                self.uses(true, "particle_collisions");
                [*from, *to].into_iter().for_each(|v| self.place(q, v));
                if from == to {
                    self.issues.push(Issue::Type(format!("{q}/bounds")));
                }
            }
            Surface::Mask { texture, threshold } => {
                self.uses(true, "particles2");
                self.picture(&format!("{q}/bounds/mask/texture"), texture);
                self.range(&format!("{q}/bounds/mask/threshold"), *threshold, 0.0, 1.0);
            }
            Surface::Layer { id } => {
                self.uses(true, "particle_collisions");
                let drawn = self.scene.layer(id).is_some_and(crate::Layer::drawn);
                self.known(&format!("{q}/bounds/layer/id"), "layer", id, drawn);
                let system = q.split('/').nth(2).unwrap_or_default();
                if self.scene.layer(id).is_some_and(|l| super::checks::holds(l, system)) {
                    self.issues.push(Issue::Circle(format!("{q}/bounds/layer/id")));
                }
            }
            _ => {}
        }
    }

    pub(super) fn axis(&mut self, path: &str, axis: [f32; 3]) {
        let length = axis.iter().map(|v| v * v).sum::<f32>().sqrt();
        self.range(path, length, 0.999, 1.001);
    }

    fn force2(&mut self, q: &str, force: &Force, s: &System) {
        let reach = |c: &mut Self, radius: f32, falloff: Falloff, plain: Falloff| {
            c.uses(radius != 0.0 || falloff != plain, "particles2");
            c.range(&format!("{q}/radius"), radius, 0.0, 1e5);
        };
        match force {
            Force::Gravity(v) => self.value(q, v),
            Force::Drag(v) | Force::Noise(v) => self.value(q, v),
            Force::Turbulence { strength, scale, speed, time_scale, mask } => {
                self.value(&format!("{q}/strength"), strength);
                self.range(&format!("{q}/scale"), *scale, 1e-6, 1.0);
                self.uses(*speed != 0.0 || *time_scale != 1.0 || *mask != [1.0; 2], "particles2");
                self.range(&format!("{q}/speed"), *speed, 0.0, 100.0);
                self.range(&format!("{q}/time_scale"), *time_scale, 0.0, 100.0);
                mask.iter().enumerate().for_each(|(k, v)| self.range(&format!("{q}/mask/{k}"), *v, 0.0, 1.0));
            }
            Force::Attractor { target: t, strength, radius, falloff } => {
                self.value(&format!("{q}/strength"), strength);
                self.target(q, t, s);
                reach(self, *radius, *falloff, Falloff::InverseSquare);
            }
            Force::Orbit { center: t, strength, radius, falloff } => {
                self.value(&format!("{q}/strength"), strength);
                self.target(q, t, s);
                reach(self, *radius, *falloff, Falloff::Constant);
            }
            Force::Vortex { center, strength, radius, falloff, inner, outer, ring, axis } => {
                self.axis(&format!("{q}/axis"), *axis);
                self.value(&format!("{q}/strength"), strength);
                self.target(q, center, s);
                reach(self, *radius, *falloff, Falloff::Linear);
                self.uses(*inner != 0.0 || outer.is_some() || ring.is_some(), "particles2");
                let lo = *inner;
                self.range(&format!("{q}/inner"), lo, 0.0, 1e5);
                outer.iter().for_each(|o| self.range(&format!("{q}/outer"), *o, lo, 1e5));
                if let Some(r) = ring {
                    self.range(&format!("{q}/ring/radius"), r.radius, 0.0, 1e5);
                    self.range(&format!("{q}/ring/width"), r.width, 0.0, 1e5);
                    self.range(&format!("{q}/ring/pull"), r.pull, -1e3, 1e3);
                }
            }
            Force::Oscillate(o) => {
                self.uses(true, "particles2");
                self.uses(o.random.is_some(), "oscillate_random");
                let p = format!("{q}/oscillate");
                if o.what == Wobble::Position && o.from == Base::Floor {
                    self.issues.push(Issue::Type(format!("{p}/from")));
                }
                self.range(&format!("{p}/freq/0"), o.freq[0], 0.0, 60.0);
                self.range(&format!("{p}/freq/1"), o.freq[1], o.freq[0], 60.0);
                self.range(&format!("{p}/scale/1"), o.scale[1], o.scale[0], 1e5);
                o.phase.iter().for_each(|v| self.range(&format!("{p}/phase"), *v, 0.0, 1.0));
                o.mask.iter().for_each(|v| self.range(&format!("{p}/mask"), *v, 0.0, 1.0));
            }
            Force::NoiseField(n) => {
                self.uses(true, "particle_noise");
                let p = format!("{q}/noise_field");
                for (name, v) in [("strength", n.strength), ("phase", n.phase)] {
                    self.range(&format!("{p}/{name}/min"), v.min, -1e5, 1e5);
                    self.range(&format!("{p}/{name}/max"), v.max, -1e5, 1e5);
                }
                self.range(&format!("{p}/scale"), n.scale, 1e-6, 10.0);
                self.range(&format!("{p}/drift"), n.drift, -1e5, 1e5);
                n.mask
                    .iter()
                    .enumerate()
                    .for_each(|(k, v)| self.range(&format!("{p}/mask/{k}"), *v, 0.0, 1.0));
            }
            Force::Cap(v) | Force::Torque(v) | Force::SpinDrag(v) => {
                self.uses(true, "particles2");
                self.value(q, v);
            }
            Force::SlowNear(n) => {
                self.uses(true, "particles2");
                self.target(q, &n.target, s);
                self.range(&format!("{q}/slow_near/inner"), n.inner, 0.0, 1e5);
                self.range(&format!("{q}/slow_near/outer"), n.outer, n.inner, 1e5);
                self.range(&format!("{q}/slow_near/factor"), n.factor, 0.0, 1.0);
            }
            Force::KeepDistance(k) => {
                self.uses(true, "particle_steering");
                self.target(q, &k.target, s);
                self.range(&format!("{q}/keep_distance/distance"), k.distance, 0.0, 1e5);
                self.range(&format!("{q}/keep_distance/strength"), k.strength, -1e3, 1e3);
            }
            Force::KeepBetween(k) => {
                self.uses(true, "particle_steering");
                self.target(q, &k.from, s);
                self.target(q, &k.to, s);
                self.range(&format!("{q}/keep_between/strength"), k.strength, -1e3, 1e3);
            }
            Force::Boids(b) => {
                self.uses(true, "boids");
                let p = format!("{q}/boids");
                self.range(&format!("{p}/radius"), b.radius, 1.0, 1e4);
                for (name, v) in
                    [("separation", b.separation), ("alignment", b.alignment), ("cohesion", b.cohesion)]
                {
                    self.range(&format!("{p}/{name}"), v, 0.0, 10.0);
                }
                self.range(&format!("{p}/max_speed"), b.max_speed, 0.0, 1e5);
                let system = q.split("/forces/").next().unwrap_or(q);
                self.range(&format!("{system}/capacity"), s.capacity as f32, 1.0, super::MAX_BOIDS as f32);
            }
            Force::Remap(r) => self.remap(&format!("{q}/remap"), r, (s, "particle_steering")),
        }
    }
}
