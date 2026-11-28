use crate::{Error, Result};

pub fn check_name(name: &str) -> Result<()> {
    let bad = name.is_empty()
        || name.starts_with('/')
        || name.contains('\\')
        || name.contains('\0')
        || name.contains(':')
        || name.split('/').any(|part| part == ".." || part == ".");
    if bad { Err(Error::Unsafe(name.chars().take(80).collect())) } else { Ok(()) }
}

pub fn check_id(id: &str) -> Result<()> {
    let fine = !id.is_empty()
        && id.len() <= 128
        && id != "."
        && id != ".."
        && id.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'));
    if fine { Ok(()) } else { Err(Error::Unsafe(id.chars().take(80).collect())) }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
