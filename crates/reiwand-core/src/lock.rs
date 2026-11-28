use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub const PREFIX: &str = "lock:";

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lock {
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::empty")]
    pub image: Option<PathBuf>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::empty")]
    pub playlist: Option<String>,
    pub outputs: BTreeMap<String, LockShow>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockShow {
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::empty")]
    pub image: Option<PathBuf>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::empty")]
    pub playlist: Option<String>,
    /// Only for one screen: `true` shows its desktop even while every lock screen has a choice.
    /// Absent, the screen follows the choice for every screen, as before.
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value", extend("x-null" = true, "x-block" = true)))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::explicit::block")]
    pub desktop: Option<bool>,
}

impl LockShow {
    pub fn chosen(&self) -> bool {
        self.image.is_some() || self.playlist.is_some() || self.desktop()
    }

    pub fn desktop(&self) -> bool {
        self.desktop == Some(true)
    }

    pub fn image(path: PathBuf) -> Self {
        LockShow { image: Some(path), ..Self::default() }
    }

    pub fn playlist(uid: String) -> Self {
        LockShow { playlist: Some(uid), ..Self::default() }
    }
}

impl Lock {
    pub fn every(&self) -> LockShow {
        LockShow { image: self.image.clone(), playlist: self.playlist.clone(), desktop: None }
    }

    fn tidy(mut self) -> Option<Self> {
        let every = self.every().chosen();
        self.outputs.retain(|_, show| show.chosen() && (every || !show.desktop()));
        (self.every().chosen() || !self.outputs.is_empty()).then_some(self)
    }
}

pub fn screen_of(output: &str) -> Option<&str> {
    output.strip_prefix(PREFIX)
}

pub fn seat(output: &str) -> &str {
    screen_of(output).unwrap_or(output)
}

impl Config {
    pub fn lock_choice(&self, screen: &str) -> Option<LockShow> {
        let lock = self.lock.as_ref()?;
        let own = lock.outputs.get(seat(screen)).filter(|show| show.chosen()).cloned();
        if own.as_ref().is_some_and(LockShow::desktop) {
            return None;
        }
        own.or_else(|| Some(lock.every()).filter(LockShow::chosen))
    }

    pub fn set_lock(&mut self, output: &str, show: LockShow) {
        let mut lock = self.lock.take().unwrap_or_default();
        match output {
            "" => {
                lock.outputs.clear();
                (lock.image, lock.playlist) = (show.image, show.playlist);
            }
            name => drop(lock.outputs.insert(name.to_owned(), show)),
        }
        self.lock = lock.tidy();
    }

    pub fn forget_lock(&mut self, gone: impl Fn(&LockShow) -> bool) -> bool {
        let Some(mut lock) = self.lock.take() else { return false };
        let before = lock.clone();
        if gone(&lock.every()) {
            (lock.image, lock.playlist) = (None, None);
        }
        lock.outputs.retain(|_, show| !gone(show));
        let changed = lock != before;
        self.lock = lock.tidy();
        changed
    }

    pub fn forget_lock_files(&mut self, files: &[PathBuf]) -> bool {
        self.forget_lock(|show| show.image.as_deref().is_some_and(|i: &Path| files.iter().any(|f| f == i)))
    }
}
