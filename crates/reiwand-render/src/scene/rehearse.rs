mod reel;
mod versus;
mod wall;

pub use versus::split;

pub use wall::{local_wall, on_today};

use super::SceneAssets;
use super::scripted::Scripted;
use crate::{RenderError, Renderer};
use image::RgbaImage;
use reiwand_audio::{Played, Track};
use reiwand_scene::{MediaState, PointerState, Quality, Runtime, Scene, Values};
use std::sync::Arc;

const PRESS_FROM: f32 = 0.4;
const PRESS_TO: f32 = 0.7;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trace {
    pub from: [f32; 2],
    pub to: [f32; 2],
    pub click: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rehearsal {
    pub quality: Quality,
    pub battery: bool,
    pub calm: bool,
    pub edge: u32,
    pub screen: Option<(u32, u32)>,
    pub time: f64,
    pub wall: f64,
    pub lang: String,
    pub script_host: Option<super::scripted::ScriptHost>,
    pub media: Option<MediaState>,
    pub audio: Option<Arc<Track>>,
    pub frames: usize,
    pub fps: u32,
    pub pointer: Option<Trace>,
}

impl Default for Rehearsal {
    fn default() -> Rehearsal {
        Rehearsal {
            quality: Quality::Q3,
            battery: false,
            calm: false,
            edge: 1280,
            screen: None,
            time: 0.0,
            wall: local_wall(),
            lang: "en".into(),
            script_host: None,
            media: None,
            audio: None,
            frames: 1,
            fps: 30,
            pointer: None,
        }
    }
}

#[derive(Debug)]
pub struct Reel {
    pub frames: Vec<RgbaImage>,
    pub quality: Quality,
    pub render_size: (u32, u32),
    pub fps: u32,
    pub left_out: Vec<String>,
    pub console: Vec<String>,
    pub stopped: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Versus {
    pub quality: Option<Quality>,
    pub without_bloom: bool,
    pub split: f32,
}

pub type Opened = (Scene, SceneAssets, Values);

impl Rehearsal {
    pub fn level(&self) -> Quality {
        match self.battery {
            true => self.quality.on_battery(),
            false => self.quality,
        }
    }

    pub fn rate(&self) -> u32 {
        let fps = self.fps.clamp(1, 240);
        if self.battery { fps.min(60) } else { fps }
    }

    pub fn pointer_at(&self, frame: usize) -> (Option<[f32; 2]>, u8) {
        let Some(trace) = self.pointer else { return (None, 0) };
        let along = frame as f32 / self.frames.saturating_sub(1).max(1) as f32;
        let pos = [0, 1].map(|i| trace.from[i] + (trace.to[i] - trace.from[i]) * along);
        let pressed = trace.click && (PRESS_FROM..PRESS_TO).contains(&along);
        (Some(pos), u8::from(pressed))
    }
}

impl Renderer {
    pub fn rehearse(
        &self,
        scene: &Scene,
        assets: &SceneAssets,
        props: &Values,
        wanted: &Rehearsal,
    ) -> Result<Reel, RenderError> {
        let quality = wanted.level();
        let [w, h] = scene.canvas.size.map(|v| v.max(1) as f32);
        let scale = (wanted.edge.max(16) as f32 / w.max(h)).min(1.0);
        let canvas = ((w * scale).round().max(1.0) as u32, (h * scale).round().max(1.0) as u32);
        let size = wanted.screen.unwrap_or(canvas);
        let mut runtime = Runtime::new(Arc::new(scene.clone()), props.clone());
        assets.sheets.iter().for_each(|(id, d)| runtime.set_sheet(id, d.clone()));
        runtime.set_battery(wanted.battery);
        runtime.set_reduced_motion(wanted.calm);
        runtime.set_quality(quality);
        runtime.set_wall(wanted.wall);
        runtime.set_language(&wanted.lang);
        runtime.seek(wanted.time.clamp(0.0, f64::from(reiwand_scene::MAX_POSTER_TIME)));
        if let Some(media) = &wanted.media {
            let _ = runtime.set_media(media.clone());
        }
        let mut scripts =
            Scripted::start(scene, assets, &runtime, (&wanted.lang, wanted.script_host.as_ref()));
        let mut view = self.scene_view(scene, assets, runtime.props())?;
        runtime.set_magnify(view.magnify(size));
        let shown = |view: [f32; 4]| (view, [size.0, size.1], wanted.rate(), quality as u8);
        let (step, mut pointer) = (1.0 / f64::from(wanted.rate()), PointerState::default());
        let (mut frames, mut inner) = (Vec::new(), size);
        let start = wanted.time.clamp(0.0, f64::from(reiwand_scene::MAX_POSTER_TIME));
        let mut heard = wanted.audio.clone().map(|track| Played::new(track, start, step));
        for i in 0..wanted.frames.max(1) {
            if i > 0 {
                runtime.advance(step);
                if let Some(media) = &wanted.media {
                    let _ = runtime.set_media(media.at(i as f64 * step));
                }
            }
            if let Some(heard) = heard.as_mut().map(|h| h.at(start + i as f64 * step))
                && runtime.uses().audio
            {
                runtime.set_audio(heard.clone());
            }
            let (pos, buttons) = wanted.pointer_at(i);
            pointer = pointer.follow(pos, buttons, step as f32);
            runtime.set_pointer(pointer);
            inner = view.inner(self, size, quality);
            if let Some(scripts) = scripts.as_mut() {
                let at = shown(view.view(inner));
                if i == 0 {
                    scripts.warm(&mut runtime, at);
                }
                scripts.step(&mut runtime, at);
            }
            let frame = runtime.frame(view.view(inner), quality);
            let image = self.render_scene(&mut view, &runtime, &frame, inner, quality, &[])?;
            frames.push(self.read_scene(image, size)?);
        }
        let (console, stopped) = scripts.map(|s| (s.console.clone(), s.stopped.clone())).unwrap_or_default();
        Ok(Reel {
            frames,
            quality,
            render_size: inner,
            fps: wanted.rate(),
            left_out: view.left_out(),
            console,
            stopped,
        })
    }
}
