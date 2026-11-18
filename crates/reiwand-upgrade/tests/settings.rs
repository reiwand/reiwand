use reiwand_core::{Config, Fit};
use reiwand_upgrade::settings;
use std::path::PathBuf;

const ORIGINAL: &str = include_str!("pairs/config.realistic.toml");
const SCREENS: [&str; 6] = ["DP-1", "HDMI-A-1", "preview:1", "DP-3", "DP-2", ""];

fn raw(doc: &toml::Value, screen: &str, key: &str) -> Option<toml::Value> {
    let own = doc.get("outputs").and_then(|o| o.get(screen)).and_then(|o| o.get(key));
    own.or_else(|| doc.get("default").and_then(|d| d.get(key))).cloned()
}

fn expected_fit(doc: &toml::Value, screen: &str) -> Option<Fit> {
    raw(doc, screen, "fit").and_then(|v| v.try_into().ok())
}

#[test]
fn every_screen_resolves_the_same_after_the_conversion() {
    let before: toml::Value = toml::from_str(ORIGINAL).unwrap();
    let converted = settings::config(ORIGINAL).unwrap();
    let after = Config::parse(&converted).unwrap();
    assert_eq!(after.profile.as_deref(), Some("night"));
    for screen in SCREENS {
        let resolved = after.resolve(screen);
        let own = before.get("outputs").and_then(|o| o.get(screen));
        let sets = |key: &str| own.is_some_and(|o| o.get(key).is_some());
        let content =
            if sets("image") || sets("playlist") || sets("blank") { own } else { before.get("default") };
        let image = content.and_then(|c| c.get("image")).and_then(|v| v.as_str()).map(PathBuf::from);
        let playlist = content.and_then(|c| c.get("playlist")).and_then(|v| v.as_str()).map(str::to_owned);
        let private = screen.starts_with("preview:") && !sets("image");
        if !private {
            assert_eq!(resolved.playlist, playlist, "{screen}");
            assert_eq!(resolved.image, image.filter(|_| resolved.playlist.is_none()), "{screen}");
        }
        assert_eq!(after.chosen(screen).0, expected_fit(&before, screen), "{screen}");
        let color = raw(&before, screen, "background").and_then(|v| v.as_str().map(str::to_owned));
        assert_eq!(
            after.chosen(screen).1,
            color.and_then(|c| reiwand_core::config::parse_hex_color(&c)),
            "{screen}"
        );
        assert_eq!(
            after.clicks(screen),
            raw(&before, screen, "clicks").is_some_and(|v| v.as_bool() == Some(true))
        );
    }
    assert_eq!(after.playback.max_fps, 30);
    assert_eq!((after.library.watch, after.layout.bezel_mm), (false, 6.0));
    assert_eq!(after.surfaces.layer_namespace(), "desktop-bg");
    assert!(after.rgb.enabled && after.read_providers::<toml::Value>().count() == 1);
}

#[test]
fn the_converted_file_is_whole_keeps_comments_and_converts_once() {
    let converted = settings::config(ORIGINAL).unwrap();
    assert!(converted.contains("# my desk") && converted.contains("format = 3"));
    assert_eq!(settings::config(&converted).unwrap(), converted);
    let again = Config::parse(&converted).unwrap();
    assert_eq!(Config::parse(&Config::complete(&toml::to_string(&again).unwrap())).unwrap(), again);
}

#[test]
fn an_empty_old_file_becomes_the_first_start_template() {
    let converted = settings::config("").unwrap();
    assert_eq!(Config::parse(&converted).unwrap(), Config::default());
}

#[test]
fn run_converts_in_place_and_keeps_the_original_copy() {
    let dir = tempfile::tempdir().unwrap();
    let (config, copies) = (dir.path().join("config"), dir.path().join("copies"));
    std::fs::create_dir_all(&config).unwrap();
    std::fs::write(config.join("config.toml"), ORIGINAL).unwrap();
    assert!(settings::run(&config, &copies).unwrap());
    assert_eq!(std::fs::read_to_string(copies.join("config.toml")).unwrap(), ORIGINAL);
    Config::load(&config.join("config.toml")).unwrap();
    assert!(!settings::run(&config, &copies).unwrap());
}
