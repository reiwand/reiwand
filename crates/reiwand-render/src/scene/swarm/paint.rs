use super::{Pass, mode};
use crate::scene::Op;
use crate::scene::item::ndc;
use crate::scene::paint::{Painter, Target};
use crate::scene::pipes::Mode;
use reiwand_scene::{Affine, Blend, BlendSpace, Content, Mask};

const TINTS: [Mode; 2] = [Mode::Multiply, Mode::Add];

impl Painter<'_, '_, '_> {
    pub(crate) fn particles(
        &mut self,
        i: usize,
        target: Target,
        (opacity, blend, mask, space): (f32, Blend, Option<&Mask>, Option<BlendSpace>),
        atop: bool,
    ) {
        let frame = self.frame;
        let d = &frame.draws[i];
        let Content::Particles { flock, size, .. } = d.content else { return };
        self.chart(flock);
        if mask.is_some() || space.is_some() || !blend.fixed() || self.lean(i).is_some() {
            return self.apart(i, target, atop);
        }
        let seen = match d.content {
            Content::Particles { seen: Some((stage, far)), .. } => self.seen(stage, far),
            _ => None,
        };
        let (r, screen, srgb) = (self.r, self.screen, self.srgb);
        let Some(swarms) = self.swarms.as_deref_mut() else { return };
        let mode = mode(blend, atop);
        swarms.pipeline(r, mode);
        let Some(herd) = swarms.herds.get(flock) else { return };
        let [w, h] = swarms.canvas;
        let m = match herd.world {
            true => screen.then(frame.camera),
            false => screen.then(d.world).then(Affine::translate([-d.origin[0] * w, -d.origin[1] * h])),
        };
        let tinted = d.tint != reiwand_scene::Color([1.0; 4]) || d.brightness != 1.0;
        let [cr, cg, cb, _] = d.tint.0;
        let toned =
            crate::scene::item::toned(reiwand_scene::Color([cr, cg, cb, 1.0]), d.brightness, srgb, 1.0);
        let tint = [toned[0], toned[1], toned[2], d.tint.0[3]];
        let even = crate::scene::item::toned(reiwand_scene::Color([cr, cg, cb, 1.0]), 1.0, srgb, 1.0);
        let even = [even[0], even[1], even[2], d.tint.0[3]];
        let slot = (swarms.looks.len() as u64 * super::LOOK) as u32;
        let (mut timed, mut bent, mut tints) = (0, [false; 2], [false; 2]);
        let (strength, glow) = d.lit.map_or((1.0, 0.0), |l| (l.strength, l.emissive));
        let lit = f32::from(u8::from(d.lit.is_some()));
        for (p, system) in herd.bake.systems.iter().enumerate() {
            let own = (p == 0).then_some(d.layer.id.as_str());
            let found = |layer: Option<&str>| {
                swarms.kits.iter().position(|k| k.system == *system && k.layer.as_deref() == layer)
            };
            let Some(k) = own.and_then(|l| found(Some(l))).or_else(|| found(None)) else { continue };
            let staged = swarms.kits[k].stage.as_ref().map(|s| s.0);
            if let Some((key, why)) = staged.and_then(|key| Some((key, super::staging::left(key)?))) {
                super::stage::leave(&mut swarms.kits, key, &why);
            }
            if let Some(key) = swarms.kits[k].stage.as_ref().map(|s| s.0) {
                self.drawn.push(key);
                timed = if timed == 0 { key } else { timed };
            }
            let kit = &swarms.kits[k];
            if let Some((id, why)) = &kit.refused {
                self.refused.entry(id.clone()).or_insert_with(|| why.clone());
            }
            let mut look = kit.look;
            let pixeled = kit.pixeled && kit.stage.is_some();
            if tinted {
                let tint = if pixeled { even } else { tint };
                look.colors.iter_mut().for_each(|c| (0..4).for_each(|j| c[j] *= tint[j]));
            }
            if pixeled {
                look.shade[0] = d.brightness;
            }
            (look.m0, look.m1) = ndc(target.rect, m);
            look.turn[2] = m.0[1].atan2(m.0[0]);
            (look.light[0], look.light[3]) = (look.light[0].max(lit), crate::scene::item::scale_of(m));
            look.place = [target.rect.x, target.rect.y, strength, glow];
            bent = [bent[0] || (kit.bend.is_some() && !kit.light), bent[1] || kit.light];
            (tints[0], tints[1]) =
                (tints[0] || kit.tint == Some(Mode::Multiply), tints[1] || kit.tint == Some(Mode::Add));
            let middle = m.inverse().apply(screen.apply([w / 2.0, h / 2.0]));
            (look.deep[1], look.deep[2]) = (middle[0], middle[1]);
            look.tint[0] = opacity;
            look.tint[2] = herd.bake.sizes[p] * size;
            let trail = herd.bake.trails[p];
            let q = swarms.quality;
            let drawn =
                if kit.long { q.long_trail(kit.trail.min(trail)) } else { q.trail(kit.trail.min(trail)) };
            (look.extra[1], look.extra[2]) = (drawn as f32, trail as f32);
            look.strand[0] = q.subdivide(look.strand[0] as u32) as f32;
            let scroll = |v: f32| (f64::from(v) * f64::from(frame.time)).rem_euclid(1.0) as f32;
            (look.strand[2], look.cord[2]) = (scroll(look.strand[2]), scroll(look.cord[2]));
            if let Some((_, rows, window)) = &seen {
                let shrink = swarms.kits[k].bend.as_ref().and(frame.quality.bend()).unwrap_or(1.0);
                (look.seen, look.window) = (*rows, [window[0], window[1], window[2], 1.0 / shrink]);
            }
            swarms.seen.push(seen.as_ref().map(|s| s.0.clone()));
            swarms.looks.push(look);
            swarms.kitted.push(k);
        }
        let lights = [tints[0], tints[1] || bent[1]];
        TINTS.iter().zip(lights).filter(|(_, on)| *on).for_each(|(m, _)| swarms.pipeline(r, *m));
        let tinted = match (lights.contains(&true), seen.is_some()) {
            (true, true) => swarms.unshrunk(slot),
            _ => slot,
        };
        let lit = self.lit_swarms() && self.pool.glow.pictures() > 0;
        let cast = if lit { self.gobos() } else { [crate::scene::Src::White; 4] };
        let (to, system, pass) = (target.src, flock, Pass::Plain);
        self.ops.push(Op::Particles { target: to, system, slot, mode, timed, pass, cast });
        for light in [false, true].into_iter().filter(|l| bent[usize::from(*l)]) {
            self.bend(target, flock, (slot, tinted), (tints, light), cast);
        }
    }
}
