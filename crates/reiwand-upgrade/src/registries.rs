use crate::sink::Sink;
use serde_json::Value;
use std::path::Path;

pub fn plain(path: &Path, sink: &mut Sink) -> anyhow::Result<bool> {
    let Ok(bytes) = std::fs::read(path) else { return Ok(false) };
    let mut state: Value = serde_json::from_slice(&bytes)?;
    let old = state["url"].as_str().unwrap_or_default().to_string();
    if !bare(&mut state["url"], true) {
        return Ok(false);
    }
    let (from, to) = (old.split('?').next().unwrap_or_default(), state["url"].as_str().unwrap_or_default());
    let (from, to) = (from.to_string(), to.to_string());
    for base in state["bases"].as_array_mut().into_iter().flatten() {
        if let Some(rest) = base.as_str().and_then(|b| b.strip_prefix(&from)) {
            *base = format!("{to}{rest}").into();
        }
    }
    sink.save(path, &serde_json::to_vec(&state)?)?;
    Ok(true)
}

fn bare(url: &mut Value, server: bool) -> bool {
    let Some(text) = url.as_str().filter(|u| u.contains('?')) else { return false };
    let mut plain = text.split('?').next().unwrap_or(text).to_string();
    if server && plain.starts_with("http") && plain.trim_end_matches('/').ends_with("/repo") {
        plain = format!("{}/r/main/", plain.trim_end_matches('/').trim_end_matches("/repo"));
    }
    *url = plain.into();
    true
}
