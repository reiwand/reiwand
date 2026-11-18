use crate::sink::Sink;
use rusqlite::{Connection, OpenFlags, params};
use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

fn open(data: &Path, flags: OpenFlags) -> Option<Connection> {
    let path = data.join("library.db");
    path.is_file().then(|| Connection::open_with_flags(path, flags).ok()).flatten()
}

fn paths(db: &Connection, sql: &str) -> Vec<PathBuf> {
    let Ok(mut ask) = db.prepare(sql) else { return Vec::new() };
    let Ok(rows) = ask.query_map([], |r| r.get::<_, String>(0)) else { return Vec::new() };
    rows.flatten().map(PathBuf::from).collect()
}

pub(crate) fn edits(data: &Path, sink: &mut Sink) -> anyhow::Result<()> {
    let Some(db) = open(data, OpenFlags::SQLITE_OPEN_READ_ONLY) else { return Ok(()) };
    let loose = "SELECT path FROM items WHERE kind = 'image' AND (source_id IS NULL OR \
        source_id IN (SELECT id FROM sources WHERE kind != 'folder'))";
    let folders: BTreeSet<PathBuf> =
        paths(&db, loose).iter().filter_map(|p| p.parent().map(Path::to_path_buf)).collect();
    drop(db);
    for folder in folders {
        for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.file_name().is_some_and(|n| n.to_string_lossy().ends_with(".reiwand.json")) {
                crate::edits::file(&path, sink)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn packages(data: &Path, sink: &mut Sink) -> anyhow::Result<()> {
    let Some(db) = open(data, OpenFlags::SQLITE_OPEN_READ_ONLY) else { return Ok(()) };
    // A package the library already refused, or one whose manifest does not load in the current format: the
    // second covers the first start of a new version, before the daemon has read the package once.
    let listed = paths(&db, "SELECT path FROM items WHERE kind = 'package' AND missing = 0");
    let refused: BTreeSet<PathBuf> =
        paths(&db, "SELECT path FROM items WHERE kind = 'package' AND missing = 0 AND error IS NOT NULL")
            .into_iter()
            .collect();
    drop(db);
    let mut converted = Vec::new();
    let stale = |p: &PathBuf| p.is_file() && (refused.contains(p) || reiwand_format::peek(p).is_err());
    for path in listed.into_iter().filter(stale) {
        if crate::packages::wpak(&path, sink).unwrap_or(false) {
            converted.push(path);
        }
    }
    if sink.is_check() || converted.is_empty() {
        return Ok(());
    }
    let Some(db) = open(data, OpenFlags::SQLITE_OPEN_READ_WRITE) else { return Ok(()) };
    for path in converted {
        let meta = fs::metadata(&path)?;
        let hash = reiwand_library::qhash(&path)?;
        let sql = "UPDATE items SET size = ?2, mtime = ?3, qhash = ?4, pkg_read = NULL WHERE path = ?1";
        db.execute(sql, params![path.to_string_lossy(), meta.len() as i64, meta.mtime(), hash.as_slice()])?;
    }
    Ok(())
}
