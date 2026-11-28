use super::super::SceneAssets;
use crate::Renderer;
use crate::texture::GpuImage;
use image::RgbaImage;
use reiwand_scene::{Runtime, Scene};
use std::collections::HashMap;

/// Decodes the videos of a scene for renders made outside the daemon, which has players of its own.
pub trait Films: Send + Sync + std::fmt::Debug {
    /// Seconds the video at this path of the package lasts, when its file says.
    fn length(&self, path: &str) -> Option<f32>;
    /// The picture of the video at this path of the package at this second.
    fn frame(&self, path: &str, at: f64) -> Result<RgbaImage, String>;
}

const SAME: f64 = 1e-4;

/// The frames of the videos a render shows, decoded where the clock of each video is.
#[derive(Default)]
pub(crate) struct Filmed {
    shown: HashMap<String, (f64, GpuImage)>,
    notes: Vec<String>,
    failed: Vec<String>,
}

fn path<'a>(scene: &'a Scene, key: &str) -> Option<&'a str> {
    let id = key.split_once('@').map_or(key, |(id, _)| id);
    scene.assets.get(id).map(|a| a.path.as_str())
}

impl Filmed {
    pub(crate) fn start(scene: &Scene, assets: &SceneAssets, runtime: &mut Runtime) -> Filmed {
        if let Some(films) = &assets.films {
            for tape in runtime.tapes().to_vec() {
                let length = path(scene, &tape.asset).and_then(|p| films.length(p));
                length.into_iter().for_each(|s| runtime.set_length(&tape.asset, s));
            }
        }
        Filmed::default()
    }

    pub(crate) fn update(&mut self, r: &Renderer, scene: &Scene, assets: &SceneAssets, runtime: &Runtime) {
        for tape in runtime.tapes().iter().filter(|t| t.shown) {
            if self.failed.contains(&tape.asset) {
                continue;
            }
            let fresh = self.shown.get(&tape.asset).is_none_or(|(at, _)| (at - tape.at).abs() > SAME);
            let Some(file) = path(scene, &tape.asset).filter(|_| fresh) else { continue };
            let decoded = match &assets.films {
                Some(films) => films.frame(file, tape.at),
                None => Err("this render cannot decode videos".to_owned()),
            };
            let uploaded = decoded.and_then(|i| {
                r.upload_rgba(&i, i.width(), i.height(), i.width() * 4).map_err(|e| e.to_string())
            });
            match uploaded {
                Ok(image) => _ = self.shown.insert(tape.asset.clone(), (tape.at, image)),
                Err(e) => {
                    let layer = tape.layer.as_deref().unwrap_or(&tape.asset);
                    self.notes.push(format!("the video layer {layer} is drawn empty: {e}"));
                    self.failed.push(tape.asset.clone());
                }
            }
        }
    }

    pub(crate) fn videos(&self) -> Vec<(&str, &GpuImage)> {
        self.shown.iter().map(|(key, (_, image))| (key.as_str(), image)).collect()
    }

    pub(crate) fn notes(self) -> Vec<String> {
        self.notes
    }
}
