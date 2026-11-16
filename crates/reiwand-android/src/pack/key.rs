use super::Stamp;
use crate::Error;
use std::path::Path;
use std::sync::{Mutex, PoisonError};

static HASHED: Mutex<Vec<(Stamp, String)>> = Mutex::new(Vec::new());

fn sha256(path: &Path, stamp: &Stamp) -> Result<String, Error> {
    let mut hashed = HASHED.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some((_, sha256)) = hashed.iter().find(|(s, _)| s == stamp) {
        return Ok(sha256.clone());
    }
    let sha256 = reiwand_format::sha256_file(path)?;
    hashed.retain(|(s, _)| s.0 != stamp.0);
    hashed.push((stamp.clone(), sha256.clone()));
    Ok(sha256)
}

pub(crate) fn of(path: &Path) -> Option<String> {
    key(path, &super::stamp(path).ok()?).ok()
}

pub(super) fn key(path: &Path, stamp: &Stamp) -> Result<String, Error> {
    let sha256 = sha256(path, stamp)?;
    let installed = crate::repos::dir().ok().and_then(|dir| reiwand_format::installed(&dir).ok());
    let record = installed.unwrap_or_default().into_iter().find(|i| i.sha256 == sha256);
    let signer = record.and_then(|i| i.signer);
    Ok(signer.map_or_else(|| format!("sha256:{sha256}"), |fingerprint| format!("key:{fingerprint}")))
}
