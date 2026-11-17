use anyhow::Context;
use rusqlite::Connection;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

const NOTE: &str = "Records exactly as the server stored them. When \"encrypted\" is true the payloads are \
sealed with a key only the account's devices held, so the server cannot read them; they are kept for \
whoever still has that key. Files of the account are in the folder of the same name.";

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn has(conn: &Connection, table: &str) -> rusqlite::Result<bool> {
    conn.query_row("SELECT count(*) FROM sqlite_master WHERE name = ?1", [table], |r| r.get::<_, i64>(0))
        .map(|n| n > 0)
}

fn records(conn: &Connection, user: i64) -> anyhow::Result<Vec<Value>> {
    let mut query = conn.prepare(
        "SELECT collection, item, deleted, updated_at, payload FROM sync_records WHERE user_id = ?1 ORDER BY seq",
    )?;
    let rows = query.query_map([user], |r| {
        let payload: Option<Vec<u8>> = r.get(4)?;
        Ok(json!({
            "collection": r.get::<_, String>(0)?, "item": r.get::<_, String>(1)?,
            "deleted": r.get::<_, i64>(2)? != 0, "updated_at": r.get::<_, i64>(3)?,
            "payload_hex": payload.map(|p| hex(&p)),
        }))
    })?;
    Ok(rows.collect::<Result<_, _>>()?)
}

pub fn write(conn: &Connection, data: &Path) -> anyhow::Result<()> {
    if !has(conn, "sync_records")? {
        return Ok(());
    }
    let out = data.join("sync-export");
    let mut users = conn.prepare(
        "SELECT u.id, u.handle, coalesce(h.encrypted, 0) FROM users u LEFT JOIN sync_heads h ON h.user_id = u.id",
    )?;
    let people: Vec<(i64, String, bool)> = users
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get::<_, i64>(2)? != 0)))?
        .collect::<Result<_, _>>()?;
    for (id, handle, encrypted) in people {
        let list = records(conn, id)?;
        if list.is_empty() {
            continue;
        }
        fs::create_dir_all(&out)?;
        let body = json!({"handle": handle, "encrypted": encrypted, "note": NOTE, "records": list});
        fs::write(out.join(format!("{handle}.json")), serde_json::to_vec_pretty(&body)?)
            .with_context(|| format!("writing the export of {handle}"))?;
    }
    Ok(())
}

pub fn move_files(data: &Path, people: &[(i64, String)]) -> anyhow::Result<()> {
    for (id, handle) in people {
        let old = data.join("private").join(id.to_string());
        let new = data.join("sync-export").join(handle);
        if old.is_dir() && !new.exists() {
            fs::rename(&old, &new)?;
        }
    }
    Ok(())
}

pub fn people(conn: &Connection) -> anyhow::Result<Vec<(i64, String)>> {
    let mut users = conn.prepare("SELECT id, handle FROM users")?;
    Ok(users.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<Result<_, _>>()?)
}
