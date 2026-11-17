use reiwand_core::Fit;
use reiwand_core::strip::{Plan, covering, filling, place};
use reiwand_render::{GpuImage, Layer, RenderError, Renderer};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

const SLIDE: Duration = Duration::from_millis(170);
const FRAME: [u8; 3] = [236, 238, 244];

pub struct Pic {
    pub key: u64,
    pub size: (u32, u32),
    pub rgba: Vec<u8>,
}

pub struct StripSpec {
    pub cards: Vec<(i32, Arc<Pic>)>,
    pub motion: Vec<(Duration, Arc<Pic>)>,
    pub caption: Option<Arc<Pic>>,
    pub before: usize,
    pub after: usize,
    pub moved: i32,
    pub smooth: bool,
}

pub(super) struct Strip {
    spec: Arc<StripSpec>,
    images: HashMap<u64, GpuImage>,
    from: f32,
    began: Option<Instant>,
    clock: Instant,
    pub(super) due: Option<Instant>,
    pub(super) changed: bool,
}

fn ease(t: f32) -> f32 {
    let t = 1.0 - t.clamp(0.0, 1.0);
    t * t * t
}

impl Strip {
    pub(super) fn new(
        renderer: &Renderer,
        spec: Arc<StripSpec>,
        before: Option<Strip>,
        now: Instant,
    ) -> Result<Strip, RenderError> {
        let (mut images, slide, clock) = match before {
            Some(old) => {
                let slide = old.slide(now);
                (old.images, slide, old.clock)
            }
            None => (HashMap::new(), 0.0, now),
        };
        let wanted: Vec<&Arc<Pic>> = (spec.cards.iter().map(|(_, p)| p))
            .chain(spec.motion.iter().map(|(_, p)| p))
            .chain(spec.caption.iter())
            .collect();
        images.retain(|key, _| wanted.iter().any(|p| p.key == *key));
        for pic in wanted {
            if let std::collections::hash_map::Entry::Vacant(slot) = images.entry(pic.key) {
                slot.insert(renderer.upload_rgba(&pic.rgba, pic.size.0, pic.size.1, pic.size.0 * 4)?);
            }
        }
        let from = if spec.smooth { (slide + spec.moved as f32).clamp(-2.0, 2.0) } else { 0.0 };
        let began = (from != 0.0).then_some(now);
        Ok(Strip { spec, images, from, began, clock, due: Some(now), changed: true })
    }

    fn slide(&self, now: Instant) -> f32 {
        let Some(began) = self.began else { return 0.0 };
        let t = now.saturating_duration_since(began).as_secs_f32() / SLIDE.as_secs_f32();
        self.from * ease(t)
    }

    fn picture(&self) -> Option<&Arc<Pic>> {
        let total: Duration = self.spec.motion.iter().map(|(d, _)| *d).sum();
        if total.is_zero() {
            return None;
        }
        let nanos = self.clock.elapsed().as_nanos() % total.as_nanos().max(1);
        let mut at = Duration::from_nanos(nanos as u64);
        for (length, pic) in &self.spec.motion {
            if at < *length {
                return Some(pic);
            }
            at -= *length;
        }
        self.spec.motion.last().map(|(_, pic)| pic)
    }

    pub(super) fn advance(&mut self, now: Instant, refresh: u32) {
        self.changed = true;
        let sliding = self.began.is_some_and(|b| now.saturating_duration_since(b) < SLIDE);
        if !sliding {
            self.began = None;
        }
        let step = Duration::from_secs(1) / refresh.clamp(30, 120);
        let still = self.spec.motion.len() < 2;
        self.due = match (sliding, still) {
            (true, _) => Some(now + step),
            (false, false) => Some(now + step.max(Duration::from_millis(40))),
            (false, true) => None,
        };
    }

    pub(super) fn plan(&self, size: (u32, u32), now: Instant) -> Plan {
        place(size, self.spec.before, self.spec.after, self.slide(now))
    }

    pub(super) fn layers<'a>(&'a self, size: (u32, u32), now: Instant, out: &mut Vec<Layer<'a>>) {
        let plan = self.plan(size, now);
        let flat = |image, uv, clip, background, opacity| Layer {
            image,
            fit: Fit::Stretch,
            background,
            opacity,
            uv,
            relief: None,
            flip: [false; 2],
            grade: None,
            clip,
        };
        out.push(flat(None, None, None, [0, 0, 0], 0.55));
        let live = self.picture();
        for slot in plan.shown() {
            let pic = match (slot.offset, live) {
                (0, Some(live)) => Some(live),
                _ => self.spec.cards.iter().find(|(o, _)| *o == slot.offset).map(|(_, p)| p),
            };
            let Some((pic, image)) = pic.and_then(|p| Some((p, self.images.get(&p.key)?))) else { continue };
            if slot.offset == 0 {
                out.push(flat(None, None, plan.frame.seen(size), FRAME, 1.0));
            }
            let uv = covering(pic.size, slot.rect, size, 1.0);
            out.push(flat(Some(image), Some(uv), slot.rect.seen(size), [0, 0, 0], 1.0));
        }
        if let Some(image) = self.spec.caption.as_ref().and_then(|p| self.images.get(&p.key)) {
            let rect = plan.caption;
            out.push(flat(Some(image), Some(filling(rect, size)), rect.seen(size), [0, 0, 0], 1.0));
        }
    }
}

impl super::state::State {
    pub(super) fn strip(&mut self, name: &str, spec: Option<Arc<StripSpec>>) {
        let renderer = self.renderer;
        let Some(output) = self.outputs.get_mut(name) else { return };
        let before = output.strip.take();
        output.strip = spec.and_then(|spec| {
            Strip::new(renderer, spec, before, Instant::now())
                .inspect_err(|e| tracing::warn!(output = name, "cannot draw the wallpaper strip: {e}"))
                .ok()
        });
        (output.dirty, output.stale, output.composed) = (true, true, None);
    }
}
