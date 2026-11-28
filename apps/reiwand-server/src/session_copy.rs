use crate::auth::Auth;
use crate::creds::{self, digest};
use crate::db::{DAY, now};
use crate::error::{Problem, Res, require};
use crate::{Router, St};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;

const SHARED: [&str; 2] = ["catalog:read", "upload"];

pub fn routes() -> Router {
    Router::new().route("/api/v1/auth/session/copy", post(copy))
}

#[derive(Deserialize, Default)]
struct Copy {
    #[serde(default)]
    name: Option<String>,
}

async fn copy(State(app): St, auth: Auth, body: axum::body::Bytes) -> Res<impl IntoResponse> {
    let asked: Copy = if body.is_empty() {
        Copy::default()
    } else {
        serde_json::from_slice(&body).map_err(|e| Problem::bad(e.to_string()))?
    };
    let name = asked.name.unwrap_or_else(|| "Copy".into());
    let name = name.trim().to_string();
    require(!name.is_empty() && name.chars().count() <= 64, "name: 1 to 64 characters")?;
    let kept: Vec<&str> = SHARED.iter().copied().filter(|s| auth.scope(s).is_ok()).collect();
    require(
        !kept.is_empty(),
        "this token cannot be copied: it has neither the catalog:read nor the upload scope",
    )?;
    let scopes = kept.join(" ");
    let (token, user, source) = (creds::token('p'), auth.user, auth.token);
    let (hash, shown, called) = (digest(&token), scopes.clone(), name.clone());
    let made = app.db.run(move |c| {
        // A copy never outlives a year nor the token it was made from (a device token has no end).
        let expires: Option<i64> = match source {
            Some(id) => c.query_row("SELECT expires_at FROM tokens WHERE id = ?1", [id], |r| r.get(0))?,
            None => None,
        };
        let expires = Some(expires.map_or(now() + 365 * DAY, |at: i64| at.min(now() + 365 * DAY)));
        let sql = "INSERT INTO tokens (user_id, name, hash, scopes, created_at, expires_at) \
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
        c.execute(sql, params![user, name, hash, scopes, now(), expires])?;
        Ok((c.last_insert_rowid(), expires))
    });
    let (id, expires) = made.await?;
    let created = json!({"id": id, "name": called, "token": token, "scopes": shown, "expires_at": expires});
    Ok((StatusCode::CREATED, Json(created)))
}
