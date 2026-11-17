use super::hex;
use crate::daemon::Daemon;
use crate::error::{Error, Result};
use crate::jobs::now;
use crate::keyring::Stored;
use crate::kinds::kind_of;
use reiwand_carry::{
    Content, Installed, Item, List, OwnFile, Provider, Rating, Repository, Secrets, Tagging, portable,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

pub struct Gathered {
    pub content: Content,
    pub own: Vec<(OwnFile, PathBuf)>,
    pub uids: HashMap<Item, String>,
}

fn origin(address: &str) -> String {
    let origin = reqwest::Url::parse(address).map(|u| u.origin().ascii_serialization());
    origin.ok().filter(|o| o != "null").unwrap_or_default()
}

fn providers(map: &BTreeMap<String, toml::Value>) -> Vec<Provider> {
    let mut found = Vec::new();
    for (name, value) in map {
        let Some(table) = value.as_table() else { continue };
        let Some(kind) = table.get("kind").and_then(toml::Value::as_str) else { continue };
        let mut config = serde_json::to_value(table).unwrap_or_default();
        config.as_object_mut().map(|o| o.remove("kind"));
        found.push(Provider { kind: kind.to_owned(), name: name.clone(), config });
    }
    found
}

impl Daemon {
    pub fn carry_state(&self) -> Result<Gathered> {
        let repos = reiwand_format::repos(&self.store.data, now())?;
        let repositories = repos
            .into_iter()
            .filter(|r| r.broken.is_none() && !self.is_examples(&r.repo_id))
            .map(|r| Repository { origin: origin(&r.url), address: r.url, repo_id: r.repo_id, name: r.name })
            .collect();
        let installed = self.state().installed.clone();
        let (entries, lists) = self.store.read(|lib| {
            let mut lists = Vec::new();
            for info in lib.playlists()?.into_iter().filter(|p| p.query.is_none()) {
                let items = lib.playlist_items(&info.uid, now())?;
                lists.push((info.name, items.into_iter().map(|i| i.uid).collect::<Vec<_>>()));
            }
            Ok::<_, Error>((lib.hashed()?, lists))
        })?;
        let by_path: HashMap<_, _> = installed.iter().map(|i| (&i.path, i)).collect();
        let files = self.store.files();
        let (mut uids, mut names, mut own) = (HashMap::new(), HashMap::new(), BTreeMap::new());
        let (mut favourites, mut ratings, mut tags) = (BTreeSet::new(), Vec::new(), Vec::new());
        for (entry, hash) in &entries {
            let key = match by_path.get(&entry.path) {
                Some(i) if self.is_examples(&i.repo_id) => continue,
                Some(i) => Item { repo_id: Some(i.repo_id.clone()), id: i.id.clone() },
                None if entry.path.starts_with(&files) && !entry.missing => {
                    let name = entry.path.file_name().map(|n| n.to_string_lossy().into_owned());
                    let id = hex(hash);
                    let file = OwnFile { id: id.clone(), name: name.unwrap_or_else(|| id.clone()) };
                    own.entry(id).or_insert((file, entry.path.clone()));
                    Item { repo_id: None, id: hex(hash) }
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
            })
            .collect();
        let mut installed: Vec<Installed> = (installed.iter())
            .filter(|i| !self.is_examples(&i.repo_id))
            .map(|i| Installed { repo_id: i.repo_id.clone(), id: i.id.clone(), version: i.version.clone() })
            .collect();
        installed.sort();
        let options = self.state().config.options();
        let settings = options.iter().filter_map(|(k, v)| Some((k.clone(), serde_json::to_value(v).ok()?)));
        let content = Content {
            repositories,
            providers: providers(&self.state().config.providers),
            lists,
            favourites: favourites.into_iter().collect(),
            ratings,
            tags,
            installed,
            settings: portable(&settings.collect::<BTreeMap<String, Value>>()),
            own: own.values().map(|(file, _)| file.clone()).collect(),
            secrets: None,
        };
        Ok(Gathered { content, own: own.into_values().collect(), uids })
    }

    fn secret_ids(&self) -> Vec<String> {
        let providers = self.state().config.providers.clone();
        let wants = |value: &toml::Value| {
            let kind = value.get("kind").and_then(toml::Value::as_str);
            kind.and_then(|k| kind_of(k).ok()).is_some_and(|k| k.secret.is_some())
        };
        providers.iter().filter(|(_, v)| wants(v)).map(|(id, _)| id.clone()).collect()
    }

    pub fn carry_keys_held(&self) -> BTreeSet<String> {
        let Ok(keyring) = self.keyring() else { return BTreeSet::new() };
        let held = |id: &String| self.jobs.block_on(keyring.has(id)).unwrap_or(false);
        self.secret_ids().into_iter().filter(held).collect()
    }

    pub fn carry_secrets(&self) -> Result<Secrets> {
        let keyring = self.keyring()?;
        let mut provider_keys = BTreeMap::new();
        for id in self.secret_ids() {
            if let Stored::Secret(secret) = self.jobs.block_on(keyring.read(&id, false))? {
                provider_keys.insert(id, secret.to_string());
            }
        }
        Ok(Secrets { sessions: Vec::new(), provider_keys })
    }
}
