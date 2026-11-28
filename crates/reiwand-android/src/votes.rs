use crate::account::using::{needed, token_for};
use crate::library::now;
use crate::{Error, said, settings};
use reiwand_account::{Api, origin};
use reiwand_format::{RepoInfo, server_repo_name, vote_queries, voted};
use serde_json::{Map, Value, json};

fn runtime() -> Result<tokio::runtime::Runtime, Error> {
    Ok(tokio::runtime::Builder::new_current_thread().enable_all().build()?)
}

fn voting() -> Vec<(RepoInfo, String, String)> {
    let known =
        crate::repos::dir().ok().and_then(|d| reiwand_format::repos(&d, now()).ok()).unwrap_or_default();
    let named = known.into_iter().filter_map(|r| {
        let name = server_repo_name(&r.url)?.to_owned();
        let site = origin(&r.url)?;
        Some((r, name, site))
    });
    named.collect()
}

fn api(site: &str, url: &str) -> Result<Api, Error> {
    let pin = crate::account::pins::all().get(site).cloned();
    let api = crate::net::api(site, pin.as_deref())?;
    Ok(match token_for(url) {
        Some(token) => api.with_token(&token),
        None => api,
    })
}

pub(crate) fn counts(ids: &[String]) -> Result<Value, Error> {
    if voted(ids, None).is_none() {
        return Err(said("android.vote_ids", "give at most 1000 package ids", &[]));
    }
    let none = json!({"items": {}, "shown": false});
    if ids.is_empty() || !settings::config().repos.vote_counts || crate::net::private() {
        return Ok(none);
    }
    let mut items = Map::new();
    let rt = runtime()?;
    for (repo, name, site) in voting() {
        let prefix = format!("{}/", repo.repo_id);
        let asked: Vec<_> = voted(ids, Some(&repo.repo_id)).unwrap_or_default();
        let asked: Vec<_> = asked.into_iter().filter(|(spec, _)| spec.starts_with(&prefix)).collect();
        if asked.is_empty() {
            continue;
        }
        let api = api(&site, &repo.url)?;
        for query in vote_queries(&name, &asked) {
            let Ok(found) = rt.block_on(api.get(&query)) else { continue };
            for (spec, id) in &asked {
                if let Some(count) = found["items"].get(*id) {
                    items.insert((*spec).to_owned(), count.clone());
                }
            }
        }
    }
    Ok(json!({"items": items, "shown": true}))
}

pub(crate) fn cast(spec: &str, vote: &str) -> Result<Value, Error> {
    let specs = [spec.to_owned()];
    if voted(&specs, None).is_none() || !matches!(vote, "up" | "down" | "none") {
        return Err(said("android.vote_value", "give a package id and up, down or none", &[]));
    }
    let found = voting().into_iter().find(|(repo, ..)| spec.starts_with(&format!("{}/", repo.repo_id)));
    let Some((repo, name, site)) = found else {
        return Err(said(
            "android.vote_elsewhere",
            "this package is not in a repository that takes votes",
            &[],
        ));
    };
    if token_for(&repo.url).is_none() {
        return Err(needed(&site));
    }
    let id = spec.split_once('/').map_or(spec, |(_, id)| id);
    let api = api(&site, &repo.url)?;
    Ok(runtime()?
        .block_on(api.post(&format!("/api/v1/r/{name}/packages/{id}/vote"), json!({ "vote": vote })))?)
}
