use crate::compositor::lock;
use crate::daemon::Daemon;
use std::sync::Arc;
use zbus::object_server::SignalEmitter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Do {
    Move(i32),
    Apply,
    Cancel,
}

impl Daemon {
    pub(crate) fn browse_key(&self) -> String {
        let sessions = lock(&self.rotations.browse.sessions);
        let mut open: Vec<&String> = sessions.keys().collect();
        open.sort();
        open.first().map_or_else(|| self.browse_output(""), |key| (*key).clone())
    }
}

pub(crate) async fn run(daemon: &Arc<Daemon>, emitter: &SignalEmitter<'static>, act: Do) {
    let done = daemon
        .run(move |d| {
            let key = d.browse_key();
            match act {
                Do::Move(steps) => d.browse_move(&key, "", steps),
                Do::Apply => d.browse_apply(&key, ""),
                Do::Cancel => d.browse_cancel(&key),
            }
        })
        .await;
    match done {
        Ok(told) => drop(super::told(&Arc::downgrade(daemon), emitter.clone(), told).await),
        Err(e) => tracing::info!("cannot browse wallpapers: {e}"),
    }
}
