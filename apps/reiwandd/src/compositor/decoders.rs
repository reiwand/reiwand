use super::slot::PlayerSlot;
use super::steer::Steer;
use super::{Command, Reel, lock};
use reiwand_render::Renderer;
use reiwand_video::Still;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex, mpsc};

/// Video decoders that may run at once on the whole desktop.
pub(super) const MAX_DECODERS: usize = 16;
const STILL_SIZE: u32 = 1920;
const IN_STEP: f64 = 0.1;

/// Where the frames of a video come from: a decoder of its own, the decoder of another video that
/// plays the same file in step, or a still frame when every decoder is taken.
#[derive(Debug, Clone, PartialEq, Default)]
pub(super) enum Source {
    #[default]
    Own,
    Follows(Reel),
    Still,
}

/// What a video wants from its decoder: two videos with the same wish show the same frames.
pub(super) struct Wish<'a> {
    pub(super) reel: &'a Reel,
    pub(super) steer: Option<Steer>,
    pub(super) volume: u32,
    pub(super) decoding: bool,
}

fn in_step(a: &Wish, b: &Wish) -> bool {
    let same = |a: &Steer, b: &Steer| {
        a.cut == b.cut && a.wants == b.wants && a.jumps == 0 && b.jumps == 0 && (a.at - b.at).abs() < IN_STEP
    };
    let steered = match (&a.steer, &b.steer) {
        (Some(a), Some(b)) => same(a, b),
        (None, None) => a.reel.cut == b.reel.cut,
        _ => false,
    };
    a.reel.path == b.reel.path && a.reel.once == b.reel.once && a.volume == b.volume && steered
}

/// Gives every video a source: those that decode keep their decoder, a video in step with one that
/// decodes follows it, and the rest decode until `MAX_DECODERS` run; any further ones show a still.
pub(super) fn plan(wishes: &[Wish]) -> HashMap<Reel, Source> {
    let mut order: Vec<&Wish> = wishes.iter().collect();
    order.sort_by_key(|w| !w.decoding);
    let mut leaders: Vec<&Wish> = Vec::new();
    let mut planned = HashMap::new();
    for wish in order {
        let source = match leaders.iter().find(|l| in_step(l, wish)) {
            Some(leader) => Source::Follows(leader.reel.clone()),
            None if leaders.len() < MAX_DECODERS => {
                leaders.push(wish);
                Source::Own
            }
            None => Source::Still,
        };
        planned.insert(wish.reel.clone(), source);
    }
    planned
}

type Job = (PathBuf, f64, Arc<Mutex<Option<Still>>>, mpsc::Sender<Command>);

/// One thread decodes the still frames, one after another, so stills never add decoders.
fn stills() -> &'static Mutex<mpsc::Sender<Job>> {
    static JOBS: LazyLock<Mutex<mpsc::Sender<Job>>> = LazyLock::new(|| {
        let (send, receive) = mpsc::channel::<Job>();
        std::thread::Builder::new()
            .name("video-stills".into())
            .spawn(move || {
                for (path, at, out, wake) in receive {
                    match reiwand_video::still_within(&path, Some(at), STILL_SIZE) {
                        Ok(still) => drop(lock(&out).replace(still)),
                        Err(e) => tracing::debug!(path = %path.display(), "no still frame of the video: {e}"),
                    }
                    let _ = wake.send(Command::Wake);
                }
            })
            .ok();
        Mutex::new(send)
    });
    &JOBS
}

impl PlayerSlot {
    pub(super) fn take_source(&mut self, source: Source, reel: &Reel, tx: &mpsc::Sender<Command>) {
        if self.source == source {
            return;
        }
        match &source {
            Source::Own => self.due = Some(std::time::Instant::now()),
            _ => (self.player, self.due, self.steered) = (None, None, None),
        }
        if source == Source::Still {
            let at = f64::from(reel.cut.start) / 1000.0;
            let job = (reel.path.clone(), at, self.picture.clone(), tx.clone());
            let _ = lock(stills()).send(job);
        }
        self.source = source;
    }

    pub(super) fn show_still(&mut self, renderer: &Renderer) -> bool {
        let Some(still) = lock(&self.picture).take() else { return false };
        let (w, h) = (still.width, still.height);
        match renderer.upload_rgba(&still.rgba, w, h, w * 4) {
            Ok(texture) => self.texture = Some(texture),
            Err(e) => tracing::warn!("cannot upload the still frame of a video: {e}"),
        }
        true
    }
}

impl super::state::State {
    pub(super) fn share_decoders(&mut self, wishes: &[(Reel, Option<Steer>, u32)]) {
        let wishes: Vec<Wish> = wishes
            .iter()
            .map(|(reel, steer, volume)| {
                let decoding = self.players.get(reel).is_some_and(|s| s.source == Source::Own);
                Wish { reel, steer: *steer, volume: *volume, decoding }
            })
            .collect();
        let planned = plan(&wishes);
        let stills = planned.values().filter(|s| **s == Source::Still).count();
        if stills > self.stills {
            tracing::warn!("{stills} videos show a still frame: at most {MAX_DECODERS} videos play at once");
        }
        self.stills = stills;
        for (reel, source) in planned {
            if let Some(slot) = self.players.get_mut(&reel) {
                slot.take_source(source, &reel, &self.tx);
            }
        }
    }

    pub(super) fn take_video_frames(&mut self) {
        let now = std::time::Instant::now();
        let mut fresh = Vec::new();
        for (reel, slot) in &mut self.players {
            if slot.receive(&reel.path, self.renderer, now) | slot.show_still(self.renderer) {
                fresh.push(reel.clone());
            }
        }
        let shared: Vec<(Reel, Option<reiwand_render::GpuImage>)> = (self.players.iter())
            .filter_map(|(reel, slot)| match &slot.source {
                Source::Follows(leader) if fresh.contains(leader) => {
                    Some((reel.clone(), self.players.get(leader)?.texture.clone()))
                }
                _ => None,
            })
            .collect();
        for (reel, texture) in shared {
            if let (Some(slot), Some(texture)) = (self.players.get_mut(&reel), texture) {
                slot.texture = Some(texture);
                fresh.push(reel);
            }
        }
        for reel in fresh {
            let wants = |o: &super::output::Output| o.uses_video(&reel) && (o.active || o.missing_frame);
            self.outputs.values_mut().filter(|o| wants(o)).for_each(|o| o.dirty = true);
        }
    }
}

#[cfg(test)]
mod tests;
