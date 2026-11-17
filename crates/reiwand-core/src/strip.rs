pub const REACH: usize = 3;
pub const SLOTS: usize = 2 * REACH + 3;
const MIDDLE: f32 = 0.30;
const SIDE: f32 = 0.62;
const WIDEST: f32 = 0.34;
const WIDE: f32 = 16.0 / 9.0;
const NOTCH_PIXELS: f32 = 40.0;
pub const CAPTION_RATIO: f32 = 16.0;
const FAR: f32 = REACH as f32 - 0.2;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.width && y < self.y + self.height
    }

    pub fn grown(self, by: f32) -> Rect {
        Rect { x: self.x - by, y: self.y - by, width: self.width + 2.0 * by, height: self.height + 2.0 * by }
    }

    pub fn seen(self, (width, height): (u32, u32)) -> Option<crate::Region> {
        let (x0, y0) = (self.x.max(0.0), self.y.max(0.0));
        let (x1, y1) = ((self.x + self.width).min(width as f32), (self.y + self.height).min(height as f32));
        (x1 > x0 && y1 > y0).then(|| crate::Region {
            x: x0 as u32,
            y: y0 as u32,
            width: (x1.ceil() - x0.floor()) as u32,
            height: (y1.ceil() - y0.floor()) as u32,
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Slot {
    pub offset: i32,
    pub rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plan {
    pub slots: [Slot; SLOTS],
    pub len: usize,
    pub frame: Rect,
    pub caption: Rect,
}

impl Plan {
    pub fn shown(&self) -> &[Slot] {
        &self.slots[..self.len]
    }

    pub fn hit(&self, x: f32, y: f32) -> Option<i32> {
        let middle = self.shown().iter().find(|s| s.offset == 0).filter(|s| s.rect.contains(x, y));
        let side = || self.shown().iter().find(|s| s.rect.contains(x, y));
        middle.or_else(side).map(|s| s.offset)
    }

    pub fn inside(&self, x: f32, y: f32) -> bool {
        let first = self.shown().iter().map(|s| s.rect.x).fold(f32::MAX, f32::min);
        let last = self.shown().iter().map(|s| s.rect.x + s.rect.width).fold(0.0, f32::max);
        let band = Rect {
            x: first,
            y: self.frame.y,
            width: last - first,
            height: self.caption.y + self.caption.height - self.frame.y,
        };
        band.contains(x, y)
    }
}

fn spot(p: f32, middle: f32, gap: f32) -> (f32, f32) {
    let a = p.abs().min(FAR + 1.0);
    let scale = 1.0 + (SIDE - 1.0) * a.min(1.0);
    let first = middle / 2.0 + gap + middle * SIDE / 2.0;
    let along = if a <= 1.0 { a * first } else { first + (a - 1.0) * (middle * SIDE + gap) };
    (along.copysign(p), scale)
}

pub fn place(size: (u32, u32), before: usize, after: usize, slide: f32) -> Plan {
    let (w, h) = (size.0 as f32, size.1 as f32);
    let width = (h * MIDDLE * WIDE).min(w * WIDEST);
    let height = width / WIDE;
    let (cx, cy) = (w / 2.0, h * 0.46);
    let gap = width * 0.12;
    let mut slots = [Slot::default(); SLOTS];
    let mut len = 0;
    let (low, high) = (-(before.min(REACH + 1) as i32), after.min(REACH + 1) as i32);
    for offset in low..=high {
        let p = offset as f32 + slide;
        if p.abs() > FAR {
            continue;
        }
        let (along, scale) = spot(p, width, gap);
        let (cw, ch) = (width * scale, height * scale);
        let rect = Rect { x: cx + along - cw / 2.0, y: cy - ch / 2.0, width: cw, height: ch };
        if rect.x + rect.width > 0.0 && rect.x < w && len < SLOTS {
            slots[len] = Slot { offset, rect };
            len += 1;
        }
    }
    slots[..len].sort_by(|a, b| {
        let far = |s: &Slot| (s.offset as f32 + slide).abs();
        far(b).total_cmp(&far(a))
    });
    let border = (height * 0.03).max(3.0);
    let frame = Rect { x: cx - width / 2.0, y: cy - height / 2.0, width, height }.grown(border);
    let tall = width * 1.5 / CAPTION_RATIO;
    let caption = Rect {
        x: cx - width * 0.75,
        y: frame.y + frame.height + tall * 0.4,
        width: width * 1.5,
        height: tall,
    };
    Plan { slots, len, frame, caption }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Wheel {
    rest: f32,
}

impl Wheel {
    pub fn push(&mut self, notches: f32) -> i32 {
        if notches * self.rest < 0.0 {
            self.rest = 0.0;
        }
        self.rest += notches;
        let whole = self.rest.trunc();
        self.rest -= whole;
        whole as i32
    }

    pub fn reset(&mut self) {
        self.rest = 0.0;
    }
}

pub fn notches(discrete: i32, pixels: f64) -> f32 {
    if discrete != 0 { discrete as f32 / 120.0 } else { pixels as f32 / NOTCH_PIXELS }
}

pub fn filling(rect: Rect, size: (u32, u32)) -> crate::UvRect {
    let (extent, whole) = ([rect.width.max(1.0), rect.height.max(1.0)], [size.0 as f32, size.1 as f32]);
    crate::UvRect {
        offset: [-rect.x / extent[0], -rect.y / extent[1]],
        scale: [whole[0] / extent[0], whole[1] / extent[1]],
        repeat: false,
    }
}

pub fn covering(image: (u32, u32), rect: Rect, size: (u32, u32), zoom: f32) -> crate::UvRect {
    let ratio = |w: f32, h: f32| w / h.max(1.0);
    let (ia, ra) = (ratio(image.0 as f32, image.1 as f32), ratio(rect.width, rect.height));
    let (mut fx, mut fy) = if ia > ra { (ra / ia, 1.0) } else { (1.0, ia / ra) };
    (fx, fy) = (fx / zoom.max(1.0), fy / zoom.max(1.0));
    let (start, span) = ([(1.0 - fx) / 2.0, (1.0 - fy) / 2.0], [fx, fy]);
    let (origin, extent, whole) =
        ([rect.x, rect.y], [rect.width.max(1.0), rect.height.max(1.0)], [size.0 as f32, size.1 as f32]);
    crate::UvRect {
        offset: [0, 1].map(|i| start[i] - span[i] * origin[i] / extent[i]),
        scale: [0, 1].map(|i| span[i] * whole[i] / extent[i]),
        repeat: false,
    }
}

#[cfg(test)]
mod tests;
