use super::{MAX_CURVE, MAX_SHIFT, Refract, Render, StretchDepth, StretchMode};
use crate::effects::Stage;
use crate::validate::{Check, Issue};

impl Check<'_> {
    pub(super) fn render2(&mut self, p: &str, r: &Render) {
        let wide = r.brightness.is_some()
            || r.order.is_some()
            || r.cycles.is_some()
            || r.stretch.is_some_and(|s| {
                s.mode == StretchMode::Relative || s.depth != StretchDepth::Flat || s.min > 0.0 || s.max < 1e4
            });
        self.uses(wide, "particles2");
        self.uses(r.aspect != 1.0, "particles2");
        self.range(&format!("{p}/aspect"), r.aspect, 0.05, 20.0);
        r.brightness.iter().for_each(|b| self.number(&format!("{p}/brightness"), b, 0.0, 16.0));
        if let Some(n) = r.cycles {
            self.range(&format!("{p}/cycles"), n as f32, 1.0, 64.0);
            if r.frames != Some(super::Frames::Life) {
                self.issues.push(Issue::Type(format!("{p}/cycles")));
            }
        }
        if let Some(s) = r.stretch {
            self.range(&format!("{p}/stretch/min"), s.min, 0.0, 1e4);
            self.range(&format!("{p}/stretch/max"), s.max, s.min, 1e4);
            self.uses(s.anchor.is_some(), "stretch_anchor");
        }
        if let Some([start, end, opacity]) = r.cutout {
            self.uses(true, "particle_looks");
            self.range(&format!("{p}/cutout/0"), start, 0.0, 1.0);
            self.range(&format!("{p}/cutout/1"), end, start.next_up(), 1.0);
            self.range(&format!("{p}/cutout/2"), opacity, 0.0, 1.0);
        }
        self.uses(r.orient.is_some() || r.angle.is_some(), "particle_looks");
        r.angle.iter().for_each(|a| self.range(&format!("{p}/angle"), *a, -360.0, 360.0));
        if r.angle.is_some() && r.orient != Some(super::Orient::Fixed) {
            self.issues.push(Issue::Type(format!("{p}/angle")));
        }
        if let Some(rope) = r.rope {
            self.uses(true, "particle_ropes");
            self.range(&format!("{p}/rope/subdivide"), rope.subdivide as f32, 0.0, 7.0);
            self.range(&format!("{p}/rope/width"), rope.width, 0.0, 1e3);
            self.range(&format!("{p}/rope/uv_scale"), rope.uv_scale, 0.0, 100.0);
            self.range(&format!("{p}/rope/uv_scroll"), rope.uv_scroll, -100.0, 100.0);
        }
        if let Some(t) = &r.trail {
            let more = [t.segments.is_some(), t.subdivide != 0, t.uv_scale != 1.0, t.uv_scroll != 0.0];
            self.uses(t.length > 16 || more.contains(&true) || t.textured, "particle_ropes");
            t.segments.iter().for_each(|n| self.range(&format!("{p}/trail/segments"), *n as f32, 1.0, 64.0));
            self.range(&format!("{p}/trail/subdivide"), t.subdivide as f32, 0.0, 7.0);
            self.range(&format!("{p}/trail/uv_scale"), t.uv_scale, 0.0, 100.0);
            self.range(&format!("{p}/trail/uv_scroll"), t.uv_scroll, -100.0, 100.0);
        }
        self.uses(r.lit || r.normal.is_some() || r.refract.is_some(), "particle_lighting");
        r.normal.iter().for_each(|n| self.picture(&format!("{p}/normal"), n));
        if let Some(f) = &r.refract {
            self.picture(&format!("{p}/refract/normal"), f.normal());
            match f {
                Refract::Size { amount, .. } => self.range(&format!("{p}/refract/amount"), *amount, 0.0, 1.0),
                Refract::Pixels { shift, .. } => {
                    self.range(&format!("{p}/refract/shift"), *shift, 0.0, MAX_SHIFT)
                }
            }
        }
        if let Some(id) = &r.shader {
            self.uses(true, "particle_shaders");
            self.staged(&format!("{p}/shader"), id, Stage::Particle);
        }
    }

    pub(crate) fn staged(&mut self, path: &str, id: &str, stage: Stage) {
        let scene = self.scene;
        let declared = scene.effects.contains_key(id);
        self.known(path, "shader", id, declared);
        if scene.loaded.get(id).is_some_and(|d| d.stage != stage) {
            self.issues.push(Issue::Type(path.into()));
        }
    }
}

impl Check<'_> {
    pub(crate) fn render(&mut self, p: &str, r: &Render) {
        if let Some(texture) = &r.texture {
            self.named(&format!("{p}/render/texture"), texture);
        }
        self.range(&format!("{p}/render/fps"), r.fps, 0.0, 240.0);
        if let Some(s) = r.stretch {
            self.range(&format!("{p}/render/stretch"), s.time, 0.0, 10.0);
        }
        let sizes = r.size_over_life.iter().map(|k| k[0]);
        let colors = r.color_over_life.iter().map(|k| k.0);
        for (name, times) in
            [("size_over_life", sizes.collect::<Vec<_>>()), ("color_over_life", colors.collect())]
        {
            let q = format!("{p}/render/{name}");
            self.count(&q, times.len(), MAX_CURVE);
            self.keys_sorted(&q, times.iter().copied());
            times.iter().for_each(|t| self.range(&q, *t, 0.0, 1.0));
        }
        r.size_over_life
            .iter()
            .for_each(|k| self.range(&format!("{p}/render/size_over_life"), k[1], 0.0, 100.0));
        for (k, (_, c)) in r.color_over_life.iter().enumerate() {
            self.color(&format!("{p}/render/color_over_life/{k}/1"), *c);
        }
        if let Some(t) = &r.trail {
            self.range(&format!("{p}/render/trail/length"), t.length as f32, 1.0, 64.0);
            self.range(&format!("{p}/render/trail/width"), t.width, 0.0, 1.0);
        }
        self.render2(&format!("{p}/render"), r);
    }
}
