use crate::build::Hasher;
use crate::{Error, MANIFEST, Package, Result, SIGNATURE, ensure, sha256, sha256_file};
use base64::prelude::{BASE64_STANDARD, Engine};
use minisign_verify::{PublicKey, Signature};
use std::collections::BTreeSet;
use std::io::{Read, Seek};
use std::path::Path;

const MAX_BLOCKS: usize = 16;

pub fn public_key(text: &str) -> Result<String> {
    let line = text.lines().map(str::trim).rfind(|l| !l.is_empty()).unwrap_or_default();
    PublicKey::from_base64(line).map_err(|e| Error::Key(e.to_string()))?;
    Ok(line.into())
}

pub fn fingerprint(key: &str) -> Result<String> {
    let raw = BASE64_STANDARD.decode(public_key(key)?).map_err(|e| Error::Key(e.to_string()))?;
    sha256(&raw[10..])
}

pub fn manifest_comment(id: &str, version: &str, files: &str) -> String {
    format!("wpak manifest {id} {version} files:{files}")
}

fn digest(mut files: Vec<(String, String)>) -> Result<String> {
    files.sort();
    let listed = files.iter().map(|(path, sha)| format!("{sha} {path}\n")).collect::<String>();
    sha256(listed.as_bytes())
}

pub fn source_digest(dir: &Path) -> Result<String> {
    let mut files = Vec::new();
    crate::build::collect(dir, "", &mut files)?;
    let mut hashed = Vec::with_capacity(files.len());
    for file in files.into_iter().filter(|f| f.path != SIGNATURE) {
        hashed.push((file.path.clone(), sha256_file(dir.join(&file.path))?));
    }
    digest(hashed)
}

impl<R: Read + Seek> Package<R> {
    pub fn files_digest(&mut self) -> Result<String> {
        let mut files = Vec::with_capacity(self.entries.len());
        for index in 0..self.entries.len() {
            if self.entries[index].path == SIGNATURE {
                continue;
            }
            let mut hasher = Hasher::default();
            self.copy(index, &mut hasher)?;
            files.push((self.entries[index].path.clone(), hasher.hex()));
        }
        digest(files)
    }
}

const NOTE: &str = "wpak manifest sha256:";

pub fn manifest_note(manifest: &[u8]) -> String {
    format!("{NOTE}{}", sha256(manifest).unwrap_or_default())
}

pub fn noted_hash(sigs: &str) -> Option<String> {
    let line = sigs.lines().next()?;
    let hash = line.split_once(NOTE)?.1.split_whitespace().next()?;
    Some(hash.to_owned())
}

pub fn verify_blocks(
    data: &[u8],
    sigs: &str,
    comment: &str,
    keys: &[(&str, &str)],
) -> Result<BTreeSet<String>> {
    let lines: Vec<&str> = sigs.lines().collect();
    let blocks = lines.len() / 4;
    let shaped = lines.len().is_multiple_of(4) && (1..=MAX_BLOCKS).contains(&blocks);
    ensure(shaped, || Error::Signature("expected 1 to 16 minisign blocks".into()))?;
    let mut verified = BTreeSet::new();
    for block in lines.chunks(4) {
        let sig = Signature::decode(&block.join("\n")).map_err(|e| Error::Signature(e.to_string()))?;
        let expected = sig.trusted_comment() == comment;
        ensure(expected, || Error::Signature(format!("trusted comment is not {comment:?}")))?;
        let valid = keys.iter().filter(|(_, key)| {
            PublicKey::from_base64(key).is_ok_and(|key| key.verify(data, &sig, false).is_ok())
        });
        verified.extend(valid.map(|(fp, _)| fp.to_string()));
    }
    Ok(verified)
}

pub fn threshold(
    data: &[u8],
    sigs: &str,
    comment: &str,
    keys: &[(&str, &str)],
    threshold: usize,
) -> Result<BTreeSet<String>> {
    let verified = verify_blocks(data, sigs, comment, keys)?;
    let count = verified.len();
    ensure(count >= threshold, || Error::Signature(format!("{count} of {threshold} required keys signed")))?;
    Ok(verified)
}

pub fn verify_package<R: Read + Seek>(package: &mut Package<R>, keys: &[(&str, &str)]) -> Result<String> {
    let sig = String::from_utf8(package.read_file(SIGNATURE)?)
        .map_err(|_| Error::Signature(format!("{SIGNATURE} is not UTF-8")))?;
    let files = package.files_digest()?;
    let comment = manifest_comment(&package.manifest.id, &package.manifest.version, &files);
    let verified = threshold(&package.read_file(MANIFEST)?, &sig, &comment, keys, 1)?;
    Ok(verified.into_iter().next().unwrap_or_default())
}
