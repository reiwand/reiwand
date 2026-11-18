use crate::Outcome;
#[cfg(feature = "client")]
use crate::download;
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
    run_options(data, vault, false)
}

#[cfg(not(feature = "client"))]
fn fetched(_: &Path, _: &mut dyn crate::Vault, _: bool) -> anyhow::Result<()> {
    Ok(())
}

#[cfg(feature = "client")]
fn fetched(data: &Path, vault: &mut dyn crate::Vault, skip: bool) -> anyhow::Result<()> {
    let (done, pending) = (data.join(download::DONE), data.join(download::PENDING));
    if skip {
        let _ = fs::remove_file(&pending);
        return Ok(());
    }
    if done.exists() || !data.join("sync.json").exists() {
        return Ok(());
    }
    match download::run(data, vault) {
        Ok(report) => {
            if let Some(report) = report {
                println!(
                    "downloaded from the old server: {} lists, {} favourites, {} repositories, {} private files: {}",
                    report.lists,
                    report.favourites,
                    report.repositories,
                    report.files,
                    report.file.map(|f| f.display().to_string()).unwrap_or_default()
                );
            }
            fs::write(&done, "")?;
            let _ = fs::remove_file(&pending);
            Ok(())
        }
        Err(e) => {
            fs::write(&pending, format!("{e:#}\n"))?;
            anyhow::bail!(
                "the last download of the synced data failed ({e:#}); nothing was changed and the daemon stays stopped. \
                 Run reiwand-upgrade client again when the old server answers, or run reiwand-upgrade client --skip-download \
                 to go on without what only the server had"
            )
        }
    }
}

pub fn run_options(data: &Path, vault: &mut dyn crate::Vault, skip: bool) -> anyhow::Result<Outcome> {
    if !data.is_dir() {
        return Ok(Outcome::Nothing);
    }
    crate::phone::settings(data)?;
    let found = match fs::read_to_string(data.join(MARK)) {
        Ok(text) => text.trim().parse::<i64>().context("the version mark is damaged")?,
        Err(_) => 1,
    };
    if found >= CURRENT {
        return Ok(Outcome::Nothing);
    }
    fetched(data, vault, skip)?;
    for item in fs::read_dir(data.join("repos")).into_iter().flatten().flatten() {
        plain(&item.path().join("state.json"))?;
    }
    crate::apply(data, vault)?;
    if data.join("sync.json").exists() {
        vault.drop_item("server")?;
        fs::remove_file(data.join("sync.json"))?;
    }
    let _ = fs::remove_file(data.join("sync-downloaded"));
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
