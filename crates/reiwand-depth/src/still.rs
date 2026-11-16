use crate::Places;
use crate::stack::{SUFFIX, Settings, load};
use reiwand_core::Fill;
use reiwand_core::edit::Edit;
use reiwand_core::travel::Still;
use std::path::{Path, PathBuf};

pub fn failed_of(set: &Path) -> PathBuf {
    let name = set.file_name().map(|n| n.to_string_lossy().replace(SUFFIX, ".layers.failed"));
    set.with_file_name(name.unwrap_or_default())
}

pub fn mark_failed(set: &Path, error: &anyhow::Error) {
    std::fs::write(failed_of(set), format!("{error:#}")).ok();
}

pub fn has_failed(set: &Path) -> bool {
    failed_of(set).is_file()
}

impl Places {
    pub fn still(&self, (image, sha256): (&Path, &str), edit: &Edit, set: &Settings) -> Option<Still> {
        edit.moving()?;
        let depth = edit.depth.clone().unwrap_or_default();
        let Some(path) = self.layer_set(image, sha256, &depth, set) else {
            let ready = self.installed(&crate::MODEL).is_some() && !self.library(None).is_empty();
            return Some(if ready { Still::NoMap } else { Still::NoModel });
        };
        let found = match () {
            _ if has_failed(&path) => return Some(Still::Failed),
            _ => load(&path).ok().flatten(),
        };
        match found {
            None if set.fill == Fill::Simple && self.fill(Fill::Lama) == Fill::Simple => {
                Some(Still::BuildingSimple)
            }
            None => Some(Still::Building),
            Some(layers) if !layers.info.reach.moves(layers.info.size[0]) => Some(Still::Flat),
            Some(_) => None,
        }
    }
}

pub fn model_size(places: &Places) -> String {
    let runtime = crate::RUNTIME.filter(|_| places.library(None).is_empty());
    let parts = [Some(crate::MODEL), runtime].into_iter().flatten();
    let parts: Vec<String> = parts.map(|a| format!("{:.0} MB", a.bytes as f64 / 1e6)).collect();
    parts.join(" + ")
}

pub fn lama_size(places: &Places) -> String {
    let runtime = crate::RUNTIME.filter(|_| places.library(None).is_empty());
    let bytes = crate::LAMA.bytes + runtime.map_or(0, |r| r.bytes);
    format!("{:.0} MB", bytes as f64 / 1e6)
}

pub fn still_note(places: &Places, still: Still) -> reiwand_core::message::Message {
    match still {
        Still::BuildingSimple => still.sized(&lama_size(places)),
        _ => still.sized(&model_size(places)),
    }
}
