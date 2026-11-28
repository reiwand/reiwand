use crate::state::{engine, live};
use reiwand_web::Start;
use rustix::fs::{MemfdFlags, memfd_create};
use std::cell::RefCell;
use std::collections::HashSet;
use std::io::Write;
use std::os::fd::{AsRawFd, OwnedFd};
use std::path::PathBuf;

const KEPT: u64 = 64 << 20;

pub fn package(start: &Start) -> Result<(HashSet<String>, Option<String>), String> {
    let mut package = reiwand_format::open(&start.package, &reiwand_format::Limits::default())
        .map_err(|e| e.to_string())?;
    let kind = package.manifest.kind == reiwand_format::Kind::Web && package.manifest.id == start.id;
    if !kind || package.manifest.entry != start.entry {
        return Err("the package is not this web page".into());
    }
    let entries: HashSet<String> = package.entries.iter().map(|e| e.path.clone()).collect();
    let preload = match &start.preload {
        Some(path) => Some(
            String::from_utf8(package.read_file(path).map_err(|e| e.to_string())?)
                .map_err(|_| "the preload is not text")?,
        ),
        None => None,
    };
    Ok((entries, preload))
}

enum Found {
    Packed(PathBuf, String),
    User(PathBuf),
}

fn find(uri: &str) -> Option<Found> {
    let engine = engine()?;
    let uri = uri.split(['?', '#']).next()?;
    if let Some((scheme, rest)) = uri.split_once("://")
        && scheme.eq_ignore_ascii_case("wpak")
    {
        let (id, path) = rest.split_once('/')?;
        let path = reiwand_format::page::decoded(path)?;
        let path = if path.is_empty() { engine.start.entry.clone() } else { path };
        let fine = id.eq_ignore_ascii_case(&engine.start.id)
            && engine.entries.contains(&path)
            && !path.starts_with("manifest.json");
        return fine.then(|| Found::Packed(engine.start.package.clone(), path));
    }
    live(|l| l.file(uri)).map(Found::User)
}

thread_local! {
    static CACHE: RefCell<Vec<(String, OwnedFd, u64)>> = const { RefCell::new(Vec::new()) };
}

fn memory(reader: &mut impl std::io::Read) -> std::io::Result<(OwnedFd, u64)> {
    let fd = memfd_create("reiwand-web-entry", MemfdFlags::CLOEXEC)?;
    let mut file = std::fs::File::from(fd);
    let size = std::io::copy(reader, &mut file)?;
    file.flush()?;
    Ok((file.into(), size))
}

fn reopen(fd: &OwnedFd) -> std::io::Result<OwnedFd> {
    Ok(std::fs::File::open(format!("/proc/self/fd/{}", fd.as_raw_fd()))?.into())
}

fn packed(package: &PathBuf, path: &str) -> std::io::Result<(OwnedFd, u64)> {
    let key = format!("{}:{path}", package.display());
    let hit =
        CACHE.with(|c| c.borrow().iter().find(|(k, ..)| *k == key).map(|(_, fd, size)| (reopen(fd), *size)));
    if let Some((fd, size)) = hit {
        return Ok((fd?, size));
    }
    let mut archive = zip::ZipArchive::new(std::fs::File::open(package)?).map_err(std::io::Error::other)?;
    let mut entry = archive.by_name(path).map_err(std::io::Error::other)?;
    let (fd, size) = memory(&mut entry)?;
    let reader = reopen(&fd)?;
    let held = size <= KEPT;
    CACHE.with(|c| {
        let mut c = c.borrow_mut();
        if !held {
            return;
        }
        while c.iter().map(|(.., s)| s).sum::<u64>() + size > KEPT {
            c.remove(0);
        }
        c.push((key, fd, size));
    });
    Ok((reader, size))
}

fn opened(found: Found) -> std::io::Result<(OwnedFd, u64, String)> {
    match found {
        Found::User(path) => {
            use std::os::unix::fs::OpenOptionsExt;
            let nofollow = rustix::fs::OFlags::NOFOLLOW.bits() as i32;
            let file = std::fs::OpenOptions::new().read(true).custom_flags(nofollow).open(&path)?;
            let meta = file.metadata()?;
            if !meta.is_file() {
                return Err(std::io::ErrorKind::InvalidInput.into());
            }
            Ok((file.into(), meta.len(), reiwand_web::mime(&path.to_string_lossy()).into()))
        }
        Found::Packed(package, path) => {
            let (fd, size) = packed(&package, &path)?;
            Ok((fd, size, reiwand_web::mime(&path).to_owned()))
        }
    }
}

pub fn serve(uri: &str) -> Option<(OwnedFd, u64, String)> {
    find(uri).and_then(|found| opened(found).ok())
}
