use super::SceneAssets;
use super::sampling::Sampling;
use reiwand_scene::{Scene, Value};
use std::collections::{HashMap, HashSet};

pub(crate) type Inked = reiwand_scene::Inked;

fn bounds(image: &image::RgbaImage) -> Inked {
    reiwand_scene::ink_bounds(image.width(), image.height(), image.as_raw())
}

impl SceneAssets {
    pub fn ink_scene(&self, scene: &Scene) {
        let image = |id: &str| self.images.get(id)?.image();
        let boxes =
            scene.inked_ids().into_iter().filter_map(|id| Some((id.to_string(), bounds(&*image(id)?))));
        scene.fill_inked(boxes.collect());
    }
}

pub(crate) fn ink(scene: &Scene, assets: &SceneAssets, samplings: &mut HashMap<String, Sampling>) {
    let uses = || scene.drawable().flat_map(|l| &l.effects);
    let masks = uses().filter_map(|e| match e.mask.as_ref()?.texture.as_ref()? {
        Value::Fixed(id) => Some(id.as_str()),
        Value::Bound(_) => None,
    });
    let limits = uses()
        .filter_map(|e| Some((e, scene.effect(&e.effect)?)))
        .flat_map(|(e, d)| d.bounded.iter().filter_map(move |b| d.params.get(d.param(b)?)?.named(e)));
    let masks = masks.chain(limits);
    for id in masks.collect::<HashSet<_>>() {
        let found = scene.inked_box(id).or_else(|| Some(bounds(&*assets.images.get(id)?.image()?)));
        let Some(found) = found else { continue };
        samplings.entry(id.to_string()).or_default().inked = Some(found);
    }
}
