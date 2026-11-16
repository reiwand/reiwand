use crate::daemon::Daemon;
use futures_util::StreamExt;
use reiwand_core::dbus;
use reiwand_core::model::ShortcutsState;
use reiwand_core::shortcuts::Shortcuts;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Notify;
use zbus::Variant as Value;
use zbus::object_server::SignalEmitter;

pub(crate) mod launch;
mod portal;
#[cfg(test)]
mod portal_tests;

type Options<'a> = HashMap<&'a str, Value<'a>>;

static REBIND: Notify = Notify::const_new();
pub const NEXT: &str = "browse-next";
pub const PREVIOUS: &str = "browse-previous";

pub fn rebind() {
    REBIND.notify_one();
}

pub fn proposed(wanted: &Shortcuts) -> Vec<(&'static str, &'static str, Option<&str>)> {
    vec![
        (NEXT, "Preview the next wallpaper", Shortcuts::trigger(&wanted.next)),
        (PREVIOUS, "Preview the previous wallpaper", Shortcuts::trigger(&wanted.previous)),
    ]
}

pub fn steps(id: &str) -> Option<i32> {
    match id {
        NEXT => Some(1),
        PREVIOUS => Some(-1),
        _ => None,
    }
}

pub async fn serve(conn: zbus::Connection, daemon: Arc<Daemon>) {
    let emitter = SignalEmitter::new(&conn, dbus::OBJECT_PATH).map(SignalEmitter::into_owned);
    loop {
        let wanted = daemon.state().config.shortcuts.clone();
        let browse = &daemon.rotations.browse;
        if !wanted.enabled {
            browse.set_shortcuts(ShortcutsState::Off);
            REBIND.notified().await;
            continue;
        }
        browse.set_shortcuts(ShortcutsState::Asking);
        let bound = match zbus::Connection::session().await {
            Ok(own) => portal::bind(&own, &wanted).await.map(|(p, s)| (own, p, s)),
            Err(e) => Err(e.into()),
        };
        let (own, portal, session) = match bound {
            Ok(bound) => bound,
            Err(e) => {
                let refused = e.to_string() == "refused";
                browse.set_shortcuts(if refused {
                    ShortcutsState::Refused
                } else {
                    ShortcutsState::Unavailable
                });
                tracing::warn!("the desktop offers no global shortcuts for browsing wallpapers: {e:#}");
                REBIND.notified().await;
                continue;
            }
        };
        browse.set_shortcuts(ShortcutsState::Bound);
        let Ok(mut activated) = portal.receive_activated().await else { continue };
        loop {
            tokio::select! {
                () = REBIND.notified() => break,
                signal = activated.next() => {
                    let Some(signal) = signal else { break };
                    let Ok(args) = signal.args() else { continue };
                    let (Some(steps), Ok(emitter)) = (steps(args.shortcut_id), &emitter) else { continue };
                    if args.session_handle.as_str() == session.as_str() {
                        launch::pressed(&daemon, emitter, steps).await;
                    }
                }
            }
        }
        portal::close(&own, &session).await;
    }
}
