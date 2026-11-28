use super::Playing;
use crate::daemon::{Daemon, State};
use crate::sceneprops::{Stored, stored};
use reiwand_library::Kind;
use std::path::{Path, PathBuf};

type Turn = (Option<PathBuf>, Option<String>, Option<(String, Stored)>);

fn is_preset(path: &Path) -> bool {
    Kind::detect(path) == Some(Kind::Package)
        && reiwand_format::peek(path).is_ok_and(|m| m.kind == reiwand_format::Kind::Preset)
}

impl Daemon {
    /// A preset's turn in a playlist shows its base with the preset's values for that turn, as
    /// applying the preset does, without writing them as the settings of the base. The same item
    /// keeps the turn it had, with any setting tuned during it.
    pub(super) fn through_preset(
        &self,
        before: Option<&Playing>,
        uid: &str,
        item: &Option<String>,
        path: Option<PathBuf>,
        error: Option<String>,
    ) -> Turn {
        let same = before.filter(|p| p.playlist.uid == uid && p.item == *item && p.path.is_some());
        if let Some(p) = same.filter(|p| p.preset.is_some()) {
            return (p.path.clone(), error, p.preset.clone());
        }
        let Some(file) = path.as_deref().filter(|p| is_preset(p)) else { return (path, error, None) };
        match self.preset_package(file) {
            Ok((base, props, values)) => {
                let values = values.iter().filter_map(|(k, v)| Some((k.clone(), stored(v)?))).collect();
                (Some(base.path), error, Some((props.id.clone(), values)))
            }
            Err(e) => {
                tracing::warn!(preset = %file.display(), "cannot show a preset of the playlist: {e}");
                (None, Some(e.to_string()), None)
            }
        }
    }
}

impl State {
    fn turn(&self, output: &str) -> Option<&Playing> {
        let playlist = self.config.resolve(output).playlist;
        self.playing.get(output).filter(|p| playlist.as_ref() == Some(&p.playlist.uid))
    }

    /// The preset whose turn it is on an output: the package id of its base and its values.
    pub fn preset_turn(&self, output: &str) -> Option<&(String, Stored)> {
        self.turn(output).and_then(|p| p.preset.as_ref())
    }

    pub fn preset_turn_mut(&mut self, output: &str) -> Option<&mut (String, Stored)> {
        self.turn(output)?;
        self.playing.get_mut(output).and_then(|p| p.preset.as_mut())
    }

    /// The stored settings of a package on an output, first wins: a preset's turn, then the
    /// output's own settings, then the shared ones.
    pub fn scene_values(&self, output: &str, package: &str) -> Vec<&Stored> {
        let turn = self.preset_turn(output).filter(|(id, _)| id == package).map(|(_, values)| values);
        turn.into_iter().chain(self.config.scene_values(output, package)).collect()
    }
}
