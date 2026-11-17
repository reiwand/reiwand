use crate::Facts;
use crate::build::collect;
use crate::contents::{binaries, preset_files, scripts, sounds};
use crate::features::facts;
use crate::locale::{LOCALE, MAX_LOCALE_FILE};
use crate::{Error, Kind, Limits, MANIFEST, Manifest, Result, SIGNATURE, Scene, media, validate};
use reiwand_core::message::said;
use reiwand_scene::Files;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn inspect(source: impl AsRef<Path>) -> Result<(Manifest, Facts)> {
    let source = source.as_ref();
    let raw = fs::read(source.join(MANIFEST))?;
    let manifest = with_strings(source, Manifest::from_json(&raw)?)?;
    let facts = facts_of(source, &manifest)?;
    Ok((manifest, facts))
}

pub fn locale_names(source: &Path) -> Vec<String> {
    let files = fs::read_dir(source.join(LOCALE)).into_iter().flatten().flatten();
    let mut names: Vec<String> =
        files.map(|f| format!("{LOCALE}{}", f.file_name().to_string_lossy())).collect();
    names.sort();
    names
}

pub fn with_strings(source: &Path, mut manifest: Manifest) -> Result<Manifest> {
    manifest.read_strings(locale_names(source), |name| {
        crate::validate::within("file size", fs::metadata(source.join(name))?.len(), MAX_LOCALE_FILE)?;
        Ok(fs::read(source.join(name))?)
    })?;
    Ok(manifest)
}

pub struct Folder<'a>(pub &'a Path);

impl reiwand_scene::Files for Folder<'_> {
    type Error = Error;

    fn read(&mut self, path: &str, max: usize) -> Result<Vec<u8>> {
        self.head(path, max, usize::MAX).map(|(bytes, _)| bytes)
    }

    fn head(&mut self, path: &str, max: usize, keep: usize) -> Result<(Vec<u8>, u64)> {
        crate::check_path(path)?;
        let size = fs::metadata(self.0.join(path))?.len();
        crate::validate::within("scene file size", size, max as u64)?;
        let mut data = Vec::new();
        File::open(self.0.join(path))?.take(keep as u64).read_to_end(&mut data)?;
        Ok((data, size))
    }
}

fn facts_of(source: &Path, manifest: &Manifest) -> Result<Facts> {
    let mut files = Vec::new();
    collect(source, "", &mut files)?;
    validate(manifest, &files, &Limits::default())?;
    let entry = source.join(&manifest.entry);
    let text = match manifest.kind {
        Kind::Scene | Kind::Assets => Some(fs::read_to_string(&entry)?),
        _ => None,
    };
    let scene = match (manifest.kind, &text) {
        (Kind::Scene | Kind::Assets, Some(text)) => Some(Scene::load(text, &manifest.properties)),
        (Kind::Preset, _) => {
            let preset = crate::PresetFile::parse(&fs::read(&entry)?, manifest)?;
            preset_files(&preset, &files).map(|()| None)?
        }
        (Kind::Web, _) => {
            binaries(&files, |path| Folder(source).head(path, usize::MAX, 4).map(|r| r.0)).map(|()| None)?
        }
        _ => None,
    };
    let invalid =
        |e: reiwand_scene::SceneError| Error::Invalid { field: "scene".into(), reason: e.message().into() };
    let mut scene = scene.transpose().map_err(invalid)?;
    if let Some(scene) = scene.as_mut() {
        if manifest.kind == Kind::Assets {
            let size = |path: &str| files.iter().find(|f| f.path == path).map(|f| f.size);
            crate::parts::check(scene, size, |path| Ok(fs::read(source.join(path))?))?;
        }
        scripts(scene, &files)?;
        sounds(scene, &files, |path, edge| match edge {
            Some(edge) => Ok(media::ends(&source.join(path), edge)?),
            None => Ok((fs::read(source.join(path))?, Vec::new())),
        })?;
        scene.adopt(&manifest.properties, Folder(source)).map_err(invalid)?;
        scene.read_inked(&mut Folder(source));
    }
    let mut head = Vec::new();
    if manifest.kind == Kind::Image {
        File::open(&entry)?.take(1 << 16).read_to_end(&mut head)?;
    }
    let size = media::probe(&head).map(|m| [m.width, m.height]);
    let mut found = facts(manifest, scene.as_ref(), size);
    if let Some(scene) = scene.as_ref() {
        crate::weigh(&mut found, scene);
    }
    found.notes.extend(scene.iter().flat_map(|s| uneven(source, s)));
    found.notes.extend(manifest.unread_texts(locale_names(source)));
    for path in media::videos(manifest, scene.as_ref()) {
        let (head, tail) = media::ends(&source.join(&path), media::EDGE)?;
        if media::hdr(&head) || media::hdr(&tail) {
            media::mark(&mut found, &path);
        }
    }
    let sizes: Vec<(String, u64)> = files.iter().map(|f| (f.path.clone(), f.size)).collect();
    let peek = |path: &str| Folder(source).head(path, usize::MAX, crate::closer::HEAD as usize).ok();
    if let Some(note) = crate::closer::phone_note(&sizes, |path| peek(path).map(|r| r.0)) {
        found.facets.retain(|f| f != "android");
        found.notes.push(note);
    }
    Ok(found)
}

fn uneven(source: &Path, scene: &Scene) -> Vec<String> {
    let ratio = |id: &str| {
        let mut head = Vec::new();
        File::open(source.join(&scene.assets.get(id)?.path))
            .ok()?
            .take(1 << 16)
            .read_to_end(&mut head)
            .ok()?;
        media::probe(&head).map(|m| f64::from(m.width) / f64::from(m.height.max(1)))
    };
    let varied = |(place, ids): (String, Vec<&str>)| {
        let ratios: Vec<f64> = ids.into_iter().filter_map(ratio).collect();
        let (low, high) = ratios.iter().fold((f64::MAX, 0.0f64), |(a, b), r| (a.min(*r), b.max(*r)));
        (ratios.len() > 1 && high > low * 1.01)
            .then(|| format!("the images of {place} do not share one proportion, so some will be cropped"))
    };
    reiwand_scene::variants(scene).into_iter().filter_map(varied).collect()
}

pub fn fill(source: impl AsRef<Path>) -> Result<Vec<String>> {
    let source = source.as_ref();
    let (mut manifest, facts) = inspect(source)?;
    let listed = manifest.features();
    let added: Vec<String> = facts.features.into_iter().filter(|f| !listed.contains(f)).collect();
    if added.is_empty() {
        return Ok(Vec::new());
    }
    let names = format!("{added:?}");
    let reason = || {
        said(
            "format.unsigned_features",
            "misses %1; run pack sign again, which fills the list before signing",
            &[&names],
        )
    };
    crate::ensure(!source.join(SIGNATURE).exists(), || crate::refuse("requires.features", reason()))?;
    let features = &mut manifest.requires.get_or_insert_default().features;
    features.extend(added.iter().cloned());
    features.sort();
    let json = manifest.to_json()? + "\n";
    // The author's manifest stays whole, old or new, if the write is stopped midway.
    crate::store::place(&source.join(MANIFEST), false, |file| Ok(file.write_all(json.as_bytes())?))?;
    Ok(added)
}
