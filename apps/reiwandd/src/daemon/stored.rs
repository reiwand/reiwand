use reiwand_core::stored::StoredError;
use std::path::Path;

pub(crate) fn check(data: &Path) -> Result<(), StoredError> {
    reiwand_account::File::read(&data.join("accounts.json"))?;
    Ok(())
}
