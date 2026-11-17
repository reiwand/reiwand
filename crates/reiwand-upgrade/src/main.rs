use anyhow::{Context, bail};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

const HELP: &str = "reiwand-upgrade: one-time upgrade of stored Reiwand data from 1.x

usage:
  reiwand-upgrade server --data DIR [--then PROGRAM [ARGS...]]
      server data folder (database, repositories, keys); with --then, runs the program afterwards
  reiwand-upgrade client [--data DIR] [--then PROGRAM [ARGS...]]
      desktop or phone data folder (followed repositories); default: the reiwand folder of the user data directory

It is safe to run again: a folder that is already current is left alone.";

fn user_data() -> anyhow::Result<PathBuf> {
    let base = match std::env::var_os("XDG_DATA_HOME").filter(|v| !v.is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(std::env::var_os("HOME").context("HOME is not set")?).join(".local/share"),
    };
    Ok(base.join("reiwand"))
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let what = args.first().map(String::as_str);
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
            let mut vault = reiwand_upgrade::vault::KeyringVault::open(&data)?;
            reiwand_upgrade::client::run_with(&data, &mut vault)?
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
