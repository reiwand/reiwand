use crate::compositor::Deck;
use crate::daemon::Daemon;
use anyhow::Context;
use reiwand_core::Fill;
use reiwand_core::edit::{Edit, Geometry};
use reiwand_core::travel::Still;
use reiwand_depth::{Layers, Places, Settings};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};

mod making;

#[derive(Default)]
pub(crate) struct Memory {
    sets: HashMap<PathBuf, Weak<Layers>>,
    turned: HashMap<(PathBuf, String), Weak<Layers>>,
    failed: HashSet<PathBuf>,
    building: HashMap<PathBuf, PathBuf>,
    cancelled: HashSet<PathBuf>,
    still: HashMap<PathBuf, Still>,
    me: Weak<Daemon>,
}

impl Memory {
    pub(crate) fn still(&self, file: &Path) -> Option<Still> {
        self.still.get(file).copied()
    }

    pub(crate) fn mark(&mut self, file: &Path, still: Option<Still>) {
        match still {
            Some(still) => drop(self.still.insert(file.to_owned(), still)),
            None => drop(self.still.remove(file)),
        }
    }

    pub(crate) fn cancelled(&mut self, file: &Path) {
        let sets = self.building.iter().filter(|(_, f)| *f == file).map(|(set, _)| set.clone());
        let sets: Vec<PathBuf> = sets.collect();
        self.cancelled.extend(sets);
        self.mark(file, Some(Still::Cancelled));
    }

    pub(crate) fn again(&mut self, file: &Path) {
        let building = &self.building;
        self.cancelled.retain(|set| building.get(set).is_none_or(|f| f != file));
    }

    pub(crate) fn unfail(&mut self, file: &Path, sha: &str) {
        let prefix = format!("{sha}-");
        let own = |p: &PathBuf| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with(&prefix));
        self.failed.retain(|p| !own(p));
        self.again(file);
    }
}

pub(crate) fn start(daemon: &Arc<Daemon>) {
    daemon.state().decks.me = Arc::downgrade(daemon);
}

impl Daemon {
    pub(crate) fn retry_layers(&self, path: &Path) -> anyhow::Result<()> {
        let file = std::fs::canonicalize(path).with_context(|| crate::flatpak::not_found(path))?;
        let sha = reiwand_core::edit::Source::read(&file, [0; 2])?.sha256;
        let places = Places { data: self.store.data.clone(), cache: self.store.cache.clone() };
        places.unfail(&sha);
        {
            let mut st = self.state();
            st.decks.unfail(&file, &sha);
            st.decks.again(path);
        }
        self.configure_all();
        Ok(())
    }

    pub(crate) fn deck_for(
        &self,
        file: &Path,
        sha: &str,
        edit: &Edit,
        (places, edge): (&Places, u32),
    ) -> Option<Deck> {
        let made = self.deck_or_still(file, sha, edit, (places, edge));
        let still = made.as_ref().err().copied().flatten();
        self.state().decks.mark(file, still);
        made.ok()
    }

    fn deck_or_still(
        &self,
        file: &Path,
        sha: &str,
        edit: &Edit,
        (places, edge): (&Places, u32),
    ) -> Result<Deck, Option<Still>> {
        let moving = edit.moving().ok_or(None)?;
        let depth = edit.depth.clone().unwrap_or_default();
        let set = Settings {
            fill: places.fill(moving.fill()),
            count: usize::from(moving.layers()),
            shift: moving.strength * reiwand_core::travel::gain(false, 1.0),
            edge: reiwand_depth::edge_for(edge),
        };
        let path = places.layer_set(file, sha, &depth, &set).ok_or(Some(Still::NoMap))?;
        let known = self.state().decks.sets.get(&path).and_then(Weak::upgrade);
        let found = match known {
            Some(layers) => Some(layers),
            None => match reiwand_depth::load(&path) {
                Ok(found) => found.map(Arc::new),
                Err(e) => {
                    tracing::warn!("cannot use the depth layers of a picture");
                    tracing::debug!("{e:#}");
                    places.discard(&path);
                    reiwand_depth::mark_failed(&path, &e);
                    self.state().decks.failed.insert(path.clone());
                    None
                }
            },
        };
        let Some(layers) = found else {
            let failed = self.state().decks.failed.contains(&path) || reiwand_depth::has_failed(&path);
            let cancelled = self.state().decks.cancelled.contains(&path);
            let simple = set.fill == Fill::Simple && places.fill(Fill::Lama) == Fill::Simple;
            let still = match (failed, cancelled, simple) {
                (true, ..) => Still::Failed,
                (_, true, _) => Still::Cancelled,
                (.., true) => Still::BuildingSimple,
                _ => Still::Building,
            };
            if !failed && !cancelled {
                self.build_deck(file, sha, (depth, set), path);
            }
            return Err(Some(still));
        };
        if !layers.info.reach.moves(layers.info.size[0]) {
            return Err(Some(Still::Flat));
        }
        {
            let mut st = self.state();
            st.decks.sets.retain(|_, weak| weak.strong_count() > 0);
            st.decks.sets.insert(path.clone(), Arc::downgrade(&layers));
        }
        Ok(Deck { layers: self.turned(&path, layers, &edit.geometry), file: path })
    }

    fn turned(&self, path: &Path, layers: Arc<Layers>, geometry: &Geometry) -> Arc<Layers> {
        if geometry.is_identity() {
            return layers;
        }
        let key = (path.to_owned(), serde_json::to_string(geometry).unwrap_or_default());
        if let Some(known) = self.state().decks.turned.get(&key).and_then(Weak::upgrade) {
            return known;
        }
        let made = Arc::new(reiwand_depth::turned(&layers, geometry));
        let mut st = self.state();
        st.decks.turned.retain(|_, weak| weak.strong_count() > 0);
        st.decks.turned.insert(key, Arc::downgrade(&made));
        made
    }
}
