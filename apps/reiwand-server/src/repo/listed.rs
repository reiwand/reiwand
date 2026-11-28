use std::path::Path;
use std::sync::PoisonError;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Shown {
    pub hidden: bool,
    pub unlisted: bool,
}

impl Shown {
    pub(super) fn mark(self, dir: &Path, id: &str, version: &str) -> reiwand_format::Result<()> {
        if self.hidden {
            reiwand_format::hide(dir, id, true)?;
        }
        if self.unlisted {
            reiwand_format::list(dir, id, Some(version), false)?;
        }
        Ok(())
    }
}

impl super::Repo {
    pub fn list(&self, id: &str, version: Option<&str>, listed: bool) -> reiwand_format::Result<()> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        reiwand_format::list(&self.dir, id, version, listed)
    }

    pub fn feature(&self, id: &str, note: Option<&str>, on: bool) -> reiwand_format::Result<()> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        reiwand_format::feature(&self.dir, id, note, on)
    }

    pub fn erase(&self) -> std::io::Result<()> {
        let _guard = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
        for key in [self.key.clone(), crate::seal::root_path(&self.data, &self.name)] {
            match std::fs::remove_file(key) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(e),
                _ => {}
            }
        }
        match std::fs::remove_dir_all(&self.dir) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            done => done,
        }
    }
}

#[cfg(test)]
mod tests;
