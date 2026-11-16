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
}

#[cfg(test)]
mod tests;
