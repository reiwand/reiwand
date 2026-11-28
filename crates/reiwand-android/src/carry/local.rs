use reiwand_carry::{Content, Local};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn local_of(content: &Content, keys: BTreeSet<String>) -> Local {
    let mut local = Local {
        repo_ids: content.repositories.iter().map(|r| r.repo_id.clone()).collect(),
        providers: content.providers.iter().map(|p| (p.kind.clone(), p.name.clone())).collect(),
        ..Local::default()
    };
    for list in &content.lists {
        local.lists.entry(list.name.clone()).or_default().extend(list.items.iter().cloned());
    }
    local.favourites = content.favourites.iter().cloned().collect();
    local.rated = content.ratings.iter().map(|r| r.item.clone()).collect();
    let mut tags: BTreeMap<_, BTreeSet<String>> = BTreeMap::new();
    for tagging in &content.tags {
        tags.entry(tagging.item.clone()).or_default().extend(tagging.tags.iter().cloned());
    }
    local.tags = tags;
    local.installed = content.installed.iter().map(|i| (i.repo_id.clone(), i.id.clone())).collect();
    local.own = content.own.iter().map(|o| o.id.clone()).collect();
    local.provider_keys = keys;
    local
}
