use crate::auth::{Auth, Role};
use crate::db::{audit, changed, rows};
use crate::error::{Input, Res, require};
use crate::{Reply, Router, St};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, put};
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;

pub fn routes() -> Router {
    Router::new()
        .route("/api/v1/admin/repos/{repo}/members", get(list))
        .route("/api/v1/admin/repos/{repo}/members/{user}", put(set).delete(remove))
}

const MEMBERS: &str = "SELECT m.user_id AS user, u.handle, m.may_read, m.may_publish FROM repo_members m \
                       JOIN users u ON u.id = m.user_id WHERE m.repo = ?1 ORDER BY u.handle";

async fn list(State(app): St, auth: Auth, Path(repo): Path<String>) -> Reply {
    auth.at_least(Role::Admin)?;
    let name = crate::scope::resolve(&app, &repo)?.name.clone();
    let found = app.db.run(move |c| rows(c, MEMBERS, [name])).await?;
    Ok(Json(json!({"items": found})))
}

#[derive(Deserialize)]
struct Rights {
    #[serde(default)]
    may_read: bool,
    #[serde(default)]
    may_publish: bool,
}

pub fn grant(c: &rusqlite::Connection, repo: &str, user: i64, read: bool, publish: bool) -> Res<()> {
    let exists = c.query_row("SELECT COUNT(*) FROM users WHERE id = ?1", [user], |r| r.get::<_, i64>(0))?;
    changed(exists as usize, "account")?;
    if !read && !publish {
        c.execute("DELETE FROM repo_members WHERE repo = ?1 AND user_id = ?2", params![repo, user])?;
        return Ok(());
    }
    let sql = "INSERT INTO repo_members (repo, user_id, may_read, may_publish) VALUES (?1, ?2, 1, ?3) \
               ON CONFLICT (repo, user_id) DO UPDATE SET may_read = 1, may_publish = ?3";
    c.execute(sql, params![repo, user, publish])?;
    Ok(())
}

async fn set(
    State(app): St,
    auth: Auth,
    Path((repo, user)): Path<(String, i64)>,
    Input(r): Input<Rights>,
) -> Reply {
    auth.at_least(Role::Admin)?;
    let name = crate::scope::resolve(&app, &repo)?.name.clone();
    let detail = format!("read={} publish={}", r.may_read || r.may_publish, r.may_publish);
    app.db
        .run(move |c| {
            grant(c, &name, user, r.may_read, r.may_publish)?;
            audit(c, Some(auth.user), "repo-member", &format!("{name}/{user}"), &detail)
        })
        .await?;
    Ok(Json(
        json!({"repo": repo, "user": user, "may_read": r.may_read || r.may_publish, "may_publish": r.may_publish}),
    ))
}

async fn remove(State(app): St, auth: Auth, Path((repo, user)): Path<(String, i64)>) -> Res<StatusCode> {
    auth.at_least(Role::Admin)?;
    let name = crate::scope::resolve(&app, &repo)?.name.clone();
    app.db
        .run(move |c| {
            let sql = "DELETE FROM repo_members WHERE repo = ?1 AND user_id = ?2";
            changed(c.execute(sql, params![name, user])?, "member")?;
            audit(c, Some(auth.user), "repo-member-remove", &format!("{name}/{user}"), "")
        })
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct Settings {
    pub private: Option<bool>,
    pub publishing: Option<String>,
}

pub async fn settings(
    State(app): St,
    auth: Auth,
    Path(repo): Path<String>,
    Input(s): Input<Settings>,
) -> Reply {
    auth.at_least(Role::Admin)?;
    let name = crate::scope::resolve(&app, &repo)?.name.clone();
    let mode = s.publishing.clone();
    require(mode.as_deref().is_none_or(|m| m == "open" || m == "members"), "publishing: open or members")?;
    let shown = name.clone();
    app.db
        .run(move |c| {
            let known =
                c.query_row("SELECT COUNT(*) FROM repos WHERE name = ?1", [&name], |r| r.get::<_, i64>(0))?;
            changed(known as usize, "repository")?;
            if let Some(private) = s.private {
                c.execute("UPDATE repos SET private = ?2 WHERE name = ?1", params![name, private])?;
            }
            if let Some(mode) = &s.publishing {
                c.execute("UPDATE repos SET publishing = ?2 WHERE name = ?1", params![name, mode])?;
            }
            let detail = format!("private={:?} publishing={:?}", s.private, s.publishing);
            audit(c, Some(auth.user), "repo-settings", &name, &detail)
        })
        .await?;
    let (private, publishing) = app
        .db
        .run(move |c| {
            Ok(c.query_row("SELECT private, publishing FROM repos WHERE name = ?1", [shown], |r| {
                Ok((r.get::<_, bool>(0)?, r.get::<_, String>(1)?))
            })?)
        })
        .await?;
    Ok(Json(json!({"repo": repo, "private": private, "publishing": publishing})))
}
