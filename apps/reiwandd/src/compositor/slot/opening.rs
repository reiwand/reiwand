use super::*;
use reiwand_video::{Event, Options};

const POLL: Duration = Duration::from_millis(30);

impl PlayerSlot {
    pub(super) fn open(
        &mut self,
        reel: &Reel,
        (settings, _): (Settings, bool),
        tx: &mpsc::Sender<Command>,
        now: Instant,
    ) {
        let (max_size, max_fps, cut, path) = (self.max_size, self.fps, reel.cut, reel.path.clone());
        let (volume, muted) = (loudness(settings.volume, self.volume), settings.muted || self.volume == 0);
        let (inbox, tx) = (self.inbox.clone(), tx.clone());
        let options = Options { max_size, max_fps, volume, muted, cut, zero_copy: self.shares };
        self.opening = Some(Later::spawn(move || {
            let (frames, wake) = (inbox.clone(), tx.clone());
            let opened = Player::open(&path, options, move |event| {
                let first = match event {
                    Event::Frame(frame) => lock(&frames.frame).replace(frame).is_none(),
                    Event::Error(e) => lock(&frames.error).replace(e).is_none(),
                };
                if first {
                    let _ = wake.send(Command::Wake);
                }
            });
            let _ = tx.send(Command::Wake);
            match opened {
                Ok(player) => Some(player),
                Err(e) => {
                    drop(lock(&inbox.error).replace(e.to_string()));
                    None
                }
            }
        }));
        self.due = Some(now + POLL);
    }

    pub(super) fn arrive(&mut self, reel: &Reel, (settings, locked): (Settings, bool), now: Instant) {
        let Some(opening) = self.opening.as_mut() else { return };
        if !opening.arrived() {
            return self.due = Some(now + POLL);
        }
        let Some(mut player) = self.opening.take().and_then(Later::take) else { return self.due = None };
        tracing::debug!(path = %reel.path.display(), "video opened");
        let audible = self.volume > 0 && !settings.muted && settings.volume > 0.0;
        if audible && !player.audio_enabled() {
            return self.due = Some(now);
        }
        let volume = loudness(settings.volume * settings.duck, self.volume);
        player.set_volume(volume, settings.muted || locked || self.volume == 0);
        player.play_once(reel.once);
        player.set_max_size(self.max_size);
        player.set_max_fps(self.fps);
        player.set_playing(self.playing);
        (self.player, self.due, self.steered) = (Some(player), Some(now + FIRST_FRAME), None);
    }
}
