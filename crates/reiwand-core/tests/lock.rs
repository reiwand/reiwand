use reiwand_core::{Config, Fit, LockShow};
use std::path::{Path, PathBuf};

fn picture(path: &str) -> LockShow {
    LockShow::image(path.into())
}

fn list(uid: &str) -> LockShow {
    LockShow::playlist(uid.into())
}

fn desktop() -> Config {
    let text = "[default]\nimage = \"/desk.png\"\nfit = \"tile\"\n[outputs.DP-2]\nplaylist = \"P1\"\nfit = \"center\"\n";
    Config::parse(&Config::complete(text)).unwrap()
}

#[test]
fn the_lock_screen_follows_the_desktop_until_something_is_chosen() {
    let c = desktop();
    for screen in ["DP-1", "DP-2"] {
        assert_eq!(c.resolve(&format!("lock:{screen}")), c.resolve(screen));
        assert_eq!(c.lock_choice(screen), None);
    }
}

#[test]
fn one_choice_for_every_lock_screen_and_one_per_screen_keep_the_placement_of_the_screen() {
    let mut c = desktop();
    c.set_lock("", picture("/calm.png"));
    c.set_lock("DP-2", list("P9"));
    let (dp1, dp2) = (c.resolve("lock:DP-1"), c.resolve("lock:DP-2"));
    assert_eq!(
        (dp1.image.as_deref(), dp1.playlist, dp1.fit),
        (Some(Path::new("/calm.png")), None, Fit::Tile)
    );
    assert_eq!((dp2.image, dp2.playlist.as_deref(), dp2.fit), (None, Some("P9"), Fit::Center));
    assert_eq!(c.resolve("DP-1").image.as_deref(), Some(Path::new("/desk.png")));
    assert_eq!(c.resolve("DP-2").playlist.as_deref(), Some("P1"));
}

#[test]
fn following_the_desktop_again_drops_the_choice_and_the_block() {
    let mut c = desktop();
    c.set_lock("DP-2", list("P9"));
    c.set_lock("", picture("/calm.png"));
    assert!(c.lock.as_ref().unwrap().outputs.is_empty(), "every screen replaces the screens' own");
    c.set_lock("DP-1", list("P9"));
    c.set_lock("DP-1", LockShow::default());
    assert_eq!(c.lock_choice("DP-1"), Some(picture("/calm.png")));
    c.set_lock("", LockShow::default());
    assert_eq!(c.lock, None);
    assert_eq!(c.resolve("lock:DP-1"), c.resolve("DP-1"));
}

#[test]
fn a_removed_picture_or_playlist_leaves_the_lock_screen() {
    let mut c = desktop();
    c.set_lock("", picture("/calm.png"));
    c.set_lock("DP-2", list("P9"));
    assert!(c.forget_lock_files(&[PathBuf::from("/calm.png")]));
    assert_eq!(c.lock_choice("DP-1"), None);
    assert!(c.forget_lock(|show| show.playlist.as_deref() == Some("P9")));
    assert_eq!(c.lock, None);
    assert!(!c.forget_lock_files(&[PathBuf::from("/calm.png")]));
}

#[test]
fn the_lock_block_is_written_whole_and_read_back() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    let mut c = desktop();
    c.set_lock("DP-1", picture("/calm.png"));
    c.save(&path).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.contains("[lock]\nimage = \"\"\nplaylist = \"\"\n"), "{text}");
    assert!(text.contains("[lock.outputs.DP-1]\nimage = \"/calm.png\"\nplaylist = \"\"\n"), "{text}");
    assert_eq!(Config::load(&path).unwrap(), c);
    let cut = text.replace("[lock.outputs.DP-1]\nimage = \"/calm.png\"\n", "[lock.outputs.DP-1]\n");
    let error = Config::parse(&cut).unwrap_err().to_string();
    assert!(error.contains("lock.outputs.DP-1.image"), "{error}");
    assert!(!Config::template().contains("[lock]"));
}

#[test]
fn one_screen_can_keep_its_desktop_under_a_choice_for_every_screen() {
    let keep = LockShow { desktop: Some(true), ..LockShow::default() };
    let mut c = desktop();
    c.set_lock("DP-1", keep.clone());
    assert_eq!(c.lock, None, "without a choice for every screen there is nothing to override");
    c.set_lock("", picture("/calm.png"));
    c.set_lock("DP-1", keep);
    assert_eq!((c.lock_choice("DP-1"), c.lock_choice("DP-2")), (None, Some(picture("/calm.png"))));
    assert_eq!(c.resolve("lock:DP-1"), c.resolve("DP-1"));
    let text = toml::to_string_pretty(&c).unwrap();
    assert!(text.contains("[lock.outputs.DP-1]\nimage = \"\"\nplaylist = \"\"\ndesktop = true\n"), "{text}");
    assert_eq!(Config::parse(&text).unwrap(), c);
    c.set_lock("DP-1", LockShow::default());
    assert_eq!(c.lock_choice("DP-1"), Some(picture("/calm.png")), "following again takes every screen's");
    c.set_lock("DP-1", LockShow { desktop: Some(true), ..LockShow::default() });
    assert!(c.forget_lock_files(&[PathBuf::from("/calm.png")]));
    assert_eq!(c.lock, None, "the mark goes with the choice it overrode");
}
