use crate::model::{Installed, Item, List, OwnFile, Provider, Rating, Repository, Secrets};
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
