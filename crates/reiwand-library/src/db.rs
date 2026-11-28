use crate::patient::patient;
use crate::{Error, Library, Result};
use rusqlite::types::Type;
use rusqlite::{Connection, OpenFlags, OptionalExtension, Params, Row, TransactionBehavior};
use serde::de::DeserializeOwned;
use std::fs::{self, DirBuilder, OpenOptions, Permissions};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::path::Path;

const SCHEMA: &str = include_str!("schema.sql");

pub const SCHEMA_VERSION: u32 = 9;

impl Library {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty() && !p.exists()) {
            DirBuilder::new().recursive(true).mode(0o700).create(parent)?;
        }
        if !path.exists() {
            OpenOptions::new().create(true).append(true).mode(0o600).open(path)?;
        }
        let library = Self::setup(Connection::open(path)?)?;
        for suffix in ["", "-wal", "-shm"] {
            let mut file = path.as_os_str().to_owned();
            file.push(suffix);
            if Path::new(&file).exists() {
                fs::set_permissions(&file, Permissions::from_mode(0o600))?;
            }
        }
        Ok(library)
    }

    pub fn open_read_only(path: impl AsRef<Path>) -> Result<Self> {
        let flags = OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        let db = Connection::open_with_flags(path, flags)?;
        match version(&db)? {
            SCHEMA_VERSION => Ok(Self { db }),
            other => Err(Error::Foreign(other)),
        }
    }

    pub fn open_in_memory() -> Result<Self> {
        Self::setup(Connection::open_in_memory()?)
    }

    fn setup(mut db: Connection) -> Result<Self> {
        db.set_transaction_behavior(TransactionBehavior::Immediate);
        patient(|| {
            db.execute_batch(
                "PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL; PRAGMA foreign_keys = ON;",
            )?;
            Ok(())
        })?;
        patient(|| create(&mut db))?;
        Ok(Self { db })
    }
}

fn version(db: &Connection) -> Result<u32> {
    Ok(db.pragma_query_value(None, "user_version", |row| row.get(0))?)
}

fn create(db: &mut Connection) -> Result<()> {
    let tx = db.transaction()?;
    match version(&tx)? {
        SCHEMA_VERSION => return Ok(()),
        0 if empty(&tx)? => {}
        other => return Err(Error::Foreign(other)),
    }
    tx.execute_batch(SCHEMA)?;
    tx.pragma_update(None, "user_version", SCHEMA_VERSION)?;
    Ok(tx.commit()?)
}

fn empty(db: &Connection) -> Result<bool> {
    Ok(db.query_row("SELECT count(*) = 0 FROM sqlite_master", [], |row| row.get(0))?)
}

pub(crate) fn ensure(ok: bool, error: impl FnOnce() -> Error) -> Result<()> {
    if ok { Ok(()) } else { Err(error()) }
}

pub(crate) fn uid() -> String {
    ulid::Ulid::generate().to_string()
}

pub(crate) fn printable(text: &str, max: usize) -> bool {
    !text.is_empty() && text.chars().count() <= max && !text.chars().any(char::is_control)
}

pub(crate) fn label<'a>(what: &'static str, text: &'a str, max: usize) -> Result<&'a str> {
    let text = text.trim();
    let reason = || format!("{text:?} must have 1 to {max} printable characters");
    ensure(printable(text, max), || Error::Invalid(what, reason()))?;
    Ok(text)
}

pub(crate) fn unique<T>(result: rusqlite::Result<T>, what: &'static str, name: &str) -> Result<T> {
    match result {
        Err(rusqlite::Error::SqliteFailure(e, _))
            if e.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE =>
        {
            Err(Error::Exists(what, name.into()))
        }
        other => Ok(other?),
    }
}

pub(crate) fn row_id(db: &Connection, what: &'static str, key: &str) -> Result<i64> {
    let named = matches!(what, "playlist" | "profile");
    let sql = match named {
        true => format!("SELECT id FROM {what}s WHERE uid = ?1 OR name = ?1 ORDER BY uid = ?1 DESC LIMIT 1"),
        false => format!("SELECT id FROM {what}s WHERE uid = ?1"),
    };
    let found = db.query_row(&sql, [key], |row| row.get(0)).optional()?;
    match found {
        Some(id) => Ok(id),
        None if named && key.chars().count() >= 6 => ending(db, what, key),
        None => Err(Error::NotFound(what, key.into())),
    }
}

fn ending(db: &Connection, what: &'static str, key: &str) -> Result<i64> {
    let sql = format!("SELECT id FROM {what}s WHERE substr(uid, -length(?1)) = ?1");
    let ids: Vec<i64> = rows(db, &sql, [key.to_ascii_uppercase()], |row| row.get(0))?;
    match ids[..] {
        [id] => Ok(id),
        [] => Err(Error::NotFound(what, key.into())),
        _ => Err(Error::Invalid(what, format!("{key:?} matches {} {what}s; use more characters", ids.len()))),
    }
}

pub(crate) fn item_ids(db: &Connection, uids: &[&str]) -> Result<Vec<i64>> {
    uids.iter().map(|uid| row_id(db, "item", uid)).collect()
}

pub(crate) fn remove(db: &Connection, what: &'static str, key: &str) -> Result<()> {
    let id = row_id(db, what, key)?;
    db.execute(&format!("DELETE FROM {what}s WHERE id = ?1"), [id])?;
    Ok(())
}

pub(crate) fn rows<T>(
    db: &Connection,
    sql: &str,
    params: impl Params,
    map: impl FnMut(&Row<'_>) -> rusqlite::Result<T>,
) -> Result<Vec<T>> {
    let mut statement = db.prepare(sql)?;
    let rows = statement.query_map(params, map)?.collect::<rusqlite::Result<_>>()?;
    Ok(rows)
}

pub(crate) fn json<T: DeserializeOwned>(row: &Row<'_>, index: usize) -> rusqlite::Result<T> {
    let text: String = row.get(index)?;
    serde_json::from_str(&text)
        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(index, Type::Text, e.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn a_second_writer_waits_for_the_first_instead_of_failing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("library.db");
        let (mut first, mut second) = (Library::open(&path).unwrap(), Library::open(&path).unwrap());
        let tx = first.db.transaction().unwrap();
        tx.execute("INSERT INTO confirmations VALUES ('a', 'x')", []).unwrap();
        let other = std::thread::spawn(move || {
            let tx = second.db.transaction().unwrap();
            let seen: i64 = tx.query_row("SELECT count(*) FROM confirmations", [], |r| r.get(0)).unwrap();
            tx.execute("INSERT INTO confirmations VALUES ('b', ?1)", [seen.to_string()]).unwrap();
            tx.commit().unwrap();
            seen
        });
        std::thread::sleep(Duration::from_millis(200));
        tx.commit().unwrap();
        assert_eq!(other.join().unwrap(), 1, "the second writer read before the first one committed");
    }
}
