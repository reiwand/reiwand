use anyhow::{Context, bail};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

const HELP: &str = "reiwand-upgrade: one-time upgrade of stored Reiwand data from 1.x

usage:
  reiwand-upgrade server --data DIR [--then PROGRAM [ARGS...]]
      server data folder (database, repositories, keys); with --then, runs the program afterwards
  reiwand-upgrade client [--data DIR] [--config DIR] [--skip-download] [--then PROGRAM [ARGS...]]
      desktop or phone data folder (followed repositories); default: the reiwand folder of the user data directory.
      first downloads, from the old server, the lists, favourites, repository offers and private files that only it
      held, into from-server.reiwand (bring it in with reiwand-ctl import); if the server does not answer, nothing is
      changed and the run is repeated later; --skip-download goes on without it. Upgrade the desktop before the server

      --config DIR is the folder with config.toml (default: the reiwand folder of the user config directory): it is
      rewritten with every setting written out and the first-start words, and the original is kept in upgrade-copias

  reiwand-upgrade edits DIR...
      converts the image edits that sit beside the pictures (name.ext.reiwand.json) under each DIR; the ones kept in
      the data folder are converted by `client`

  reiwand-upgrade scenes DIR
      writes into every scene.json under DIR each value the scene format leaves out, with the value the
      engine used for it until now, so the scene reads exactly as before; packed .wpak files are unpacked,
      completed and packed again by their author

  reiwand-upgrade effects FILE...
      writes into each effect descriptor (or descriptor catalog) every field it leaves out, with the value
      applied until now, so each parameter and pass reads exactly as before

It is safe to run again: a folder that is already current is left alone.";

fn user_data() -> anyhow::Result<PathBuf> {
    let base = match std::env::var_os("XDG_DATA_HOME").filter(|v| !v.is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(std::env::var_os("HOME").context("HOME is not set")?).join(".local/share"),
    };
    Ok(base.join("reiwand"))
}

fn user_config() -> anyhow::Result<PathBuf> {
    let base = match std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(std::env::var_os("HOME").context("HOME is not set")?).join(".config"),
    };
    Ok(base.join("reiwand"))
}

fn settings(args: &[String], data: &std::path::Path) -> anyhow::Result<()> {
    let given = args.iter().position(|a| a == "--config").and_then(|at| args.get(at + 1));
    let dir = given.map_or_else(user_config, |dir| Ok(PathBuf::from(dir)))?;
    let copies = copies_of(data);
    if reiwand_upgrade::settings::run(&dir, &copies)? {
        println!("{}: config.toml converted, the original is in {}", dir.display(), copies.display());
    }
    let changed = reiwand_upgrade::edits::folder(&data.join("edits"), &copies)?;
    if changed > 0 {
        println!("{changed} image edits converted, the originals are in {}", copies.display());
    }
    let changed = reiwand_upgrade::stored::files(data, &copies)?;
    if changed > 0 {
        println!(
            "{changed} stored files converted (permissions, accounts), the originals are in {}",
            copies.display()
        );
    }
    if reiwand_upgrade::library::run(data, &copies)? {
        println!(
            "library.db converted to schema {}, the original is in {}",
            reiwand_upgrade::library::SCHEMA,
            copies.display()
        );
    }
    Ok(())
}

fn copies_of(data: &std::path::Path) -> PathBuf {
    data.join("upgrade-copias").join(std::time::UNIX_EPOCH.elapsed().map_or(0, |d| d.as_secs()).to_string())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let what = args.first().map(String::as_str);
    if what == Some("scenes") {
        let dir = args.get(1).context("scenes needs a folder")?;
        println!("{dir}: {} scenes upgraded", reiwand_upgrade::scenes::run(std::path::Path::new(dir))?);
        return Ok(());
    }
    if what == Some("edits") {
        let data = user_data()?;
        let copies = copies_of(&data);
        let mut changed = 0;
        for dir in &args[1..] {
            changed += reiwand_upgrade::edits::sidecars(std::path::Path::new(dir), &copies)?;
        }
        println!("{changed} image edits upgraded, the originals are in {}", copies.display());
        return Ok(());
    }
    if what == Some("effects") {
        let mut changed = 0;
        for file in &args[1..] {
            changed += usize::from(reiwand_upgrade::scenes::effect(std::path::Path::new(file))?);
        }
        println!("{changed} effect descriptors upgraded");
        return Ok(());
    }
    let given = args.iter().position(|a| a == "--data").and_then(|at| args.get(at + 1));
    let data = match (what, given) {
        (Some("client"), None) => user_data()?,
        (_, Some(dir)) => PathBuf::from(dir),
        _ => {
            println!("{HELP}");
            return Ok(());
        }
    };
    let done = match what {
        Some("server") => reiwand_upgrade::server::run(&data)?,
        Some("client") => {
            settings(&args, &data)?;
            let mut vault = reiwand_upgrade::vault::KeyringVault::open(&data)?;
            reiwand_upgrade::client::run_options(
                &data,
                &mut vault,
                args.iter().any(|a| a == "--skip-download"),
            )?
        }
        other => bail!("unknown target {other:?}: use server or client"),
    };
    println!("{}: {done}", data.display());
    if let Some(at) = args.iter().position(|a| a == "--then") {
        let program = args.get(at + 1).context("--then needs a program")?;
        let failed = Command::new(program).args(&args[at + 2..]).exec();
        bail!("cannot run {program}: {failed}");
    }
    Ok(())
}
