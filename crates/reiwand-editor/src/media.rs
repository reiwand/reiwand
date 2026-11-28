use anyhow::{Result, ensure};
use serde_json::Value;
use std::path::Path;

pub fn simulated(path: &Path) -> Result<Value> {
    let bytes = std::fs::read(path).map_err(|e| anyhow::anyhow!("cannot read {}: {e}", path.display()))?;
    ensure!(bytes.len() <= 64 << 10, "{} is larger than 64 KiB", path.display());
    let value: Value = serde_json::from_slice(&bytes)?;
    ensure!(value.is_object(), "{} holds a JSON object of the made-up track", path.display());
    Ok(value)
}

pub struct Played {
    pub state: reiwand_scene::MediaState,
    pub cover: Option<image::RgbaImage>,
}

fn smaller(image: image::RgbaImage) -> image::RgbaImage {
    let side = reiwand_scene::COVER_SIDE;
    let (w, h) = image.dimensions();
    if w <= side && h <= side {
        return image;
    }
    let scale = f64::from(side) / f64::from(w.max(h));
    let to = |v: u32| ((f64::from(v) * scale).round() as u32).clamp(1, side);
    image::imageops::resize(&image, to(w), to(h), image::imageops::FilterType::Triangle)
}

pub fn played(path: &Path, folder: &Path) -> Result<Played> {
    played_from(simulated(path)?, folder)
}

pub fn played_from(media: Value, folder: &Path) -> Result<Played> {
    let track = reiwand_scene::MadeTrack::read(media).map_err(|e| anyhow::anyhow!(e))?;
    let cover = match track.cover.as_deref().filter(|c| !c.is_empty()) {
        None => None,
        Some(rel) => {
            ensure!(
                Path::new(rel).components().all(|c| matches!(c, std::path::Component::Normal(_))),
                "media.cover is a picture inside the scene folder"
            );
            let file = folder.join(rel);
            let image = image::open(&file)
                .map_err(|e| anyhow::anyhow!("cannot read the cover {}: {e}", file.display()))?;
            Some(smaller(image.into_rgba8()))
        }
    };
    let colors = cover.as_ref().map(|c| {
        let small = image::imageops::resize(
            c,
            reiwand_scene::PALETTE_SIDE,
            reiwand_scene::PALETTE_SIDE,
            image::imageops::FilterType::Triangle,
        );
        (1, reiwand_scene::palette(small.as_raw()).unwrap_or(reiwand_scene::FALLBACK))
    });
    Ok(Played { state: track.state(colors).map_err(|e| anyhow::anyhow!(e))?, cover })
}

pub fn dress(
    assets: &mut reiwand_render::SceneAssets,
    scene: &reiwand_scene::Scene,
    cover: &image::RgbaImage,
) {
    for (id, asset) in &scene.assets {
        if asset.media == Some(reiwand_scene::Cover::Current) {
            assets.images.insert(id.clone(), reiwand_render::Pixels::new(cover.clone()));
        }
    }
}

pub fn sounded(path: &Path, (start, end): (f64, f64)) -> Result<std::sync::Arc<reiwand_render::Track>> {
    ensure!(path.is_file(), "{} is not a sound file", path.display());
    let span = reiwand_render::Track::span(start, end);
    let (first, clip) = reiwand_video::listen(path, span)
        .map_err(|e| anyhow::anyhow!("cannot read the sound {}: {e}", path.display()))?;
    let track = reiwand_render::Track::new(reiwand_video::RATE, first, clip.channels, &clip.samples);
    Ok(std::sync::Arc::new(track))
}
