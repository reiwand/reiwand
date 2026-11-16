use crate::auth::Auth;
use crate::db::{audit, changed};
use crate::error::{Input, Problem};
use crate::upload::listed;
use crate::{Reply, Router, St};
use axum::Json;
use axum::extract::{Path, State};
use axum::routing::post;
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;

pub fn routes() -> Router {
    crate::scope::both(Router::new(), "/api/v1/me/packages/{id}/visibility", post(set))
}

#[derive(Deserialize)]
struct Visibility {
    visibility: String,
    #[serde(default)]
    version: Option<String>,
}

async fn set(State(app): St, auth: Auth, sc: crate::scope::Scope, Input(v): Input<Visibility>) -> Reply {
    let (repo, id) = (app.repos.get(&sc.repo).ok_or_else(|| Problem::missing("repository"))?, sc.id);
    let name = repo.name.clone();
    auth.scope("upload")?;
    let shown = listed(Some(&v.visibility))?;
    let version = v.version.filter(|v| !v.trim().is_empty());
    let (key, number, user, scope) = (id.clone(), version.clone(), auth.user, name);
    let published: Vec<String> = app
        .db
        .run(move |c| {
            let owner = "SELECT COUNT(*) FROM packages WHERE repo = ?3 AND id = ?1 AND owner_id = ?2";
            if c.query_row(owner, params![key, user, scope], |r| r.get::<_, i64>(0))? == 0 {
                return Err(Problem::missing("package of this account"));
            }
            let sql = "UPDATE versions SET listed = ?3 WHERE repo = ?4 AND package_id = ?1 AND (?2 IS NULL OR version = ?2) \
                       AND state != 'rejected'";
            changed(c.execute(sql, params![key, number, shown, scope])?, "version")?;
            audit(c, Some(user), if shown { "list" } else { "unlist" }, &key, number.as_deref().unwrap_or("*"))?;
            let sql = "SELECT version FROM versions WHERE repo = ?3 AND package_id = ?1 AND (?2 IS NULL OR version = ?2) \
                       AND state = 'published'";
            let mut found = c.prepare(sql)?;
            let rows = found.query_map(params![key, number, scope], |r| r.get(0))?;
            let versions = rows.collect::<Result<Vec<String>, _>>()?;
            c.execute("UPDATE repos SET dirty = 1 WHERE name = ?1", [&scope])?;
            Ok(versions)
        })
        .await?;
    let key = id.clone();
    tokio::task::spawn_blocking(move || published.iter().try_for_each(|v| repo.list(&key, Some(v), shown)))
        .await
        .map_err(Problem::internal)?
        .map_err(Problem::internal)?;
    app.changed().await;
    let visibility = if shown { "public" } else { "unlisted" };
    Ok(Json(json!({"id": id, "version": version, "visibility": visibility})))
}
