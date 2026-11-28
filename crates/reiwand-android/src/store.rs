use crate::{Error, library, media, pack, settings};
mod thumbs;

use image::DynamicImage;
use reiwand_format::Kind as PackKind;
use reiwand_library::{Item, Kind, Meta, qhash};
use reiwand_render::{decode_image, image_size};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use thumbs::describe;
pub(crate) use thumbs::{colors, thumb};
#[cfg(any(test, target_os = "android"))]
pub(crate) use thumbs::{poster, poster_source};

pub(crate) const MEDIA: &str = "media";
const THUMBS: &str = "thumbs";
const EDGE: u32 = 512;

static FILES: OnceLock<PathBuf> = OnceLock::new();
static GENERATION: AtomicU64 = AtomicU64::new(1);

pub(crate) fn prepare(files: &str) -> Result<(), Error> {
    if FILES.set(files.into()).is_err() {
        return Ok(());
    }
    let dir = dir()?;
    reiwand_core::roots::load_added(&dir.join("roots.der"));
    [MEDIA, THUMBS].iter().try_for_each(|name| fs::create_dir_all(dir.join(name)))?;
    [dir.to_path_buf(), dir.join(MEDIA)].iter().for_each(|folder| unfinished(folder));
    reiwand_scene::keep_strikes(&dir.join("stopped-effects"));
    settings::prepare()?;
    Ok(())
}

fn unfinished(folder: &Path) {
    let Ok(entries) = fs::read_dir(folder) else { return };
    for entry in entries.flatten() {
        let left = entry.file_name().to_str().is_some_and(|name| name.starts_with(".tmp"));
        if left && entry.file_type().is_ok_and(|kind| kind.is_file()) {
            drop(fs::remove_file(entry.path()));
        }
    }
}

pub(crate) fn dir() -> Result<&'static Path, Error> {
    FILES.get().map(PathBuf::as_path).ok_or(Error::Unprepared)
}

pub(crate) fn generation() -> u64 {
    GENERATION.load(Relaxed)
}

pub(crate) fn changed() {
    GENERATION.fetch_add(1, Relaxed);
}

pub(crate) fn write(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    write_from(path, |file| Ok(file.write_all(bytes)?))
}

/// Writes a file as it comes, so a large video or sound of a package never sits whole in memory.
pub(crate) fn write_from(
    path: &Path,
    fill: impl FnOnce(&mut std::fs::File) -> Result<(), Error>,
) -> Result<(), Error> {
    let mut file = tempfile::NamedTempFile::new_in(dir()?)?;
    fill(file.as_file_mut())?;
    file.as_file().sync_all()?;
    Ok(file.persist(path).map(drop).map_err(|e| e.error)?)
}

pub(crate) fn thumb_file(uid: &str, extension: &str) -> Result<PathBuf, Error> {
    Ok(dir()?.join(THUMBS).join(format!("{uid}.{extension}")))
}

pub(crate) fn store(source: &mut impl Read, name: &str) -> Result<(String, Kind), Error> {
    let media = dir()?.join(MEDIA);
    let mut copy = tempfile::NamedTempFile::new_in(&media)?;
    std::io::copy(source, &mut copy)?;
    let unreadable = || Error::Unreadable(name.into());
    let extension = media::extension(copy.path()).ok_or_else(unreadable)?;
    let kind = match extension {
        "wpak" => Kind::Package,
        _ if media::is_video(copy.path()) => Kind::Video,
        _ => Kind::Image,
    };
    let (small, size) = match kind {
        Kind::Video => (None, None),
        Kind::Package => pack::check(copy.path())?,
        Kind::Image => {
            let small = decode_image(copy.path(), EDGE).map_err(|_| unreadable())?;
            (Some(small), image_size(copy.path()).ok())
        }
    };
    copy.as_file().sync_all()?;
    let hash: String = qhash(copy.path())?[..8].iter().map(|b| format!("{b:02x}")).collect();
    let path = media.join(format!("{hash}.{extension}"));
    let title =
        Path::new(name).file_stem().and_then(|stem| stem.to_str()).unwrap_or_default().trim().to_owned();
    let stored = library::with(|lib| {
        let known = path.to_str().filter(|_| path.is_file()).and_then(|text| lib.resolve_item(text).ok());
        let uid = match known {
            Some(uid) => uid,
            None => {
                copy.persist(&path).map_err(|e| e.error)?;
                let uid = lib.import_file(&path, library::now())?;
                if !title.is_empty() && kind != Kind::Package {
                    let _ = lib.set_title(&uid, &title);
                }
                match small {
                    Some(small) => describe(lib, &uid, DynamicImage::ImageRgba8(small), size, None)?,
                    None if kind == Kind::Package => {
                        let (width, height) = (size.map(|s| s.0), size.map(|s| s.1));
                        lib.set_meta(&uid, &Meta { width, height, ..Meta::default() })?;
                    }
                    None => {}
                }
                uid
            }
        };
        Ok((uid, kind))
    })?;
    let shown = |m: reiwand_format::Manifest| matches!(m.kind, PackKind::Image | PackKind::Video);
    let inner = kind == Kind::Package && reiwand_format::peek(&path).is_ok_and(shown);
    if inner && let Err(e) = pack::entry(&path) {
        log::warn!("{e}");
    }
    Ok(stored)
}

pub(crate) fn forget(item: &Item) -> Result<(), Error> {
    crate::depth::forget(&item.path)?;
    if item.kind == Kind::Package {
        let inner = item.package.as_ref().is_some_and(|p| p.kind == "image");
        if let Some((file, _)) = inner.then(|| pack::media(item).ok()).flatten() {
            crate::depth::forget(&file)?;
        }
        drop(fs::remove_dir_all(pack::folder(&item.path)?));
    }
    let owned = item.path.starts_with(fs::canonicalize(dir()?.join(MEDIA))?).then(|| item.path.clone());
    let animated = ["animated.gif", "animated.webp"].map(|ext| thumb_file(&item.uid, ext));
    let files = [thumb_file(&item.uid, "jpg")?, thumb_file(&item.uid, "colors")?]
        .into_iter()
        .chain(animated.into_iter().flatten());
    for path in files.chain(owned) {
        drop(fs::remove_file(path));
    }
    Ok(())
}
