use super::{STOP, places};
use crate::{Error, said};
use anyhow::Context;
use image::{GrayImage, RgbImage};
use reiwand_core::Fill;
use reiwand_core::edit::Edit;
use reiwand_depth::{Device, Inpainter, Settings, threads};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

pub(crate) const PHONE_EDGE: u32 = reiwand_depth::EDGE;

pub(crate) fn chosen(own: &reiwand_core::edit::Parallax) -> Fill {
    own.fill.unwrap_or(Fill::Simple)
}

fn settings_of(edit: &Edit) -> Option<Settings> {
    let own = edit.moving()?;
    let fill = places().ok()?.fill(chosen(own));
    let shift = crate::tilt::wanted_shift(own.strength, crate::tilt::visible());
    let edge = reiwand_depth::edge_for(PHONE_EDGE);
    Some(Settings { fill, count: usize::from(own.layers()), shift, edge })
}

pub(crate) fn still(image: &Path, edit: &Edit) -> Option<reiwand_core::travel::Still> {
    let places = places().ok()?;
    let fallback = || Settings {
        fill: reiwand_core::Fill::Simple,
        count: usize::from(reiwand_core::STANDARD_LAYERS),
        shift: 0.0,
        edge: PHONE_EDGE,
    };
    let set = settings_of(edit).unwrap_or_else(fallback);
    let found = path(image, edit);
    let built = found.as_deref().and_then(|p| reiwand_depth::load(p).ok().flatten());
    match (places.still((image, &edit.source.sha256), edit, &set), built) {
        (Some(reiwand_core::travel::Still::Building), Some(set))
            if set.info.reach.moves(set.info.size[0]) =>
        {
            None
        }
        (Some(reiwand_core::travel::Still::Building), Some(_)) => Some(reiwand_core::travel::Still::Flat),
        (still, _) => still,
    }
}

pub(crate) fn shown(own: &reiwand_core::edit::Parallax) -> Value {
    let wanted = chosen(own);
    let used = places().map_or(Fill::Simple, |places| places.fill(wanted));
    json!({ "chosen": wanted.as_str(), "used": used.as_str(), "count": own.layers() })
}

pub(crate) fn path(image: &Path, edit: &Edit) -> Option<PathBuf> {
    let (places, set) = (places().ok()?, settings_of(edit)?);
    let (depth, sha) = (edit.depth.clone().unwrap_or_default(), &edit.source.sha256);
    let exact = places.layer_set(image, sha, &depth, &set).filter(|p| p.is_file());
    let newest = || {
        places.layers_of(sha).into_iter().max_by_key(|p| std::fs::metadata(p).and_then(|m| m.modified()).ok())
    };
    exact.or_else(newest)
}

pub(crate) fn build(
    image: &Path,
    edit: &Edit,
    library: Option<&Path>,
    device: Device,
) -> Result<PathBuf, Error> {
    let places = places()?;
    let off =
        || said("android.layers_still", "The picture does not move: turn its depth movement on first", &[]);
    let set = settings_of(edit).ok_or_else(off)?;
    let threads = (threads() / 2).max(1);
    let mut loaded =
        (set.fill == Fill::Lama).then(|| Inpainter::open(&places, library, threads, device)).transpose()?;
    let mut paint =
        |image: &RgbImage, mask: &GrayImage| loaded.as_mut().context("LaMa is not loaded")?.run(image, mask);
    let inpaint: Option<&mut reiwand_depth::Inpaint<'_>> = (set.fill == Fill::Lama).then_some(&mut paint);
    let depth = edit.depth.clone().unwrap_or_default();
    let made =
        places.build_layers((image, &edit.source.sha256), (&depth, &set), inpaint, &mut |_| {}, &STOP)?;
    Ok(made)
}

pub(crate) fn best(image: &Path, library: Option<&Path>, device: Device) -> Value {
    let built = super::edited(image).and_then(|edit| build(image, &edit, library, device));
    built.inspect_err(|e| log::warn!("cannot build the depth layers: {e:?}")).ok();
    Value::Null
}
