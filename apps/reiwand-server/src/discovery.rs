use crate::St;
use crate::access::{mode, visible};
use crate::auth::Auth;
use crate::db::now;
use crate::error::{Problem, Res};
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::{Value, json};

fn expiry(info: &Value) -> Option<chrono::DateTime<chrono::FixedOffset>> {
    info["entry_expires"].as_str().and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
}

pub async fn ready(State(app): St) -> Response {
    let database = app.db.run(|c| Ok(c.query_row("SELECT 1", [], |r| r.get::<_, i64>(0))?)).await.is_ok();
    let infos: Vec<Value> = app.repos.ready().iter().filter_map(|r| r.info(&app.cfg)).collect();
    let fresh = infos.iter().all(|r| expiry(r).is_some_and(|t| t.timestamp_millis() > now()));
    let status = if database && fresh { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    let body = json!({"database": database, "repository": !infos.is_empty(), "index_fresh": fresh});
    (status, Json(body)).into_response()
}

#[derive(serde::Deserialize)]
pub struct Ask {
    repo: Option<String>,
}

fn wanted(text: &str) -> &str {
    let text = text.trim_matches('/');
    text.strip_prefix("r/").unwrap_or(if text == "repo" { "" } else { text })
}

async fn one(app: &crate::App, auth: Option<&Auth>, text: &str) -> Res<Value> {
    let repo = crate::scope::resolve(app, wanted(text))?;
    let seen = visible(app, auth, vec![repo.name.clone()]).await;
    let (_, rights) = seen.first().ok_or_else(|| Problem::missing("repository"))?;
    let registration = format!("{:?}", app.settings().registration).to_lowercase();
    let mut found = json!({
        "name": repo.name, "private": rights.private, "publishing": mode(rights), "registration": registration,
    });
    if rights.may_read {
        let info = repo.info(&app.cfg).ok_or_else(|| Problem::missing("repository"))?;
        found["url"] = info["address"].clone();
        found["repo_id"] = info["repo_id"].clone();
    }
    Ok(found)
}

pub async fn discovery(State(app): St, auth: Option<Auth>, Query(ask): Query<Ask>) -> Res<Json<Value>> {
    if let Some(text) = ask.repo.as_deref() {
        return Ok(Json(one(&app, auth.as_ref(), text).await?));
    }
    let c = &app.cfg;
    let all: Vec<Value> = app.repos.ready().iter().filter_map(|r| r.info(c)).collect();
    let names = all.iter().map(|r| r["name"].as_str().unwrap_or_default().to_string()).collect();
    let seen = visible(&app, auth.as_ref(), names).await;
    let infos: Vec<Value> = all
        .into_iter()
        .filter(|r| seen.iter().any(|(n, k)| k.may_read && r["name"].as_str() == Some(n)))
        .collect();
    let short =
        |r: &Value| json!({"url": r["url"], "repo_id": r["repo_id"], "entry_expires": r["entry_expires"]});
    let repo = infos.first().map(short);
    let repositories: Vec<Value> = infos
        .iter()
        .map(|r| {
            let (private, publishing) = seen
                .iter()
                .find(|(n, _)| r["name"].as_str() == Some(n))
                .map_or((false, "open"), |(_, k)| (k.private, mode(k)));
            json!({"name": r["name"], "url": r["address"], "repo_id": r["repo_id"], "entry_expires": r["entry_expires"], "private": private, "publishing": publishing})
        })
        .collect();
    let registration = format!("{:?}", app.settings().registration).to_lowercase();
    let sync = json!({
        "end_to_end_default": true, "plaintext_requires_confirmation": true,
        "record_max_bytes": 262144, "quota_bytes": c.sync_quota, "files_quota_bytes": c.files_quota,
    });
    let legal = json!({
        "notice": c.url("/legal/notice"), "terms": c.url("/legal/terms"),
        "privacy": c.url("/legal/privacy"), "contact": c.contact,
    });
    Ok(Json(json!({
        "name": c.name, "version": crate::VERSION, "api": ["v1"], "registration": registration,
        "sync": sync, "repository": repo, "repositories": repositories, "legal": legal,
        "uploads": {"max_bytes": c.package_max, "per_day": c.uploads_per_day, "licenses": c.licenses},
    })))
}
