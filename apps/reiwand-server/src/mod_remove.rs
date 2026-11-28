use crate::auth::{Auth, Role};
use crate::db::now;
use crate::error::{Input, Problem, require};
use crate::review::{Decision, decide};
use crate::scope::{Scope, at};
use crate::{Reply, Router, St};
use axum::Json;
use axum::extract::State;
use axum::routing::post;
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;

pub const DETAIL: &str = "Removed by a moderator from the page of the package.";

pub fn routes() -> Router {
    at(Router::new(), "/api/v1/mod/removed", post(remove))
}

#[derive(Deserialize)]
struct Remove {
    id: String,
    #[serde(default)]
    statement: String,
}

async fn remove(State(app): St, auth: Auth, sc: Scope, Input(r): Input<Remove>) -> Reply {
    auth.at_least(Role::Moderator)?;
    let statement = r.statement.trim().to_string();
    require(statement.chars().count() >= 10, "statement: at least 10 characters for the author")?;
    let (repo, id) = (sc.repo.clone(), r.id.clone());
    let opened = app.db.run(move |c| {
        let tx = c.transaction()?;
        let sql = "SELECT id FROM versions WHERE repo = ?1 AND package_id = ?2 AND state = 'published'";
        let found: Vec<i64> =
            tx.prepare(sql)?.query_map([&repo, &id], |r| r.get(0))?.collect::<Result<_, _>>()?;
        let sql = "INSERT INTO cases (version_id, reason, detail, priority, state, created_at) \
                   VALUES (?1, 'report', ?2, 1, 'open', ?3)";
        let cases = found
            .iter()
            .map(|v| Ok(tx.execute(sql, params![v, DETAIL, now()]).map(|_| tx.last_insert_rowid())?))
            .collect::<Result<Vec<i64>, Problem>>()?;
        tx.commit()?;
        Ok(cases)
    });
    let cases = opened.await?;
    if cases.is_empty() {
        return Err(Problem::missing("published package"));
    }
    let mut removed = Vec::new();
    for case in cases {
        let d = Decision { decision: "remove".into(), statement: statement.clone(), ..Decision::default() };
        removed.push(decide(&app, Some(auth.user), case, d).await?["subject"].clone());
    }
    Ok(Json(json!({"id": r.id, "removed": removed})))
}
