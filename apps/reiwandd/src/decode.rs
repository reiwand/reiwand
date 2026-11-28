use image::{DynamicImage, ImageReader, RgbaImage};
use reiwand_format::{Kind as PackageKind, Package, Platform, features};
use reiwand_library::Meta;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek, Write};
use std::path::Path;

const POSTER: u32 = 1024;

pub struct Picture {
    pub meta: Meta,
    pub image: Option<DynamicImage>,
    pub failure: Option<String>,
    pub undrawn: Option<String>,
}

pub enum Problem {
    Missing(String),
    Decode(String),
}

impl<E: std::error::Error + 'static> From<E> for Problem {
    fn from(e: E) -> Self {
        let text = e.to_string();
        let source = std::iter::successors(Some(&e as &(dyn std::error::Error + 'static)), |e| e.source());
        let io = source.filter_map(|e| e.downcast_ref::<std::io::Error>()).next().map(std::io::Error::kind);
        match io {
            Some(std::io::ErrorKind::NotFound | std::io::ErrorKind::PermissionDenied) => {
                Problem::Missing(text)
            }
            _ => Problem::Decode(text),
        }
    }
}

impl Picture {
    pub fn of(image: DynamicImage) -> Picture {
        let meta = Meta { width: Some(image.width()), height: Some(image.height()), ..Meta::default() };
        Picture { meta, image: Some(image), failure: None, undrawn: None }
    }
}

pub fn decode<R: BufRead + Seek>(reader: ImageReader<R>) -> Result<DynamicImage, Problem> {
    Ok(reiwand_render::decode_limited(reader, reiwand_render::picture_limits())?)
}

fn bytes(data: Vec<u8>) -> Result<DynamicImage, Problem> {
    decode(ImageReader::new(Cursor::new(data)))
}

pub fn video(path: &Path) -> Result<Picture, Problem> {
    let still = reiwand_video::still(path).map_err(|e| match e {
        reiwand_video::Error::NotLocal(_) => Problem::Missing(e.to_string()),
        e => Problem::Decode(e.to_string()),
    })?;
    let (duration_ms, audio) = (still.duration_ms, Some(still.audio));
    let frame = RgbaImage::from_raw(still.width, still.height, still.rgba).map(DynamicImage::ImageRgba8);
    let picture = Picture::of(frame.ok_or_else(|| Problem::Decode("the video frame is incomplete".into()))?);
    Ok(Picture { meta: Meta { duration_ms, audio, ..picture.meta }, ..picture })
}

pub fn open(path: &Path) -> reiwand_format::Result<Package<BufReader<File>>> {
    let package = reiwand_format::open(path, &reiwand_format::Limits::default())?;
    features::playable(&package.manifest, &package.facts, Platform::Desktop)?;
    Ok(package)
}

pub fn package(path: &Path, draw: u32, turn: Option<&Path>) -> Result<Picture, Problem> {
    let mut package: Package<_> = match open(path) {
        Err(e @ (reiwand_format::Error::Newer(_) | reiwand_format::Error::Elsewhere(_))) => {
            return Ok(newer(path, e.to_string()));
        }
        package => package?,
    };
    let manifest = package.manifest.clone();
    let preview = manifest.thumbnail().map(|p| bytes(package.read_file(p)?)).transpose()?;
    let entry = match manifest.kind {
        PackageKind::Scene => {
            let scene = package.scene()?;
            let [width, height] = scene.canvas.size.map(Some);
            let wanted = draw.min(scene.canvas.size[0].max(scene.canvas.size[1]));
            let (drawn, undrawn) = match preview {
                Some(image) if image.width().max(image.height()) >= wanted => (Ok(image), None),
                preview => {
                    let _turn = turn.and_then(|t| File::create(t).ok()).filter(|f| f.lock().is_ok());
                    match poster(&mut package, &scene, &manifest, draw.max(POSTER)) {
                        Ok(image) => (Ok(image), None),
                        Err(e) => (preview.ok_or_else(|| e.clone()), Some(e)),
                    }
                }
            };
            let (image, failure) = match drawn {
                Ok(image) => (Some(image), None),
                Err(e) => (None, Some(format!("no preview: {e}"))),
            };
            let meta = Meta { width, height, ..Meta::default() };
            return Ok(Picture { meta, image, failure, undrawn });
        }
        PackageKind::Web | PackageKind::Preset | PackageKind::Assets => {
            return Ok(Picture { meta: Meta::default(), image: preview, failure: None, undrawn: None });
        }
        PackageKind::Image => Picture::of(bytes(package.read_file(&manifest.entry)?)?),
        PackageKind::Video => {
            let extension = Path::new(&manifest.entry).extension().unwrap_or_default().to_string_lossy();
            let mut file = tempfile::Builder::new().suffix(&format!(".{extension}")).tempfile()?;
            package.copy_file(&manifest.entry, &mut file)?;
            file.flush()?;
            video(file.path())?
        }
    };
    Ok(Picture { image: preview.or(entry.image), ..entry })
}

fn newer(path: &Path, error: String) -> Picture {
    let image = reiwand_format::thumbnail(path).ok().flatten().and_then(|b| bytes(b).ok());
    Picture { meta: Meta { error: Some(error), ..Meta::default() }, image, failure: None, undrawn: None }
}

fn poster(
    package: &mut Package<BufReader<File>>,
    scene: &reiwand_scene::Scene,
    manifest: &reiwand_format::Manifest,
    edge: u32,
) -> Result<DynamicImage, String> {
    let read = |package: &mut Package<BufReader<File>>, path: &str| {
        package.read_asset(path).map_err(|e| e.to_string())
    };
    let assets = reiwand_render::SceneAssets::read(scene, |path| read(package, path))?;
    let assets = match assets.clone().with_scripts(scene, |path| read(package, path)) {
        Ok(scripted) => scripted,
        Err(_) => assets,
    };
    let props = reiwand_scene::effective(&manifest.properties, &[]);
    reiwand_render::Renderer::shared().map_err(|_| "no graphics device".to_owned()).and_then(|gpu| {
        let lang = manifest.scene_locale_in(scene, &reiwand_core::locale::languages());
        gpu.poster(scene, &assets, &props, (crate::zone::wall(), &lang), edge)
            .map(DynamicImage::ImageRgba8)
            .map_err(|e| e.to_string())
    })
}
