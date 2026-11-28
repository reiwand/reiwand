use crate::fetch::Progress;
use crate::files::{folder, sha256, sweep};
use crate::models::NOTICES;
use crate::{Asset, Places, download};
use anyhow::{Context, Result, anyhow, bail, ensure};
use reiwand_core::message::said;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

const TEMPORARY: [&str; 2] = [".install-", ".unpack-"];

fn notes(asset: &Asset, dir: &Path) -> Result<()> {
    std::fs::write(dir.join("NOTICE.txt"), asset.notice())?;
    Ok(std::fs::write(dir.join("LICENSE.txt"), asset.license_text)?)
}

/// One download or install of an asset at a time, across the app, the daemon and the command line:
/// they would otherwise write the same partial file.
fn hold(dir: &Path, asset: &Asset) -> Result<File> {
    let mut options = std::fs::OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    let file = options.open(dir.join(format!(".{}.lock", asset.file)))?;
    file.lock()?;
    Ok(file)
}

impl Places {
    pub fn download(&self, asset: &Asset, progress: Progress<'_>) -> Result<PathBuf> {
        let dest = self.file(asset);
        let dir = dest
            .parent()
            .context(said("depth.no_folder_for_the_download", "no folder for the download", &[]).local())?;
        folder(dir)?;
        let _held = hold(dir, asset)?;
        if self.installed(asset).is_some() {
            return Ok(dest);
        }
        sweep(dir, &TEMPORARY);
        if asset.member.is_empty() {
            download(asset.urls, &dest, asset.bytes, asset.sha256, progress)?;
        } else {
            let archive = dir.join(asset.archive());
            download(asset.urls, &archive, asset.bytes, asset.sha256, progress)?;
            let unpacked = self.unpack(asset, &archive, &dest);
            std::fs::remove_file(&archive).ok();
            unpacked?;
        }
        notes(asset, dir)?;
        Ok(dest)
    }

    pub fn install(&self, asset: &Asset, file: &Path) -> Result<PathBuf> {
        let wrong = || {
            anyhow!(
                "{}",
                said(
                    "depth.is_not_its_sha_256",
                    "%1 is not %2: its SHA-256 differs",
                    &[&file.display(), &asset.title]
                )
                .local()
            )
        };
        let dest = self.file(asset);
        let dir = dest
            .parent()
            .context(said("depth.no_folder_for_the_file", "no folder for the file", &[]).local())?;
        folder(dir)?;
        let _held = hold(dir, asset)?;
        sweep(dir, &TEMPORARY);
        let mut copy = tempfile::Builder::new().prefix(".install-").tempfile_in(dir)?;
        std::io::copy(&mut File::open(file)?, copy.as_file_mut())?;
        copy.as_file().sync_all()?;
        ensure!(sha256(copy.path())? == asset.sha256, wrong());
        match asset.member.is_empty() {
            true => drop(copy.persist(&dest).map_err(|e| e.error)?),
            false => self.unpack(asset, copy.path(), &dest)?,
        }
        notes(asset, dir)?;
        Ok(dest)
    }

    fn unpack(&self, asset: &Asset, archive: &Path, dest: &Path) -> Result<()> {
        let dir = dest
            .parent()
            .context(said("depth.no_folder_for_the_library", "no folder for the library", &[]).local())?;
        let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(File::open(archive)?));
        let (mut library, mut notices) = (None, None);
        for entry in tar.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.into_owned();
            let found = match () {
                _ if path == Path::new(asset.member) => &mut library,
                _ if path == Path::new(asset.notices) => &mut notices,
                _ => continue,
            };
            let mut out = tempfile::Builder::new().prefix(".unpack-").tempfile_in(dir)?;
            std::io::copy(&mut entry.by_ref().take(512 << 20), out.as_file_mut())?;
            out.as_file().sync_all()?;
            *found = Some(out);
        }
        let (Some(library), Some(notices)) = (library, notices) else {
            bail!(
                "{}",
                said(
                    "depth.the_archive_does_not_hold",
                    "the archive does not hold %1 and its notices",
                    &[&asset.member]
                )
                .local()
            )
        };
        ensure!(
            sha256(library.path())? == asset.file_sha256,
            "{}",
            said("depth.in_the_archive_has_another", "%1 in the archive has another SHA-256", &[&asset.file])
                .local()
        );
        notices.persist(dir.join(NOTICES)).map_err(|e| e.error)?;
        library.persist(dest).map(drop).map_err(|e| e.error.into())
    }

    pub fn verify(&self, asset: &Asset) -> Result<bool> {
        match self.installed(asset) {
            Some(path) => Ok(sha256(&path)? == asset.file_sha256),
            None => Ok(false),
        }
    }

    pub fn remove(&self, asset: &Asset) -> Result<bool> {
        let dir = self.file(asset).parent().map(Path::to_path_buf).unwrap_or_default();
        match std::fs::remove_dir_all(&dir) {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e.into()),
        }
    }
}
