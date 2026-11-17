use super::Session;
use crate::compositor::{Pic, StripSpec, lock};
use crate::daemon::Daemon;
use reiwand_core::model::BrowseSource;
use reiwand_core::strip::{REACH, Wheel};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock, Weak};
use std::time::Duration;
use zbus::object_server::SignalEmitter;

mod input;
mod pics;

pub(crate) use input::{Do, pointer, run, wheel};

pub(crate) struct Link {
    pub(crate) daemon: Weak<Daemon>,
    pub(crate) emitter: SignalEmitter<'static>,
    pub(crate) runtime: tokio::runtime::Handle,
}

static LINK: OnceLock<Link> = OnceLock::new();

pub(crate) fn attach(daemon: &Arc<Daemon>, emitter: &SignalEmitter<'static>) {
    let link = Link {
        daemon: Arc::downgrade(daemon),
        emitter: emitter.clone(),
        runtime: tokio::runtime::Handle::current(),
    };
    LINK.set(link).ok();
}

pub(crate) fn link() -> Option<&'static Link> {
    LINK.get()
}

type Frames = Vec<(Duration, Arc<Pic>)>;

#[derive(Default)]
pub struct Stage {
    pics: HashMap<String, Arc<Pic>>,
    motion: Option<(String, Frames)>,
    caption: Option<(String, Arc<Pic>)>,
    waiting: HashSet<String>,
    steps: Option<i32>,
    shape: (usize, usize),
    wheel: Wheel,
    buttons: u8,
}

pub fn around(len: usize, index: Option<usize>, reach: usize) -> Vec<(i32, usize)> {
    let mut seen = HashSet::from([index.unwrap_or(usize::MAX)]);
    let mut found = Vec::new();
    for offset in (1..=reach as i32).flat_map(|n| [n, -n]) {
        let Some(at) = super::list::moved(index, len, offset) else { continue };
        if seen.insert(at) {
            found.push((offset, at));
        }
    }
    found
}

impl Daemon {
    pub(crate) fn session_key(&self, output: &str) -> Option<String> {
        let sessions = lock(&self.rotations.browse.sessions);
        [output, ""].into_iter().find(|k| sessions.contains_key(*k)).map(str::to_owned)
    }

    fn lineup(&self, output: &str, session: &Session) -> Vec<(i32, String)> {
        let middle = session.candidate.clone().or_else(|| session.shown.clone());
        let mut found: Vec<(i32, String)> = middle.into_iter().map(|u| (0, u)).collect();
        if session.source == BrowseSource::Playlist {
            for n in (1..=REACH as i32).flat_map(|n| [n, -n]) {
                let peeked = self.peek(output, session.steps + n).ok().and_then(|p| p.item);
                found.extend(peeked.filter(|u| found.iter().all(|(_, f)| f != u)).map(|u| (n, u)));
            }
        } else {
            let near = around(session.items.len(), session.index, REACH);
            found.extend(near.into_iter().map(|(n, at)| (n, session.items[at].clone())));
        }
        found
    }

    pub(crate) fn stage_sync(self: &Arc<Self>, output: &str) {
        let session = lock(&self.rotations.browse.sessions).get(output).cloned();
        let targets = self.targets(output);
        let Some(session) = session else {
            lock(&self.rotations.browse.stages).remove(output);
            targets.iter().for_each(|t| {
                self.compositor.strip(t, None);
                crate::inputs::browsing(t, false);
            });
            return;
        };
        let lineup = self.lineup(output, &session);
        let mut stages = lock(&self.rotations.browse.stages);
        let stage = stages.entry(output.to_owned()).or_default();
        let cards: Vec<(i32, Arc<Pic>)> =
            lineup.iter().filter_map(|(n, uid)| Some((*n, self.still(stage, output, uid)?))).collect();
        stage.pics.retain(|uid, _| lineup.iter().any(|(_, u)| u == uid));
        let middle = lineup.iter().find(|(n, _)| *n == 0).map(|(_, uid)| uid.clone());
        let motion = middle.as_deref().map(|uid| self.motion(stage, uid)).unwrap_or_default();
        let caption = middle.as_deref().and_then(|uid| self.caption(stage, uid));
        let before = lineup.iter().filter(|(n, _)| *n < 0).count();
        let after = lineup.iter().filter(|(n, _)| *n > 0).count();
        let moved = stage.steps.map_or(0, |was| session.steps - was);
        stage.steps = Some(session.steps);
        stage.shape = (before, after);
        let less = self.state().config.transition.less_motion.reduces(self.compositor.calmed());
        drop(stages);
        let spec = Arc::new(StripSpec { cards, motion, caption, before, after, moved, smooth: !less });
        targets.iter().for_each(|t| {
            self.compositor.strip(t, Some(spec.clone()));
            crate::inputs::browsing(t, true);
        });
    }
}
