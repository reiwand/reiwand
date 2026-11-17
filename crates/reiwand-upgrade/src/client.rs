use crate::Outcome;
use anyhow::Context;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub const CURRENT: i64 = 2;

const MARK: &str = "format-version";

pub fn run(data: &Path) -> anyhow::Result<Outcome> {
    run_with(data, &mut crate::NoVault)
}

pub fn run_with(data: &Path, vault: &mut dyn crate::Vault) -> anyhow::Result<Outcome> {
    if !data.is_dir() {
        return Ok(Outcome::Nothing);
    }
    let found = match fs::read_to_string(data.join(MARK)) {
        Ok(text) => text.trim().parse::<i64>().context("the version mark is damaged")?,
        Err(_) => 1,
    };
    if found >= CURRENT {
        return Ok(Outcome::Nothing);
    }
    for item in fs::read_dir(data.join("repos")).into_iter().flatten().flatten() {
        plain(&item.path().join("state.json"))?;
    }
    crate::apply(data, vault)?;
    account(&data.join("sync.json"))?;
    fs::write(data.join(MARK), format!("{CURRENT}\n"))?;
    Ok(Outcome::Upgraded { from: found, to: CURRENT })
}

fn plain(path: &Path) -> anyhow::Result<()> {
    let Ok(bytes) = fs::read(path) else { return Ok(()) };
    let mut state: Value = serde_json::from_slice(&bytes)?;
    let old = state["url"].as_str().unwrap_or_default().to_string();
    if !bare(&mut state["url"], true) {
        return Ok(());
    }
    let (from, to) = (old.split('?').next().unwrap_or_default(), state["url"].as_str().unwrap_or_default());
    let (from, to) = (from.to_string(), to.to_string());
    for base in state["bases"].as_array_mut().into_iter().flatten() {
        if let Some(rest) = base.as_str().and_then(|b| b.strip_prefix(&from)) {
            *base = format!("{to}{rest}").into();
        }
    }
    replace(path, &state)
}

fn account(path: &Path) -> anyhow::Result<()> {
    let Ok(bytes) = fs::read(path) else { return Ok(()) };
    let mut state: Value = serde_json::from_slice(&bytes)?;
    let mut changed = bare(&mut state["repo_url"], true);
    for offer in state["offers"].as_array_mut().into_iter().flatten() {
        changed |= bare(&mut offer["url"], false);
    }
    if changed { replace(path, &state) } else { Ok(()) }
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

fn replace(path: &Path, state: &Value) -> anyhow::Result<()> {
    let staged = path.with_extension("json.upgrading");
    fs::write(&staged, serde_json::to_vec(state)?)?;
    Ok(fs::rename(staged, path)?)
}
