use super::*;

impl Speaker {
    pub(super) fn open(pack: &Arc<ScenePack>, voices: &[Voice], tracks: &[Track], volume: f64) -> Speaker {
        let sounds: Vec<_> = pack.scene.sounds().collect();
        let file = |id: &str| pack.sounds.iter().position(|(sound, _)| sound == id);
        let lanes: Vec<(Vec<Option<usize>>, Track)> = (sounds.iter().zip(tracks))
            .map(|(s, track)| {
                let silent = s.play == Value::Fixed(false);
                (s.clips().map(|id| file(id).filter(|_| !silent)).collect(), track.clone())
            })
            .collect();
        let (bank, voices_now) = (pack.clone(), voices.to_vec());
        let track = Later::spawn(move || {
            let failed = |event| {
                if let Event::Error(e) = event {
                    tracing::warn!("the scene sound stopped: {e}");
                }
            };
            let track = Soundtrack::open(&bank.clips, &lanes, &voices_now, volume, failed);
            let track = track.inspect_err(|e| tracing::warn!("cannot play the scene sound: {e}")).ok();
            tracing::info!(scene = bank.props.id, "scene sound on");
            track
        });
        let fade = sounds.iter().map(|s| Duration::from_secs_f32(s.fade_out)).max().unwrap_or_default();
        let (pack, output, voices) = (pack.clone(), String::new(), voices.to_vec());
        let synced = Instant::now();
        Speaker { pack, output, track, playing: false, volume, voices, fade, quiet: None, synced }
    }

    pub(super) fn settle(&mut self, volume: f64, playing: bool) {
        if let Some(track) = self.track.get() {
            track.set(&self.voices);
            track.set_volume(volume);
            track.set_playing(playing);
        }
        (self.volume, self.playing) = (volume, playing);
    }

    pub(super) fn follow(&mut self, voices: &[Voice], now: Instant) {
        if self.voices != voices {
            self.track.get().into_iter().for_each(|t| t.set(voices));
            self.voices = voices.to_vec();
        }
        self.synced = now;
    }

    pub(super) fn faded(&self, now: Instant) -> bool {
        self.quiet.is_some_and(|at| at + self.fade <= now)
    }

    pub(super) fn due(&self) -> Option<Instant> {
        let faded =
            self.quiet.map(|quiet| quiet + self.fade + if self.playing { Duration::ZERO } else { LINGER });
        let opening = self.track.waiting().then(|| Instant::now() + OPENING);
        faded.into_iter().chain(opening).chain(self.playing.then_some(self.synced + RESYNC)).min()
    }
}

impl Drop for Speaker {
    fn drop(&mut self) {
        tracing::info!(scene = self.pack.props.id, "scene sound off");
    }
}
