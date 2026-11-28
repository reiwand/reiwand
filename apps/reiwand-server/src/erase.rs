use crate::App;
use crate::auth::{Auth, Role};
use crate::db::{audit, now, one};
use crate::error::{Input, Problem, Res, require};
use crate::scope::{Scope, at, resolve};
use crate::{Reply, Router, St};
use axum::Json;
use axum::extract::State;
use axum::routing::delete;
use reiwand_format::Reason;
use rusqlite::{Connection, params};
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::sync::Arc;

pub fn routes() -> Router {
    let mine = delete(|s: St, a: Auth, sc: Scope, Input(c): Input<Sure>| withdraw(s, a, sc, c));
    let router = at(Router::new(), "/api/v1/me/packages/{id}", mine);
    let admin = delete(|s: St, a: Auth, sc: Scope, Input(c): Input<Sure>| remove(s, a, sc, c));
    at(router, "/api/v1/admin/packages/{id}", admin)
}

#[derive(Deserialize)]
pub struct Sure {
    #[serde(default)]
    pub confirm: bool,
}

pub fn sure(s: &Sure) -> Res<()> {
    require(s.confirm, "confirm: send confirm true; this cannot be undone")
}

async fn withdraw(State(app): St, auth: Auth, sc: Scope, s: Sure) -> Reply {
    auth.scope("upload")?;
    sure(&s)?;
    let (repo, id, user) = (sc.repo.clone(), sc.id.clone(), auth.user);
    let sql = "SELECT owner_id = ?3 FROM packages WHERE repo = ?1 AND id = ?2";
    let own: Option<bool> = app.db.run(move |c| one(c, sql, params![repo, id, user])).await?;
    if own != Some(true) {
        return Err(Problem::missing("package of this account"));
    }
    erase(&app, &sc.repo, &sc.id, auth.user, "withdraw", "Withdrawn by its author.").await?;
    Ok(Json(json!({"id": sc.id, "withdrawn": true})))
}

async fn remove(State(app): St, auth: Auth, sc: Scope, s: Sure) -> Reply {
    auth.at_least(Role::Admin)?;
    sure(&s)?;
    let message = "Deleted by the administrator of this server.";
    erase(&app, &sc.repo, &sc.id, auth.user, "delete-package", message).await?;
    Ok(Json(json!({"id": sc.id, "deleted": true})))
}

type Version = (i64, String, String, String);

fn versions(c: &Connection, repo: &str, id: &str) -> Res<Vec<Version>> {
    let sql = "SELECT id, version, state, sha256 FROM versions WHERE repo = ?1 AND package_id = ?2";
    let mut q = c.prepare(sql)?;
    let found = q.query_map([repo, id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?;
    Ok(found.collect::<Result<_, _>>()?)
}

pub fn unused(c: &Connection, sha: &str) -> Res<bool> {
    let sql = "SELECT COUNT(*) FROM versions WHERE sha256 = ?1 AND state IN ('pending', 'published')";
    Ok(c.query_row(sql, [sha], |r| r.get::<_, i64>(0))? == 0)
}

pub fn forget(app: &App, sha: &str) {
    for place in ["held/{}.wpak", "held/{}.jpg", "blobs/{}.wpak", "thumbs/{}.jpg"] {
        let _ = fs::remove_file(app.cfg.data.join(place.replace("{}", sha)));
    }
}

pub async fn erase(app: &Arc<App>, name: &str, id: &str, actor: i64, action: &str, message: &str) -> Res<()> {
    let repo = resolve(app, name)?;
    let _turn = app.deciding.lock().await;
    let (key, named) = (id.to_string(), name.to_string());
    let sql = "SELECT COUNT(*) > 0 FROM featured WHERE repo = ?1 AND package_id = ?2";
    let (found, featured) = app
        .db
        .run(move |c| Ok((versions(c, &named, &key)?, c.query_row(sql, [&named, &key], |r| r.get(0))?)))
        .await?;
    if !found.iter().any(|v| v.2 == "pending" || v.2 == "published") {
        return Err(Problem::missing("package"));
    }
    let live: Vec<String> = found.iter().filter(|v| v.2 == "published").map(|v| v.1.clone()).collect();
    let (tree, key, note) = (repo.clone(), id.to_string(), message.to_string());
    tokio::task::spawn_blocking(move || {
        if featured {
            tree.feature(&key, None, false)?;
        }
        live.iter().try_for_each(|v| tree.revoke(&key, v, Reason::Other, &note))
    })
    .await
    .map_err(Problem::internal)??;
    let (key, named, action, said) =
        (id.to_string(), name.to_string(), action.to_string(), message.to_string());
    let shas: Vec<String> = found.iter().map(|v| v.3.clone()).collect();
    let gone = app.db.run(move |c| {
        let (tx, t) = (c.transaction()?, now());
        let sql = "UPDATE versions SET state = CASE state WHEN 'pending' THEN 'rejected' WHEN 'published' THEN 'removed' \
                   ELSE state END WHERE repo = ?1 AND package_id = ?2";
        tx.execute(sql, [&named, &key])?;
        let sql = "UPDATE cases SET state = 'actioned', decision = ?3, statement = ?4, decided_by = ?5, decided_at = ?6 \
                   WHERE state = 'open' AND version_id IN (SELECT id FROM versions WHERE repo = ?1 AND package_id = ?2)";
        tx.execute(sql, params![named, key, action, said, actor, t])?;
        for table in ["votes", "featured"] {
            tx.execute(&format!("DELETE FROM {table} WHERE repo = ?1 AND package_id = ?2"), [&named, &key])?;
        }
        tx.execute("UPDATE repos SET dirty = 1 WHERE name = ?1", [&named])?;
        audit(&tx, Some(actor), &action, &format!("{named}/{key}"), &said)?;
        let gone = shas.iter().map(|s| Ok((s.clone(), unused(&tx, s)?))).collect::<Res<Vec<_>>>()?;
        tx.commit()?;
        Ok(gone)
    });
    gone.await?.iter().filter(|(_, unused)| *unused).for_each(|(sha, _)| forget(app, sha));
    let _ = fs::remove_dir_all(repo.dir.join("packages").join(id));
    app.changed().await;
    Ok(())
}
