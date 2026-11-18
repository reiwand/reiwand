use crate::callers::caller;
use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::Result;
use crate::inputs::{declined_path, grants_path, load, load_bound};
use std::sync::Arc;
use zbus::message::Header;

pub(crate) fn start(daemon: &Arc<Daemon>, connection: zbus::Connection) -> anyhow::Result<()> {
    let path = grants_path(daemon);
    let grants = (load_bound(&path)?, load(&declined_path(&path))?);
    crate::inputs::start(daemon, connection, grants);
    Ok(())
}

impl super::Inputs {
    pub(super) async fn give(
        &self,
        call: (&zbus::Connection, &Header<'_>),
        scene: &str,
        key: Option<&str>,
        input: &str,
    ) -> Result<()> {
        let (d, hub) = self.parts()?;
        let (input, scene) = (super::ids::parse(input)?, super::ids::scene_id(scene)?.to_owned());
        let (key, title) = lock(&hub.control).package(&scene, key)?;
        let trusted = caller(call.0, call.1).await.trusted();
        if !crate::confirm::askable(input) && !trusted {
            return Err(crate::confirm::lights());
        }
        if !trusted {
            if lock(&hub.control).has(&scene, &key, input) {
                return Ok(());
            }
            return crate::confirm::grant(call.0, hub.clone(), grants_path(&d), (scene, key, title), input)
                .await;
        }
        d.run(move |d| {
            hub.grant(&grants_path(d), &scene, &key, input)?;
            super::ids::reopen(d, input)
        })
        .await
    }
}

mod decide;
