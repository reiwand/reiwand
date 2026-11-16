use crate::{Device, Inpaint, Inpainter, Places, Settings, background, load, threads};
use anyhow::{Context, Result};
use image::{GrayImage, RgbImage};
use reiwand_core::Fill;
use reiwand_core::edit::{Depth, Edit};
use reiwand_core::message::said;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

pub fn settings(places: &Places, edit: &Edit, fill: Option<Fill>) -> Settings {
    sized(places, edit, fill, crate::EDGE)
}

pub fn sized(places: &Places, edit: &Edit, fill: Option<Fill>, edge: u32) -> Settings {
    let own = edit.parallax.clone().unwrap_or_default();
    let (fill, count) = (places.fill(fill.unwrap_or_else(|| own.fill())), usize::from(own.layers()));
    let shift = own.strength;
    Settings { fill, count, shift, edge: crate::edge_for(edge) }
}

impl Places {
    pub fn ensure_layers(
        &self,
        (image, sha): (&Path, &str),
        (depth, set): (&Depth, &Settings),
        (device, force): (Device, bool),
        progress: &mut dyn FnMut(u64),
        stop: &AtomicBool,
    ) -> Result<PathBuf> {
        let dest = self.layer_set(image, sha, depth, set);
        let dest = dest.context(
            said(
                "depth.the_image_has_no_depth",
                "the image has no depth map; generate or import one first",
                &[],
            )
            .local(),
        )?;
        if dest.is_file() && !force && load(&dest).is_ok_and(|l| l.is_some()) {
            return Ok(dest);
        }
        background();
        let mut lama =
            (set.fill == Fill::Lama).then(|| Inpainter::open(self, None, threads(), device)).transpose()?;
        let mut paint = |image: &RgbImage, mask: &GrayImage| {
            lama.as_mut()
                .context(said("depth.lama_is_not_loaded", "LaMa is not loaded", &[]).local())?
                .run(image, mask)
        };
        let inpaint: Option<&mut Inpaint<'_>> = (set.fill == Fill::Lama).then_some(&mut paint);
        self.build_layers((image, sha), (depth, set), inpaint, progress, stop)
    }
}
