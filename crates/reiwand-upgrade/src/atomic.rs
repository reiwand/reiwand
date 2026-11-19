use anyhow::Context;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn target(path: &Path) -> PathBuf {
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.file_type().is_symlink() => {
            fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
        }
        _ => path.to_path_buf(),
    }
}

pub fn write(path: &Path, bytes: &[u8]) -> anyhow::Result<()> {
    let real = target(path);
    let name = real.file_name().context("no file name")?.to_string_lossy().into_owned();
    let staged = real.with_file_name(format!("{name}.upgrading"));
    let mut file = File::create(&staged)?;
    file.write_all(bytes)?;
    if let Ok(meta) = fs::metadata(&real) {
        fs::set_permissions(&staged, meta.permissions())?;
    }
    file.sync_all()?;
    drop(file);
    fs::rename(&staged, &real)?;
    if let Some(parent) = real.parent()
        && let Ok(dir) = File::open(parent)
    {
        let _ = dir.sync_all();
    }
    Ok(())
}
