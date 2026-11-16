use crate::auth::{Auth, Role};
use crate::db::{audit, now};
use crate::error::{Input, Problem, Res, require};
use crate::lang::t;
use crate::web::{page, text};
use crate::{App, Reply, Router, St};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use maud::html;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

pub fn routes() -> Router {
    Router::new()
        .route("/admin/repos", get(screen))
        .route("/api/v1/admin/repos", get(list).post(create))
        .route("/api/v1/admin/repos/{repo}", get(show).patch(crate::members::settings))
}

#[derive(Deserialize)]
struct Create {
    name: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    require_author_signature: bool,
    #[serde(default)]
    online_authors: bool,
}

pub async fn infos(app: &Arc<App>) -> Vec<Value> {
    app.repos
        .all()
        .iter()
        .map(|r| r.info(&app.cfg).unwrap_or_else(|| json!({"name": r.name, "ready": false})))
        .collect()
}

async fn list(State(app): St, auth: Auth) -> Reply {
    auth.at_least(Role::Admin)?;
    Ok(Json(json!({"items": infos(&app).await})))
}

async fn show(State(app): St, auth: Auth, Path(name): Path<String>) -> Reply {
    auth.at_least(Role::Admin)?;
    let repo = app.repos.get(&name).ok_or_else(|| Problem::missing("repository"))?;
    Ok(Json(repo.info(&app.cfg).ok_or_else(|| Problem::missing("repository"))?))
}

async fn create(State(app): St, auth: Auth, Input(c): Input<Create>) -> Res<Response> {
    auth.at_least(Role::Admin)?;
    require(crate::repo::valid_name(&c.name), "name: 1 to 32 lowercase letters, digits or hyphens")?;
    let title = if c.title.trim().is_empty() { c.name.clone() } else { c.title.trim().to_string() };
    require(title.chars().count() <= 80, "title: at most 80 characters")?;
    if app.repos.get(&c.name).is_some_and(|r| r.ready()) {
        return Err(Problem::conflict("repository", "a repository with that name already exists"));
    }
    let (cfg, name) = (app.cfg.clone(), c.name.clone());
    let made = tokio::task::spawn_blocking(move || {
        crate::init_repo(&cfg, &name, &title, c.require_author_signature, c.online_authors)
    });
    let root = made
        .await
        .map_err(Problem::internal)?
        .map_err(|e| Problem::conflict("repository", format!("{e:#}")))?;
    app.repos.reload();
    let name = c.name.clone();
    app.db.run(move |conn| audit(conn, Some(auth.user), "repo-create", &name, "")).await?;
    app.changed().await;
    let info = app.repos.get(&c.name).and_then(|r| r.info(&app.cfg));
    let key = json!({"fingerprint": root.fingerprint, "secret": root.secret, "shown_once": true});
    Ok((StatusCode::CREATED, Json(json!({"repository": info, "root_key": key, "at": now()}))).into_response())
}

async fn screen(State(app): St, auth: Option<Auth>) -> Res<Response> {
    let Some(auth) = auth.filter(|a| a.role >= Role::Admin) else {
        return Ok(Redirect::to("/login?next=/admin/repos").into_response());
    };
    let items = infos(&app).await;
    let state: Vec<(String, bool, String)> = app
        .db
        .run(|c| {
            let mut q = c.prepare("SELECT name, private, publishing FROM repos")?;
            let found = q.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
            Ok(found.collect::<Result<Vec<_>, _>>()?)
        })
        .await?;
    let body = html! {
        p.texto { (t("Each repository has its own address and its own keys. Moderation and accounts are shared by all of them.")) }
        p.texto { (t("A private repository shows only to the accounts you give read rights in the People page; publishing there needs publish rights.")) }
        @for i in &items {
            @let name = text(i, "name");
            @let (private, mode) = state.iter().find(|s| s.0 == name).map_or((false, "open"), |s| (s.1, s.2.as_str()));
            section.seccion {
                h2 { code { (name) } }
                (crate::ui::copyable(text(i, "url"), t("Copy"), t("Copied")))
                form method={ "post" } action={ "/api/v1/admin/repos/" (name) "?_method=PATCH&back=/admin/repos" } {
                    label { (t("Who can see it"))
                        select name="private" {
                            option value="false" selected[!private] { (t("Anyone")) }
                            option value="true" selected[private] { (t("Only members")) }
                        }
                    }
                    label { (t("Who can publish"))
                        select name="publishing" {
                            option value="open" selected[mode == "open"] { (t("Any account, for review")) }
                            option value="members" selected[mode == "members"] { (t("Only members with publish rights")) }
                        }
                    }
                    button { (t("Save")) }
                }
            }
        }
        form method="post" action="/api/v1/admin/repos?back=/admin/repos" {
            label { (t("Address name")) input name="name" required maxlength="32" pattern="[a-z0-9][a-z0-9\\-]*"; }
            label { (t("Repository name")) input name="title" maxlength="80"; }
            button { (t("Create the repository")) }
        }
    };
    Ok(page(&app, Some(&auth), "/admin", t("Repositories"), body).into_response())
}
