mod common;
use common::*;
use reiwand_carry::*;
use std::collections::{BTreeMap, BTreeSet};

fn local_with_calm() -> Local {
    let mut local = Local::default();
    local.lists.insert("calm".into(), [item("a"), item("z")].into());
    local
}

fn pick(mode: ListMode) -> Pick {
    Pick::all(mode, " (imported)")
}

#[test]
fn everything_is_added_to_an_empty_device() {
    let plan = plan(&content(), &Local::default(), &pick(ListMode::Join));
    let names: std::collections::HashSet<_> = plan.changes.iter().map(std::mem::discriminant).collect();
    assert_eq!(names.len(), 8);
    assert!(
        plan.changes.iter().all(|c| !matches!(c, Change::SetSetting { key, .. } if key == "playback.volume"))
    );
}

#[test]
fn same_repository_provider_favourite_are_skipped() {
    let mut local = Local::default();
    local.repo_ids.insert("r1".into());
    local.providers.insert(("photos".into(), "Gallery".into()));
    local.favourites.insert(item("a"));
    local.rated.insert(item("a"));
    local.installed.insert(("r1".into(), "a".into()));
    local.tags.insert(item("b"), ["blue".to_string()].into());
    let plan = plan(&content(), &local, &pick(ListMode::Join));
    for change in &plan.changes {
        assert!(matches!(change, Change::AddList(_) | Change::SetSetting { .. }), "{change:?}");
    }
}

#[test]
fn lists_join_adds_only_the_missing_items() {
    let plan = plan(&content(), &local_with_calm(), &pick(ListMode::Join));
    let join = plan.changes.iter().find_map(|c| match c {
        Change::JoinList { name, items } => Some((name.clone(), items.clone())),
        _ => None,
    });
    assert_eq!(join, Some(("calm".to_string(), vec![item("b")])));
    assert!(!plan.changes.iter().any(|c| matches!(c, Change::AddList(_))));
}

#[test]
fn lists_keep_both_uses_the_suffix_and_never_collides() {
    let mut local = local_with_calm();
    local.lists.insert("Calm (imported)".into(), BTreeSet::new());
    let plan = plan(&content(), &local, &pick(ListMode::Both));
    let added: Vec<_> = plan
        .changes
        .iter()
        .filter_map(|c| if let Change::AddList(l) = c { Some(l.name.clone()) } else { None })
        .collect();
    assert_eq!(added, vec!["Calm (imported) 2".to_string()]);
}

#[test]
fn duplicate_lists_inside_the_file_do_not_collide() {
    let mut c = content();
    c.lists.push(List { name: "calm".into(), items: vec![item("q")], query: None, rules: None });
    let plan = plan(&c, &Local::default(), &pick(ListMode::Both));
    let names: Vec<_> = plan
        .changes
        .iter()
        .filter_map(|c| if let Change::AddList(l) = c { Some(l.name.clone()) } else { None })
        .collect();
    assert_eq!(names, vec!["Calm".to_string(), "calm (imported)".to_string()]);
}

#[test]
fn duplicate_lists_inside_the_file_join_when_joining() {
    let mut c = content();
    c.lists.push(List { name: "calm".into(), items: vec![item("q")], query: None, rules: None });
    let plan = plan(&c, &Local::default(), &pick(ListMode::Join));
    let lists: Vec<_> = plan
        .changes
        .iter()
        .filter_map(|c| if let Change::AddList(l) = c { Some(l.items.len()) } else { None })
        .collect();
    assert_eq!(lists, vec![3]);
}

#[test]
fn unticked_sections_change_nothing() {
    let mut p = pick(ListMode::Join);
    p.settings = false;
    p.lists = false;
    let plan = plan(&content(), &Local::default(), &p);
    assert!(!plan.changes.iter().any(|c| matches!(c, Change::SetSetting { .. } | Change::AddList(_))));
}

#[test]
fn secrets_skip_repositories_with_a_session() {
    let mut c = content();
    c.secrets = Some(Secrets {
        sessions: vec![
            Session { repo_id: "r1".into(), origin: "o".into(), account: "a".into(), token: "t".into() },
            Session { repo_id: "r2".into(), origin: "o".into(), account: "a".into(), token: "t".into() },
        ],
        provider_keys: BTreeMap::new(),
    });
    let mut local = Local::default();
    local.sessions.insert("r1".into());
    let plan = plan(&c, &local, &pick(ListMode::Join));
    let got = plan
        .changes
        .iter()
        .find_map(|c| if let Change::AddSecrets(s) = c { Some(s.sessions.len()) } else { None });
    assert_eq!(got, Some(1));
}

#[test]
fn preview_counts_new_skipped_and_conflicts() {
    let mut local = local_with_calm();
    local.repo_ids.insert("r1".into());
    let rows = preview(&content(), &local).rows;
    let row = |s: &str| rows.iter().find(|r| r.section == s).unwrap().clone();
    assert_eq!((row("repositories").new, row("repositories").skipped), (0, 1));
    assert_eq!((row("lists").conflicts, row("lists").new), (1, 1));
    assert_eq!(row("settings").new, 1);
    assert_eq!(row("settings").skipped, 1);
    assert_eq!(row("favourites").new, 1);
}

#[test]
fn a_smart_list_keeps_its_search_and_rules() {
    let mut c = content();
    let rules = serde_json::json!({"order": "random"});
    c.lists.push(List {
        name: "Sky".into(),
        items: Vec::new(),
        query: Some("tag:sky".into()),
        rules: Some(rules.clone()),
    });
    let plan = plan(&c, &Local::default(), &pick(ListMode::Join));
    let kept = plan.changes.iter().find_map(|c| match c {
        Change::AddList(l) if l.name == "Sky" => Some((l.query.clone(), l.rules.clone())),
        _ => None,
    });
    assert_eq!(kept, Some((Some("tag:sky".to_string()), Some(rules))));
    let text = serde_json::to_string(&c).unwrap();
    assert_eq!(serde_json::from_str::<Content>(&text).unwrap().lists[1].query.as_deref(), Some("tag:sky"));
}
