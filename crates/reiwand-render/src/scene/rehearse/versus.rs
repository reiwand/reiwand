use super::{Opened, Reel, Rehearsal, Versus};
use crate::{RenderError, Renderer};
use image::RgbaImage;

pub fn split(left: &RgbaImage, right: &RgbaImage, at: f32) -> RgbaImage {
    let (w, h) = left.dimensions();
    let cut = (w as f32 * at.clamp(0.0, 1.0)).round() as u32;
    RgbaImage::from_fn(w, h, |x, y| match (x < cut, right.get_pixel_checked(x, y)) {
        (false, Some(p)) => *p,
        _ => *left.get_pixel(x, y),
    })
}

impl Renderer {
    pub fn rehearse_versus<E: From<RenderError>>(
        &self,
        open: impl Fn(bool) -> Result<Opened, E>,
        wanted: &Rehearsal,
        versus: Option<&Versus>,
    ) -> Result<Reel, E> {
        let draw = |wanted: &Rehearsal, bloom: bool| -> Result<Reel, E> {
            let (scene, assets, props) = open(bloom)?;
            Ok(self.rehearse(&scene, &assets, &props, wanted)?)
        };
        let mut reel = draw(wanted, true)?;
        if let Some(v) = versus {
            let other = Rehearsal { quality: v.quality.unwrap_or(wanted.quality), ..wanted.clone() };
            let rival = draw(&other, !v.without_bloom)?;
            let joined = reel.frames.iter().zip(&rival.frames).map(|(a, b)| split(a, b, v.split));
            reel.frames = joined.collect();
        }
        Ok(reel)
    }
}
