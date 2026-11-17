use crate::admin_repos_page::screen;
use crate::auth::{Auth, Role};
use crate::db::{audit, now};
use crate::error::{Input, Problem, Res, require};
use crate::{App, Reply, Router, St};
use axum::Json;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

pub fn routes() -> Router {
    Router::new()
        .route("/admin/repos", get(screen).post(submit))
        .route("/api/v1/admin/repos", get(list).post(create))
        .route("/api/v1/admin/repos/{repo}", get(show).patch(crate::members::settings))
}

#[derive(Deserialize)]
pub struct Create {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub require_author_signature: bool,
    #[serde(default)]
    pub online_authors: bool,
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

#[derive(Deserialize)]
struct Named {
    #[serde(default)]
    name: String,
}

pub async fn make(app: &Arc<App>, user: i64, c: Create) -> Res<String> {
    let title = c.title.trim().to_string();
    let name = match c.name.as_deref().map(str::trim).filter(|n| !n.is_empty()) {
        Some(name) => {
            require(crate::repo::valid_name(name), "name: 1 to 32 lowercase letters, digits or hyphens")?;
            name.to_string()
        }
        None => {
            require(!title.is_empty(), "title: a name for the repository is required")?;
            let taken: Vec<String> = app.repos.all().iter().map(|r| r.name.clone()).collect();
            crate::repo::free_name(&title, &taken)
        }
    };
    let title = if title.is_empty() { name.clone() } else { title };
    require(title.chars().count() <= 80, "title: at most 80 characters")?;
    if app.repos.get(&name).is_some_and(|r| r.ready()) {
        return Err(Problem::conflict("repository", "a repository with that name already exists"));
    }
    let (cfg, made_name, kept) = (app.cfg.clone(), name.clone(), title);
    tokio::task::spawn_blocking(move || {
        crate::init_repo(&cfg, &made_name, &kept, c.require_author_signature, c.online_authors)
    })
    .await
    .map_err(Problem::internal)?
    .map_err(|e| Problem::conflict("repository", format!("{e:#}")))?;
    app.repos.reload();
    let audited = name.clone();
    app.db.run(move |conn| audit(conn, Some(user), "repo-create", &audited, "")).await?;
    app.changed().await;
    Ok(name)
}

async fn create(State(app): St, auth: Auth, Input(c): Input<Create>) -> Res<Response> {
    auth.at_least(Role::Admin)?;
    let name = make(&app, auth.user, c).await?;
    let info = app.repos.get(&name).and_then(|r| r.info(&app.cfg));
    Ok((StatusCode::CREATED, Json(json!({"repository": info, "at": now()}))).into_response())
}

async fn submit(State(app): St, auth: Auth, Form(f): Form<Named>) -> Res<Response> {
    auth.at_least(Role::Admin)?;
    let made = make(
        &app,
        auth.user,
        Create { name: None, title: f.name.clone(), require_author_signature: false, online_authors: false },
    );
    match made.await {
        Ok(name) => Ok(Redirect::to(&format!("/admin/repos?new={name}")).into_response()),
        Err(problem) => crate::admin_repos_page::failed(&app, &auth, &f.name, problem).await,
    }
}
