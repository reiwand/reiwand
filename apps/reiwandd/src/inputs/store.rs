use super::{Bound, Grants};
use crate::daemon::Daemon;
use std::io::Write;
use std::path::{Path, PathBuf};

pub(crate) fn grants_path(daemon: &Daemon) -> PathBuf {
    daemon.store.data.join("inputs.json")
}

pub(crate) fn declined_path(grants: &Path) -> PathBuf {
    grants.with_file_name("inputs-declined.json")
}

fn read<T: serde::de::DeserializeOwned + Default>(path: &Path) -> T {
    let Ok(bytes) = std::fs::read(path) else { return T::default() };
    serde_json::from_slice(&bytes).unwrap_or_else(|e| {
        tracing::warn!("ignoring unreadable input permissions: {e}");
        T::default()
    })
}

pub(crate) fn load(path: &Path) -> Grants {
    read(path)
}

pub(crate) fn load_bound(path: &Path) -> Bound {
    read(path)
}

pub(crate) fn save(path: &Path, value: &impl serde::Serialize) -> std::io::Result<()> {
    let dir = path.parent().unwrap_or(Path::new("."));
    crate::store::private_dir(dir)?;
    let mut file = tempfile::Builder::new().prefix(".tmp-").tempfile_in(dir)?;
    file.write_all(&serde_json::to_vec_pretty(value)?)?;
    file.persist(path).map(drop).map_err(|e| e.error)
}
