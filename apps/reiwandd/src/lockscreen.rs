use crate::apply::target_output;
use crate::daemon::Daemon;
use crate::error::{Result, invalid};
use crate::items::reply;
use reiwand_core::LockShow;
use reiwand_core::lock::screen_of;
use reiwand_core::model::{LockChoice, LockReply, MODEL};
use std::path::PathBuf;

pub const TOPIC: &str = "lock";

fn screen(output: &str) -> Result<String> {
    let output = target_output(output)?;
    if screen_of(&output).is_some() || crate::drafts::virtual_output(&output) {
        return Err(invalid("name a screen, such as DP-1, or leave it empty for every screen"));
    }
    Ok(output)
}

impl Daemon {
    pub fn set_lock_wallpaper(&self, uid: &str, output: &str) -> Result<()> {
        let output = screen(output)?;
        let item = self.store.read(|lib| lib.item(uid))?;
        self.showable(&item)?;
        if item.package.as_ref().is_some_and(|p| p.kind == "preset") {
            return Err(invalid("a preset changes the settings of a wallpaper on a screen; choose its base"));
        }
        let _decoded = self.check_media(&item.path)?;
        self.change_lock(&output, LockShow::image(item.path))
    }

    pub fn set_lock_playlist(&self, key: &str, output: &str) -> Result<()> {
        let output = screen(output)?;
        let uid = self.store.read(|lib| lib.playlist(key))?.uid;
        self.change_lock(&output, LockShow::playlist(uid))
    }

    pub fn follow_desktop_on_lock(&self, output: &str) -> Result<()> {
        let output = screen(output)?;
        self.change_lock(&output, LockShow::default())
    }

    /// One screen shows its desktop while locked even when every lock screen has a choice.
    pub fn show_desktop_on_lock(&self, output: &str) -> Result<()> {
        let output = screen(output)?;
        if output.is_empty() {
            return Err(invalid("name a screen; for every screen use FollowDesktopOnLock"));
        }
        self.change_lock(&output, LockShow { desktop: Some(true), ..LockShow::default() })
    }

    fn change_lock(&self, output: &str, show: LockShow) -> Result<()> {
        self.update_config(|c| {
            c.set_lock(output, show);
            Ok(())
        })?;
        tracing::info!(output, "lock screen wallpaper changed");
        self.lock_changed();
        Ok(())
    }

    pub(crate) fn lock_changed(&self) {
        let shown: Vec<String> =
            self.state().outputs.keys().filter(|o| screen_of(o).is_some()).cloned().collect();
        shown.iter().for_each(|output| drop(self.configure_output(output)));
        self.changes.changed(TOPIC, Vec::new());
    }

    pub(crate) fn forget_lock_files(&self, files: &[PathBuf]) -> Result<()> {
        if !self.state().config.clone().forget_lock_files(files) {
            return Ok(());
        }
        self.update_config(|c| {
            c.forget_lock_files(files);
            Ok(())
        })?;
        self.lock_changed();
        Ok(())
    }

    pub fn lock_json(&self) -> Result<String> {
        let serial = self.changes.serial();
        let lock = self.state().config.lock.clone().unwrap_or_default();
        let every = self.lock_choice(&lock.every());
        let outputs =
            lock.outputs.iter().filter_map(|(name, show)| Some((name.clone(), self.lock_choice(show)?)));
        reply(LockReply { model: MODEL, serial, every, outputs: outputs.collect() })
    }

    fn lock_choice(&self, show: &LockShow) -> Option<LockChoice> {
        if show.desktop() {
            let (image, item, playlist, name) = (None, None, None, String::new());
            return Some(LockChoice { image, item, playlist, name, desktop: true });
        }
        let found = |path: &PathBuf| self.store.read(|lib| lib.resolve_item(&path.to_string_lossy())).ok();
        let item = show.image.as_ref().and_then(found);
        let titled =
            item.as_deref().and_then(|uid| self.store.read(|lib| lib.item(uid)).ok()).map(|i| i.title);
        let file = show.image.as_ref().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().into_owned());
        let listed = show.playlist.as_deref().and_then(|uid| self.store.read(|lib| lib.playlist(uid)).ok());
        let name = titled.or(file).or(listed.map(|p| p.name)).or_else(|| show.playlist.clone())?;
        Some(LockChoice {
            image: show.image.clone(),
            item,
            playlist: show.playlist.clone(),
            name,
            desktop: false,
        })
    }
}
