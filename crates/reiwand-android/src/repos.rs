use crate::library::now;
use crate::{Error, library, store};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::PathBuf;

mod adding;
pub(crate) mod bundled;
mod install;
pub(crate) mod preview;
pub(crate) mod server;

pub(crate) use install::{fetch, fetched, install};

const DIR: &str = "repos";
pub(crate) const ITEMS: &str = "items.json";

pub(crate) fn dir() -> Result<PathBuf, Error> {
    Ok(store::dir()?.join(DIR))
}

fn shared(url: &str) -> bool {
    let loopback = |u: &reqwest::Url| {
        let host = u.host_str().unwrap_or_default().trim_matches(['[', ']']);
        host == "localhost" || host.parse::<std::net::IpAddr>().is_ok_and(|ip| ip.is_loopback())
    };
    reqwest::Url::parse(url).is_ok_and(|u| u.scheme() == "https" || u.scheme() == "http" && loopback(&u))
}

pub(crate) fn installed() -> BTreeMap<String, (String, String)> {
    let file = dir().ok().and_then(|d| std::fs::read(d.join(ITEMS)).ok());
    file.and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default()
}

pub(crate) fn added() -> std::collections::HashSet<String> {
    let known = dir().ok().and_then(|d| reiwand_format::repos(&d, now()).ok()).unwrap_or_default();
    known.into_iter().map(|r| r.repo_id).collect()
}

pub(crate) fn add(url: &str, trust: Option<&str>) -> Result<Value, Error> {
    adding::add(url.trim(), trust)
}

pub(crate) fn list() -> Result<Value, Error> {
    let repos = reiwand_format::repos(&dir()?, now()).unwrap_or_default();
    let repos: Vec<Value> = repos
        .iter()
        .map(|r| {
            json!({"repo_id": r.repo_id, "name": r.name, "url": r.url,
        "packages": r.packages, "stale": r.stale, "fingerprints": r.fingerprints, "broken": r.broken,
        "warnings": r.warnings,
        "bundled": bundled::is_bundled(&r.repo_id)})
        })
        .collect();
    Ok(json!({"repos": repos}))
}

pub(crate) fn remove(repo_id: &str) -> Result<Value, Error> {
    preview::forget(repo_id);
    reiwand_format::remove_repo(&dir()?, repo_id)?;
    bundled::declined(repo_id);
    store::changed();
    Ok(Value::Null)
}

pub(crate) fn browse(query: &str) -> Result<Value, Error> {
    Ok(listed(reiwand_format::browse(&dir()?, query)?, library::hidden(), &library::languages()))
}

pub(crate) fn listed(found: Vec<reiwand_format::Item>, hide: bool, languages: &[String]) -> Value {
    let mature =
        |i: &reiwand_format::Item| i.release.content.as_ref().is_some_and(reiwand_format::Rated::mature);
    let items = found.iter().filter(|i| !hide || !mature(i)).take(200).map(|i| {
        json!({"repo_id": i.repo_id, "repo": i.repo_name, "id": i.id,
        "version": i.release.version, "title": i.release.title.pick(languages), "kind": i.release.kind,
        "size": i.release.size, "installed": !i.installed.is_empty(), "mature": mature(i), "featured": i.featured})
    });
    json!(items.collect::<Vec<_>>())
}
