use crate::auth::{Auth, Role};
use crate::db::{audit, changed, now, rows};
use crate::error::{Input, Problem, Res, require};
use crate::lang::t;
use crate::scope::{Scope, at, resolve};
use crate::web::{page, text};
use crate::{Reply, Router, St};
use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{delete, get};
use maud::html;
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;

const LIST: &str = "SELECT f.repo, f.package_id AS id, f.note, f.at FROM featured f \
                    WHERE (?1 = '' OR f.repo = ?1) ORDER BY f.repo, f.at DESC";

pub fn routes() -> Router {
    let router = Router::new().route("/admin/featured", get(screen));
    let all = get(list).post(|s: St, a: Auth, sc: Scope, Input(f): Input<Feature>| {
        set(s, a, Feature { repo: sc.repo, ..f }, true)
    });
    let one = delete(|s: St, a: Auth, sc: Scope| {
        set(s, a, Feature { repo: sc.repo, id: sc.id, note: String::new() }, false)
    });
    let router = at(router, "/api/v1/admin/featured", all);
    at(router, "/api/v1/admin/featured/{id}", one)
}

#[derive(Deserialize)]
pub struct Feature {
    #[serde(default, skip_deserializing)]
    repo: String,
    id: String,
    #[serde(default)]
    note: String,
}

async fn list(State(app): St, auth: Auth, sc: Scope) -> Reply {
    auth.at_least(Role::Admin)?;
    Ok(Json(json!({"items": app.db.run(move |c| rows(c, LIST, [sc.repo])).await?})))
}

async fn set(State(app): St, auth: Auth, f: Feature, on: bool) -> Reply {
    auth.at_least(Role::Admin)?;
    let (id, note) = (f.id, f.note.trim().to_string());
    let repo = resolve(&app, &f.repo)?;
    let name = repo.name.clone();
    let plain = note.chars().count() <= 200 && !note.contains(['<', '>', '\n', '\r']);
    require(plain, "note: at most 200 characters on one line, without < or >")?;
    let (key, words, user) = (id.clone(), note.clone(), auth.user);
    app.db
        .run(move |c| {
            if on {
                let sql = "SELECT COUNT(*) FROM versions WHERE repo = ?2 AND package_id = ?1 AND state = 'published'";
                require(c.query_row(sql, [&key, &name], |r| r.get::<_, i64>(0))? > 0, "id: a published package")?;
                let count = c.query_row("SELECT COUNT(*) FROM featured WHERE repo = ?1", [&name], |r| r.get::<_, i64>(0))?;
                let known =
                    c.query_row("SELECT COUNT(*) FROM featured WHERE repo = ?2 AND package_id = ?1", [&key, &name], |r| {
                        r.get::<_, i64>(0)
                    })?;
                require(count < 64 || known > 0, "at most 64 featured packages")?;
                let sql = "INSERT INTO featured (package_id, note, at, repo) VALUES (?1, ?2, ?3, ?4) \
                           ON CONFLICT (repo, package_id) DO UPDATE SET note = ?2";
                c.execute(sql, params![key, (!words.is_empty()).then_some(&words), now(), name])?;
            } else {
                changed(
                    c.execute("DELETE FROM featured WHERE repo = ?2 AND package_id = ?1", [&key, &name])?,
                    "featured package",
                )?;
            }
            audit(c, Some(user), if on { "feature" } else { "unfeature" }, &key, &words)?;
            Ok(c.execute("UPDATE repos SET dirty = 1 WHERE name = ?1", [&name])?)
        })
        .await?;
    let key = id.clone();
    let note = (!note.is_empty()).then_some(note);
    let marked = tokio::task::spawn_blocking(move || repo.feature(&key, note.as_deref(), on))
        .await
        .map_err(Problem::internal)?;
    if let Err(e) = marked {
        tracing::info!("a featured package is not in the repository yet: {e}");
    }
    app.changed().await;
    Ok(Json(json!({"id": id, "featured": on})))
}

async fn screen(State(app): St, auth: Option<Auth>) -> Res<Response> {
    let Some(auth) = auth.filter(|a| a.role >= Role::Admin) else {
        return Ok(Redirect::to("/login?next=/admin/featured").into_response());
    };
    let items = app.db.run(|c| rows(c, LIST, [""])).await?;
    let body = html! {
        p.text { (t("Featured packages are signed into the repository; clients show them apart in their catalog.")) }
        table { tbody { @for i in &items { tr {
            td { (text(i, "repo")) } td { a href={ "/r/" (text(i, "repo")) "/packages/" (text(i, "id")) } { code { (text(i, "id")) } } } td { (text(i, "note")) }
            td { form method="post" action={ "/api/v1/r/" (text(i, "repo")) "/admin/featured/" (text(i, "id")) "?_method=DELETE&back=/admin/featured" } {
                button { (t("Stop featuring")) }
            } }
        } } } }
        @for repo in app.repos.ready() {
            form method="post" action={ "/api/v1/r/" (repo.name) "/admin/featured?back=/admin/featured" } {
                h3 { (repo.name) }
                label { (t("Package id")) input name="id" required; }
                label { (t("Note")) input name="note" maxlength="200"; }
                button { (t("Feature")) }
            }
        }
    };
    Ok(page(&app, Some(&auth), "/admin", t("Featured packages"), body).into_response())
}
