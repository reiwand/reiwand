#![allow(dead_code)]

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

pub const EXAMPLES: [&str; 7] = ["dusk", "lights", "orbits", "pond", "pulse", "snowfall", "solids"];

pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }
}

pub fn example(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenes").join(name)
}

pub fn files(dir: &Path) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(next) = stack.pop() {
        for entry in std::fs::read_dir(&next).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let name = path.strip_prefix(dir).unwrap().to_string_lossy().into_owned();
                out.push((name, std::fs::read(&path).unwrap()));
            }
        }
    }
    out.sort();
    out
}

pub fn capped<T: Send + 'static>(what: String, work: impl FnOnce() -> T + Send + 'static) -> T {
    let (send, receive) = mpsc::channel();
    std::thread::spawn(move || send.send(work()));
    match receive.recv_timeout(Duration::from_secs(20)) {
        Ok(value) => value,
        Err(mpsc::RecvTimeoutError::Timeout) => panic!("hung on {what}"),
        Err(mpsc::RecvTimeoutError::Disconnected) => panic!("panicked on {what}"),
    }
}

pub fn bytes(rng: &mut Rng, data: &[u8]) -> Vec<u8> {
    let mut out = data.to_vec();
    if out.is_empty() {
        return out;
    }
    let at = rng.below(out.len());
    match rng.below(6) {
        0 => out[at] ^= 1 << rng.below(8),
        1 => out[at] = [0, 0xff, 0x7f, 0x80, 0xc3][rng.below(5)],
        2 => drop(out.drain(at..(at + 1 + rng.below(16)).min(out.len()))),
        3 => {
            let piece = out[at..(at + 1 + rng.below(64)).min(out.len())].to_vec();
            out.splice(at..at, piece);
        }
        4 => out.splice(at..at, [0xc3, 0x28, 0xff, 0xfe]).for_each(drop),
        _ => out.truncate(at),
    }
    out
}

const NUMBERS: [&str; 12] = [
    "1e308",
    "-1e308",
    "-1",
    "0",
    "1e-310",
    "4294967296",
    "18446744073709551615",
    "-9223372036854775808",
    "1000000000",
    "1e-7",
    "0.5",
    "65536",
];

pub fn json(rng: &mut Rng, text: &str) -> String {
    let mut value: Value = serde_json::from_str(text).unwrap();
    let mut slots = Vec::new();
    pointers(&value, String::new(), &mut slots);
    let choice = rng.below(10);
    let wanted = |v: &Value| match choice {
        0..=3 => v.is_number(),
        5 => v.is_string(),
        7 => v.is_array(),
        _ => true,
    };
    let fitting: Vec<&String> = slots.iter().filter(|p| value.pointer(p).is_some_and(wanted)).collect();
    let pointer = fitting.get(rng.below(fitting.len())).map_or(String::new(), |p| (*p).clone());
    let slot = value.pointer_mut(&pointer).unwrap();
    let mut raw = None;
    match choice {
        0..=3 => raw = Some(NUMBERS[rng.below(NUMBERS.len())].to_string()),
        4 => {
            *slot = [Value::Null, Value::Bool(true), "".into(), Value::Array(vec![]), serde_json::json!({})]
                [rng.below(5)]
            .clone()
        }
        5 => *slot = Value::String("é".repeat(1 << 16)),
        6 => raw = Some(format!("{}0{}", "[".repeat(200), "]".repeat(200))),
        7 => {
            let items = slot.as_array().cloned().unwrap_or_default();
            *slot = Value::Array(items.iter().cycle().take(items.len() * 64).cloned().collect());
        }
        8 => *slot = Value::String(format!("{}\u{0}\u{202e}../../x", slot.as_str().unwrap_or("x"))),
        _ => return repeated(rng, &value.to_string()),
    }
    let Some(raw) = raw else { return value.to_string() };
    *value.pointer_mut(&pointer).unwrap() = Value::String("\u{1}mark\u{1}".into());
    value.to_string().replacen("\"\\u0001mark\\u0001\"", &raw, 1)
}

fn pointers(value: &Value, at: String, out: &mut Vec<String>) {
    out.push(at.clone());
    match value {
        Value::Object(map) => map
            .iter()
            .for_each(|(k, v)| pointers(v, format!("{at}/{}", k.replace('~', "~0").replace('/', "~1")), out)),
        Value::Array(items) => {
            items.iter().enumerate().for_each(|(i, v)| pointers(v, format!("{at}/{i}"), out))
        }
        _ => {}
    }
}

fn repeated(rng: &mut Rng, text: &str) -> String {
    let opens: Vec<usize> = text.match_indices("{\"").map(|(i, _)| i + 1).collect();
    let Some(&start) = opens.get(rng.below(opens.len())) else { return text.to_string() };
    let (mut depth, mut quoted, mut escaped) = (0i32, false, false);
    for (offset, c) in text[start..].char_indices() {
        match (quoted, escaped, c) {
            (true, true, _) => escaped = false,
            (true, false, '\\') => escaped = true,
            (true, false, '"') => quoted = false,
            (true, false, _) => {}
            (false, _, '"') => quoted = true,
            (false, _, '{' | '[') => depth += 1,
            (false, _, '}' | ']') if depth > 0 => depth -= 1,
            (false, _, ',' | '}') if depth == 0 => {
                let member = &text[start..start + offset];
                return format!("{}{member},{}", &text[..start], &text[start..]);
            }
            _ => {}
        }
    }
    text.to_string()
}
