use crate::{Error, said};
use reiwand_carry::{Limits, Opened};
use std::path::Path;

pub(super) fn open(path: &str, password: &str) -> Result<Opened, Error> {
    let password = Some(password).filter(|p| !p.is_empty());
    reiwand_carry::open(Path::new(path), password, Limits::default()).map_err(|e| match e {
        reiwand_carry::Error::NeedsPassword => {
            said("android.carry_needs_password", "The file is encrypted: type its password", &[])
        }
        reiwand_carry::Error::WrongPassword => {
            said("android.carry_wrong_password", "Wrong password, or the file is damaged", &[])
        }
        other => said("android.carry_unreadable", "This file cannot be read: %1", &[&other.to_string()]),
    })
}
