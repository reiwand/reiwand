use crate::registry::{keep_anchors, records, save};
use crate::store::locked;
use crate::{Error, Installed, Result, Trusted, anchors, ensure};
use std::path::Path;
use std::{fs, io};

pub fn remove(data: &Path, id: &str, version: Option<&str>) -> Result<Vec<Installed>> {
    withdraw(data, id, |i| version.is_none_or(|v| v == i.version))
}

pub fn remove_from(data: &Path, repo_id: &str, id: &str) -> Result<Vec<Installed>> {
    withdraw(data, id, |i| i.repo_id == repo_id)
}

fn withdraw(data: &Path, id: &str, wanted: impl Fn(&Installed) -> bool) -> Result<Vec<Installed>> {
    locked(data, true, || {
        let (gone, kept): (Vec<_>, Vec<_>) =
            records(data)?.into_iter().partition(|i| i.id == id && wanted(i));
        ensure(!gone.is_empty(), || Error::NotFound(id.into()))?;
        for item in &gone {
            match fs::remove_file(&item.path) {
                Err(e) if e.kind() != io::ErrorKind::NotFound => return Err(e.into()),
                _ => {}
            }
        }
        gone.iter().filter_map(|i| i.path.parent()).for_each(|dir| drop(fs::remove_dir(dir)));
        save(data, &kept)?;
        let left = |a: &Trusted| kept.iter().any(|i| i.repo_id == a.repo_id && i.id == a.id);
        let (held, dropped): (Vec<_>, Vec<_>) = anchors(data)?.into_iter().partition(left);
        if !dropped.is_empty() {
            keep_anchors(data, &held)?;
        }
        Ok(gone)
    })
}
