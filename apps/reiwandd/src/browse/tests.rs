use super::act::Do;
use super::list::moved;
use super::shortcuts::{action, proposed};
use super::{Browsing, source};
use reiwand_core::model::{BrowseSource, ShortcutsState};
use reiwand_core::shortcuts::Shortcuts;
use std::time::Duration;

#[test]
fn moving_wraps_both_ways() {
    assert_eq!(moved(None, 3, 1), Some(0));
    assert_eq!(moved(None, 3, -1), Some(2));
    assert_eq!(moved(None, 3, 0), None);
    assert_eq!(moved(Some(2), 3, 1), Some(0));
    assert_eq!(moved(Some(0), 3, -1), Some(2));
    assert_eq!(moved(Some(1), 3, 7), Some(2));
    assert_eq!(moved(Some(1), 0, 1), None);
}

#[test]
fn sources_are_named_or_chosen() {
    assert_eq!(source("").unwrap(), None);
    assert_eq!(source("playlist").unwrap(), Some(BrowseSource::Playlist));
    assert_eq!(source(" library ").unwrap(), Some(BrowseSource::Library));
    assert!(source("folder").is_err());
}

#[test]
fn a_held_transition_is_used_once() {
    let browsing = Browsing::default();
    assert_eq!(browsing.held("A"), None);
    browsing.hold(&["A".into(), "B".into()], Duration::from_millis(300));
    assert_eq!(browsing.held("A"), Some(Duration::from_millis(300)));
    assert_eq!(browsing.held("A"), None);
    assert_eq!(browsing.held("B"), Some(Duration::from_millis(300)));
}

#[test]
fn shortcut_state_starts_off() {
    let browsing = Browsing::default();
    assert_eq!(browsing.shortcuts(), ShortcutsState::Off);
    browsing.set_shortcuts(ShortcutsState::Bound);
    assert_eq!(browsing.shortcuts(), ShortcutsState::Bound);
}

#[test]
fn shortcuts_propose_valid_triggers_only() {
    let wanted = Shortcuts {
        enabled: true,
        next: "LOGO+ALT+N".into(),
        previous: "bad key;".into(),
        ..Shortcuts::default()
    };
    let shown = proposed(&wanted);
    assert_eq!(shown[0].2, Some("LOGO+ALT+N"));
    assert_eq!(shown[1].2, None);
    assert_eq!(shown[2].2, Some("LOGO+ALT+Return"));
    assert_eq!(shown[3].2, Some("LOGO+ALT+Escape"));
    assert_eq!(action(shown[0].0), Some(Do::Move(1)));
    assert_eq!(action(shown[1].0), Some(Do::Move(-1)));
    assert_eq!(action(shown[2].0), Some(Do::Apply));
    assert_eq!(action(shown[3].0), Some(Do::Cancel));
    assert_eq!(action("other"), None);
}
