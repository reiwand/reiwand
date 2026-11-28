use reiwand_schedule::{Order, Playlist, parse_interval};
use std::time::Duration;

const DAY: &str = r#"
id = "day"
items = ["/w/a.png", "/w/b.png", "/w/c.mp4"]
order = "shuffle"
interval = "1h30m"
per_output = true
"#;

#[test]
fn a_playlist_reads_and_writes_its_interval_in_words() {
    let day: Playlist = toml::from_str(DAY).unwrap();
    assert_eq!((day.order, day.interval, day.per_output), (Order::Shuffle, Duration::from_secs(5400), true));
    let night: Playlist =
        toml::from_str("id = \"night\"\nitems = [\"/w/n.png\"]\ninterval = \"45s\"").unwrap();
    assert_eq!(
        (night.order, night.interval, night.per_output),
        (Order::Sequential, Duration::from_secs(45), false)
    );
    let text = toml::to_string(&day).unwrap();
    assert!(text.contains("interval = \"1h30m\""), "{text}");
    assert_eq!(toml::from_str::<Playlist>(&text).unwrap(), day);
    assert_eq!(serde_json::from_str::<Playlist>(&serde_json::to_string(&day).unwrap()).unwrap(), day);
}

#[test]
fn intervals_use_unit_suffixes() {
    assert_eq!(parse_interval("1d 2h 3m 4s"), Some(Duration::from_secs(93_784)));
    assert_eq!(parse_interval("90s"), Some(Duration::from_secs(90)));
    for bad in ["10", "5x", "", "m", "1h 30", "-1m"] {
        assert_eq!(parse_interval(bad), None, "{bad:?}");
    }
    assert_eq!(reiwand_schedule::format_interval(Duration::from_secs(5400)), "1h30m");
}

#[test]
fn a_playlist_refuses_a_bare_number_or_an_unknown_field() {
    let playlist = "id = \"a\"\nitems = [\"/a\"]\ninterval = \"1m\"\n";
    assert!(toml::from_str::<Playlist>(&playlist.replace("1m", "1")).is_err());
    assert!(toml::from_str::<Playlist>(&format!("{playlist}shufle = true")).is_err());
}
