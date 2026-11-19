use super::later::Later;
use super::{Command, Reel, Settings, lock};
use reiwand_render::{GpuImage, Imported, Renderer};
use reiwand_video::{Frame, Player};
use std::path::Path;
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};

fn loudness(volume: f64, own: u32) -> f64 {
    volume * f64::from(own.min(100)) / 100.0
}

const RETRIES: u32 = 3;
const FIRST_RETRY: Duration = Duration::from_secs(2);
const FIRST_FRAME: Duration = Duration::from_secs(10);

pub(super) type Demand = ((u32, u32), bool, u32, u32);

#[derive(Default)]
struct Inbox {
    frame: Mutex<Option<Frame>>,
    error: Mutex<Option<String>>,
}

pub(super) struct PlayerSlot {
    pub(super) player: Option<Player>,
    pub(super) texture: Option<GpuImage>,
    pub(super) error: Option<String>,
    pub(super) due: Option<Instant>,
    pub(super) playing: bool,
    pub(super) max_size: (u32, u32),
    pub(super) fps: u32,
    pub(super) volume: u32,
    pub(super) decode: &'static str,
    failures: u32,
    opening: Option<Later<Player>>,
    inbox: Arc<Inbox>,
    shares: bool,
    imported: Option<Imported>,
    deep: Option<reiwand_render::Deep>,
    held: Option<Frame>,
    pub(super) steered: Option<(reiwand_video::Cut, u32)>,
    pub(super) lit: f32,
}

impl PlayerSlot {
    pub(super) fn bright(&self) -> bool {
        self.deep.is_some()
    }

    pub(super) fn new((max_size, playing, fps, volume): Demand, open_at: Instant) -> Self {
        let (player, texture, error, inbox, due) = (None, None, None, Arc::default(), Some(open_at));
        let (decode, shares, imported, held) =
            ("", std::env::var_os("REIWAND_ZERO_COPY").is_none_or(|v| v != "0"), None, None);
        Self {
            player,
            texture,
            error,
            due,
            playing,
            max_size,
            fps,
            volume,
            decode,
            failures: 0,
            opening: None,
            inbox,
            shares,
            imported,
            deep: None,
            held,
            steered: None,
            lit: 1.0,
        }
    }

    pub(super) fn hush(&self, (settings, locked): (Settings, bool)) {
        let (volume, muted) = (
            loudness(settings.volume * settings.duck, self.volume),
            settings.muted || locked || self.volume == 0,
        );
        self.player.iter().for_each(|p| p.set_volume(volume, muted));
    }

    pub(super) fn set_volume(&mut self, volume: u32, heard: (Settings, bool), now: Instant) {
        self.volume = volume;
        let audible = volume > 0 && !heard.0.muted && heard.0.volume > 0.0;
        if self.player.as_ref().is_some_and(|p| audible && !p.audio_enabled()) {
            (self.player, self.due) = (None, Some(now));
        }
        self.hush(heard);
    }

    pub(super) fn tick(
        &mut self,
        reel: &Reel,
        heard: (Settings, bool),
        tx: &mpsc::Sender<Command>,
        now: Instant,
    ) {
        if self.opening.is_some() {
            return self.arrive(reel, heard, now);
        }
        let Some(_) = self.due.take_if(|at| *at <= now) else { return };
        match self.player {
            Some(_) => {
                self.fail(&reel.path, format!("no video frame within {} s", FIRST_FRAME.as_secs()), now)
            }
            None => self.open(reel, heard, tx, now),
        }
    }

    pub(super) fn revive(&mut self, now: Instant) {
        if self.player.is_none() && self.opening.is_none() && self.due.is_none() {
            (self.failures, self.due) = (0, Some(now));
        }
    }

    pub(super) fn receive(&mut self, path: &Path, renderer: &Renderer, now: Instant) -> bool {
        let (frame, error) = (lock(&self.inbox.frame).take(), lock(&self.inbox.error).take());
        let uploaded = frame.is_some_and(|frame| self.upload(path, renderer, frame, now));
        match error {
            Some(error) if error == reiwand_video::UNSHARED && self.shares => self.copy_instead(now),
            Some(error) => self.fail(path, error, now),
            None => {}
        }
        uploaded
    }

    fn copy_instead(&mut self, now: Instant) {
        tracing::info!("the GPU cannot read the decoded video frames in place, copying them instead");
        (self.shares, self.player, self.imported, self.held, self.due) = (false, None, None, None, Some(now));
        self.opening = None;
    }

    fn fail(&mut self, path: &Path, error: String, now: Instant) {
        (self.player, self.opening) = (None, None);
        self.failures += 1;
        let retry = self.failures <= RETRIES;
        self.due = retry.then(|| now + FIRST_RETRY * 2u32.pow(self.failures - 1));
        let file = std::path::absolute(path).unwrap_or_else(|_| path.into());
        let shown = error.replace(&*file.to_string_lossy(), "<video>");
        tracing::warn!(retry, "video playback failed: {shown}");
        tracing::debug!(path = %path.display(), "video that failed: {error}");
        self.error = Some(error);
    }
}

mod opening;
#[cfg(test)]
mod tests;
mod upload;
