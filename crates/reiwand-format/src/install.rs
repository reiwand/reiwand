pub use crate::record::Installed;
use crate::registry::{keep_anchors, records, save};
use crate::store::{Snapshot, load, locked};
use crate::{Error, Fetch, Listing, OnlineAuthor, Release, Result, Trusted, anchors, ensure, require};
use crate::{
    authors::trust,
    broken::{scan, sound},
    browse::{available, listing, newer},
    download::package,
};
use reiwand_core::message::said;
use semver::Version;
use std::fs;

mod uninstall;
use std::path::Path;
pub use uninstall::{remove, remove_from};

pub fn install<F: Fetch>(
    data: &Path,
    fetch: &F,
    spec: &str,
    now: i64,
    allow_stale: bool,
    allow_downgrade: bool,
) -> Result<Installed> {
    let (repo, rest) = spec.rsplit_once('/').map_or((None, spec), |(r, s)| (Some(r), s));
    let (id, version) = rest.split_once('@').map_or((rest, None), |(i, v)| (i, Some(v)));
    let offered = |s: &Snapshot| repo.is_none_or(|r| r == s.state.repo_id) && pick(s, id, version).is_some();
    let (snaps, list) = locked(data, false, || Ok((sound(scan(data)?), records(data)?)))?;
    let mut found: Vec<Snapshot> = snaps.into_iter().filter(offered).collect();
    require(found.len() <= 1, "id", "is offered by several repositories; prefix it with the repository id")?;
    let snap = found.pop().ok_or_else(|| Error::NotFound(spec.into()))?;
    let (listing, release) = pick(&snap, id, version).ok_or_else(|| Error::NotFound(spec.into()))?;
    let (root, repo_id, t) = (&snap.root.value, &snap.state.repo_id, now.max(snap.state.max_timestamp));
    let foreign = |list: &[Installed]| list.iter().any(|i| i.id == id && &i.repo_id != repo_id);
    require(!foreign(&list), "id", "is installed from another repository")?;
    ensure(root.expires.timestamp() > t, || Error::Expired(format!("root {}", root.version)))?;
    ensure(allow_stale || snap.entry.value.expires.timestamp() > t, || Error::Stale(repo_id.clone()))?;
    if let Some(reason) = snap.revoked.version(id, &release.version) {
        return Err(Error::Revoked(format!("{id} {} ({reason:?})", release.version)));
    }
    let newest = list.iter().filter(|i| i.id == id).max_by_key(|i| Version::parse(&i.version).ok());
    let explicit = version.is_some();
    let older = newest.filter(|n| explicit && !allow_downgrade && newer(&n.version, &release.version));
    if let Some((new, old)) = older.map(|n| (&n.version, &release.version)) {
        let reason = said(
            "format.downgrade",
            "%1 %2 is older than the installed %3; confirm the downgrade",
            &[&id, old, new],
        );
        return Err(crate::refuse("version", reason));
    }
    let newest = newest.filter(|i| !explicit && !newer(&release.version, &i.version));
    let same = list.iter().find(|i| i.id == id && i.version == release.version);
    if let Some(done) = same.map_or(newest, |s| whole(s).then_some(s)) {
        return Ok(done.clone());
    }
    let path = data.join("packages").join(id).join(format!("{}.wpak", release.version));
    let (tmp, signer, online) = package(fetch, &snap, listing, release, &path)?;
    let (version, sha256, size) = (release.version.clone(), release.sha256.clone(), release.size);
    let (repo_id, id) = (repo_id.clone(), id.to_string());
    let author = signer.as_deref().and_then(|fp| signed_by(&snap, online.as_ref(), fp));
    let done = Installed { repo_id, id, version, sha256, size, path, signer, revoked: None, author };
    locked(data, true, || {
        let mut list = records(data)?;
        let at = list.iter().position(|i| i.id == done.id && i.version == done.version);
        if let Some(same) = at.map(|i| &list[i]).filter(|s| whole(s)) {
            return Ok(same.clone());
        }
        require(!foreign(&list), "id", "is installed from another repository")?;
        still(data, &done)?;
        let anchors = trusted(data, &done, &list, online.as_ref(), &snap, listing.author.as_deref())?;
        tmp.keep(&done.path)?;
        match at {
            Some(i) => list[i] = done.clone(),
            None => list.push(done.clone()),
        }
        save(data, &list)?;
        anchors.map_or(Ok(()), |a| keep_anchors(data, &a))?;
        Ok(done)
    })
}

pub fn current(data: &Path, repo_id: &str, id: &str, version: &str) -> Result<Installed> {
    locked(data, false, || {
        let found =
            records(data)?.into_iter().find(|i| i.repo_id == repo_id && i.id == id && i.version == version);
        let found = found.ok_or_else(|| Error::NotFound(format!("{id} {version} was uninstalled")))?;
        if let Some(reason) = found.revoked {
            return Err(Error::Revoked(format!("{id} {version} ({reason:?})")));
        }
        still(data, &found).map(|()| found)
    })
}

fn trusted(
    data: &Path,
    done: &Installed,
    list: &[Installed],
    online: Option<&OnlineAuthor>,
    snap: &Snapshot,
    named: Option<&str>,
) -> Result<Option<Vec<Trusted>>> {
    let (root, revoked) = (&snap.root.value, &snap.revoked.keys);
    let mut anchors = anchors(data)?;
    let same = |r: &str, i: &str| r == done.repo_id && i == done.id;
    let at = anchors.iter().position(|a| same(&a.repo_id, &a.id));
    let signed: Vec<&str> =
        list.iter().filter(|i| same(&i.repo_id, &i.id)).filter_map(|i| i.signer.as_deref()).collect();
    let refuse = |why: &str| Err(Error::Signature(format!("{} {why}", done.id)));
    let owner =
        |fp: &str| root.authors.iter().find(|a| root.author_keys(a, &[]).iter().any(|(f, _)| *f == fp));
    let other = list
        .iter()
        .filter(|i| same(&i.repo_id, &i.id))
        .any(|i| i.author.is_some() && i.author != done.author);
    match (done.signer.as_deref(), online) {
        (None, _) if at.is_some() || !signed.is_empty() => {
            refuse("is unsigned, but its installed versions were signed")
        }
        (None, _) => Ok(None),
        (Some(_), Some(_)) if at.is_none() && !signed.is_empty() => {
            refuse("was installed from an author of the root and cannot pass to an online author")
        }
        (Some(fp), Some(author)) => {
            let key = author.keys.get(fp).map(|k| k.minisign.clone()).unwrap_or_default();
            let new =
                Trusted { repo_id: done.repo_id.clone(), id: done.id.clone(), fingerprint: fp.into(), key };
            trust(&mut anchors, new, author, revoked)?;
            Ok(Some(anchors))
        }
        (Some(_), None)
            if signed.iter().any(|s| owner(s).is_some_and(|o| Some(o.id.as_str()) != named)) || other =>
        {
            refuse("is signed by another author of the root than its installed versions")
        }
        (Some(_), None) => Ok(at.map(|i| {
            anchors.remove(i);
            anchors
        })),
    }
}

fn signed_by(snap: &Snapshot, online: Option<&OnlineAuthor>, fp: &str) -> Option<String> {
    let root = &snap.root.value;
    let owner = root.authors.iter().find(|a| root.author_keys(a, &[]).iter().any(|(f, _)| *f == fp));
    owner.map(|a| a.id.clone()).or_else(|| online.map(|a| a.id.clone()))
}

fn still(data: &Path, item: &Installed) -> Result<()> {
    let removed = |_| Error::NotFound(format!("repository {} was removed or broken", item.repo_id));
    let snap = load(data, &item.repo_id).map_err(removed)?;
    let key = item.signer.as_deref().and_then(|fp| snap.revoked.key(fp));
    match snap.revoked.version(&item.id, &item.version).or(key) {
        Some(reason) => Err(Error::Revoked(format!("{} {} ({reason:?})", item.id, item.version))),
        None => Ok(()),
    }
}

fn whole(item: &Installed) -> bool {
    let sized = fs::metadata(&item.path).is_ok_and(|m| m.is_file() && m.len() == item.size);
    sized && crate::sha256_file(&item.path).is_ok_and(|h| h == item.sha256)
}

pub fn installed(data: &Path) -> Result<Vec<Installed>> {
    locked(data, false, || records(data))
}

fn pick<'a>(snap: &'a Snapshot, id: &str, version: Option<&str>) -> Option<(&'a Listing, &'a Release)> {
    let listing = listing(snap, id)?;
    let release = match version {
        Some(version) => listing.versions.iter().find(|r| r.version == version),
        None => available(snap, listing).pop(),
    };
    Some((listing, release?))
}
