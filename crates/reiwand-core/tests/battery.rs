use reiwand_core::{BatteryPause, Config};

#[test]
fn pausing_on_battery_is_one_setting_with_three_values_kept_in_range() {
    let mut c = Config::default();
    assert_eq!((c.pause.on_battery, c.playback.lowest_below_percent), (BatteryPause::Always, 0));
    for (text, want) in [
        ("never", BatteryPause::Never),
        ("below:20", BatteryPause::Below(20)),
        ("always", BatteryPause::Always),
    ] {
        c.set_option("pause.on_battery", text).unwrap();
        assert_eq!(c.pause.on_battery, want);
        assert_eq!(c.options()["pause.on_battery"], toml::Value::String(text.into()));
    }
    c.set_option("pause.on_battery", "below:20").unwrap();
    c.set_option("playback.lowest_below_percent", "35").unwrap();
    for bad in ["below:0", "below:101", "below:-1", "below:half", "20", "true", ""] {
        assert!(c.set_option("pause.on_battery", bad).is_err(), "{bad}");
    }
    for bad in ["101", "-1", "half"] {
        assert!(c.set_option("playback.lowest_below_percent", bad).is_err(), "{bad}");
    }
    assert_eq!((c.pause.on_battery, c.playback.lowest_below_percent), (BatteryPause::Below(20), 35));
}

#[test]
fn each_value_pauses_only_on_battery_and_the_threshold_only_with_a_known_charge() {
    let below = BatteryPause::Below(20);
    assert!(below.pauses(true, Some(19)) && !below.pauses(true, Some(20)));
    assert!(!below.pauses(false, Some(5)) && !below.pauses(true, None));
    assert!(BatteryPause::Always.pauses(true, None) && !BatteryPause::Always.pauses(false, Some(1)));
    assert!(!BatteryPause::Never.pauses(true, Some(1)));
}

#[test]
fn the_two_old_keys_are_rejected_in_a_file() {
    assert!(toml::from_str::<Config>("[pause]\non_battery = true\n").is_err());
    assert!(toml::from_str::<Config>("[pause]\nbelow_percent = 20\n").is_err());
}

#[test]
fn a_saved_file_reads_back_with_the_battery_and_flash_settings_as_set() {
    let mut c = Config::default();
    c.set_option("pause.on_battery", "below:35").unwrap();
    c.set_option("playback.allow_flashing", "true").unwrap();
    let text = toml::to_string_pretty(&c).unwrap();
    let back = Config::parse(&text).unwrap();
    assert_eq!((back.pause.on_battery, back.playback.allow_flashing), (BatteryPause::Below(35), true));
    assert_eq!(back, c);
}
