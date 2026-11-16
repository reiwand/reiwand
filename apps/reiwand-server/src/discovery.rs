use crate::St;
use crate::db::now;
use axum::Json;
use axum::extract::State;
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

pub async fn discovery(State(app): St) -> Json<Value> {
    let c = &app.cfg;
    let infos: Vec<Value> = app.repos.ready().iter().filter_map(|r| r.info(c)).collect();
    let short =
        |r: &Value| json!({"url": r["url"], "repo_id": r["repo_id"], "entry_expires": r["entry_expires"]});
    let repo = infos.first().map(short);
    let repositories: Vec<Value> = infos
        .iter()
        .map(|r| json!({"name": r["name"], "url": r["address"], "repo_id": r["repo_id"], "entry_expires": r["entry_expires"]}))
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
    Json(json!({
        "name": c.name, "version": crate::VERSION, "api": ["v1"], "registration": registration,
        "sync": sync, "repository": repo, "repositories": repositories, "legal": legal,
        "uploads": {"max_bytes": c.package_max, "per_day": c.uploads_per_day, "licenses": c.licenses},
    }))
}
