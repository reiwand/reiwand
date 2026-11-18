use crate::daemon::Daemon;
use std::sync::Weak;
use std::time::Duration;

const KWIN: &str = "org.kde.KWin";
const WAIT: Duration = Duration::from_millis(150);

async fn active(conn: &zbus::Connection) -> Option<String> {
    let bus = zbus::fdo::DBusProxy::new(conn).await.ok()?;
    let name = zbus::names::BusName::try_from(KWIN).ok()?;
    if !bus.name_has_owner(name).await.ok()? {
        return None;
    }
    let kwin = zbus::Proxy::new(conn, KWIN, "/KWin", KWIN).await.ok()?;
    tokio::time::timeout(WAIT, kwin.call::<_, _, String>("activeOutputName", &())).await.ok()?.ok()
}

pub async fn resolve(daemon: &Weak<Daemon>, conn: &zbus::Connection, output: String) -> String {
    let Some(daemon) = daemon.upgrade().filter(|_| output.is_empty()) else { return output };
    if !daemon.browse_output("").is_empty() {
        return output;
    }
    match active(conn).await {
        Some(name) if daemon.state().outputs.contains_key(&name) => name,
        _ => output,
    }
}
