//! `scene render` of a web package: the page runs in the web helper on a frame clock and each frame is drawn
//! at its own time, without a window.
use crate::told::{fill, told, tr};
use anyhow::{Context, Result, bail};
use reiwand_web::{Allowed, Isolation, Settings, Start, ToWeb};
use serde_json::{Map, Value, json};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const LOAD: Duration = Duration::from_secs(30);
const QUIET: Duration = Duration::from_millis(100);

pub struct Asked {
    pub out: PathBuf,
    pub time: f64,
    pub wall: f64,
    pub frames: u32,
    pub fps: u32,
    pub size: (u32, u32),
    pub pointer: Option<([f32; 2], [f32; 2], bool)>,
    pub language: Option<String>,
    pub quality: reiwand_scene::Quality,
}

/// The web package at `path`: a built package or a folder with the manifest of one; `None` for anything else.
pub fn package(path: &Path) -> Result<Option<PathBuf>> {
    if path.is_file() {
        let package = reiwand_format::open(path, &reiwand_format::Limits::default())?;
        return Ok((package.manifest.kind == reiwand_format::Kind::Web).then(|| path.to_owned()));
    }
    let manifest: Value = match std::fs::read(path.join(reiwand_format::MANIFEST)) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
        Err(_) => return Ok(None),
    };
    if manifest["kind"] != "web" {
        return Ok(None);
    }
    let out = std::env::temp_dir().join(format!("reiwand-render-{}.wpak", std::process::id()));
    reiwand_format::build(path, &out, &reiwand_format::Limits::default())?;
    Ok(Some(out))
}

fn helper() -> Option<PathBuf> {
    let given = std::env::var_os("REIWAND_WEB").map(PathBuf::from);
    let beside = std::env::current_exe().ok().and_then(|exe| Some(exe.parent()?.join("reiwand-web")));
    let path = std::env::var_os("PATH").unwrap_or_default();
    let searched = std::env::split_paths(&path).map(|dir| dir.join("reiwand-web"));
    given.into_iter().chain(beside).chain(searched).find(|p| p.is_file())
}

fn start(wpak: &Path, asked: &Asked) -> Result<Start> {
    let m = reiwand_format::open(wpak, &reiwand_format::Limits::default())?.manifest.localized();
    let tags = asked.language.clone().map_or_else(reiwand_core::locale::languages, |l| vec![l]);
    let languages = reiwand_web::languages(&tags, false);
    let values = reiwand_scene::effective(&m.properties, &[]);
    let mut properties: Map<String, Value> = Map::new();
    for (id, value) in reiwand_scene::Leveling::of(&m.properties).tuned(&values, asked.quality) {
        let hidden = reiwand_scene::veiled(m.properties.get(&id), &value);
        let file = value.as_str().is_some_and(|v| v.starts_with('/'));
        properties.insert(id, hidden.unwrap_or(if file { Value::Null } else { value }));
    }
    let kinds: Map<String, Value> =
        m.properties.iter().map(|(id, p)| (id.clone(), json!(p.kind()))).collect();
    let web = m.web.clone().unwrap_or_default();
    let hex = web.background.as_deref().and_then(|t| t.strip_prefix('#'));
    let n = hex.and_then(|h| u32::from_str_radix(h, 16).ok()).unwrap_or(0);
    let locale = languages.first().cloned().unwrap_or_default();
    Ok(Start {
        package: wpak.to_owned(),
        id: m.id.clone(),
        entry: m.entry.clone(),
        preload: web.preload,
        background: [(n >> 16) as u8, (n >> 8) as u8, n as u8],
        languages: languages.clone(),
        mask_gpu: true,
        properties,
        shape: json!({"kinds": kinds, "labels": reiwand_scene::labels(&m.properties, &languages)}),
        settings: Settings { fps: asked.fps, quality: asked.quality as u8, locale, volume: 0.0 },
        allowed: Allowed {
            audio: false,
            pointer: asked.pointer.is_some(),
            clicks: asked.pointer.is_some_and(|p| p.2),
        },
        clock: Some(asked.wall),
        ..Start::default()
    })
}

const UNUSED: [&str; 6] = ["--media", "--audio", "--battery", "--calm", "--against", "--without-bloom"];

/// `scene render` of the web package `wpak` found at `dir`, with the options in `unused` given or not.
pub fn run(dir: &Path, wpak: &Path, asked: &Asked, unused: [bool; 6]) -> Result<()> {
    let refused = unused.iter().zip(UNUSED).find(|(given, _)| **given);
    let most = reiwand_render::Rehearsal::MAX_SIMULATE;
    let written = match refused {
        Some((_, flag)) => Err(anyhow::anyhow!(fill(told("%1 does not apply to a web package"), &[&flag]))),
        None if asked.time > most => Err(anyhow::anyhow!(fill(
            told("a web package plays from second 0, at most %1 seconds"),
            &[&most]
        ))),
        None => render(wpak, asked),
    };
    if wpak != dir {
        let _ = std::fs::remove_file(wpak);
    }
    written?.iter().for_each(|p| println!("{}", p.display()));
    Ok(())
}

fn render(wpak: &Path, asked: &Asked) -> Result<Vec<PathBuf>> {
    let program = helper().with_context(|| tr(told("the web helper reiwand-web was not found")))?;
    let start = start(wpak, asked)?;
    let isolation = Isolation { network: true, devices: true, limits: false };
    let (link, child) = reiwand_web::spawn(&program, isolation, |_| {})?;
    let mut run = crate::webframes::Run { link, child, ring: None, loaded: false };
    let properties = start.properties.clone();
    run.link.send(&ToWeb::Start(Box::new(start)), &[])?;
    run.link.send(&ToWeb::Open { view: 1, size: asked.size, scale: 1.0, properties }, &[])?;
    let until = Instant::now() + LOAD;
    while !run.loaded && Instant::now() < until {
        run.pump(Instant::now() + QUIET, false)?;
    }
    if !run.loaded {
        bail!(tr(told("the page did not load in 30 seconds")));
    }
    run.link.send(&ToWeb::Fps { view: 1, fps: 240 }, &[])?;
    let step = 1.0 / f64::from(asked.fps);
    let ahead = (asked.time / step).round() as u32;
    let (w, h) = (f64::from(asked.size.0), f64::from(asked.size.1));
    if let Some((from, ..)) = asked.pointer {
        let at = (f64::from(from[0]) * w, f64::from(from[1]) * h);
        run.link.send(&ToWeb::Pointer { view: 1, at: Some(at) }, &[])?;
    }
    run.pump(Instant::now() + QUIET, true)?;
    let last = f64::from(asked.frames.saturating_sub(1).max(1));
    let gif = asked.out.extension().is_some_and(|e| e.eq_ignore_ascii_case("gif"));
    let (mut written, mut kept) = (Vec::new(), Vec::new());
    for n in 0..ahead + asked.frames {
        let k = n.saturating_sub(ahead);
        let at = asked.pointer.map(|(from, to, _)| {
            let f = |i: usize| f64::from(from[i] + (to[i] - from[i]) * (f64::from(k) / last) as f32);
            (f(0) * w, f(1) * h)
        });
        if let (Some(at), Some((.., true))) = (at, asked.pointer)
            && n == ahead + asked.frames / 2
        {
            run.link.send(&ToWeb::Press { view: 1, at, button: 1 }, &[])?;
        }
        let shot = run.frame(f64::from(n) * step, at)?;
        if n < ahead {
            continue;
        }
        match (gif, asked.frames) {
            (true, _) => kept.push(shot),
            (false, 1) => {
                reiwand_render::save(&shot, &asked.out, 95).map(|()| written.push(asked.out.clone()))?
            }
            (false, _) => {
                let stem = asked.out.file_stem().and_then(|s| s.to_str()).unwrap_or("frame");
                let ext = asked.out.extension().and_then(|e| e.to_str()).unwrap_or("png");
                let file = asked.out.with_file_name(format!("{stem}-{:04}.{ext}", k + 1));
                reiwand_render::save(&shot, &file, 95)?;
                written.push(file);
            }
        }
    }
    if gif {
        let reel = reiwand_render::Reel {
            frames: kept,
            quality: asked.quality,
            render_size: asked.size,
            fps: asked.fps,
            left_out: Vec::new(),
            console: Vec::new(),
            stopped: Vec::new(),
        };
        written = reel.write(&asked.out)?;
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_web_package_is_found_as_a_folder_or_built_and_a_scene_folder_is_not() {
        let probe = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../reiwand-web/tests/probe"));
        let built = package(probe).unwrap().expect("the web folder is not taken as a web package");
        assert_eq!(package(&built).unwrap(), Some(built.clone()));
        std::fs::remove_file(&built).unwrap();
        let scene = tempfile::tempdir().unwrap();
        std::fs::write(scene.path().join(reiwand_format::MANIFEST), r#"{"kind": "scene"}"#).unwrap();
        assert_eq!(package(scene.path()).unwrap(), None);
        assert_eq!(package(&scene.path().join("missing")).unwrap(), None);
    }
}
