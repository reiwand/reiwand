use crate::limits::{FILE_BYTES, FILES, TOTAL_BYTES};
use crate::manifest::{FILE, Manifest};
use crate::{Error, strip};
use reiwand_core::explicit::{Fault, ReadError, read_str};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const MAIN: &str = "main.ts";

#[derive(Debug, Clone)]
pub struct Bundle {
    pub manifest: Manifest,
    pub files: BTreeMap<String, Vec<u8>>,
}

fn format(why: impl Into<String>) -> Error {
    Error::Format(why.into())
}

fn allowed(name: &str) -> bool {
    let parts: Vec<&str> = name.split('/').collect();
    let clean = parts.iter().all(|p| !p.is_empty() && !p.starts_with('.') && p.len() <= 64);
    let plain = name.bytes().all(|b| b.is_ascii_alphanumeric() || b"-_./".contains(&b));
    let top = parts.len() == 1 && (name == FILE || name.starts_with("LICENSE") || name == "README.md");
    clean && plain && parts.len() <= 4 && (name.ends_with(".ts") || top)
}

pub fn manifest(text: &str) -> Result<Manifest, Error> {
    let manifest: Manifest = read_str(FILE, text).map_err(|e| format(e.said().to_string()))?;
    manifest.check().map_err(|(path, why)| {
        format(ReadError { file: FILE.into(), path, fault: Fault::Invalid(why) }.said().to_string())
    })?;
    Ok(manifest)
}

impl Bundle {
    /// What the user agreed to: a hash of every file, so an install refuses a plugin whose files
    /// changed after its consent was shown.
    pub fn token(&self) -> String {
        let mut hash = blake3::Hasher::new();
        for (name, bytes) in &self.files {
            for part in [name.as_bytes(), bytes] {
                hash.update(&(part.len() as u64).to_le_bytes());
                hash.update(part);
            }
        }
        hash.finalize().to_hex().to_string()
    }

    pub fn read(path: &Path) -> Result<Bundle, Error> {
        let mut files = BTreeMap::new();
        match path.is_dir() {
            true => walk(path, "", &mut files)?,
            false => unzip(path, &mut files)?,
        }
        let top = files.keys().next().and_then(|n| n.split_once('/')).map(|(top, _)| format!("{top}/"));
        if let Some(top) = top.filter(|t| !files.contains_key(FILE) && files.keys().all(|n| n.starts_with(t)))
        {
            files = files.into_iter().map(|(n, b)| (n[top.len()..].to_owned(), b)).collect();
        }
        Bundle::of(files)
    }

    pub fn of(files: BTreeMap<String, Vec<u8>>) -> Result<Bundle, Error> {
        if let Some(name) = files.keys().find(|n| !allowed(n)) {
            return Err(format(format!(
                "{name}: a plugin holds only {FILE}, .ts modules, LICENSE and README.md, in at most 4 folder levels"
            )));
        }
        let text = |name: &str| match files.get(name) {
            Some(bytes) => {
                std::str::from_utf8(bytes).map_err(|_| format(format!("{name} is not UTF-8 text")))
            }
            None => Err(format(format!("the plugin has no {name}"))),
        };
        let manifest = manifest(text(FILE)?)?;
        text(MAIN)?;
        files.keys().filter(|n| n.ends_with(".ts")).try_for_each(|n| text(n).map(drop))?;
        Ok(Bundle { manifest, files })
    }

    pub fn sources(&self) -> impl Iterator<Item = (&str, &str)> {
        let typed = self.files.iter().filter(|(n, _)| n.ends_with(".ts"));
        typed.map(|(n, b)| (n.as_str(), std::str::from_utf8(b).unwrap_or_default()))
    }

    pub fn strip(&self) -> Result<BTreeMap<String, String>, Error> {
        self.sources().map(|(name, source)| Ok((name.to_owned(), strip::strip(name, source)?))).collect()
    }
}

fn add(files: &mut BTreeMap<String, Vec<u8>>, name: String, bytes: Vec<u8>) -> Result<(), Error> {
    let total: u64 = files.values().map(|b| b.len() as u64).sum::<u64>() + bytes.len() as u64;
    if files.len() >= FILES || bytes.len() as u64 > FILE_BYTES || total > TOTAL_BYTES {
        return Err(format(format!(
            "a plugin holds at most {FILES} files of {} KiB each and {} KiB in all",
            FILE_BYTES >> 10,
            TOTAL_BYTES >> 10
        )));
    }
    files.insert(name, bytes);
    Ok(())
}

fn capped(reader: impl Read) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    reader.take(FILE_BYTES + 1).read_to_end(&mut bytes).map_err(|e| Error::Io(e.to_string()))?;
    Ok(bytes)
}

fn walk(dir: &Path, prefix: &str, files: &mut BTreeMap<String, Vec<u8>>) -> Result<(), Error> {
    let io = |e: std::io::Error| Error::Io(format!("{}: {e}", dir.display()));
    let mut entries = std::fs::read_dir(dir).map_err(io)?.collect::<Result<Vec<_>, _>>().map_err(io)?;
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let name = format!("{prefix}{}", entry.file_name().to_string_lossy());
        let kind = entry.file_type().map_err(io)?;
        if kind.is_dir() && prefix.matches('/').count() < 4 {
            walk(&entry.path(), &format!("{name}/"), files)?;
        } else if kind.is_file() {
            add(files, name, capped(File::open(entry.path()).map_err(io)?)?)?;
        } else {
            return Err(format(format!("{name}: a plugin holds only plain files and folders")));
        }
    }
    Ok(())
}

fn unzip(path: &Path, files: &mut BTreeMap<String, Vec<u8>>) -> Result<(), Error> {
    let io = |e: std::io::Error| Error::Io(format!("{}: {e}", path.display()));
    let zip = |e: zip::result::ZipError| format(format!("{}: not a plugin zip: {e}", path.display()));
    let mut archive = zip::ZipArchive::new(File::open(path).map_err(io)?).map_err(zip)?;
    if archive.len() > FILES * 2 {
        return Err(format(format!("a plugin holds at most {FILES} files")));
    }
    for i in 0..archive.len() {
        let entry = archive.by_index(i).map_err(zip)?;
        if entry.is_dir() {
            continue;
        }
        let name = entry.name().map_err(zip)?.into_owned();
        if entry.is_symlink() || !entry.is_file() {
            return Err(format(format!("{name}: a plugin holds only plain files and folders")));
        }
        let bytes = capped(entry)?;
        add(files, name, bytes)?;
    }
    Ok(())
}
