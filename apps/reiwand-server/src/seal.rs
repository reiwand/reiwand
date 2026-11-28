use anyhow::{Context, bail};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use std::fs::{self, DirBuilder, OpenOptions};
use std::io::{ErrorKind, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

const NONCE: usize = 24;

fn seal_key_path(data: &Path) -> PathBuf {
    data.join("keys/seal.key")
}

pub fn root_path(data: &Path, name: &str) -> PathBuf {
    data.join(format!("keys/{name}.root.sealed"))
}

fn write_private(path: &Path, bytes: &[u8], fresh: bool) -> anyhow::Result<()> {
    DirBuilder::new().recursive(true).mode(0o700).create(path.parent().context("bad key path")?)?;
    let mut options = OpenOptions::new();
    options.write(true).mode(0o600);
    if fresh {
        options.create_new(true);
    } else {
        options.create(true).truncate(true);
    }
    Ok(options.open(path)?.write_all(bytes)?)
}

fn cipher(data: &Path) -> anyhow::Result<XChaCha20Poly1305> {
    let path = seal_key_path(data);
    loop {
        match fs::read(&path) {
            Ok(bytes) if bytes.len() == 32 => {
                return XChaCha20Poly1305::new_from_slice(&bytes).map_err(|e| anyhow::anyhow!("{e}"));
            }
            Ok(_) => bail!("{} is damaged", path.display()),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                let mut fresh = [0u8; 32];
                getrandom::fill(&mut fresh)?;
                match write_private(&path, &fresh, true) {
                    Err(e)
                        if e.downcast_ref::<std::io::Error>()
                            .is_some_and(|i| i.kind() == ErrorKind::AlreadyExists) => {}
                    other => other?,
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
}

pub fn keep_root(data: &Path, name: &str, secret: &str) -> anyhow::Result<()> {
    let mut nonce = [0u8; NONCE];
    getrandom::fill(&mut nonce)?;
    let sealed = cipher(data)?
        .encrypt(&XNonce::try_from(&nonce[..]).map_err(|_| anyhow::anyhow!("bad nonce"))?, secret.as_bytes())
        .map_err(|_| anyhow::anyhow!("could not seal the root key"))?;
    write_private(&root_path(data, name), &[nonce.as_slice(), &sealed].concat(), false)
}

pub fn has_root(data: &Path, name: &str) -> bool {
    root_path(data, name).is_file()
}

pub fn open_root(data: &Path, name: &str) -> anyhow::Result<Option<String>> {
    let bytes = match fs::read(root_path(data, name)) {
        Ok(bytes) => bytes,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    anyhow::ensure!(bytes.len() > NONCE, "the sealed root key is damaged");
    let (nonce, sealed) = bytes.split_at(NONCE);
    let plain = cipher(data)?
        .decrypt(&XNonce::try_from(nonce).map_err(|_| anyhow::anyhow!("bad nonce"))?, sealed)
        .map_err(|_| anyhow::anyhow!("the sealed root key does not open with this server's seal key"))?;
    Ok(Some(String::from_utf8(plain)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_kept_root_opens_again_and_is_not_stored_in_clear() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(open_root(dir.path(), "main").unwrap(), None);
        keep_root(dir.path(), "main", "untrusted comment: secret").unwrap();
        assert!(has_root(dir.path(), "main"));
        let raw = fs::read(root_path(dir.path(), "main")).unwrap();
        assert!(!String::from_utf8_lossy(&raw).contains("secret"));
        assert_eq!(open_root(dir.path(), "main").unwrap().as_deref(), Some("untrusted comment: secret"));
        fs::write(seal_key_path(dir.path()), [1u8; 32]).unwrap();
        assert!(open_root(dir.path(), "main").is_err());
    }
}
