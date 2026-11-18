use crate::adopt::Files;
use crate::{Descriptor, EffectUse, MaskChannel, MaskMode, Scene, Value, Wrapping};
use std::collections::{BTreeMap, BTreeSet};

pub type Inked = [[f32; 4]; 4];

const EMPTY: [f32; 4] = [1.0, 1.0, 0.0, 0.0];
const MAX_PICTURE: usize = 64 << 20;

pub fn ink_bounds(width: u32, height: u32, rgba: &[u8]) -> Inked {
    let mut found = [[u32::MAX, u32::MAX, 0, 0]; 4];
    for (i, p) in rgba.as_chunks::<4>().0.iter().enumerate() {
        let (x, y) = (i as u32 % width, i as u32 / width);
        for (b, v) in found.iter_mut().zip(p) {
            if *v > 0 {
                *b = [b[0].min(x), b[1].min(y), b[2].max(x + 1), b[3].max(y + 1)];
            }
        }
    }
    let (w, h) = (width as f32, height as f32);
    found.map(|[x0, y0, x1, y1]| match x1 > x0 {
        true => [x0 as f32 / w, y0 as f32 / h, x1 as f32 / w, y1 as f32 / h],
        false => EMPTY,
    })
}

fn span(boxes: impl Iterator<Item = [f32; 4]>) -> Option<[f32; 4]> {
    let full = boxes.filter(|b| b[2] > b[0] && b[3] > b[1]);
    full.reduce(|a, b| [a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])])
}

fn textures<'a>(scene: &'a Scene, e: &'a EffectUse) -> Vec<&'a str> {
    let Some(d) = scene.effect(&e.effect) else { return Vec::new() };
    let named = d.bounded.iter().filter_map(|b| d.params.get(d.param(b)?)?.named(e));
    let masks = fixed_mask(e, d).map(|(id, _)| id);
    named.chain(masks).collect()
}

fn fixed_mask<'a>(e: &'a EffectUse, d: &Descriptor) -> Option<(&'a str, MaskChannel)> {
    let m = e.mask.as_ref().filter(|m| m.layer.is_none() && m.shape.is_none() && !m.invert)?;
    let whole = matches!(m.strength, Value::Fixed(v) if v >= 1.0);
    if d.mask_mode != MaskMode::Strength || !whole || m.feather > 0.0 {
        return None;
    }
    match m.texture.as_ref()? {
        Value::Fixed(id) => Some((id.as_str(), m.channel)),
        Value::Bound(_) => None,
    }
}

impl Scene {
    pub fn picture_paths(&self) -> Vec<&str> {
        let uses = || self.layers().flat_map(|l| &l.effects);
        let masks = uses().filter_map(|e| match e.mask.as_ref()?.texture.as_ref()? {
            Value::Fixed(id) => Some(id.as_str()),
            Value::Bound(_) => None,
        });
        let named = uses().flat_map(|e| e.params.values()).filter_map(|v| match v {
            Value::Fixed(crate::Param::Text(id)) => Some(id.as_str()),
            _ => None,
        });
        let ids: BTreeSet<&str> = masks.chain(named).collect();
        ids.into_iter().filter_map(|id| Some(self.assets.get(id)?.path.as_str())).collect()
    }

    pub fn inked_ids(&self) -> Vec<&str> {
        let uses = self.layers().flat_map(|l| &l.effects);
        let ids: BTreeSet<&str> = uses.flat_map(|e| textures(self, e)).collect();
        let clamped =
            |id: &&str| self.assets.get(*id).is_some_and(|a| a.wrap.unwrap_or_default() == Wrapping::Clamp);
        ids.into_iter().filter(clamped).collect()
    }

    pub fn fill_inked(&self, boxes: BTreeMap<String, Inked>) {
        let _ = self.inked.set(boxes);
    }

    pub fn inked_box(&self, id: &str) -> Option<Inked> {
        self.inked.get()?.get(id).copied()
    }

    pub fn read_inked(&self, files: &mut impl Files) {
        let mut found = BTreeMap::new();
        for id in self.inked_ids() {
            let Some(asset) = self.assets.get(id) else { continue };
            let Ok(bytes) = files.read(&asset.path, MAX_PICTURE) else { continue };
            let Ok(image) = image::load_from_memory(&bytes) else { continue };
            let image = image.to_rgba8();
            found.insert(id.to_string(), ink_bounds(image.width(), image.height(), image.as_raw()));
        }
        self.fill_inked(found);
    }

    pub fn box_share(&self, e: &EffectUse, d: &Descriptor) -> Option<f32> {
        let single = d.passes.len() == 1 && d.state.is_none() && d.targets.is_empty() && !d.below();
        if !single {
            return None;
        }
        let masked = fixed_mask(e, d).map(|(id, channel)| {
            let on: &[usize] = match channel {
                MaskChannel::R => &[0],
                MaskChannel::G => &[1],
                MaskChannel::B => &[2],
                MaskChannel::A => &[3],
                MaskChannel::Luminance => &[0, 1, 2],
            };
            self.inked_box(id).map(|i| span(on.iter().map(|c| i[*c])))
        });
        let limits: Vec<_> = d.bounded.iter().filter_map(|b| d.params.get(d.param(b)?)?.named(e)).collect();
        let bounded = (!limits.is_empty()).then(|| {
            let all: Option<Vec<Inked>> = limits.iter().map(|id| self.inked_box(id)).collect();
            all.map(|a| span(a.into_iter().flat_map(|i| [i[0], i[1], i[2]])))
        });
        let found = match (masked, bounded) {
            (None, None) => return None,
            (Some(a), None) | (None, Some(a)) => a?,
            (Some(a), Some(b)) => {
                a?.zip(b?).map(|(a, b)| [a[0].max(b[0]), a[1].max(b[1]), a[2].min(b[2]), a[3].min(b[3])])
            }
        };
        let [gx, gy] = self.grown(e, d);
        let area = found.map_or(0.0, |[x0, y0, x1, y1]| {
            let (w, h) = ((x1 - x0).max(0.0) + 2.0 * gx, (y1 - y0).max(0.0) + 2.0 * gy);
            w.min(1.0) * h.min(1.0)
        });
        Some(area)
    }

    fn grown(&self, e: &EffectUse, d: &Descriptor) -> [f32; 2] {
        let reach = d.reach.as_deref().and_then(|r| d.params.get(d.param(r)?));
        let by = reach.and_then(|p| match e.params.get(&p.id) {
            Some(Value::Fixed(crate::Param::Number(v))) => Some(*v),
            None => match &p.prefill {
                crate::Param::Number(v) => Some(*v),
                _ => None,
            },
            _ => None,
        });
        let [w, h] = self.canvas.size.map(|v| v.max(1) as f32);
        by.map_or([0.0; 2], |v| [v.abs() / w, v.abs() / h])
    }
}
