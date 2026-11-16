use reiwand_server::{App, Config};
use rusqlite::Connection;
use std::collections::HashMap;
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
INSERT INTO votes (package_id, user_id, value, at) VALUES ('org.example.dawn', 1, 1, 40), ('org.example.dawn', 2, -1, 41);
INSERT INTO featured (package_id, note, at) VALUES ('org.example.dawn', 'start here', 50);
UPDATE repo_state SET dirty = 0, signed_at = 500;
PRAGMA user_version = 11;
";

fn old_database(path: &Path) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let conn = Connection::open(path).unwrap();
    conn.execute_batch(OLD).unwrap();
    conn.execute_batch(ROWS).unwrap();
}

fn config(data: &Path) -> Config {
    let vars: HashMap<&str, String> = HashMap::from([
        ("REIWAND_DATA", data.display().to_string()),
        ("REIWAND_PUBLIC_URL", "http://127.0.0.1:1".into()),
    ]);
    Config::from_vars(|k| vars.get(k).cloned()).unwrap()
}

fn count(conn: &Connection, sql: &str) -> i64 {
    conn.query_row(sql, [], |r| r.get(0)).unwrap()
}

fn assert_kept(conn: &Connection) {
    assert_eq!(count(conn, "PRAGMA user_version"), 13);
    assert_eq!(count(conn, "SELECT count(*) FROM packages WHERE repo = 'main'"), 2);
    assert_eq!(count(conn, "SELECT count(*) FROM versions WHERE repo = 'main'"), 2);
    assert_eq!(count(conn, "SELECT id FROM versions WHERE package_id = 'org.example.dawn'"), 7);
    assert_eq!(count(conn, "SELECT count(*) FROM cases WHERE version_id IN (7, 9)"), 2);
    assert_eq!(
        count(conn, "SELECT sum(value) FROM votes WHERE repo = 'main' AND package_id = 'org.example.dawn'"),
        0
    );
    assert_eq!(count(conn, "SELECT count(*) FROM votes"), 2);
    assert_eq!(count(conn, "SELECT count(*) FROM featured WHERE repo = 'main' AND note = 'start here'"), 1);
    assert_eq!(count(conn, "SELECT signed_at FROM repos WHERE name = 'main'"), 500);
    assert_eq!(count(conn, "SELECT count(*) FROM repos"), 1);
    assert_eq!(count(conn, "SELECT private FROM repos WHERE name = 'main'"), 0);
    assert_eq!(count(conn, "SELECT count(*) FROM repos WHERE publishing = 'open'"), 1);
    assert_eq!(count(conn, "SELECT count(*) FROM repo_members"), 0);
    assert_eq!(count(conn, "SELECT count(*) FROM search WHERE search MATCH 'sunrise'"), 1);
    assert_eq!(count(conn, "SELECT rowid FROM search WHERE search MATCH 'sunrise'"), 7);
    assert_eq!(count(conn, "SELECT count(*) FROM pragma_foreign_key_check"), 0);
    let tables: i64 = count(conn, "SELECT count(*) FROM sqlite_master WHERE name = 'repo_state'");
    assert_eq!(tables, 0);
}

#[tokio::test]
async fn a_database_of_the_previous_version_keeps_everything_and_leaves_a_copy() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = config(&dir.path().join("data"));
    let path = cfg.data.join("db/reiwand.sqlite");
    old_database(&path);
    drop(App::open(cfg.clone()).unwrap());
    assert_kept(&Connection::open(&path).unwrap());
    let copy = cfg.data.join("db/reiwand.sqlite.before-v13-from-v11");
    let before = Connection::open(&copy).unwrap();
    assert_eq!(count(&before, "PRAGMA user_version"), 11);
    assert_eq!(count(&before, "SELECT count(*) FROM votes"), 2);
    assert_eq!(count(&before, "SELECT count(*) FROM repo_state"), 1);
    drop(App::open(cfg).unwrap());
    assert_kept(&Connection::open(&path).unwrap());
}

#[tokio::test]
async fn a_backup_of_the_previous_version_restores_and_migrates() {
    let dir = tempfile::tempdir().unwrap();
    let old = dir.path().join("old-backup");
    old_database(&old.join("reiwand.sqlite"));
    let sha = reiwand_format::sha256_file(old.join("reiwand.sqlite")).unwrap();
    let manifest = serde_json::json!({
        "format": "reiwand-server-backup/1", "created_at": 1, "files": {"reiwand.sqlite": sha}, "missing": [],
    });
    fs::write(old.join("manifest.json"), serde_json::to_vec(&manifest).unwrap()).unwrap();
    let cfg = config(&dir.path().join("restored"));
    reiwand_server::restore(&cfg, &old, false).unwrap();
    let conn = Connection::open(cfg.data.join("db/reiwand.sqlite")).unwrap();
    assert_kept(&conn);
    assert_eq!(count(&conn, "SELECT dirty FROM repos WHERE name = 'main'"), 1);
    drop(conn);
    let saved = reiwand_server::backup(&cfg, Some(dir.path().join("new-backup"))).unwrap();
    assert_eq!(count(&Connection::open(saved.join("reiwand.sqlite")).unwrap(), "PRAGMA user_version"), 13);
    let again = config(&dir.path().join("again"));
    reiwand_server::restore(&again, &saved, false).unwrap();
    assert_kept(&Connection::open(again.data.join("db/reiwand.sqlite")).unwrap());
}

#[tokio::test]
async fn a_database_of_the_version_before_private_repositories_keeps_everything_and_leaves_a_copy() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = config(&dir.path().join("data"));
    let path = cfg.data.join("db/reiwand.sqlite");
    old_database(&path);
    let conn = Connection::open(&path).unwrap();
    conn.execute_batch("PRAGMA foreign_keys = OFF").unwrap();
    conn.execute_batch(include_str!("../migrations/11-12.sql")).unwrap();
    conn.execute_batch("PRAGMA user_version = 12").unwrap();
    drop(conn);
    drop(App::open(cfg.clone()).unwrap());
    assert_kept(&Connection::open(&path).unwrap());
    let before = Connection::open(cfg.data.join("db/reiwand.sqlite.before-v13-from-v12")).unwrap();
    assert_eq!(count(&before, "PRAGMA user_version"), 12);
    assert_eq!(count(&before, "SELECT count(*) FROM versions"), 2);
    let conn = Connection::open(&path).unwrap();
    conn.execute_batch("INSERT INTO repo_members (repo, user_id) VALUES ('main', 1)").unwrap();
    assert_eq!(count(&conn, "SELECT may_read FROM repo_members"), 1);
    assert_eq!(count(&conn, "SELECT may_publish FROM repo_members"), 0);
}
