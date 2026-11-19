use crate::Outcome;
#[cfg(feature = "client")]
use crate::download;
use crate::sink::Sink;
use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};

pub const CURRENT: i64 = reiwand_format::DATA_VERSION as i64;
use crate::steps::{SETTLED, steps};

const MARK: &str = "format-version";

pub struct Options {
    pub skip_download: bool,
    pub check: bool,
    pub also: Vec<PathBuf>,
    pub copies: Option<PathBuf>,
}

pub struct Done {
    pub outcome: Outcome,
    pub changed: Vec<PathBuf>,
    pub unsigned: Vec<String>,
    pub skipped: Vec<(PathBuf, String)>,
    pub copies: PathBuf,
}

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
    let options = Options { skip_download: skip, check: false, also: Vec::new(), copies: None };
    Ok(convert(data, vault, &options)?.outcome)
}

pub fn copies_of(data: &Path) -> PathBuf {
    data.join("upgrade-copias").join(std::time::UNIX_EPOCH.elapsed().map_or(0, |d| d.as_secs()).to_string())
}

pub fn convert(data: &Path, vault: &mut dyn crate::Vault, options: &Options) -> anyhow::Result<Done> {
    let copies = options.copies.clone().unwrap_or_else(|| copies_of(data));
    let mut sink = if options.check { Sink::checking() } else { Sink::copying(&copies) };
    if !data.is_dir() {
        return Ok(nothing(copies));
    }
    let found = match fs::read_to_string(data.join(MARK)) {
        Ok(text) if text.trim().is_empty() => 1,
        Ok(text) => text.trim().parse::<i64>().context("the version mark is damaged")?,
        Err(_) => 1,
    };
    if found == CURRENT {
        return Ok(nothing(copies));
    }
    if !options.check && found < SETTLED {
        fetched(data, vault, options.skip_download)?;
    }
    if let Err(e) = steps(data, &mut sink, &options.also, found) {
        let lost = sink.unsigned().join(", ");
        return Err(match lost.is_empty() {
            true => e,
            false => e.context(format!("packages already rewritten lost their signature: {lost}")),
        });
    }
    if found < SETTLED && data.join("sync.json").exists() {
        if options.check {
            sink.note(&data.join("sync.json"));
        } else {
            crate::apply(data, vault)?;
            vault.drop_item("server")?;
            fs::remove_file(data.join("sync.json"))?;
        }
    }
    if !options.check {
        let _ = fs::remove_file(data.join("sync-downloaded"));
        if !sink.skipped().is_empty() {
            let list: String =
                sink.skipped().iter().map(|(p, why)| format!("{}\t{why}\n", p.display())).collect();
            fs::create_dir_all(&copies)?;
            fs::write(copies.join("not-converted.txt"), list)?;
        }
        crate::atomic::write(&data.join(MARK), format!("{CURRENT}\n").as_bytes())?;
    }
    let outcome = Outcome::Upgraded { from: found, to: CURRENT };
    Ok(Done {
        outcome,
        changed: sink.changed().to_vec(),
        unsigned: sink.unsigned().to_vec(),
        skipped: sink.skipped().to_vec(),
        copies,
    })
}

fn nothing(copies: PathBuf) -> Done {
    Done { outcome: Outcome::Nothing, changed: Vec::new(), unsigned: Vec::new(), skipped: Vec::new(), copies }
}
