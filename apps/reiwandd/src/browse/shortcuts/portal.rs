use super::Options;
use futures_util::StreamExt;
use reiwand_core::shortcuts::Shortcuts;
use std::collections::HashMap;
use zbus::proxy::CacheProperties;
use zbus::{ObjectPath, OwnedObjectPath, OwnedVariant as OwnedValue, Variant as Value};

pub const APP_ID: &str = "io.github.reiwand";

#[zbus::proxy(
    interface = "org.freedesktop.portal.GlobalShortcuts",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
pub(super) trait GlobalShortcuts {
    fn create_session(&self, options: Options<'_>) -> zbus::Result<OwnedObjectPath>;
    fn bind_shortcuts(
        &self,
        session_handle: &ObjectPath<'_>,
        shortcuts: &[(&str, Options<'_>)],
        parent_window: &str,
        options: Options<'_>,
    ) -> zbus::Result<OwnedObjectPath>;
    #[zbus(signal)]
    fn activated(
        &self,
        session_handle: ObjectPath<'_>,
        shortcut_id: &str,
        timestamp: u64,
        options: HashMap<&str, OwnedValue>,
    ) -> zbus::Result<()>;
}

#[zbus::proxy(
    interface = "org.freedesktop.portal.Request",
    default_service = "org.freedesktop.portal.Desktop"
)]
trait Request {
    #[zbus(signal)]
    fn response(&self, response: u32, results: HashMap<&str, OwnedValue>) -> zbus::Result<()>;
}

#[zbus::proxy(
    interface = "org.freedesktop.portal.Session",
    default_service = "org.freedesktop.portal.Desktop"
)]
trait Session {
    fn close(&self) -> zbus::Result<()>;
}

async fn asked(
    conn: &zbus::Connection,
    token: &str,
    call: impl AsyncFnOnce() -> zbus::Result<OwnedObjectPath>,
) -> anyhow::Result<HashMap<String, OwnedValue>> {
    let sender = conn.unique_name().map(|n| n.trim_start_matches(':').replace('.', "_")).unwrap_or_default();
    let path = format!("/org/freedesktop/portal/desktop/request/{sender}/{token}");
    let request =
        RequestProxy::builder(conn).path(path).cache_properties(CacheProperties::No).build().await?;
    let mut responses = request.receive_response().await?;
    call().await?;
    let response = responses.next().await.ok_or_else(|| anyhow::anyhow!("the portal did not answer"))?;
    let args = response.args()?;
    anyhow::ensure!(args.response == 0, "refused");
    Ok(args.results.into_iter().map(|(k, v)| (k.to_owned(), v)).collect())
}

pub(super) async fn bind(
    conn: &zbus::Connection,
    wanted: &Shortcuts,
) -> anyhow::Result<(GlobalShortcutsProxy<'static>, OwnedObjectPath)> {
    if crate::flatpak::app_id().is_none() {
        let options: Options<'_> = HashMap::new();
        let registry = (Some("org.freedesktop.portal.Desktop"), "/org/freedesktop/portal/desktop");
        let iface = Some("org.freedesktop.host.portal.Registry");
        if let Err(e) = conn.call_method(registry.0, registry.1, iface, "Register", &(APP_ID, options)).await
        {
            tracing::warn!("the desktop portal did not accept the app id {APP_ID}: {e}");
        }
    }
    let portal = GlobalShortcutsProxy::new(conn).await?;
    let token = format!("reiwand{}", std::process::id());
    let options = || {
        HashMap::from([
            ("handle_token", Value::from(token.as_str())),
            ("session_handle_token", Value::from(token.as_str())),
        ])
    };
    let created = asked(conn, &token, async || portal.create_session(options()).await).await?;
    let handle =
        created.get("session_handle").ok_or_else(|| anyhow::anyhow!("the portal gave no session"))?;
    let session = OwnedObjectPath::try_from(String::try_from(handle.try_clone()?)?)?;
    let shortcuts = super::proposed(wanted);
    let described: Vec<(&str, Options<'_>)> = shortcuts
        .iter()
        .map(|(id, text, trigger)| {
            let mut options = HashMap::from([("description", Value::from(*text))]);
            options.extend(trigger.map(|t| ("preferred_trigger", Value::from(t))));
            (*id, options)
        })
        .collect();
    let bound = format!("{token}b");
    let options = HashMap::from([("handle_token", Value::from(bound.as_str()))]);
    asked(conn, &bound, async || portal.bind_shortcuts(&session, &described, "", options).await).await?;
    Ok((portal, session))
}

pub(super) async fn close(conn: &zbus::Connection, session: &OwnedObjectPath) {
    let proxy = SessionProxy::builder(conn).path(session.clone()).cache_properties(CacheProperties::No);
    if let Ok(proxy) = proxy.build().await {
        let _ = proxy.close().await;
    }
}
