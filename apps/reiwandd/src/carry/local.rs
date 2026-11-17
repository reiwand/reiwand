use reiwand_carry::{Content, Local};
use std::collections::{BTreeMap, BTreeSet};

pub fn local_of(content: &Content, keys: BTreeSet<String>, sessions: BTreeSet<String>) -> Local {
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
    local.sessions = sessions;
    local
}

#[cfg(test)]
mod tests {
    use super::*;
    use reiwand_carry::{Installed, Item, List, OwnFile, Repository};

    fn item(id: &str) -> Item {
        Item { repo_id: None, id: id.into() }
    }

    #[test]
    fn the_local_view_reflects_every_section() {
        let content = Content {
            repositories: vec![Repository {
                address: "a".into(),
                repo_id: "r".into(),
                origin: "o".into(),
                name: String::new(),
                pin: None,
            }],
            lists: vec![List { name: "Calm".into(), items: vec![item("a")], query: None, rules: None }],
            favourites: vec![item("a")],
            installed: vec![Installed { repo_id: "r".into(), id: "p".into(), version: "1.0.0".into() }],
            own: vec![OwnFile { id: "h".into(), name: "x.png".into() }],
            ..Content::default()
        };
        let local = local_of(&content, ["prov".to_owned()].into(), BTreeSet::new());
        assert!(
            local.repo_ids.contains("r") && local.own.contains("h") && local.provider_keys.contains("prov")
        );
        assert!(local.lists["Calm"].contains(&item("a")) && local.favourites.contains(&item("a")));
        assert!(local.installed.contains(&("r".to_owned(), "p".to_owned())));
    }
}
