use super::*;
use std::collections::{BTreeMap, BTreeSet};

const VIEW: View =
    View { active: true, covered: false, unseen: false, rect: Some([0.0, 0.0, 100.0, 50.0]), clicks: false };

fn tune(key: &str) -> SceneInputs {
    SceneInputs { media: true, ..scene(SEA, key, false, false) }
}

fn views(names: &[&str]) -> Option<(BTreeMap<String, View>, bool)> {
    Some((names.iter().map(|n| ((*n).to_owned(), VIEW)).collect(), false))
}

fn set(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|n| (*n).to_owned()).collect()
}

fn real(hub: &Hub) -> (BTreeSet<String>, BTreeSet<String>) {
    let control = lock(&hub.control);
    let keep = |set: BTreeSet<String>| set.into_iter().filter(|o| !o.starts_with("preview:")).collect();
    (keep(control.wants(Input::Media)), keep(control.wants(Input::Pointer)))
}

#[test]
fn starting_and_releasing_previews_never_changes_the_inputs_of_real_screens() {
    let hub = hub(Arc::default());
    {
        let mut control = lock(&hub.control);
        control.grants = bound("sha256:aa", &[Input::Media]);
        control.shown.insert("DP-1".into(), tune("sha256:aa"));
        control.parallax.insert("DP-2".into());
    }
    hub.update(views(&["DP-1", "DP-2"]));
    let expected = (set(&["DP-1"]), set(&["DP-2"]));
    assert_eq!(real(&hub), expected);
    assert!(*hub.tracking.borrow(), "the picture of DP-2 follows the pointer");
    for round in 0..3 {
        let card = format!("preview:card-{round}");
        {
            let mut control = lock(&hub.control);
            control.shown.insert(card.clone(), tune("sha256:aa"));
            control.parallax.insert(format!("{card}-picture"));
        }
        hub.update(views(&["DP-1", "DP-2", &card, &format!("{card}-picture")]));
        assert_eq!(real(&hub), expected, "a preview took the inputs of the screens");
        assert!(lock(&hub.control).wants(Input::Media).contains(&card));
        {
            let mut control = lock(&hub.control);
            control.shown.remove(&card);
            control.parallax.remove(&format!("{card}-picture"));
        }
        hub.update(views(&["DP-1", "DP-2"]));
        assert_eq!(real(&hub), expected, "releasing a preview dropped the inputs of the screens");
        assert!(*hub.tracking.borrow(), "the pointer feed stopped with the preview");
        assert_eq!(lock(&hub.control).followed, set(&["DP-2"]));
    }
}

#[test]
fn a_wallpaper_in_use_keeps_its_inputs_while_it_is_visible() {
    let hub = hub(Arc::default());
    {
        let mut control = lock(&hub.control);
        control.grants = bound("sha256:aa", &[Input::Media]);
        control.shown.insert("DP-1".into(), tune("sha256:aa"));
        control.parallax.insert("DP-1".into());
    }
    hub.update(views(&["DP-1"]));
    assert_eq!(real(&hub), (set(&["DP-1"]), set(&["DP-1"])));
    let mut covered = views(&["DP-1"]);
    covered.as_mut().unwrap().0.get_mut("DP-1").unwrap().covered = true;
    hub.update(covered);
    assert_eq!(real(&hub), Default::default(), "a covered screen reads nothing");
    assert!(!*hub.tracking.borrow());
    hub.update(views(&["DP-1"]));
    assert_eq!(real(&hub), (set(&["DP-1"]), set(&["DP-1"])), "showing it again starts them again");
    assert!(*hub.tracking.borrow());
}
