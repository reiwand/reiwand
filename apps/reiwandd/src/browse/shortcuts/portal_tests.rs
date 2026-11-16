use super::portal::{APP_ID, bind};
use crate::confirm::tests::{Bus, connect, private_bus};
use reiwand_core::shortcuts::Shortcuts;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use zbus::message::Header;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

#[derive(Default)]
struct Seen {
    talked: HashSet<String>,
    registered: HashMap<String, String>,
    bound: Vec<String>,
}

type Shared = Arc<Mutex<Seen>>;

struct Portal(Shared);

fn sender(header: &Header<'_>) -> String {
    header.sender().map(|s| s.to_string()).unwrap_or_default()
}

fn failure(name: &str, text: &str) -> zbus::fdo::Error {
    zbus::fdo::Error::Failed(format!("{name}: {text}"))
}

async fn answer(conn: &zbus::Connection, from: &str, token: &str, extra: &str) {
    let path = format!(
        "/org/freedesktop/portal/desktop/request/{}/{token}",
        from.trim_start_matches(':').replace('.', "_")
    );
    let results: HashMap<&str, Value<'_>> = HashMap::from([("session_handle", Value::from(extra))]);
    let body = (0u32, results);
    conn.emit_signal(None::<&str>, path.as_str(), "org.freedesktop.portal.Request", "Response", &body)
        .await
        .unwrap();
}

#[zbus::interface(name = "org.freedesktop.host.portal.Registry")]
impl Portal {
    async fn register(
        &self,
        app_id: String,
        _options: HashMap<String, OwnedValue>,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<()> {
        let from = sender(&header);
        let mut seen = self.0.lock().unwrap();
        if !seen.talked.insert(from.clone()) {
            return Err(failure("Failed", "Connection already associated with an application ID"));
        }
        seen.registered.insert(from, app_id);
        Ok(())
    }
}

struct Shortcuts1(Shared);

#[zbus::interface(name = "org.freedesktop.portal.GlobalShortcuts")]
impl Shortcuts1 {
    async fn create_session(
        &self,
        options: HashMap<String, OwnedValue>,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] conn: &zbus::Connection,
    ) -> zbus::fdo::Result<OwnedObjectPath> {
        let from = sender(&header);
        if !self.0.lock().unwrap().registered.contains_key(&from) {
            return Err(failure("NotAllowed", "An app id is required"));
        }
        let token = String::try_from(options["handle_token"].try_clone().unwrap()).unwrap();
        let path = format!(
            "/org/freedesktop/portal/desktop/request/{}/{token}",
            from.trim_start_matches(':').replace('.', "_")
        );
        let conn = conn.clone();
        tokio::spawn(async move { answer(&conn, &from, &token, "/session/one").await });
        Ok(OwnedObjectPath::try_from(path).unwrap())
    }

    async fn bind_shortcuts(
        &self,
        _session: OwnedObjectPath,
        shortcuts: Vec<(String, HashMap<String, OwnedValue>)>,
        _parent: String,
        options: HashMap<String, OwnedValue>,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] conn: &zbus::Connection,
    ) -> zbus::fdo::Result<OwnedObjectPath> {
        let from = sender(&header);
        self.0.lock().unwrap().bound.extend(shortcuts.into_iter().map(|(id, _)| id));
        let token = String::try_from(options["handle_token"].try_clone().unwrap()).unwrap();
        let path = format!(
            "/org/freedesktop/portal/desktop/request/{}/{token}",
            from.trim_start_matches(':').replace('.', "_")
        );
        let conn = conn.clone();
        tokio::spawn(async move { answer(&conn, &from, &token, "").await });
        Ok(OwnedObjectPath::try_from(path).unwrap())
    }
}

async fn fake(bus: &Bus) -> (zbus::Connection, Shared) {
    let shared = Shared::default();
    let server = connect(&bus.1).await;
    let at = "/org/freedesktop/portal/desktop";
    server.object_server().at(at, Portal(shared.clone())).await.unwrap();
    server.object_server().at(at, Shortcuts1(shared.clone())).await.unwrap();
    server.request_name("org.freedesktop.portal.Desktop").await.unwrap();
    (server, shared)
}

#[tokio::test]
async fn the_daemon_registers_before_binding() {
    let Some(bus) = private_bus() else { return };
    let (_server, seen) = fake(&bus).await;
    let own = connect(&bus.1).await;
    let wanted = Shortcuts::default();
    bind(&own, &wanted).await.unwrap();
    let seen = seen.lock().unwrap();
    assert_eq!(seen.registered.values().collect::<Vec<_>>(), [APP_ID]);
    assert_eq!(seen.bound, ["browse-next", "browse-previous"]);
}

#[tokio::test]
async fn a_connection_that_already_used_the_portal_is_refused() {
    let Some(bus) = private_bus() else { return };
    let (_server, seen) = fake(&bus).await;
    let used = connect(&bus.1).await;
    let at = "/org/freedesktop/portal/desktop";
    let _ = used
        .call_method(
            Some("org.freedesktop.portal.Desktop"),
            at,
            Some("org.freedesktop.DBus.Peer"),
            "Ping",
            &(),
        )
        .await;
    seen.lock().unwrap().talked.insert(used.unique_name().unwrap().to_string());
    let error = bind(&used, &Shortcuts::default()).await.unwrap_err();
    assert!(format!("{error:#}").contains("An app id is required"), "{error:#}");
    assert!(seen.lock().unwrap().bound.is_empty());
}
