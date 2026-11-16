use crate::Outcome;
use anyhow::{Context, bail, ensure};
use rusqlite::Connection;
use std::fs;
use std::path::Path;

pub const FIRST: i64 = 11;
pub const CURRENT: i64 = 14;

const STEPS: [&str; 2] = [include_str!("../migrations/11-12.sql"), include_str!("../migrations/12-13.sql")];

pub fn run(data: &Path) -> anyhow::Result<Outcome> {
    let path = data.join("db/reiwand.sqlite");
    if !path.is_file() {
        return Ok(Outcome::Nothing);
    }
    let found = version(&Connection::open(&path)?)?;
    if found == CURRENT {
        return Ok(Outcome::Nothing);
    }
    if found == 0 {
        let empty: i64 =
            Connection::open(&path)?.query_row("SELECT count(*) FROM sqlite_master", [], |r| r.get(0))?;
        ensure!(empty == 0, "{} has no schema version: it is not a Reiwand server database", path.display());
        return Ok(Outcome::Nothing);
    }
    if !(FIRST..CURRENT).contains(&found) {
        bail!("the database is at schema {found}; this upgrade reads {FIRST} to {}", CURRENT - 1);
    }
    let copy = path.with_extension(format!("sqlite.before-v{CURRENT}-from-v{found}"));
    if !copy.exists() {
        Connection::open(&path)?.execute("VACUUM INTO ?1", [copy.to_string_lossy().as_ref()])?;
    }
    move_files(data)?;
    let mut conn = Connection::open(&path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "OFF")?;
    let tx = conn.transaction()?;
    for step in found..CURRENT - 1 {
        tx.execute_batch(STEPS[(step - FIRST) as usize])?;
    }
    tx.execute(
        "UPDATE repos SET folder = 'repos/main', online_key = 'keys/main.online.key' \
         WHERE name = 'main' AND folder = 'repo'",
        [],
    )?;
    let broken: i64 = tx.query_row("SELECT count(*) FROM pragma_foreign_key_check", [], |r| r.get(0))?;
    ensure!(broken == 0, "the upgrade left {broken} rows without their parent");
    tx.pragma_update(None, "user_version", CURRENT)?;
    tx.commit()?;
    Ok(Outcome::Upgraded { from: found, to: CURRENT })
}

fn version(conn: &Connection) -> rusqlite::Result<i64> {
    conn.pragma_query_value(None, "user_version", |r| r.get(0))
}

fn move_files(data: &Path) -> anyhow::Result<()> {
    let (old, new) = (data.join("repo"), data.join("repos/main"));
    if old.join("root/1.json").is_file() {
        ensure!(
            !new.exists(),
            "{} and {} both exist: move one aside and run again",
            old.display(),
            new.display()
        );
        fs::create_dir_all(new.parent().context("bad path")?)?;
        fs::rename(&old, &new)?;
    }
    let (old, new) = (data.join("keys/online.key"), data.join("keys/main.online.key"));
    if old.is_file() {
        ensure!(
            !new.exists(),
            "{} and {} both exist: move one aside and run again",
            old.display(),
            new.display()
        );
        fs::rename(&old, &new)?;
    }
    Ok(())
}
