use crate::daemon::Daemon;
use crate::error::{Error, Result, denied, failed, invalid};
use crate::imports::Brought;
use crate::store::private_dir;
use reiwand_carry::{Limits, Opened, OwnFile};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub fn open(path: &str, password: &str) -> Result<Opened> {
    let path = PathBuf::from(path);
    if !path.is_absolute() {
        return Err(invalid(format!("{} is not an absolute path", path.display())));
    }
    let password = Some(password).filter(|p| !p.is_empty());
    reiwand_carry::open(&path, password, Limits::default()).map_err(|e| match e {
        reiwand_carry::Error::NeedsPassword => invalid("the file is encrypted: give its password"),
        reiwand_carry::Error::WrongPassword => denied("wrong password, or the file is damaged"),
        other => invalid(other.to_string()),
    })
}

impl Daemon {
    pub fn carry_own(&self, opened: &mut Opened, own: &OwnFile) -> Result<String> {
        let dir = self.store.cache.join("carry");
        private_dir(&dir)?;
        let name = Path::new(&own.name).file_name().map_or_else(|| own.id.clone().into(), PathBuf::from);
        let staged = dir.join(name);
        let written = File::create(&staged)
            .map_err(Error::from)
            .and_then(|mut out| opened.extract_own(&own.id, &mut out).map_err(|e| failed(e.to_string())));
        let imported = written.and_then(|()| self.import_one(&staged, (true, None)).map(Brought::uid));
        drop(fs::remove_file(&staged));
        imported
    }
}
