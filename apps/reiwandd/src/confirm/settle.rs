use super::{ASKED, QUIET, ask, askable, lights, nobody, notifications, open, start, unconfirmed};
use crate::compositor::lock;
use crate::error::Result;
use crate::inputs::Hub;
use reiwand_core::model::Input;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

pub(super) async fn ask_group(
    bus: zbus::Connection,
    hub: Arc<Hub>,
    path: PathBuf,
    (scene, key, title): (String, String, String),
    group: Vec<Input>,
) {
    let answer = ask(&bus, crate::words::request_group(&title, &group)).await;
    let answer = answer.inspect_err(|e| tracing::warn!("cannot ask the user: {e}"));
    let quiet = matches!(answer, Ok(None | Some(false)));
    if let Ok(Some(allowed)) = answer {
        let (hub, path, scene, key, group) =
            (hub.clone(), path.clone(), scene.clone(), key.clone(), group.clone());
        let set = tokio::task::spawn_blocking(move || {
            for input in group {
                let done = match allowed {
                    true => hub.grant(&path, &scene, &key, input),
                    false => hub.decline(&path, &scene, &key, input),
                };
                if let Err(e) = done {
                    tracing::warn!("cannot keep the answer of the user: {e:#}");
                }
            }
        });
        let _ = set.await;
    }
    let until = quiet.then(|| Instant::now() + QUIET);
    {
        let mut all = lock(&ASKED);
        for input in &group {
            let asked = (scene.clone(), *input);
            match until {
                Some(until) => drop(all.insert(asked, Some(until))),
                None => drop(all.remove(&asked)),
            }
        }
    }
    hub.changed();
    let Some(until) = until else { return };
    tokio::time::sleep_until(until.into()).await;
    let mut all = lock(&ASKED);
    let mut freed = false;
    for input in &group {
        let asked = (scene.clone(), *input);
        if all.get(&asked) == Some(&Some(until)) {
            all.remove(&asked);
            freed = true;
        }
    }
    drop(all);
    if freed {
        hub.changed();
    }
}

pub(crate) async fn decide(
    bus: &zbus::Connection,
    hub: Arc<Hub>,
    path: PathBuf,
    package: (String, String, String),
    allow: Vec<Input>,
) -> Result<()> {
    let (asked, kept): (Vec<Input>, Vec<Input>) = allow.into_iter().partition(|i| askable(*i));
    if asked.is_empty() {
        return Err(lights());
    }
    if !notifications(bus).await {
        return Err(nobody());
    }
    let now = Instant::now();
    let opened: Vec<_> = asked.into_iter().map(|i| (i, open(&(package.0.clone(), i), now))).collect();
    let group: Vec<Input> = opened.iter().filter(|o| matches!(o.1, Ok(true))).map(|o| o.0).collect();
    let asking = opened.iter().any(|o| o.1.is_ok());
    let resting = opened.into_iter().find_map(|o| o.1.err());
    if !group.is_empty() {
        start(bus, hub, path, package, group);
    }
    match (kept.is_empty(), resting) {
        (false, _) => Err(lights()),
        (true, Some(quiet)) if !asking => Err(quiet),
        _ => Err(unconfirmed()),
    }
}
