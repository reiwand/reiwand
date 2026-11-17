use crate::library::{now, with};
use crate::providers::carried;
use crate::{Error, repos, settings, store};
use reiwand_account::{hex, origin};
use reiwand_carry::{
    Content, Installed, Item, List, OwnFile, Provider, Rating, Repository, Tagging, portable,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

pub(super) struct Gathered {
    pub(super) content: Content,
    pub(super) own: Vec<(OwnFile, PathBuf)>,
    pub(super) uids: HashMap<Item, String>,
}

fn repositories() -> Result<Vec<Repository>, Error> {
    let known = reiwand_format::repos(&repos::dir()?, now())?;
    let mine = |r: &reiwand_format::RepoInfo| r.broken.is_none() && !repos::bundled::is_bundled(&r.repo_id);
    let found = known.into_iter().filter(mine);
    Ok(found
        .map(|r| Repository {
            origin: origin(&r.url).unwrap_or_default(),
            address: r.url,
            repo_id: r.repo_id,
            name: r.name,
            pin: None,
        })
        .collect())
}

fn providers() -> Vec<Provider> {
    let config = |options: BTreeMap<String, String>| json!(options);
    (carried::all().into_iter())
        .map(|p| Provider { kind: p.kind, name: p.id, config: config(p.options) })
        .collect()
}

fn settings_of() -> BTreeMap<String, Value> {
    let options = settings::config().options();
    let plain = options.iter().filter_map(|(k, v)| Some((k.clone(), serde_json::to_value(v).ok()?)));
    portable(&plain.collect())
}

pub(super) fn gather() -> Result<Gathered, Error> {
    let installed = repos::installed();
    let files = store::dir()?.join(store::MEDIA);
    let (entries, lists, versions) = with(|lib| {
        let mut lists = Vec::new();
        for info in lib.playlists()?.into_iter().filter(|p| p.query.is_none()) {
            let items = lib.playlist_items(&info.uid, now())?;
            lists.push((info.name, items.into_iter().map(|i| i.uid).collect::<Vec<_>>()));
        }
        let mut versions = HashMap::new();
        for uid in installed.keys() {
            if let Ok(item) = lib.item(uid) {
                versions.insert(uid.clone(), item.package.map(|p| p.version).unwrap_or_default());
            }
        }
        Ok((lib.hashed()?, lists, versions))
    })?;
    let (mut uids, mut names, mut own) = (HashMap::new(), HashMap::new(), BTreeMap::new());
    let (mut favourites, mut ratings, mut tags) = (BTreeSet::new(), Vec::new(), Vec::new());
    for (entry, hash) in &entries {
        let key = match installed.get(&entry.uid) {
            Some((repo_id, _)) if repos::bundled::is_bundled(repo_id) => continue,
            Some((repo_id, id)) => Item { repo_id: Some(repo_id.clone()), id: id.clone() },
            None if entry.path.starts_with(&files) && !entry.missing && entry.path.is_file() => {
                let id = hex(hash);
                let name = entry.path.file_name().map(|n| n.to_string_lossy().into_owned());
                own.entry(id.clone()).or_insert((
                    OwnFile { id: id.clone(), name: name.unwrap_or_else(|| id.clone()) },
                    entry.path.clone(),
                ));
                Item { repo_id: None, id }
            }
            None => continue,
        };
        if entry.favorite {
            favourites.insert(key.clone());
        }
        if entry.rating > 0 {
            ratings.push(Rating { item: key.clone(), rating: entry.rating });
        }
        if !entry.tags.is_empty() {
            tags.push(Tagging { item: key.clone(), tags: entry.tags.clone() });
        }
        names.insert(entry.uid.clone(), key.clone());
        uids.insert(key, entry.uid.clone());
    }
    let lists = lists
        .into_iter()
        .map(|(name, items)| List {
            name,
            items: items.iter().filter_map(|u| names.get(u).cloned()).collect(),
            query: None,
            rules: None,
        })
        .collect();
    let mut held: Vec<Installed> = (installed.iter())
        .filter(|(_, (repo_id, _))| !repos::bundled::is_bundled(repo_id))
        .map(|(uid, (repo_id, id))| Installed {
            repo_id: repo_id.clone(),
            id: id.clone(),
            version: versions.get(uid).cloned().unwrap_or_default(),
        })
        .collect();
    held.sort();
    let content = Content {
        repositories: repositories()?,
        providers: providers(),
        lists,
        favourites: favourites.into_iter().collect(),
        ratings,
        tags,
        installed: held,
        settings: settings_of(),
        own: own.values().map(|(file, _)| file.clone()).collect(),
        secrets: None,
    };
    Ok(Gathered { content, own: own.into_values().collect(), uids })
}
