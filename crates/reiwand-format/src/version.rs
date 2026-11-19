use crate::{Error, Result};
use std::fs::{self, DirBuilder};
use std::io::ErrorKind;
use std::os::unix::fs::DirBuilderExt;
use std::path::Path;

pub const DATA_VERSION: u32 = 5;

const MARK: &str = "format-version";

pub fn check_data(data: &Path) -> Result<()> {
    let stored = match fs::read_to_string(data.join(MARK)) {
        Ok(text) => text.trim().parse::<u32>().ok(),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let held = data.join("sync.json").is_file()
                || fs::read_dir(data.join("repos")).is_ok_and(|mut d| d.next().is_some());
            if held {
                return Err(Error::Upgrade(data.display().to_string()));
            }
            DirBuilder::new().recursive(true).mode(0o700).create(data)?;
            fs::write(data.join(MARK), format!("{DATA_VERSION}\n"))?;
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };
    match stored {
        Some(DATA_VERSION) => Ok(()),
        _ => Err(Error::Upgrade(data.display().to_string())),
    }
}
