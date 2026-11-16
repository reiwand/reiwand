use reiwand_upgrade::{Outcome, server};
use rusqlite::Connection;
use std::fs;
use std::path::Path;

const OLD: &str = include_str!("fixtures/schema-11.sql");

const ROWS: &str = "
INSERT INTO users (id, handle, role, quota_bytes, created_at) VALUES (1, 'ana', 'user', 100, 0), (2, 'bob', 'admin', 100, 0);
INSERT INTO packages (id, owner_id, created_at) VALUES ('org.example.dawn', 1, 10), ('org.example.dusk', 1, 11);
INSERT INTO versions (id, package_id, version, sha256, size, kind, title, description, license, tags, authors, state, uploaded_by, created_at, published_at)
  VALUES (7, 'org.example.dawn', '1.0.0', 'aa', 1, 'image', 'Dawn', 'A calm sunrise', 'CC0-1.0', '[]', '[]', 'published', 1, 10, 20),
         (9, 'org.example.dusk', '1.0.0', 'bb', 1, 'image', 'Dusk', 'An evening', 'CC0-1.0', '[]', '[]', 'pending', 1, 11, NULL);
INSERT INTO cases (id, version_id, reason, created_at) VALUES (3, 9, 'upload', 11), (4, 7, 'report', 30);
INSERT INTO votes (package_id, user_id, value, at) VALUES ('org.example.dawn', 1, 1, 52), ('org.example.dawn', 2, -1, 53);
INSERT INTO featured (package_id, note, at) VALUES ('org.example.dawn', 'start here', 50);
UPDATE repo_state SET dirty = 0, signed_at = 612;
PRAGMA user_version = 11;
";

fn old_data(data: &Path) {
    fs::create_dir_all(data.join("db")).unwrap();
    let conn = Connection::open(data.join("db/reiwand.sqlite")).unwrap();
    conn.execute_batch(OLD).unwrap();
    conn.execute_batch(ROWS).unwrap();
    for (path, text) in
        [("repo/a", "first"), ("repo/catalog.json", "{}"), ("keys/online.key", "secret")]
    {
        fs::create_dir_all(data.join(path).parent().unwrap()).unwrap();
        fs::write(data.join(path), text).unwrap();
    }
}

fn count(conn: &Connection, sql: &str) -> i64 {
    conn.query_row(sql, [], |r| r.get(0)).unwrap()
}

fn assert_kept(data: &Path) {
    let conn = Connection::open(data.join("db/reiwand.sqlite")).unwrap();
    assert_eq!(count(&conn, "PRAGMA user_version"), 14);
    assert_eq!(count(&conn, "SELECT count(*) FROM packages WHERE repo = 'main'"), 2);
    assert_eq!(count(&conn, "SELECT count(*) FROM versions WHERE repo = 'main'"), 2);
    assert_eq!(count(&conn, "SELECT id FROM versions WHERE package_id = 'org.example.dawn'"), 7);
    assert_eq!(count(&conn, "SELECT count(*) FROM cases WHERE version_id IN (7, 9)"), 2);
    assert_eq!(count(&conn, "SELECT count(*) FROM votes WHERE repo = 'main'"), 2);
    assert_eq!(count(&conn, "SELECT count(*) FROM featured WHERE repo = 'main' AND note = 'start here'"), 1);
    assert_eq!(count(&conn, "SELECT signed_at FROM repos WHERE name = 'main'"), 612);
    assert_eq!(
        count(
            &conn,
            "SELECT count(*) FROM repos WHERE folder = 'repos/main' AND online_key = 'keys/main.online.key'"
        ),
        1
    );
    assert_eq!(count(&conn, "SELECT count(*) FROM repos WHERE private = 0 AND publishing = 'open'"), 1);
    assert_eq!(count(&conn, "SELECT count(*) FROM search WHERE search MATCH 'sunrise'"), 1);
    assert_eq!(count(&conn, "SELECT count(*) FROM pragma_foreign_key_check"), 0);
    assert_eq!(count(&conn, "SELECT count(*) FROM sqlite_master WHERE name = 'repo_state'"), 0);
    assert_eq!(fs::read_to_string(data.join("repos/main/a")).unwrap(), "first");
    assert_eq!(fs::read_to_string(data.join("keys/main.online.key")).unwrap(), "secret");
    assert!(!data.join("repo").exists() && !data.join("keys/online.key").exists());
}

#[test]
fn a_version_11_server_is_upgraded_once_keeping_everything() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    old_data(data);
    assert_eq!(server::run(data).unwrap(), Outcome::Upgraded { from: 11, to: 14 });
    assert_kept(data);
    let copy = Connection::open(data.join("db/reiwand.sqlite.before-v14-from-v11")).unwrap();
    assert_eq!(count(&copy, "PRAGMA user_version"), 11);
    assert_eq!(count(&copy, "SELECT count(*) FROM votes"), 2);
    assert_eq!(server::run(data).unwrap(), Outcome::Nothing);
    assert_kept(data);
}

#[test]
fn a_version_12_server_is_upgraded_too() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    old_data(data);
    let conn = Connection::open(data.join("db/reiwand.sqlite")).unwrap();
    conn.execute_batch("PRAGMA foreign_keys = OFF").unwrap();
    conn.execute_batch(include_str!("../migrations/11-12.sql")).unwrap();
    conn.execute_batch(
        "UPDATE repos SET folder = 'repo', online_key = 'keys/online.key'; PRAGMA user_version = 12",
    )
    .unwrap();
    drop(conn);
    assert_eq!(server::run(data).unwrap(), Outcome::Upgraded { from: 12, to: 14 });
    assert_kept(data);
}

#[test]
fn a_folder_without_a_database_or_with_a_foreign_one_is_refused_or_left_alone() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(server::run(dir.path()).unwrap(), Outcome::Nothing);
    fs::create_dir_all(dir.path().join("db")).unwrap();
    let conn = Connection::open(dir.path().join("db/reiwand.sqlite")).unwrap();
    conn.execute_batch("CREATE TABLE x (a); PRAGMA user_version = 5").unwrap();
    assert!(server::run(dir.path()).is_err());
}
