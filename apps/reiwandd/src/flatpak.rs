use crate::daemon::Daemon;
use rustix::fs::inotify::{self, CreateFlags, WatchFlags};
use rustix::io::Errno;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::sync::Weak;
use zbus::{OwnedObjectPath, Variant};

const MONITOR: &str = "/run/host/monitor";

#[zbus::proxy(
    interface = "org.freedesktop.portal.Background",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait Background {
    fn request_background(
        &self,
        parent_window: &str,
        options: HashMap<&str, Variant<'_>>,
    ) -> zbus::Result<OwnedObjectPath>;

    fn set_status(&self, options: HashMap<&str, Variant<'_>>) -> zbus::Result<()>;
}

/// The portal takes a status of at most 96 characters; a longer translation is cut at a character.
fn status(text: &str) -> String {
    text.chars().take(96).collect()
}

pub fn app_id() -> Option<String> {
    std::env::var("FLATPAK_ID").ok().filter(|id| !id.is_empty())
}

pub fn host_path(path: &Path) -> PathBuf {
    let info = app_id().and_then(|_| std::fs::read_to_string("/.flatpak-info").ok()).unwrap_or_default();
    let app = info.lines().find_map(|line| line.strip_prefix("app-path="));
    match (app, path.strip_prefix("/app")) {
        (Some(app), Ok(rest)) => Path::new(app).join(rest),
        _ => path.to_path_buf(),
    }
}

pub async fn request_background(connection: zbus::Connection) {
    if app_id().is_some() {
        ask(&connection).await;
    }
}

/// Asks the Background portal to let the service run with no window, and sets the line the
/// desktop shows for it.
async fn ask(connection: &zbus::Connection) {
    let reason = crate::words::background_reason();
    let options = HashMap::from([("reason", Variant::from(reason.as_str()))]);
    let portal = match BackgroundProxy::new(connection).await {
        Ok(portal) => portal,
        Err(e) => return tracing::warn!("cannot reach the background portal: {e}"),
    };
    if let Err(e) = portal.request_background("", options).await {
        tracing::warn!("cannot ask the background portal to keep reiwandd running: {e}");
    }
    // What the desktop's list of background apps says about the service (portal version 2 and later;
    // an older portal leaves the line empty, as before).
    let said = status(&crate::words::background_status());
    if let Err(e) = portal.set_status(HashMap::from([("message", Variant::from(said.as_str()))])).await {
        tracing::info!("the background portal takes no status line: {e}");
    }
}

pub fn follow_zone(daemon: Weak<Daemon>) -> std::io::Result<()> {
    watch(Path::new(MONITOR), "timezone", move |zone| {
        crate::zone::follow(zone.trim());
        daemon.upgrade().map(|daemon| daemon.rotations.retime()).is_some()
    })
}

fn watch(
    dir: &Path,
    name: &'static str,
    changed: impl Fn(&str) -> bool + Send + 'static,
) -> std::io::Result<()> {
    let fd = inotify::init(CreateFlags::CLOEXEC)?;
    inotify::add_watch(&fd, dir, WatchFlags::CLOSE_WRITE | WatchFlags::MOVED_TO)?;
    let path = dir.join(name);
    let watch = move || {
        let mut buffer = [MaybeUninit::uninit(); 1024];
        let mut reader = inotify::Reader::new(&fd, &mut buffer);
        loop {
            match reader.next() {
                Ok(event) if event.file_name().is_some_and(|n| n.to_bytes() == name.as_bytes()) => {
                    if std::fs::read_to_string(&path).is_ok_and(|text| !changed(&text)) {
                        return;
                    }
                }
                Ok(_) | Err(Errno::INTR) => {}
                Err(_) => return,
            }
        }
    };
    std::thread::Builder::new().name("reiwand-zone".into()).spawn(watch).map(drop)
}

pub(crate) fn not_found(path: &std::path::Path) -> String {
    match app_id() {
        Some(id) => format!(
            "cannot find {} (if it exists, give access to its folder with: flatpak override --user --filesystem=\"{}:ro\" {id})",
            path.display(),
            path.parent().unwrap_or(path).display()
        ),
        None => format!("cannot find {}", path.display()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Portal(tokio::sync::mpsc::UnboundedSender<(String, String)>);

    #[zbus::interface(name = "org.freedesktop.portal.Background")]
    impl Portal {
        fn request_background(
            &self,
            _parent: String,
            options: HashMap<String, zbus::OwnedVariant>,
        ) -> OwnedObjectPath {
            let reason =
                options.get("reason").and_then(|v| String::try_from(v.clone()).ok()).unwrap_or_default();
            drop(self.0.send(("reason".into(), reason)));
            OwnedObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/t").unwrap()
        }

        fn set_status(&self, options: HashMap<String, zbus::OwnedVariant>) {
            let said =
                options.get("message").and_then(|v| String::try_from(v.clone()).ok()).unwrap_or_default();
            drop(self.0.send(("message".into(), said)));
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn the_service_asks_to_run_in_the_background_and_says_what_it_does() {
        use crate::confirm::tests::{connect, private_bus};
        let Some(bus) = private_bus() else { return };
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let portal = connect(&bus.1).await;
        portal.object_server().at("/org/freedesktop/portal/desktop", Portal(tx)).await.unwrap();
        portal.request_name("org.freedesktop.portal.Desktop").await.unwrap();
        ask(&connect(&bus.1).await).await;
        let mut seen = HashMap::new();
        while let Ok(Some((key, value))) =
            tokio::time::timeout(std::time::Duration::from_secs(5), rx.recv()).await
        {
            seen.insert(key, value);
            if seen.len() == 2 {
                break;
            }
        }
        assert_eq!(seen.get("reason").map(String::as_str), Some("Draws the desktop wallpaper"));
        assert_eq!(seen.get("message").map(String::as_str), Some("Renders your wallpapers"));
    }

    #[test]
    fn the_background_status_fits_the_portal() {
        assert_eq!(status(&crate::words::background_status()), "Renders your wallpapers");
        assert_eq!(status(&"é".repeat(120)).chars().count(), 96);
    }

    #[test]
    fn a_replaced_zone_file_is_read_again() {
        let dir = tempfile::tempdir().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        watch(dir.path(), "timezone", move |zone| tx.send(zone.to_owned()).is_ok()).unwrap();
        std::fs::write(dir.path().join("other"), "Etc/UTC\n").unwrap();
        std::fs::write(dir.path().join("timezone.new"), "Etc/GMT-2\n").unwrap();
        std::fs::rename(dir.path().join("timezone.new"), dir.path().join("timezone")).unwrap();
        let read = rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        assert_eq!(read, "Etc/GMT-2\n");
    }
}
