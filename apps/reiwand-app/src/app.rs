use crate::bus::{self, Daemon, Event, publish, set_link, spawn};
use futures_util::StreamExt;
use percent_encoding::percent_decode_str;
use reiwand_client::{Client, Daemon1Proxy, Library1Proxy, Playlists1Proxy, Repos1Proxy, explain};
use reiwand_core::dbus::{APP_ID, APP_PATH, BUS_NAME};
use std::collections::HashMap;
use std::process::ExitCode;
use zbus::Connection;
use zbus::fdo::{DBusProxy, RequestNameFlags};
use zbus::proxy::CacheProperties;
use zbus::{OwnedVariant, Variant};

const APPLICATION: &str = "org.freedesktop.Application";
pub const PAGES: [&str; 9] = [
    "library",
    "playlists",
    "profiles",
    "providers",
    "repos",
    "settings",
    "account",
    "scenes",
    "repositories",
];

pub fn page(name: Option<&str>) -> Option<&'static str> {
    PAGES.into_iter().find(|page| name == Some(page))
}

pub fn start(uris: Vec<String>, page: Option<&str>) -> Option<ExitCode> {
    if let Some(page) = page.map(String::from).or_else(|| crate::addlink::page(&uris)) {
        bus::request(&page, None);
    }
    if bus::checking() {
        set_link(Ok(Daemon::Fixtures));
        return None;
    }
    bus::block_on(async move {
        let conn = match reiwand_client::bus::session().await {
            Ok(conn) => conn,
            Err(e) => {
                set_link(Err(format!("cannot connect to the session bus: {e}")));
                return None;
            }
        };
        if conn.object_server().at(APP_PATH, Application).await.is_ok()
            && let Err(zbus::Error::NameTaken) =
                conn.request_name_with_flags(APP_ID, RequestNameFlags::DO_NOT_QUEUE).await
            && let Some(code) = hand_off(&conn, &uris, page).await
        {
            return Some(code);
        }
        spawn(watch(conn));
        None
    })
}

/// Gives the request to the window already open. When that one is quitting and cannot take it,
/// this one waits for it to leave (up to 5 s) and opens in its place: `None` means it is now the app.
async fn hand_off(conn: &Connection, uris: &[String], page: Option<&str>) -> Option<ExitCode> {
    let until = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let Err(e) = pass_on(conn, uris.to_vec(), page).await else { return Some(ExitCode::SUCCESS) };
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        match conn.request_name_with_flags(APP_ID, RequestNameFlags::DO_NOT_QUEUE).await {
            Ok(_) => return None,
            Err(zbus::Error::NameTaken) if std::time::Instant::now() < until => continue,
            _ => {
                eprintln!(
                    "reiwand-app: the Reiwand window already open did not take the request: {}",
                    explain(e)
                );
                return Some(ExitCode::FAILURE);
            }
        }
    }
}

pub async fn pass_on(conn: &Connection, uris: Vec<String>, page: Option<&str>) -> zbus::Result<()> {
    let to = (Some(APP_ID), APP_PATH, Some(APPLICATION));
    let names = [("activation-token", "XDG_ACTIVATION_TOKEN"), ("desktop-startup-id", "DESKTOP_STARTUP_ID")];
    let found = names.map(|(key, var)| (key, std::env::var(var).ok().filter(|v| !v.is_empty())));
    let data: HashMap<&str, Variant> =
        found.into_iter().filter_map(|(key, value)| Some((key, Variant::from(value?)))).collect();
    if !uris.is_empty() {
        conn.call_method(to.0, to.1, to.2, "Open", &(&uris, &data)).await?;
    }
    if let Some(page) = page {
        conn.call_method(to.0, to.1, to.2, "ActivateAction", &(page, Vec::<Variant>::new(), &data)).await?;
    } else if uris.is_empty() {
        conn.call_method(to.0, to.1, to.2, "Activate", &(&data,)).await?;
    }
    Ok(())
}

pub(crate) async fn watch(conn: Connection) {
    let Ok(owners) = DBusProxy::new(&conn).await else { return };
    let Ok(mut owners) = owners.receive_name_owner_changed_with_args(&[(0, BUS_NAME)]).await else { return };
    let signals = conn.clone();
    spawn(async move {
        forward(signals).await.ok();
    });
    connect(&conn).await;
    while let Some(change) = owners.next().await {
        match change.args().map(|a| a.new_owner.is_some()) {
            Ok(true) => connect(&conn).await,
            Ok(false) => set_link(Err(crate::tr::tr("The wallpaper service stopped."))),
            Err(_) => {}
        }
    }
}

async fn connect(conn: &Connection) {
    set_link(Client::new(conn.clone()).await.map(Daemon::Bus).map_err(|e| {
        match reiwand_client::error_name(&e).is_some_and(crate::events::absent) {
            true => crate::tr::tr("The wallpaper service does not answer"),
            false => e.to_string(),
        }
    }));
}

async fn forward(conn: Connection) -> zbus::Result<()> {
    let daemon = Daemon1Proxy::builder(&conn).cache_properties(CacheProperties::No).build().await?;
    let library = Library1Proxy::builder(&conn).cache_properties(CacheProperties::No).build().await?;
    let repos = Repos1Proxy::builder(&conn).cache_properties(CacheProperties::No).build().await?;
    let (mut changed, mut progress) = (daemon.receive_changed().await?, daemon.receive_job_progress().await?);
    let (mut finished, mut thumbs) =
        (daemon.receive_job_finished().await?, library.receive_thumbnail_ready().await?);
    let mut previews = repos.receive_preview_ready().await?;
    let playlists = Playlists1Proxy::builder(&conn).cache_properties(CacheProperties::No).build().await?;
    let mut browsed = playlists.receive_browse_changed().await?;
    loop {
        let event = tokio::select! {
            Some(s) = changed.next() => s.args().map(|a| {
                bus::saw(a.topic, a.serial);
                Event::Changed(a.topic.into(), a.ids, a.serial)
            }),
            Some(s) = progress.next() => s.args().map(|a| Event::Progress(a.job, a.done, a.total)),
            Some(s) = finished.next() => {
                s.args().map(|a| Event::Finished(a.job, a.state.into(), a.error_name.into(), a.message.into()))
            }
            Some(s) = thumbs.next() => s.args().map(|a| Event::Thumb(a.uid.into(), a.size, a.path.into())),
            Some(s) = previews.next() => s.args().map(|a| Event::Preview(a.spec.into(), a.path.into())),
            Some(s) = browsed.next() => s.args().map(|a| Event::Changed("browse".into(), vec![a.output.into(), a.json.into()], 0)),
            else => return Ok(()),
        };
        event.map(publish).ok();
    }
}

pub fn path(uri: &str) -> Option<String> {
    let path = match uri.strip_prefix("file://") {
        Some(rest) => percent_decode_str(rest).decode_utf8().ok()?.into_owned(),
        None => uri.to_owned(),
    };
    path.starts_with('/').then_some(path)
}

pub fn write(path: &str, text: &str) -> bool {
    if let Some(parent) = std::path::Path::new(path).parent() {
        drop(std::fs::create_dir_all(parent));
    }
    std::fs::write(path, text).is_ok()
}

pub struct Application;

fn token(data: &HashMap<String, OwnedVariant>) -> Option<String> {
    let found = ["activation-token", "desktop-startup-id"].into_iter().filter_map(|key| data.get(key));
    found.filter_map(|v| <&str>::try_from(&**v).ok()).find(|v| !v.is_empty()).map(String::from)
}

#[zbus::interface(name = "org.freedesktop.Application")]
impl Application {
    fn activate(&self, platform_data: HashMap<String, OwnedVariant>) {
        publish(Event::Activate(String::new(), token(&platform_data)));
    }

    fn open(&self, uris: Vec<String>, platform_data: HashMap<String, OwnedVariant>) {
        match crate::addlink::page(&uris) {
            Some(page) => bus::request(&page, token(&platform_data)),
            None => publish(Event::Activate(String::new(), token(&platform_data))),
        }
    }

    fn activate_action(
        &self,
        name: String,
        _parameter: Vec<OwnedVariant>,
        platform_data: HashMap<String, OwnedVariant>,
    ) {
        let token = token(&platform_data);
        match page(Some(&name)) {
            Some(page) => bus::request(page, token),
            None => publish(Event::Activate(String::new(), token)),
        }
    }
}
