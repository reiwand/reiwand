use crate::sink::Sink;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) const SETTLED: i64 = 3;
pub(crate) const GLOWING: i64 = 4;

#[cfg(any(feature = "client", feature = "server"))]
fn folders(data: &Path, also: &[PathBuf]) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = also.to_vec();
    let opened = rusqlite::Connection::open_with_flags(
        data.join("library.db"),
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    );
    let listed = opened.ok().and_then(|db| {
        let mut ask = db.prepare("SELECT uri FROM sources WHERE kind = 'folder'").ok()?;
        let rows = ask.query_map([], |r| r.get::<_, String>(0)).ok()?;
        Some(rows.flatten().collect::<Vec<_>>())
    });
    for uri in listed.into_iter().flatten() {
        found.push(PathBuf::from(uri.strip_prefix("file://").unwrap_or(&uri)));
    }
    found
}

#[cfg(not(any(feature = "client", feature = "server")))]
fn folders(_: &Path, also: &[PathBuf]) -> Vec<PathBuf> {
    also.to_vec()
}

pub(crate) fn steps(data: &Path, sink: &mut Sink, also: &[PathBuf], found: i64) -> anyhow::Result<()> {
    if found < SETTLED {
        crate::phone::settings(data, sink)?;
        crate::edits::folder(&data.join("edits"), sink)?;
        for item in fs::read_dir(data.join("repos")).into_iter().flatten().flatten() {
            let state = item.path().join("state.json");
            sink.attempt(&state, |sink| crate::registries::plain(&state, sink));
        }
        let installed = data.join("installed.json");
        if let Err(e) = crate::packages::installed(data, sink) {
            sink.skip(&installed, format!("{e:#}"));
        }
        crate::stored::files(data, sink)?;
        for folder in folders(data, also) {
            crate::scenes::walk_in(&folder, sink)?;
        }
        #[cfg(any(feature = "client", feature = "server"))]
        crate::library::run(data, sink)?;
    } else if found <= GLOWING {
        let installed = data.join("installed.json");
        if let Err(e) = crate::packages::installed(data, sink) {
            sink.skip(&installed, format!("{e:#}"));
        }
        for folder in folders(data, also) {
            crate::scenes::walk_in(&folder, sink)?;
        }
    }
    crate::examples::repaired(data, sink)?;
    #[cfg(any(feature = "client", feature = "server"))]
    {
        crate::imported::packages(data, sink, found <= GLOWING)?;
        crate::imported::edits(data, sink)?;
    }
    Ok(())
}
