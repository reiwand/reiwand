use super::link;
use crate::compositor::lock;
use crate::daemon::Daemon;
use reiwand_core::strip::{notches, place};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Do {
    Move(i32),
    Apply,
    Cancel,
}

pub(crate) async fn run(daemon: &Arc<Daemon>, key: &str, act: Do) {
    let Some(link) = link() else { return };
    let key = key.to_owned();
    let done = daemon
        .run(move |d| match act {
            Do::Move(steps) => d.browse_move(&key, "", steps),
            Do::Apply => d.browse_apply(&key, ""),
            Do::Cancel => d.browse_cancel(&key),
        })
        .await;
    match done {
        Ok(told) => drop(super::super::told(&Arc::downgrade(daemon), link.emitter.clone(), told).await),
        Err(e) => tracing::info!("cannot browse wallpapers: {e}"),
    }
}

fn spawn(daemon: Arc<Daemon>, key: String, act: Do) {
    if let Some(link) = link() {
        link.runtime.spawn(async move { run(&daemon, &key, act).await });
    }
}

pub(crate) fn pointer(output: &str, pos: Option<[f32; 2]>, buttons: u8) -> bool {
    let Some(daemon) = link().and_then(|l| l.daemon.upgrade()) else { return false };
    let Some(key) = daemon.session_key(output) else { return false };
    let size = daemon.state().outputs.get(output).map(|o| o.size);
    let mut stages = lock(&daemon.rotations.browse.stages);
    let Some(stage) = stages.get_mut(&key) else { return true };
    let pressed = buttons & 1 != 0 && stage.buttons & 1 == 0;
    stage.buttons = buttons;
    let (before, after) = stage.shape;
    let (Some([x, y]), Some(size)) = (pos, size) else { return true };
    drop(stages);
    if pressed {
        let plan = place(size, before, after, 0.0);
        let hit = plan.hit(x * size.0 as f32, y * size.1 as f32);
        match hit {
            Some(0) => spawn(daemon, key, Do::Apply),
            Some(steps) => spawn(daemon, key, Do::Move(steps)),
            None => {}
        }
    }
    true
}

pub(crate) fn wheel(output: &str, discrete: i32, pixels: f64) -> bool {
    let Some(daemon) = link().and_then(|l| l.daemon.upgrade()) else { return false };
    let Some(key) = daemon.session_key(output) else { return false };
    let steps = lock(&daemon.rotations.browse.stages)
        .get_mut(&key)
        .map_or(0, |stage| stage.wheel.push(notches(discrete, pixels)));
    if steps != 0 {
        spawn(daemon, key, Do::Move(steps));
    }
    true
}
