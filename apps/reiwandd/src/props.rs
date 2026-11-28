use crate::daemon::Daemon;
use crate::service::Service;
use reiwand_core::dbus;
use std::sync::Arc;

pub async fn properties(connection: zbus::Connection, daemon: Arc<Daemon>) {
    let mut changes = daemon.props.subscribe();
    let follow = async {
        let iface = connection.object_server().interface::<_, Service>(dbus::OBJECT_PATH).await?;
        let mut last = *changes.borrow_and_update();
        while changes.changed().await.is_ok() {
            let now = *changes.borrow_and_update();
            let (service, emitter) = (iface.get().await, iface.signal_emitter());
            if now.paused != last.paused {
                service.paused_changed(emitter).await?;
            }
            if now.volume != last.volume {
                service.volume_changed(emitter).await?;
            }
            if now.muted != last.muted {
                service.muted_changed(emitter).await?;
            }
            if now.private != last.private {
                service.private_mode_changed(emitter).await?;
            }
            last = now;
        }
        zbus::Result::Ok(())
    };
    if let Err(e) = follow.await {
        tracing::warn!("cannot announce property changes: {e}");
    }
}
