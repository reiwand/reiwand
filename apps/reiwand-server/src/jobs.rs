use crate::db::{DAY, HOUR, now};
use crate::error::{Problem, Res};
use crate::{App, repo::Repo};
use reiwand_format::Published;
use rusqlite::params;
use std::fs;
use std::sync::Arc;
use std::time::Duration;

pub fn spawn(app: Arc<App>) {
    let delay = Duration::from_secs(app.cfg.publish_delay);
    tokio::spawn(async move {
        loop {
            if let Err(problem) = purge(&app).await {
                tracing::warn!(detail = %problem.detail, "cleanup failed");
            }
            if let Err(problem) = republish(&app, false).await {
                tracing::warn!(detail = %problem.detail, "signing the repository failed");
            }
            if let Err(problem) = crate::mail::watch(&app).await {
                tracing::warn!(detail = %problem.detail, "watching the health rows failed");
            }
            tokio::select! {
                () = app.dirty.notified() => tokio::time::sleep(delay).await,
                () = tokio::time::sleep(Duration::from_secs(60)) => {}
            }
        }
    });
}

pub async fn republish(app: &Arc<App>, force: bool) -> Res<Option<Published>> {
    let (mut first, mut failure) = (None, None);
    for (n, repo) in app.repos.ready().into_iter().enumerate() {
        match republish_one(app, repo, force).await {
            Ok(published) if n == 0 => first = published,
            Ok(_) => {}
            Err(e) => failure = failure.or(Some(e)),
        }
    }
    failure.map_or(Ok(first), Err)
}

async fn republish_one(app: &Arc<App>, repo: Arc<Repo>, force: bool) -> Res<Option<Published>> {
    let name = repo.name.clone();
    let sql = "SELECT dirty, signed_at FROM repos WHERE name = ?1";
    let (dirty, signed) = app
        .db
        .run({
            let name = name.clone();
            move |c| Ok(c.query_row(sql, [name], |r| Ok((r.get::<_, bool>(0)?, r.get::<_, i64>(1)?)))?)
        })
        .await?;
    let t = now();
    if !force && !dirty && t - signed < app.cfg.resign_hours as i64 * HOUR {
        return Ok(None);
    }
    let reset = name.clone();
    app.db.run(move |c| Ok(c.execute("UPDATE repos SET dirty = 0 WHERE name = ?1", [reset])?)).await?;
    let days = app.cfg.entry_days;
    let published = tokio::task::spawn_blocking(move || repo.publish(days, t)).await?;
    let sql = "UPDATE repos SET dirty = MAX(dirty, ?1), signed_at = IIF(?1, signed_at, ?2), \
               failed_at = IIF(?1, ?2, NULL), failed_raw = ?3 WHERE name = ?4";
    let raw = published.as_ref().err().map(|e| format!("{e:#}"));
    let failed = raw.is_some();
    app.db.run(move |c| Ok(c.execute(sql, params![failed, t, raw, name])?)).await?;
    if failed {
        crate::mail::rewatch(app);
    }
    let published = published.map_err(|e| Problem::conflict("publish", format!("{e:#}")))?;
    tracing::info!(entry = published.version, packages = published.packages, "signed the repository index");
    Ok(Some(published))
}

pub async fn purge(app: &Arc<App>) -> Res<()> {
    app.clients.purge();
    let (t, notices) = (now(), app.cfg.notice_days as i64 * DAY);
    let cleaned = app.db.run(move |c| {
        let sql =
            "UPDATE notices SET name = '', email = '', signature = '' WHERE received_at < ?1 AND email != ''";
        c.execute(sql, [t - notices])?;
        c.execute(
            "DELETE FROM sessions WHERE seen_at < ?1 OR created_at < ?2",
            params![t - 7 * DAY, t - 30 * DAY],
        )?;
        for table in ["device_codes", "setup_tokens", "setup_visits", "mailbox", "invites", "strikes"] {
            c.execute(&format!("DELETE FROM {table} WHERE expires_at < ?1"), [t])?;
        }
        let old = t - 180 * DAY;
        let floor = "UPDATE sync_heads SET min_seq = MAX(min_seq, (SELECT COALESCE(MAX(seq), 0) \
                     FROM sync_records r WHERE r.user_id = sync_heads.user_id AND r.deleted = 1 \
                     AND r.updated_at < ?1))";
        c.execute(floor, [old])?;
        c.execute("DELETE FROM sync_records WHERE deleted = 1 AND updated_at < ?1", [old])?;
        c.execute("DELETE FROM uploads WHERE state != 'done' AND expires_at < ?1", [t])?;
        let sql = "DELETE FROM sync_blobs WHERE received < size AND created_at < ?1 RETURNING user_id, id";
        let mut stale = c.prepare(sql)?;
        let stale = stale.query_map([t - 7 * DAY], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)))?;
        Ok(stale.collect::<Result<Vec<_>, _>>()?)
    });
    for (user, id) in cleaned.await? {
        let _ = fs::remove_file(crate::files::dir(app, user).join(id));
    }
    let older = |days: u64| {
        move |item: &fs::DirEntry| {
            item.metadata()
                .and_then(|m| m.modified())
                .is_ok_and(|m| m.elapsed().unwrap_or_default().as_secs() > days * 86400)
        }
    };
    for (dir, days) in [("tmp", 1), ("held", app.cfg.takedown_days)] {
        for item in fs::read_dir(app.cfg.data.join(dir))?.flatten().filter(older(days)) {
            let _ = fs::remove_file(item.path());
        }
    }
    Ok(())
}
