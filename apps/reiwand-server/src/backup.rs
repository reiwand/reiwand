use crate::config::Config;
use crate::db::now;
use anyhow::{Context, Result, ensure};
use reiwand_format::sha256_file;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, DirBuilder};
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

const FORMAT: &str = "reiwand-server-backup/1";
const TREES: [&str; 5] = ["blobs", "thumbs", "held", "keys", "repos"];
const FILES: &str = "SELECT 'blobs/' || sha256 || '.wpak' FROM versions WHERE state = 'pending' \
                     UNION SELECT 'thumbs/' || sha256 || '.jpg' FROM versions WHERE state IN ('pending', 'published')";

#[derive(Serialize, Deserialize)]
struct Manifest {
    format: String,
    created_at: i64,
    files: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    missing: Vec<String>,
}

pub fn backup(cfg: &Config, dest: Option<PathBuf>) -> Result<PathBuf> {
    let stamp = chrono::DateTime::from_timestamp_millis(now()).unwrap_or_default().format("%Y%m%d-%H%M%S");
    let dest = dest.unwrap_or_else(|| cfg.data.join("backups").join(stamp.to_string()));
    ensure!(fs::read_dir(&dest).map_or(true, |mut d| d.next().is_none()), "{} is not empty", dest.display());
    DirBuilder::new().recursive(true).mode(0o700).create(&dest)?;
    let written = write(cfg, &dest);
    if written.is_err() {
        let _ = fs::remove_dir_all(&dest);
    }
    written.map(|()| dest)
}

fn write(cfg: &Config, dest: &Path) -> Result<()> {
    let mut last = None;
    let (mut files, missing) = 'taken: {
        for _ in 0..3 {
            let mut files = snapshot(cfg, dest)?;
            let missing: Vec<String> =
                files.iter().filter(|(_, sha)| sha.is_empty()).map(|(path, _)| path.clone()).collect();
            if missing.is_empty() || last.as_ref() == Some(&missing) {
                files.retain(|_, sha| !sha.is_empty());
                break 'taken (files, missing);
            }
            TREES[..3].iter().for_each(|t| drop(fs::remove_dir_all(dest.join(t))));
            last = Some(missing);
        }
        anyhow::bail!("files kept vanishing during three snapshots; try again")
    };
    for path in &missing {
        tracing::warn!("{path} is missing from the data directory and from this backup");
    }
    for tree in ["held", "keys", "repos"] {
        copy(&cfg.data.join(tree), &dest.join(tree), tree, &mut files)?;
    }
    let manifest = Manifest { format: FORMAT.into(), created_at: now(), files, missing };
    fs::write(dest.join("manifest.json"), serde_json::to_vec_pretty(&manifest)?)?;
    Ok(())
}

fn snapshot(cfg: &Config, dest: &Path) -> Result<BTreeMap<String, String>> {
    let db = Connection::open(cfg.data.join("db/reiwand.sqlite"))?;
    db.busy_timeout(Duration::from_secs(30))?;
    db.pragma_update(None, "temp_store", "MEMORY")?;
    let snapshot = dest.join("reiwand.sqlite");
    let _ = fs::remove_file(&snapshot);
    db.execute("VACUUM INTO ?1", [snapshot.to_str().context("the backup path is not UTF-8")?])?;
    let mut files = BTreeMap::from([("reiwand.sqlite".to_string(), sha256_file(&snapshot)?)]);
    let taken = Connection::open(&snapshot)?;
    let wanted = taken.prepare(FILES)?.query_map([], |r| r.get(0))?.collect::<Result<Vec<String>, _>>()?;
    for path in wanted {
        let target = dest.join(&path);
        DirBuilder::new().recursive(true).mode(0o700).create(target.parent().context("bad path")?)?;
        let copied = fs::copy(cfg.data.join(&path), &target).map(|_| ());
        let sha = if copied.is_ok() { sha256_file(&target)? } else { String::new() };
        files.insert(path, sha);
    }
    Ok(files)
}

fn copy(from: &Path, to: &Path, prefix: &str, files: &mut BTreeMap<String, String>) -> Result<()> {
    let Ok(items) = fs::read_dir(from) else { return Ok(()) };
    DirBuilder::new().recursive(true).mode(0o700).create(to)?;
    for item in items {
        let item = item?;
        let name = item.file_name().to_string_lossy().into_owned();
        let (source, target, key) = (item.path(), to.join(&name), format!("{prefix}/{name}"));
        let kind = item.file_type()?;
        if kind.is_dir() {
            copy(&source, &target, &key, files)?;
        } else if kind.is_file() && !name.starts_with(".tmp-") {
            fs::copy(&source, &target)?;
            files.insert(key, sha256_file(&target)?);
        }
    }
    Ok(())
}

pub fn restore(cfg: &Config, src: &Path, force: bool) -> Result<()> {
    let manifest: Manifest = serde_json::from_slice(
        &fs::read(src.join("manifest.json")).context("no manifest.json in the backup")?,
    )?;
    ensure!(manifest.format == FORMAT, "unknown backup format {}", manifest.format);
    for (path, sha) in &manifest.files {
        let tree = path.split('/').next().unwrap_or_default();
        let known = path == "reiwand.sqlite" || (TREES.contains(&tree) && path.len() > tree.len());
        let safe = known && !path.split('/').any(|s| s == ".." || s == "." || s.is_empty());
        ensure!(safe, "unsafe path {path} in the manifest");
        ensure!(sha256_file(src.join(path))? == *sha, "{path} does not match its checksum");
    }
    let check: String =
        Connection::open(src.join("reiwand.sqlite"))?
            .query_row("PRAGMA integrity_check", [], |r| r.get(0))?;
    ensure!(check == "ok", "the database snapshot is damaged: {check}");
    crate::db::check(&Connection::open(src.join("reiwand.sqlite"))?)?;
    let db = cfg.data.join("db/reiwand.sqlite");
    ensure!(
        force || !db.exists(),
        "{} already holds a database: stop the server and pass --force to replace it",
        cfg.data.display()
    );
    let folders = folders(&cfg.data, src);
    let seen: Vec<(String, Option<u64>)> =
        folders.iter().map(|f| (f.clone(), clear(&cfg.data, src, f))).collect();
    for path in manifest.files.keys().filter(|p| *p != "reiwand.sqlite") {
        let target = cfg.data.join(path);
        DirBuilder::new().recursive(true).mode(0o700).create(target.parent().context("bad path")?)?;
        fs::copy(src.join(path), &target)?;
    }
    DirBuilder::new().recursive(true).mode(0o700).create(cfg.data.join("db"))?;
    let staged = cfg.data.join("db/restore.sqlite");
    fs::copy(src.join("reiwand.sqlite"), &staged)?;
    for suffix in ["-wal", "-shm"] {
        let _ = fs::remove_file(format!("{}{suffix}", db.display()));
    }
    fs::rename(&staged, &db)?;
    for (folder, seen) in seen {
        let repo = cfg.data.join(folder);
        if let Some(n) = seen.filter(|n| latest(&repo).is_none_or(|r| r < *n)) {
            fs::write(repo.join("entry/latest"), format!("{n}\n"))?;
        }
    }
    let mut conn = Connection::open(&db)?;
    crate::db::create(&mut conn)?;
    conn.execute("UPDATE repos SET dirty = 1", [])?;
    Ok(())
}

fn latest(repo: &Path) -> Option<u64> {
    fs::read_to_string(repo.join("entry/latest")).ok()?.trim().parse().ok()
}

fn folders(data: &Path, src: &Path) -> Vec<String> {
    let mut found = Vec::new();
    for base in [data, src] {
        for item in fs::read_dir(base.join("repos")).into_iter().flatten().flatten() {
            let folder = format!("repos/{}", item.file_name().to_string_lossy());
            if !found.contains(&folder) {
                found.push(folder);
            }
        }
    }
    found
}

fn clear(data: &Path, src: &Path, folder: &str) -> Option<u64> {
    let repo = data.join(folder);
    let root = |dir: &Path| fs::read(dir.join("root/1.json")).ok();
    let same = root(&repo).is_some_and(|live| root(&src.join(folder)) == Some(live));
    let seen = latest(&repo).filter(|_| same);
    for item in fs::read_dir(&repo).into_iter().flatten().flatten() {
        let signed = ["root", "entry", "index", "revoked"].iter().any(|d| item.file_name() == *d);
        match item.file_type().is_ok_and(|t| t.is_dir()) {
            _ if same && signed => {}
            true => drop(fs::remove_dir_all(item.path())),
            false => drop(fs::remove_file(item.path())),
        }
    }
    seen
}
