use super::book::{self, drop_token, edit};
use crate::library::now;
use crate::{Error, repos, said};
use reiwand_account::{Link, Rec, clean, origin};
use reiwand_providers::HttpFetch;
use serde_json::{Value, json};

fn link_of(repo: &str) -> Link {
    let known = reiwand_format::repos(&repos::dir().unwrap_or_default(), now()).unwrap_or_default();
    let found = known.iter().find(|r| clean(&r.url) == clean(repo) || r.repo_id == repo);
    Link {
        url: found.map_or_else(|| clean(repo), |r| clean(&r.url)),
        repo_id: found.map(|r| r.repo_id.clone()),
    }
}

fn same(link: &Link, wanted: &Link) -> bool {
    clean(&link.url) == clean(&wanted.url) || (wanted.repo_id.is_some() && link.repo_id == wanted.repo_id)
}

pub(crate) fn with_tokens(mut fetch: HttpFetch) -> Result<HttpFetch, Error> {
    let recs: Vec<Rec> =
        book::file().accounts.into_iter().filter(|r| !r.pending && !r.links.is_empty()).collect();
    if recs.is_empty() {
        return Ok(fetch);
    }
    let tokens = book::tokens();
    let known = reiwand_format::repos(&repos::dir()?, now()).unwrap_or_default();
    for rec in recs {
        let Some(token) = tokens.get(&rec.id) else { continue };
        for link in &rec.links {
            let mut bases = vec![link.url.clone()];
            let repo = link.repo_id.as_ref().and_then(|id| known.iter().find(|r| &r.repo_id == id));
            bases.extend(repo.into_iter().flat_map(|r| r.hosts.iter().cloned().chain([r.url.clone()])));
            for base in bases.iter().filter(|b| origin(b).as_deref() == Some(rec.origin.as_str())) {
                fetch = fetch
                    .account(base, token)
                    .map_err(|e| said("android.web_client", "web connections cannot start: %1", &[&e]))?;
            }
        }
    }
    Ok(fetch)
}

pub(crate) fn needed(site: &str) -> Error {
    let host = site.split("://").nth(1).unwrap_or(site);
    said(
        "android.account_needed",
        "This repository is private: sign in with an account of %1 to open it",
        &[&host, &site],
    )
}

pub(crate) fn list() -> Value {
    let accounts = book::file().accounts.into_iter().map(|r| {
        let repos: Vec<Value> = r.links.iter().map(|l| json!({"url": l.url, "repo_id": l.repo_id})).collect();
        json!({"id": r.id, "server": r.origin, "user": r.user, "label": r.label, "repos": repos})
    });
    json!({"accounts": accounts.collect::<Vec<_>>()})
}

pub(crate) fn use_for(repo: &str, account: &str) -> Result<Value, Error> {
    let link = link_of(repo);
    let site = origin(&link.url).unwrap_or_default();
    edit(|file| {
        let Some(index) = file.accounts.iter().position(|r| r.id == account) else {
            return Err(said("android.no_account", "no account %1", &[&account]));
        };
        if file.accounts[index].origin != site {
            return Err(said(
                "android.account_other_server",
                "that account belongs to another server than the repository",
                &[],
            ));
        }
        file.accounts.iter_mut().for_each(|r| r.links.retain(|l| !same(l, &link)));
        file.accounts[index].links.push(link);
        Ok(())
    })?;
    Ok(Value::Null)
}

pub(crate) fn unlink(repo: &str) -> Result<Value, Error> {
    let link = link_of(repo);
    edit(|file| {
        file.accounts.iter_mut().for_each(|r| r.links.retain(|l| !same(l, &link)));
        Ok(())
    })?;
    Ok(Value::Null)
}

pub(crate) fn remove(account: &str) -> Result<Value, Error> {
    let gone = edit(|file| {
        let index = file.accounts.iter().position(|r| r.id == account);
        index
            .map(|i| file.accounts.remove(i))
            .ok_or_else(|| said("android.no_account", "no account %1", &[&account]))
    })?;
    drop_token(&gone.id)?;
    Ok(Value::Null)
}

pub(crate) fn resolve() {
    let known = reiwand_format::repos(&repos::dir().unwrap_or_default(), now()).unwrap_or_default();
    let open = book::file().accounts.iter().flat_map(|r| &r.links).any(|l| l.repo_id.is_none());
    if !open {
        return;
    }
    drop(edit(|file| {
        for link in file.accounts.iter_mut().flat_map(|r| r.links.iter_mut()) {
            let found = known.iter().find(|r| clean(&r.url) == clean(&link.url));
            if let (None, Some(repo)) = (&link.repo_id, found) {
                link.repo_id = Some(repo.repo_id.clone());
            }
        }
        Ok(())
    }));
}

pub(crate) fn token_for(repo_url: &str) -> Option<zeroize::Zeroizing<String>> {
    let wanted = clean(repo_url);
    let rec = book::file().accounts.into_iter().find(|r| r.links.iter().any(|l| clean(&l.url) == wanted))?;
    book::tokens().remove(&rec.id)
}
