use crate::auth::{Auth, Role};
use crate::db::{audit, changed, now};
use crate::error::{Input, Problem, require};
use crate::scope::{Scope, at, resolve};
use crate::{Reply, Router, St};
use axum::Json;
use axum::extract::State;
use axum::routing::{delete, get, post};
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;

pub fn routes() -> Router {
    let router = Router::new().route("/api/v1/mod/hidden", get(hidden));
    let router = at(
        router,
        "/api/v1/mod/hidden",
        post(|s: St, a: Auth, sc: Scope, Input(h): Input<Hide>| {
            hide(s, a, Hide { repo: sc.repo, ..h }, true)
        }),
    );
    let gone = delete(|s: St, a: Auth, sc: Scope| {
        hide(s, a, Hide { repo: sc.repo, id: sc.id, reason: String::new() }, false)
    });
    at(router, "/api/v1/mod/hidden/{id}", gone)
}

#[derive(Deserialize)]
pub struct Hide {
    #[serde(default, skip_deserializing)]
    repo: String,
    id: String,
    #[serde(default)]
    reason: String,
}

async fn hidden(State(app): St, auth: Auth) -> Reply {
    auth.at_least(Role::Moderator)?;
    let sql = "SELECT repo, id, hidden AS reason, hidden_at FROM packages WHERE hidden IS NOT NULL ORDER BY hidden_at DESC";
    app.db.items(sql, vec![]).await
}

async fn hide(State(app): St, auth: Auth, h: Hide, on: bool) -> Reply {
    auth.at_least(Role::Moderator)?;
    let (id, reason) = (h.id, h.reason.trim().to_string());
    let repo = resolve(&app, &h.repo)?;
    let name = repo.name.clone();
    require(!on || reason.chars().count() >= 10, "reason: at least 10 characters for the author")?;
    let (key, _turn) = (id.clone(), app.deciding.lock().await);
    let sql = "UPDATE packages SET hidden = ?2, hidden_at = ?3 WHERE repo = ?5 AND id = ?1 AND (hidden IS NULL) = ?4";
    app.db
        .run(move |c| {
            let (text, at) = (on.then_some(reason.clone()), on.then(now));
            let what = if on { "visible package" } else { "hidden package" };
            changed(c.execute(sql, params![key, text, at, on, name])?, what)?;
            audit(c, Some(auth.user), if on { "hide" } else { "unhide" }, &key, &reason)?;
            Ok(c.execute("UPDATE repos SET dirty = 1 WHERE name = ?1", [&name])?)
        })
        .await?;
    let key = id.clone();
    let marked = tokio::task::spawn_blocking(move || repo.hide(&key, on)).await.map_err(Problem::internal)?;
    if let Err(e) = marked {
        tracing::info!("a hidden package is not in the repository yet: {e}");
    }
    app.changed().await;
    Ok(Json(json!({"id": id, "hidden": on})))
}
