use super::{LOOK, Pass};
use crate::scene::item::{Item, Rect, ndc};
use crate::scene::paint::{Painter, Target};
use crate::scene::pipes::Mode;
use crate::scene::{Op, Src};
use wgpu::TextureFormat;

impl super::Swarms {
    pub(crate) fn bent(&self) -> impl Iterator<Item = &str> {
        self.kits.iter().filter(|k| k.bend.is_some()).map(|k| k.layer.as_deref().unwrap_or("particles"))
    }

    pub(super) fn unshrunk(&mut self, slot: u32) -> u32 {
        let (first, last) = ((u64::from(slot) / LOOK) as usize, self.looks.len());
        for at in first..last {
            let mut look = self.looks[at];
            look.window[3] = 1.0;
            let (kit, seen) = (self.kitted[at], self.seen[at].clone());
            self.looks.push(look);
            self.kitted.push(kit);
            self.seen.push(seen);
        }
        (last as u64 * LOOK) as u32
    }
}

impl Painter<'_, '_, '_> {
    pub(crate) fn lit_swarms(&self) -> bool {
        self.swarms.as_ref().is_some_and(|s| s.kits.iter().any(|k| k.look.light[0] > 0.5))
    }

    pub(super) fn bend(
        &mut self,
        target: Target,
        flock: usize,
        (slot, tinted): (u32, u32),
        (tints, light): ([bool; 2], bool),
        cast: [Src; 4],
    ) {
        let Some(scale) = self.frame.quality.bend() else { return };
        if self.state_format != TextureFormat::Rgba16Float {
            return;
        }
        let (w, h) = target.rect.size();
        let small = (((w as f32) * scale).ceil() as u32, ((h as f32) * scale).ceil() as u32);
        let Some(shift) = self.borrow(small, TextureFormat::Rgba16Float).map(Src::Pool) else { return };
        let Some(back) = self.acquire((w, h)) else { return self.release(shift) };
        let shown = self.swarms.as_ref().is_some_and(|s| s.shown);
        let glow = if light && !shown { self.acquire((w, h)) } else { None };
        if light && !shown && glow.is_none() {
            self.release(back);
            return self.release(shift);
        }
        self.ops.push(Op::Clear(shift, [0.0; 4]));
        let (system, mode) = (flock, Mode::Normal);
        let white = [Src::White; 4];
        let pass = Pass::Bend(light);
        self.ops.push(Op::Particles { target: shift, system, slot, mode, timed: 0, pass, cast: white });
        self.redraw(target.src, back, (w, h));
        if let Some(glow) = glow {
            self.ops.push(Op::Clear(glow, [0.0; 4]));
            let (mode, pass) = (Mode::Add, Pass::Light);
            self.ops.push(Op::Particles { target: glow, system, slot: tinted, mode, timed: 0, pass, cast });
        }
        let full = [0.0, 0.0, 1.0, 1.0];
        let mut item = Item { quad: full, uv: full, color: [1.0; 4], ..Item::default() };
        (item.m0, item.m1) = ndc(target.rect, target.rect.affine());
        item.lit_env = [0.0, 0.0, 1.0 / w as f32, 1.0 / h as f32];
        let slot = self.pool.push(&item);
        let (entry, mode, tex) = match glow {
            _ if shown => ("fs_shift_view", Mode::Replace, [back, shift, Src::Clear]),
            Some(glow) => ("fs_refract_light", Mode::Add, [back, shift, glow]),
            None => ("fs_refract", Mode::Replace, [back, shift, Src::Clear]),
        };
        self.ops.push(Op::Draw { target: target.src, entry, mode, slot, tex });
        [Some(back), Some(shift), glow].into_iter().flatten().for_each(|t| self.release(t));
        let modes =
            [Mode::Multiply, Mode::Add].into_iter().zip(tints).filter(|(_, on)| *on && !shown && !light);
        for (mode, _) in modes {
            let (to, slot, pass) = (target.src, tinted, Pass::Tint);
            self.ops.push(Op::Particles { target: to, system: flock, slot, mode, timed: 0, pass, cast });
        }
    }

    fn redraw(&mut self, from: Src, to: Src, (w, h): (u32, u32)) {
        let (tw, th) = self.size_of(from);
        let uv = [0.0, 0.0, w as f32 / tw as f32, h as f32 / th as f32];
        let whole = Rect { x: 0.0, y: 0.0, w: w as f32, h: h as f32 };
        let mut item = Item { quad: [0.0, 0.0, 1.0, 1.0], uv, color: [1.0; 4], ..Item::default() };
        (item.m0, item.m1) = ndc(whole, whole.affine());
        let slot = self.pool.push(&item);
        let tex = [from, Src::White, Src::Clear];
        self.ops.push(Op::Draw { target: to, entry: "fs_texture", mode: Mode::Replace, slot, tex });
    }
}
