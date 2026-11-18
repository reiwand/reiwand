use reiwand_core::stored::StoredError;
use std::path::Path;

pub(crate) fn check(data: &Path) -> Result<(), StoredError> {
    reiwand_account::File::read(&data.join("accounts.json"))?;
    reiwand_core::stored::read::<std::collections::BTreeMap<String, String>>(&data.join("repo-pins.json"))?;
    reiwand_format::installed(data)
        .map_err(|e| StoredError::Invalid { path: "installed.json".into(), why: e.to_string() })?;
    reiwand_format::anchors(data)
        .map_err(|e| StoredError::Invalid { path: "anchors.json".into(), why: e.to_string() })?;
    Ok(())
}
