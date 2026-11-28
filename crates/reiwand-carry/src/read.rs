use crate::model::{Content, Manifest};
use crate::safe::{check_id, check_name, hex};
use crate::seal::{is_sealed, open as unseal};
use crate::{Error, FORMAT, Result};
use reiwand_core::explicit;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Cursor, Read, Seek, Write};
use std::path::Path;
use zip::ZipArchive;

pub trait Source: Read + Seek {}
impl<T: Read + Seek> Source for T {}

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_entries: usize,
    pub max_total: u64,
    pub max_entry: u64,
    pub max_ratio: u64,
    pub max_json: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            max_entries: 100_000,
            max_total: 16 << 30,
            max_entry: 8 << 30,
            max_ratio: 200,
            max_json: 32 << 20,
        }
    }
}

pub struct Opened {
    pub manifest: Manifest,
    pub content: Content,
    archive: ZipArchive<Box<dyn Source>>,
    limits: Limits,
}

fn read_capped(archive: &mut ZipArchive<Box<dyn Source>>, name: &str, cap: u64) -> Result<Vec<u8>> {
    let entry = archive.by_name(name).map_err(|_| Error::Invalid(format!("{name} is missing")))?;
    let mut out = Vec::new();
    entry.take(cap + 1).read_to_end(&mut out)?;
    if out.len() as u64 > cap {
        return Err(Error::TooLarge(name.to_string()));
    }
    Ok(out)
}

fn vet(archive: &mut ZipArchive<Box<dyn Source>>, limits: &Limits) -> Result<()> {
    if archive.len() > limits.max_entries {
        return Err(Error::TooLarge("too many entries".into()));
    }
    let mut total = 0u64;
    for index in 0..archive.len() {
        let entry = archive.by_index_raw(index)?;
        let name = entry.name().map_err(|_| Error::Unsafe("a name that is not text".into()))?.to_string();
        check_name(&name)?;
        let mode = entry.unix_mode().unwrap_or(0o100644);
        if mode & 0o170000 == 0o120000 || entry.is_symlink() {
            return Err(Error::Unsafe(format!("{name} is a symbolic link")));
        }
        let (size, packed) = (entry.size(), entry.compressed_size());
        let bomb = size > limits.max_entry || (size > 1 << 20 && size / packed.max(1) > limits.max_ratio);
        total = total.saturating_add(size);
        if bomb || total > limits.max_total {
            return Err(Error::TooLarge(name));
        }
    }
    Ok(())
}

pub fn open_bytes(data: Vec<u8>, password: Option<&str>, limits: Limits) -> Result<Opened> {
    let sealed = is_sealed(&data);
    let data = if sealed {
        let password = password.ok_or(Error::NeedsPassword)?;
        unseal(&data, password)?.to_vec()
    } else {
        data
    };
    from_source(Box::new(Cursor::new(data)), sealed, limits)
}

pub fn open(path: &Path, password: Option<&str>, limits: Limits) -> Result<Opened> {
    let mut file = File::open(path)?;
    let mut magic = [0u8; 8];
    let sealed = file.read_exact(&mut magic).is_ok() && is_sealed(&magic);
    file.rewind()?;
    if sealed {
        let mut data = Vec::new();
        file.take(limits.max_total).read_to_end(&mut data)?;
        return open_bytes(data, password, limits);
    }
    from_source(Box::new(std::io::BufReader::new(file)), false, limits)
}

fn from_source(source: Box<dyn Source>, sealed: bool, limits: Limits) -> Result<Opened> {
    let mut archive = ZipArchive::new(source).map_err(|e| match e {
        zip::result::ZipError::InvalidArchive(_) | zip::result::ZipError::UnsupportedArchive(_) => {
            Error::Invalid("not a zip archive, or cut short".into())
        }
        other => other.into(),
    })?;
    vet(&mut archive, &limits)?;
    let head = read_capped(&mut archive, "carry.json", limits.max_json)?;
    let head: serde_json::Value = serde_json::from_slice(&head)?;
    let found =
        head["format"].as_u64().ok_or_else(|| Error::Invalid("carry.json has no format".into()))? as u32;
    if found > FORMAT {
        return Err(Error::Future { found, known: FORMAT });
    }
    if found < FORMAT {
        return Err(Error::Old { found, known: FORMAT });
    }
    let manifest: Manifest = explicit::read("carry.json", &head)?;
    let json = read_capped(&mut archive, "files/content.json", limits.max_json)?;
    let want = manifest
        .sums
        .get("files/content.json")
        .ok_or_else(|| Error::Invalid("no sum for the content".into()))?;
    if *want != hex(&Sha256::digest(&json)) {
        return Err(Error::Sum("files/content.json".into()));
    }
    let content: Content = explicit::read_str("files/content.json", std::str::from_utf8(&json)?)?;
    if content.sections() != manifest.sections {
        return Err(Error::Invalid("the sections in the manifest do not match the content".into()));
    }
    if content.secrets.is_some() && !sealed {
        return Err(Error::Invalid("secrets are only carried in an encrypted file".into()));
    }
    for own in &content.own {
        check_id(&own.id)?;
        if !manifest.sums.contains_key(&format!("files/own/{}", own.id)) {
            return Err(Error::Invalid(format!("no sum for own wallpaper {}", own.id)));
        }
    }
    Ok(Opened { manifest, content, archive, limits })
}

impl Opened {
    pub fn extract_own(&mut self, id: &str, out: &mut impl Write) -> Result<()> {
        check_id(id)?;
        let name = format!("files/own/{id}");
        let want = self
            .manifest
            .sums
            .get(&name)
            .cloned()
            .ok_or_else(|| Error::Invalid(format!("{id} is not in the file")))?;
        let entry = self.archive.by_name(&name).map_err(|_| Error::Invalid(format!("{name} is missing")))?;
        let declared = entry.size();
        let mut entry = entry.take(declared.min(self.limits.max_entry) + 1);
        let (mut sum, mut buf, mut done) = (Sha256::new(), vec![0u8; 1 << 16], 0u64);
        loop {
            let n = entry.read(&mut buf).map_err(|e| match e.kind() {
                std::io::ErrorKind::InvalidData => Error::Sum(name.clone()),
                _ => Error::Io(e),
            })?;
            if n == 0 {
                break;
            }
            done += n as u64;
            sum.update(&buf[..n]);
            out.write_all(&buf[..n])?;
        }
        if done != declared || hex(&sum.finalize()) != want {
            return Err(Error::Sum(name));
        }
        Ok(())
    }
}
