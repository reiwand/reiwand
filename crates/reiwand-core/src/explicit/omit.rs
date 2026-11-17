use super::error::{Fault, ReadError};
use serde_json::Value;

fn collect(value: &Value, at: &str, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                let path = if at.is_empty() { key.clone() } else { format!("{at}.{key}") };
                out.push(path.clone());
                collect(child, &path, out);
            }
        }
        Value::Array(items) => {
            for (i, child) in items.iter().enumerate() {
                collect(child, &format!("{at}[{i}]"), out);
            }
        }
        _ => {}
    }
}

fn remove(value: &mut Value, path: &[String]) {
    let Some((first, rest)) = path.split_first() else { return };
    let next = match value {
        Value::Object(map) if rest.is_empty() => {
            map.remove(first);
            return;
        }
        Value::Object(map) => map.get_mut(first),
        Value::Array(items) => first.parse::<usize>().ok().and_then(|i| items.get_mut(i)),
        _ => None,
    };
    if let Some(next) = next {
        remove(next, rest);
    }
}

fn steps(path: &str) -> Vec<String> {
    let mut out = Vec::new();
    for part in path.split('.') {
        let (name, mut rest) = part.split_once('[').map_or((part, ""), |(n, r)| (n, r));
        out.push(name.to_owned());
        while let Some((index, tail)) = rest.split_once(']') {
            out.push(index.to_owned());
            rest = tail.strip_prefix('[').unwrap_or("");
        }
    }
    out
}

pub fn omission(
    sample: &Value,
    exempt: impl Fn(&str) -> bool,
    load: impl Fn(&Value) -> Result<(), ReadError>,
) -> Vec<String> {
    let mut paths = Vec::new();
    collect(sample, "", &mut paths);
    let mut wrong = Vec::new();
    for path in paths.into_iter().filter(|p| !exempt(p)) {
        let mut cut = sample.clone();
        remove(&mut cut, &steps(&path));
        match load(&cut) {
            Err(e) if e.fault == Fault::Missing && e.path == path => {}
            Err(e) => wrong.push(format!("{path}: reported \"{}\" ({:?})", e.path, e.fault)),
            Ok(()) => wrong.push(format!("{path}: accepted without it")),
        }
    }
    wrong
}

pub fn round_trip(text: &str, again: impl Fn(&str) -> Result<String, String>) -> Result<(), String> {
    let back = again(text)?;
    if back == text { Ok(()) } else { Err("writing it again gives other bytes".to_owned()) }
}
