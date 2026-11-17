use anyhow::{Context, bail};
use reiwand_sync::{Api, Ids, SEALED, SEGMENT, open_segment, sealed_size};
use serde_json::Value;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

const BATCH: u64 = 64;

pub async fn fetch(api: &Api, ids: &Ids, blob: &Value, dir: &Path) -> anyhow::Result<(String, PathBuf)> {
    let hash = blob["hash"].as_str().unwrap_or_default();
    let size = blob["size"].as_u64().unwrap_or_default();
    let id = ids.stored(hash.as_bytes());
    if blob["id"] != id.as_str() || hash.len() != 64 || !hash.bytes().all(|b| b.is_ascii_hexdigit()) {
        bail!("the record of a private file is broken");
    }
    let key = ids.file_key(hash.as_bytes());
    let (segments, total) = (size.div_ceil(SEGMENT).max(1), sealed_size(size));
    let path = dir.join(hash);
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).mode(0o600).open(&path)?;
    let mut index = 0;
    while index < segments {
        let offset = index * SEALED;
        let length = (BATCH * SEALED).min(total - offset);
        let url = format!("/api/v1/sync/blobs/{id}?offset={offset}&length={length}");
        let bytes = api.get_bytes(&url, length as usize).await?;
        if bytes.len() as u64 != length {
            bail!("the server sent a short piece of a private file");
        }
        for chunk in bytes.chunks(SEALED as usize) {
            file.write_all(&open_segment(&key, index, index + 1 == segments, chunk)?)?;
            index += 1;
        }
    }
    drop(file);
    if reiwand_format::sha256_file(&path)? != hash {
        fs::remove_file(&path)?;
        bail!("a downloaded private file does not match its hash");
    }
    let name = blob["name"].as_str().unwrap_or_default();
    let name: String = name.chars().filter(|c| !c.is_control() && *c != '/').collect();
    let name = Some(name.trim_start_matches('.').to_owned()).filter(|n| !n.is_empty());
    Ok((name.context("a private file has no name").unwrap_or_else(|_| hash.to_owned()), path))
}
