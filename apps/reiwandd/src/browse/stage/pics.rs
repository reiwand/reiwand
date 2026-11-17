use super::Stage;
use crate::compositor::Pic;
use crate::daemon::Daemon;
use crate::thumbs::Req;
use image::{AnimationDecoder, RgbaImage};
use reiwand_core::strip::CAPTION_RATIO;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;

const THUMB: u32 = 256;
const FRAMES: usize = 60;
const FRAME_WIDTH: u32 = 360;
const CAPTION: (u32, u32) = (1280, (1280.0 / CAPTION_RATIO) as u32);

fn key(text: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

fn pic(tag: &str, image: RgbaImage) -> Arc<Pic> {
    let size = image.dimensions();
    Arc::new(Pic { key: key(tag), size, rgba: image.into_raw() })
}

fn fitted(image: RgbaImage) -> RgbaImage {
    if image.width() <= FRAME_WIDTH {
        return image;
    }
    let height = (image.height() * FRAME_WIDTH / image.width()).max(1);
    image::imageops::thumbnail(&image, FRAME_WIDTH, height)
}

fn frames(path: &std::path::Path) -> Option<Vec<(Duration, Arc<Pic>)>> {
    let file = std::io::BufReader::new(std::fs::File::open(path).ok()?);
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    let all = match ext.as_str() {
        "gif" => image::codecs::gif::GifDecoder::new(file).ok()?.into_frames().collect_frames(),
        _ => image::codecs::webp::WebPDecoder::new(file).ok()?.into_frames().collect_frames(),
    };
    let taken: Vec<_> = all.ok()?.into_iter().take(FRAMES).collect();
    let found: Vec<_> = taken
        .into_iter()
        .enumerate()
        .map(|(i, f)| {
            let delay = Duration::from(f.delay()).max(Duration::from_millis(30));
            (delay, pic(&format!("{}#{i}", path.display()), fitted(f.into_buffer())))
        })
        .collect();
    (found.len() > 1).then_some(found)
}

fn bar(text: &str) -> Option<RgbaImage> {
    let mut fonts = reiwand_type::Fonts::from_faces([]);
    let chain = fonts.chain(None, &[]);
    let layout = reiwand_scene::Layout::default();
    let ink = reiwand_type::raster(&chain, text, 44.0, reiwand_scene::Align::Center, 1.0, &layout)?;
    let ink = if ink.width() > CAPTION.0 - 40 {
        let height = (ink.height() * (CAPTION.0 - 40) / ink.width()).max(1);
        image::imageops::resize(&ink, CAPTION.0 - 40, height, image::imageops::FilterType::Triangle)
    } else {
        ink
    };
    let mut out = RgbaImage::from_pixel(CAPTION.0, CAPTION.1, image::Rgba([14, 16, 22, 255]));
    let (x, y) = ((CAPTION.0 - ink.width()) / 2, CAPTION.1.saturating_sub(ink.height()) / 2);
    for (px, py, p) in ink.enumerate_pixels() {
        let (a, ox, oy) = (u32::from(p.0[3]), x + px, y + py);
        if let Some(o) = out.get_pixel_mut_checked(ox, oy) {
            let mix = |c: u8| ((u32::from(c) * (255 - a) + 240 * a) / 255) as u8;
            *o = image::Rgba([mix(o.0[0]), mix(o.0[1]), mix(o.0[2]), 255]);
        }
    }
    Some(out)
}

impl Daemon {
    pub(super) fn still(self: &Arc<Self>, stage: &mut Stage, output: &str, uid: &str) -> Option<Arc<Pic>> {
        if let Some(found) = stage.pics.get(uid) {
            return Some(found.clone());
        }
        let item = self.store.read(|lib| lib.item(uid)).ok()?;
        let Some(path) = self.thumbs.cached(&item, THUMB) else {
            if !item.missing && stage.waiting.insert(uid.to_owned()) {
                self.wait_thumb(item, output);
            }
            return None;
        };
        let decoded = image::open(path).ok()?.to_rgba8();
        let found = pic(uid, fitted(decoded));
        stage.pics.insert(uid.to_owned(), found.clone());
        stage.waiting.remove(uid);
        Some(found)
    }

    fn wait_thumb(self: &Arc<Self>, item: reiwand_library::Item, output: &str) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.thumbs.queue(item, Req::Wait(THUMB, tx));
        let (daemon, output) = (Arc::downgrade(self), output.to_owned());
        let Some(link) = super::link() else { return };
        link.runtime.spawn(async move {
            let ready = tokio::time::timeout(Duration::from_secs(20), rx).await;
            if let (Ok(Ok(Ok(_))), Some(daemon)) = (ready, daemon.upgrade()) {
                daemon
                    .run(move |d| {
                        d.stage_sync(&output);
                        Ok::<_, crate::error::Error>(())
                    })
                    .await
                    .ok();
            }
        });
    }

    pub(super) fn motion(&self, stage: &mut Stage, uid: &str) -> Vec<(Duration, Arc<Pic>)> {
        if let Some((was, found)) = &stage.motion
            && was == uid
        {
            return found.clone();
        }
        let made = self.moving(uid).unwrap_or_default();
        stage.motion = Some((uid.to_owned(), made.clone()));
        made
    }

    fn moving(&self, uid: &str) -> Option<Vec<(Duration, Arc<Pic>)>> {
        let item = self.store.read(|lib| lib.item(uid)).ok()?;
        let manifest = item.package.as_ref().and_then(|_| reiwand_format::peek(&item.path).ok())?;
        frames(std::path::Path::new(&self.animated(&item, &manifest)?))
    }

    pub(super) fn caption(&self, stage: &mut Stage, uid: &str) -> Option<Arc<Pic>> {
        if let Some((was, found)) = &stage.caption
            && was == uid
        {
            return Some(found.clone());
        }
        let title = self.item_summary(uid)?.title;
        let found = pic(&format!("caption:{title}"), bar(&title)?);
        stage.caption = Some((uid.to_owned(), found.clone()));
        Some(found)
    }
}
