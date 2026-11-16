use reiwand_core::edit::Edit;
use reiwand_core::{Config, Fit};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

const CHANGED: &str = include_str!("changed.toml");
const EDIT: &str = include_str!("edit/full.json");
const VALUES: [&str; 14] = [
    "-1",
    "0",
    "9223372036854775807",
    "-9223372036854775808",
    "1e308",
    "nan",
    "-inf",
    "\"\"",
    "\"x\\u0000\\u202e../../y\"",
    "[]",
    "{}",
    "true",
    "[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[1]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]",
    "4294967296",
];

struct Rng(u64);

impl Rng {
    fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n.max(1) as u64) as usize
    }
}

fn capped<T: Send + 'static>(what: String, work: impl FnOnce() -> T + Send + 'static) -> T {
    let (send, receive) = mpsc::channel();
    std::thread::spawn(move || send.send(work()));
    match receive.recv_timeout(Duration::from_secs(20)) {
        Ok(value) => value,
        Err(mpsc::RecvTimeoutError::Timeout) => panic!("hung on {what}"),
        Err(mpsc::RecvTimeoutError::Disconnected) => panic!("panicked on {what}"),
    }
}

fn damaged(rng: &mut Rng, text: &str, value: impl Fn(&mut Rng) -> String) -> String {
    let mut lines: Vec<String> = text.lines().map(String::from).collect();
    let at = rng.below(lines.len());
    match rng.below(8) {
        0 => lines.insert(at, lines[at].clone()),
        1 => drop(lines.remove(at)),
        2 => {
            let cut = text.char_indices().nth(rng.below(text.chars().count())).map_or(0, |(i, _)| i);
            return text[..cut].to_string();
        }
        _ => {
            let line = &lines[at];
            let Some(split) = line.find([':', '=']) else { return lines.join("\n") };
            lines[at] = format!("{} {}", &line[..=split], value(rng));
        }
    }
    lines.join("\n")
}

fn modern() -> String {
    let mut config = Config::parse(CHANGED).unwrap();
    config.set_image("DP-1", PathBuf::from("pictures/hills.jpg"), Some(Fit::Fill));
    config.set_playlist("HDMI-A-1", "evening".into());
    config.set_background("DP-1", "#102030").unwrap();
    toml::to_string_pretty(&config).unwrap()
}

#[test]
fn a_damaged_configuration_is_refused_or_saved_back_the_same() {
    let mut rng = Rng(0x00c0_ffee_1234_9876);
    for (index, text) in [CHANGED.to_string(), modern()].iter().enumerate() {
        let mut kept = 0;
        for round in 0..400 {
            let changed = damaged(&mut rng, text, |rng| VALUES[rng.below(VALUES.len())].into());
            let what = format!("config {index} round {round}:\n{changed}");
            kept += capped(what.clone(), move || {
                let Ok(mut config) = Config::parse(&changed) else { return 0 };
                config.normalize();
                let _ = (config.resolve("DP-1"), config.chosen("HDMI-A-1"), config.filter("DP-1"));
                let saved = toml::to_string_pretty(&config).unwrap_or_else(|e| panic!("cannot save: {e}"));
                let again = Config::parse(&saved).unwrap_or_else(|e| panic!("{e}\n{saved}"));
                assert_eq!(again, config, "{what}");
                1
            });
        }
        assert!(kept > 100, "only {kept} damaged configurations were read");
    }
}

fn pointers(value: &Value, at: String, out: &mut Vec<String>) {
    out.push(at.clone());
    match value {
        Value::Object(map) => map.iter().for_each(|(k, v)| pointers(v, format!("{at}/{k}"), out)),
        Value::Array(items) => {
            items.iter().enumerate().for_each(|(i, v)| pointers(v, format!("{at}/{i}"), out))
        }
        _ => {}
    }
}

fn edited(rng: &mut Rng) -> String {
    let mut value: Value = serde_json::from_str(EDIT).unwrap();
    for _ in 0..1 + rng.below(3) {
        let mut slots = Vec::new();
        pointers(&value, String::new(), &mut slots);
        let pointer = slots[rng.below(slots.len())].clone();
        let Ok(new) = serde_json::from_str::<Value>(VALUES[rng.below(VALUES.len())]) else { continue };
        let slot = value.pointer_mut(&pointer).unwrap();
        *slot = match rng.below(3) {
            0 if slot.is_array() => {
                Value::Array(slot.as_array().unwrap().iter().cycle().take(5000).cloned().collect())
            }
            _ => new,
        };
    }
    value.to_string()
}

#[test]
fn a_damaged_picture_edit_is_refused_or_saved_back_the_same() {
    let (mut rng, mut kept) = (Rng(0x0ed1_7f11_e000_0001), 0);
    for round in 0..600 {
        let changed =
            if round % 4 == 0 { damaged(&mut rng, EDIT, |_| "1e999".into()) } else { edited(&mut rng) };
        kept += capped(format!("edit round {round}:\n{changed}"), move || {
            let Ok(edit) = Edit::parse(&changed) else { return 0 };
            let again = Edit::parse(&edit.to_json()).unwrap_or_else(|e| panic!("{e}\n{}", edit.to_json()));
            assert_eq!(again, edit);
            1
        });
    }
    assert!(kept > 30, "only {kept} damaged edits were read");
}
