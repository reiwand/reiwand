use crate::error::{Problem, Res};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use rusqlite::types::FromSql;
use rusqlite::types::ValueRef;
use rusqlite::{Connection, OptionalExtension, Params, params};
use serde_json::{Map, Value};
use std::path::Path;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

pub const MINUTE: i64 = 60_000;
pub const HOUR: i64 = 60 * MINUTE;
pub const DAY: i64 = 24 * HOUR;

const SCHEMA: &str = include_str!("../schema.sql");
pub const SCHEMA_VERSION: i64 = 13;
pub const MIN_SCHEMA: i64 = 11;
const MIGRATIONS: [&str; 2] =
    [include_str!("../migrations/11-12.sql"), include_str!("../migrations/12-13.sql")];

pub fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

#[derive(Clone)]
pub struct Db(Arc<Mutex<Connection>>);

impl Db {
    pub fn open(path: &Path) -> anyhow::Result<Db> {
        if path.exists() {
            backup_before_migrating(path)?;
        }
        let mut conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        for (name, value) in [
            ("synchronous", "NORMAL"),
            ("foreign_keys", "ON"),
            ("busy_timeout", "5000"),
            ("temp_store", "MEMORY"),
        ] {
            conn.pragma_update(None, name, value)?;
        }
        create(&mut conn)?;
        Ok(Db(Arc::new(Mutex::new(conn))))
    }

    pub async fn items(&self, sql: &'static str, values: Vec<rusqlite::types::Value>) -> crate::Reply {
        let found = self.run(move |c| rows(c, sql, rusqlite::params_from_iter(values))).await?;
        Ok(axum::Json(serde_json::json!({"items": found})))
    }

    pub fn read<T>(&self, work: impl FnOnce(&Connection) -> T) -> T {
        work(&self.0.lock().unwrap_or_else(PoisonError::into_inner))
    }

    pub async fn run<T: Send + 'static>(
        &self,
        work: impl FnOnce(&mut Connection) -> Res<T> + Send + 'static,
    ) -> Res<T> {
        let db = self.0.clone();
        tokio::task::spawn_blocking(move || work(&mut db.lock().unwrap_or_else(PoisonError::into_inner)))
            .await?
    }
}

pub fn create(conn: &mut Connection) -> anyhow::Result<()> {
    conn.pragma_update(None, "foreign_keys", "OFF")?;
    let done = migrate(conn);
    conn.pragma_update(None, "foreign_keys", "ON")?;
    done
}

fn migrate(conn: &mut Connection) -> anyhow::Result<()> {
    let tx = conn.transaction()?;
    let empty: bool = tx.query_row("SELECT count(*) = 0 FROM sqlite_master", [], |r| r.get(0))?;
    let found = version(&tx)?;
    match found {
        SCHEMA_VERSION => return Ok(()),
        0 if empty => tx.execute_batch(SCHEMA)?,
        MIN_SCHEMA..SCHEMA_VERSION => {
            for step in found..SCHEMA_VERSION {
                tx.execute_batch(MIGRATIONS[(step - MIN_SCHEMA) as usize])?;
            }
            let broken: i64 =
                tx.query_row("SELECT count(*) FROM pragma_foreign_key_check", [], |r| r.get(0))?;
            anyhow::ensure!(broken == 0, "the migration left {broken} rows without their parent");
            tracing::info!(from = found, to = SCHEMA_VERSION, "migrated the database");
        }
        other => return Err(foreign(other)),
    }
    tx.pragma_update(None, "user_version", SCHEMA_VERSION)?;
    Ok(tx.commit()?)
}

fn backup_before_migrating(path: &Path) -> anyhow::Result<()> {
    let conn = Connection::open(path)?;
    let found = version(&conn)?;
    if (MIN_SCHEMA..SCHEMA_VERSION).contains(&found) {
        let copy = path.with_extension(format!("sqlite.before-v{SCHEMA_VERSION}-from-v{found}"));
        if !copy.exists() {
            conn.execute("VACUUM INTO ?1", [copy.to_string_lossy().as_ref()])?;
            tracing::info!(copy = %copy.display(), "kept a copy of the database before migrating it");
        }
    }
    Ok(())
}

pub fn check(conn: &Connection) -> anyhow::Result<()> {
    match version(conn)? {
        MIN_SCHEMA..=SCHEMA_VERSION => Ok(()),
        other => Err(foreign(other)),
    }
}

fn version(conn: &Connection) -> rusqlite::Result<i64> {
    conn.pragma_query_value(None, "user_version", |r| r.get(0))
}

fn foreign(found: i64) -> anyhow::Error {
    let message = format!(
        "the database was made by another version of the program (schema {found}, this one reads \
         {MIN_SCHEMA} to {SCHEMA_VERSION}) and is not supported; it was left untouched"
    );
    tracing::error!("{message}");
    anyhow::anyhow!(message)
}

pub fn audit(conn: &Connection, actor: Option<i64>, action: &str, subject: &str, detail: &str) -> Res<()> {
    let sql = "INSERT INTO audit (at, actor_id, action, subject, detail) VALUES (?1, ?2, ?3, ?4, ?5)";
    conn.execute(sql, params![now(), actor, action, subject, detail])?;
    Ok(())
}

pub fn changed(rows: usize, what: &str) -> Res<()> {
    if rows == 0 { Err(Problem::missing(what)) } else { Ok(()) }
}

pub fn one<T: FromSql>(conn: &Connection, sql: &str, values: impl Params) -> Res<Option<T>> {
    Ok(conn.query_row(sql, values, |r| r.get(0)).optional()?)
}

pub fn cut(
    conn: &Connection,
    sql: &str,
    values: impl Params,
    from: i64,
    limit: i64,
    budget: i64,
) -> Res<(i64, bool)> {
    let mut statement = conn.prepare(sql)?;
    let found = statement.query_map(values, |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)))?;
    let found = found.collect::<Result<Vec<_>, _>>()?;
    let (mut last, mut total, mut taken) = (from, 0, 0);
    for (key, size) in &found {
        if taken == limit || (taken > 0 && total + size > budget) {
            break;
        }
        (last, total, taken) = (*key, total + size, taken + 1);
    }
    Ok((last, (taken as usize) < found.len()))
}

pub fn rows(conn: &Connection, sql: &str, values: impl Params) -> Res<Vec<Value>> {
    let mut statement = conn.prepare(sql)?;
    let names: Vec<String> = statement.column_names().into_iter().map(String::from).collect();
    let rows = statement.query_map(values, |r| {
        let fields = names.iter().enumerate().map(|(i, name)| Ok((name.clone(), json(r.get_ref(i)?))));
        fields.collect::<rusqlite::Result<Map<_, _>>>().map(Value::Object)
    })?;
    Ok(rows.collect::<Result<_, _>>()?)
}

fn json(value: ValueRef) -> Value {
    match value {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(n) => n.into(),
        ValueRef::Real(f) => f.into(),
        ValueRef::Text(t) => String::from_utf8_lossy(t).into(),
        ValueRef::Blob(b) => BASE64_STANDARD.encode(b).into(),
    }
}
