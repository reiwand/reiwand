use crate::App;
use crate::auth::{Auth, Role};
use crate::db::audit;
use crate::erase::{Sure, forget, sure, unused};
use crate::error::{Input, Problem, Res};
use crate::{Reply, St};
use axum::Json;
use axum::extract::{Path, State};
use serde_json::json;
use std::fs;
use std::sync::Arc;

pub async fn delete(State(app): St, auth: Auth, Path(name): Path<String>, Input(s): Input<Sure>) -> Reply {
    auth.at_least(Role::Admin)?;
    sure(&s)?;
    erase(&app, auth.user, &name).await?;
    Ok(Json(json!({"name": name, "deleted": true})))
}

async fn erase(app: &Arc<App>, actor: i64, name: &str) -> Res<()> {
    let repo =
        app.repos.all().into_iter().find(|r| r.name == name).ok_or_else(|| Problem::missing("repository"))?;
    let _turn = app.deciding.lock().await;
    let named = name.to_string();
    let gone = app.db.run(move |c| {
        let tx = c.transaction()?;
        let sql = "SELECT DISTINCT sha256 FROM versions WHERE repo = ?1";
        let shas: Vec<String> =
            tx.prepare(sql)?.query_map([&named], |r| r.get(0))?.collect::<Result<_, _>>()?;
        let sql = "SELECT id FROM uploads WHERE repo = ?1 AND state != 'done'";
        let open: Vec<String> =
            tx.prepare(sql)?.query_map([&named], |r| r.get(0))?.collect::<Result<_, _>>()?;
        tx.execute("DELETE FROM uploads WHERE repo = ?1", [&named])?;
        tx.execute("DELETE FROM versions WHERE repo = ?1", [&named])?;
        tx.execute("DELETE FROM packages WHERE repo = ?1", [&named])?;
        tx.execute("DELETE FROM repos WHERE name = ?1", [&named])?;
        audit(&tx, Some(actor), "repo-delete", &named, "")?;
        let shas = shas.into_iter().map(|s| Ok((unused(&tx, &s)?, s))).collect::<Res<Vec<_>>>()?;
        tx.commit()?;
        Ok((shas, open))
    });
    let (shas, open) = gone.await?;
    shas.iter().filter(|(unused, _)| *unused).for_each(|(_, sha)| forget(app, sha));
    for upload in open {
        let tmp = app.cfg.data.join("tmp");
        [upload.clone(), format!("{upload}.jpg")].iter().for_each(|f| drop(fs::remove_file(tmp.join(f))));
    }
    let tree = repo.clone();
    tokio::task::spawn_blocking(move || tree.erase()).await.map_err(Problem::internal)??;
    app.repos.reload();
    app.changed().await;
    Ok(())
}
