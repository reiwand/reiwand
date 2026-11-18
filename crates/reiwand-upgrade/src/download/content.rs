use reiwand_carry::{Content, Item, List, Rating, Repository, Tagging, portable};
use reiwand_sync::{Ids, View};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub struct Installed {
    pub repo_id: String,
    pub id: String,
    pub version: String,
}

fn text(view: &View, field: &str) -> String {
    view.get(field).and_then(Value::as_str).unwrap_or_default().to_owned()
}

pub fn origin(address: &str) -> String {
    reiwand_account::origin(address).unwrap_or_default()
}

pub fn who(ids: &Ids, installed: &[Installed], views: &BTreeMap<String, View>) -> BTreeMap<String, Item> {
    let mut found = BTreeMap::new();
    for package in installed {
        let key = format!("library/{}", ids.library(&package.repo_id, &package.id));
        if views.contains_key(&key) {
            found.insert(key, Item { repo_id: Some(package.repo_id.clone()), id: package.id.clone() });
        }
    }
    for (key, view) in views.iter().filter(|(key, _)| key.starts_with("library/")) {
        if let Some(hash) = view.get("blob").and_then(|b| b["hash"].as_str()) {
            found.insert(key.clone(), Item { repo_id: None, id: hash.to_owned() });
        }
    }
    found
}

pub fn build(
    views: &BTreeMap<String, View>,
    who: &BTreeMap<String, Item>,
    installed: &[Installed],
) -> Content {
    let mut content = Content::empty();
    let mut favourites = BTreeSet::new();
    for (key, view) in views.iter().filter(|(key, _)| key.starts_with("library/")) {
        let Some(item) = who.get(key) else { continue };
        if view.get("favorite") == Some(&Value::Bool(true)) {
            favourites.insert(item.clone());
        }
        if let Some(rating) = view.get("rating").and_then(Value::as_u64).filter(|r| (1..=5).contains(r)) {
            content.ratings.push(Rating { item: item.clone(), rating: rating as u8 });
        }
        let tags: Vec<String> = view.keys().filter_map(|k| k.strip_prefix("t:")).map(str::to_owned).collect();
        if !tags.is_empty() {
            content.tags.push(Tagging { item: item.clone(), tags });
        }
    }
    content.favourites = favourites.into_iter().collect();
    for (key, view) in views.iter().filter(|(key, _)| key.starts_with("playlists/")) {
        let mut order: Vec<(f64, &String, &str)> = view
            .iter()
            .filter(|(k, _)| k.starts_with("e:"))
            .filter_map(|(id, e)| Some((e["p"].as_f64()?, id, e["i"].as_str()?)))
            .collect();
        order.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(b.1)));
        let items = order.iter().filter_map(|(.., i)| who.get(&format!("library/{i}")).cloned()).collect();
        let query = view.get("query").and_then(Value::as_str).map(str::to_owned);
        let rules: serde_json::Map<String, Value> =
            view.iter().filter_map(|(k, v)| Some((k.strip_prefix("r:")?.to_owned(), v.clone()))).collect();
        let name = Some(text(view, "name")).filter(|n| !n.is_empty()).unwrap_or_else(|| key.clone());
        let rules = query.as_ref().map(|_| Value::Object(rules));
        content.lists.push(List { name, items, query, rules });
    }
    for view in views.iter().filter(|(key, _)| key.starts_with("repos/")).map(|(_, v)| v) {
        let (address, repo_id) = (text(view, "url"), text(view, "repo_id"));
        if !address.is_empty() && repo_id.len() == 64 {
            let (origin, name) = (origin(&address), text(view, "name"));
            content.repositories.push(Repository { address, repo_id, origin, name, pin: None });
        }
    }
    let mut options = BTreeMap::new();
    for view in views.iter().filter(|(key, _)| key.starts_with("settings/")).map(|(_, v)| v) {
        if !view.contains_key("kind") {
            options.extend(view.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
    }
    content.settings = portable(&options);
    content.installed = installed
        .iter()
        .map(|i| reiwand_carry::Installed {
            repo_id: i.repo_id.clone(),
            id: i.id.clone(),
            version: i.version.clone(),
        })
        .filter(|i| who.values().any(|w| w.repo_id.as_deref() == Some(&i.repo_id) && w.id == i.id))
        .collect();
    content
}
