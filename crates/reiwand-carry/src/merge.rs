use crate::device::is_device_local;
use crate::model::{Content, Installed, Item, List, OwnFile, Provider, Rating, Repository, Secrets};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListMode {
    Join,
    Both,
}

#[derive(Clone, Debug)]
pub struct Pick {
    pub repositories: bool,
    pub providers: bool,
    pub lists: bool,
    pub favourites: bool,
    pub ratings: bool,
    pub tags: bool,
    pub installed: bool,
    pub settings: bool,
    pub own: bool,
    pub secrets: bool,
    pub list_mode: ListMode,
    pub suffix: String,
}

impl Pick {
    pub fn all(list_mode: ListMode, suffix: &str) -> Pick {
        Pick {
            repositories: true,
            providers: true,
            lists: true,
            favourites: true,
            ratings: true,
            tags: true,
            installed: true,
            settings: true,
            own: true,
            secrets: true,
            list_mode,
            suffix: suffix.to_string(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Local {
    pub repo_ids: BTreeSet<String>,
    pub providers: BTreeSet<(String, String)>,
    pub lists: BTreeMap<String, BTreeSet<Item>>,
    pub favourites: BTreeSet<Item>,
    pub rated: BTreeSet<Item>,
    pub tags: BTreeMap<Item, BTreeSet<String>>,
    pub installed: BTreeSet<(String, String)>,
    pub own: BTreeSet<String>,
    pub sessions: BTreeSet<String>,
    pub provider_keys: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Change {
    AddRepository(Repository),
    AddProvider(Provider),
    AddList(List),
    JoinList { name: String, items: Vec<Item> },
    AddFavourite(Item),
    SetRating(Rating),
    AddTags { item: Item, tags: Vec<String> },
    Install(Installed),
    SetSetting { key: String, value: Value },
    AddOwn(OwnFile),
    AddSecrets(Secrets),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Plan {
    pub changes: Vec<Change>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub section: &'static str,
    pub held: usize,
    pub new: usize,
    pub skipped: usize,
    pub conflicts: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Preview {
    pub rows: Vec<Row>,
}

fn same(a: &str, b: &str) -> bool {
    a.trim().to_lowercase() == b.trim().to_lowercase()
}

fn find<'a, V>(map: &'a BTreeMap<String, V>, name: &str) -> Option<(&'a String, &'a V)> {
    map.iter().find(|(k, _)| same(k, name))
}

fn joined(out: &mut [Change], name: &str, items: &[Item]) -> bool {
    for change in out.iter_mut() {
        if let Change::AddList(list) = change
            && same(&list.name, name)
        {
            let fresh: Vec<Item> = items.iter().filter(|i| !list.items.contains(i)).cloned().collect();
            list.items.extend(fresh);
            return true;
        }
    }
    false
}

fn lists(content: &Content, local: &Local, pick: &Pick, out: &mut Vec<Change>) {
    let mut taken: BTreeSet<String> = local.lists.keys().map(|k| k.trim().to_lowercase()).collect();
    for list in &content.lists {
        let mut items: Vec<Item> = Vec::new();
        for item in &list.items {
            if !items.contains(item) {
                items.push(item.clone());
            }
        }
        match find(&local.lists, &list.name) {
            Some((name, held)) if pick.list_mode == ListMode::Join => {
                let items: Vec<Item> = items.into_iter().filter(|i| !held.contains(i)).collect();
                if !items.is_empty() {
                    out.push(Change::JoinList { name: name.clone(), items });
                }
            }
            None if pick.list_mode == ListMode::Join && joined(out, &list.name, &items) => {}
            _ if find(&local.lists, &list.name).is_some()
                || taken.contains(&list.name.trim().to_lowercase()) =>
            {
                let mut name = format!("{}{}", list.name, pick.suffix);
                let mut count = 2;
                while taken.contains(&name.trim().to_lowercase()) {
                    name = format!("{}{} {count}", list.name, pick.suffix);
                    count += 1;
                }
                taken.insert(name.trim().to_lowercase());
                out.push(Change::AddList(List { name, items }));
            }
            _ => {
                taken.insert(list.name.trim().to_lowercase());
                out.push(Change::AddList(List { name: list.name.clone(), items }));
            }
        }
    }
}

pub fn plan(content: &Content, local: &Local, pick: &Pick) -> Plan {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    if pick.repositories {
        for repo in &content.repositories {
            if !local.repo_ids.contains(&repo.repo_id) && seen.insert(repo.repo_id.clone()) {
                out.push(Change::AddRepository(repo.clone()));
            }
        }
    }
    if pick.providers {
        let mut seen = BTreeSet::new();
        for p in &content.providers {
            let id = (p.kind.clone(), p.name.clone());
            if !local.providers.contains(&id) && seen.insert(id) {
                out.push(Change::AddProvider(p.clone()));
            }
        }
    }
    if pick.lists {
        lists(content, local, pick, &mut out);
    }
    if pick.favourites {
        let mut seen = BTreeSet::new();
        for item in content.favourites.iter().filter(|i| !local.favourites.contains(i)) {
            if seen.insert(item) {
                out.push(Change::AddFavourite(item.clone()));
            }
        }
    }
    if pick.ratings {
        for r in content.ratings.iter().filter(|r| !local.rated.contains(&r.item) && r.rating > 0) {
            out.push(Change::SetRating(r.clone()));
        }
    }
    if pick.tags {
        for t in &content.tags {
            let held = local.tags.get(&t.item);
            let tags: Vec<String> =
                t.tags.iter().filter(|x| !held.is_some_and(|h| h.contains(*x))).cloned().collect();
            if !tags.is_empty() {
                out.push(Change::AddTags { item: t.item.clone(), tags });
            }
        }
    }
    if pick.installed {
        for i in
            content.installed.iter().filter(|i| !local.installed.contains(&(i.repo_id.clone(), i.id.clone())))
        {
            out.push(Change::Install(i.clone()));
        }
    }
    if pick.settings {
        for (key, value) in content.settings.iter().filter(|(k, _)| !is_device_local(k)) {
            out.push(Change::SetSetting { key: key.clone(), value: value.clone() });
        }
    }
    if pick.own {
        for own in content.own.iter().filter(|o| !local.own.contains(&o.id)) {
            out.push(Change::AddOwn(own.clone()));
        }
    }
    if let Some(secrets) = content.secrets.as_ref().filter(|_| pick.secrets) {
        let sessions =
            secrets.sessions.iter().filter(|s| !local.sessions.contains(&s.repo_id)).cloned().collect();
        let keys = secrets.provider_keys.iter().filter(|(k, _)| !local.provider_keys.contains(*k));
        let secrets =
            Secrets { sessions, provider_keys: keys.map(|(k, v)| (k.clone(), v.clone())).collect() };
        if !secrets.sessions.is_empty() || !secrets.provider_keys.is_empty() {
            out.push(Change::AddSecrets(secrets));
        }
    }
    Plan { changes: out }
}

fn section(change: &Change) -> &'static str {
    match change {
        Change::AddRepository(_) => "repositories",
        Change::AddProvider(_) => "providers",
        Change::AddList(_) | Change::JoinList { .. } => "lists",
        Change::AddFavourite(_) => "favourites",
        Change::SetRating(_) => "ratings",
        Change::AddTags { .. } => "tags",
        Change::Install(_) => "installed",
        Change::SetSetting { .. } => "settings",
        Change::AddOwn(_) => "own",
        Change::AddSecrets(_) => "secrets",
    }
}

pub fn preview(content: &Content, local: &Local) -> Preview {
    let plan = plan(content, local, &Pick::all(ListMode::Join, ""));
    let secrets = content.secrets.as_ref().map_or(0, |s| s.sessions.len() + s.provider_keys.len());
    let held = [
        ("repositories", content.repositories.len()),
        ("providers", content.providers.len()),
        ("lists", content.lists.len()),
        ("favourites", content.favourites.len()),
        ("ratings", content.ratings.len()),
        ("tags", content.tags.len()),
        ("installed", content.installed.len()),
        ("settings", content.settings.len()),
        ("own", content.own.len()),
        ("secrets", secrets),
    ];
    let clash = content.lists.iter().filter(|l| find(&local.lists, &l.name).is_some()).count();
    let rows = held.into_iter().filter(|(_, n)| *n > 0).map(|(name, held)| {
        let of = plan.changes.iter().filter(|c| section(c) == name);
        let new = match name {
            "secrets" => {
                of.map(|c| {
                    if let Change::AddSecrets(s) = c { s.sessions.len() + s.provider_keys.len() } else { 0 }
                })
                .sum()
            }
            "lists" => {
                content.lists.len() - clash + of.filter(|c| matches!(c, Change::JoinList { .. })).count()
            }
            _ => of.count(),
        };
        let conflicts = if name == "lists" { clash } else { 0 };
        Row { section: name, held, new, skipped: held.saturating_sub(new), conflicts }
    });
    Preview { rows: rows.collect() }
}
