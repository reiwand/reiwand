use super::session::confirm;
use crate::St;
use crate::auth::Auth;
use crate::creds::{self, check_password};
use crate::db::changed;
use crate::error::{Input, Res};
use crate::net::Client;
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use rusqlite::params;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Forget {
    #[serde(default)]
    password: String,
}

pub(super) async fn forget(
    State(app): St,
    auth: Auth,
    client: Client,
    Input(f): Input<Forget>,
) -> Res<impl IntoResponse> {
    auth.browser()?;
    confirm(&app, &auth, &client, f.password).await?;
    app.db
        .run(move |c| changed(c.execute("DELETE FROM users WHERE id = ?1", [auth.user])?, "account"))
        .await?;
    app.changed().await;
    Ok((StatusCode::NO_CONTENT, [(header::SET_COOKIE, creds::cookie(&app.cfg, "", 0))]))
}

#[derive(Deserialize)]
pub(super) struct Change {
    current: String,
    new: String,
    #[serde(default)]
    devices: bool,
}

pub(super) async fn password(
    State(app): St,
    auth: Auth,
    client: Client,
    Input(change): Input<Change>,
) -> Res<impl IntoResponse> {
    auth.browser()?;
    confirm(&app, &auth, &client, change.current).await?;
    check_password(&auth.handle, &change.new)?;
    let (hash, user, devices) = (creds::hash(&app, change.new).await?, auth.user, change.devices);
    let session = app
        .db
        .run(move |c| {
            let tx = c.transaction()?;
            tx.execute("UPDATE users SET password_hash = ?2 WHERE id = ?1", params![user, hash])?;
            tx.execute("DELETE FROM sessions WHERE user_id = ?1", [user])?;
            tx.execute("DELETE FROM tokens WHERE user_id = ?1 AND device_id IS NULL", [user])?;
            tx.execute("DELETE FROM device_codes WHERE user_id = ?1", [user])?;
            tx.execute("DELETE FROM devices WHERE user_id = ?1 AND ?2", params![user, devices])?;
            let session = creds::session(&tx, user)?;
            tx.commit()?;
            Ok(session)
        })
        .await?;
    Ok((StatusCode::NO_CONTENT, [(header::SET_COOKIE, creds::cookie(&app.cfg, &session, 30 * 86400))]))
}
