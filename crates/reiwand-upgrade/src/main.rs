use anyhow::{Context, bail};
use reiwand_upgrade::client::{self, Options};
use reiwand_upgrade::sink::Sink;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

const HELP: &str = "reiwand-upgrade: one-time conversion of stored Reiwand data to the current version

usage:
  reiwand-upgrade client [--data DIR] [--config DIR] [--also DIR]... [--skip-download] [--check] [--then PROGRAM [ARGS...]]
      the whole user data folder (default: the reiwand folder of the user data directory): settings, image edits,
      permissions, accounts, the library database, repository registries, installed packages (rewritten, with
      their size and checksum updated), and the scenes and packages in the folders of the library; --also adds
      other folders to walk (editor projects, work folders). --config DIR is the folder with config.toml (default:
      the reiwand folder of the user config directory)
      first downloads, from the old server, what only it held, into from-server.reiwand (bring it in with
      reiwand-ctl import); if the server does not answer nothing is changed; --skip-download goes on without it

  reiwand-upgrade file PATH... [--password-file FILE] [--copies DIR] [--check]
      one scene or package folder (everything below it), .wpak package, .reiwand export file (an encrypted one
      stays encrypted: give its password with --password-file, or type it when asked), image edit
      (name.ext.reiwand.json) or effect descriptor. Authors use it on their work folders before signing again

  reiwand-upgrade server --data DIR [--check] [--then PROGRAM [ARGS...]]
      the server data folder and database; with --then, runs the program afterwards

  --check (in every command) says what would change and writes nothing. Every run keeps a copy of each file it
  changes in upgrade-copias/<time>/ (with index.txt) and can be repeated: what is already current is left alone.";

fn flag(args: &[String], name: &str) -> Option<usize> {
    args.iter().position(|a| a == name)
}

fn value(args: &[String], name: &str) -> Option<PathBuf> {
    flag(args, name).and_then(|at| args.get(at + 1)).map(PathBuf::from)
}

fn base(var: &str, home: &str, leaf: &str) -> anyhow::Result<PathBuf> {
    let dir = match std::env::var_os(var).filter(|v| !v.is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(std::env::var_os("HOME").context("HOME is not set")?).join(home),
    };
    Ok(dir.join(leaf))
}

fn then(args: &[String]) -> anyhow::Result<()> {
    if let Some(at) = flag(args, "--then") {
        let program = args.get(at + 1).context("--then needs a program")?;
        let failed = Command::new(program).args(&args[at + 2..]).exec();
        bail!("cannot run {program}: {failed}");
    }
    Ok(())
}

fn say_unsigned(unsigned: &[String], check: bool) {
    for package in unsigned {
        let verb = if check { "would lose" } else { "lost" };
        println!(
            "{verb} its signature: {package} (now local, with an unverified signature; its author can publish it again)"
        );
    }
}

fn report(sink: &Sink, check: bool) {
    let mut seen = std::collections::BTreeSet::new();
    for path in sink.changed().iter().filter(|p| seen.insert(p.as_path())) {
        println!("{} {}", if check { "would change" } else { "changed" }, path.display());
    }
    for (path, reason) in sink.skipped() {
        eprintln!("not converted {}: {reason}", path.display());
    }
    say_unsigned(sink.unsigned(), check);
    if !check && !seen.is_empty() {
        println!("the originals are in {}", sink.copies().display());
    }
}

fn clients(args: &[String]) -> anyhow::Result<()> {
    let data = value(args, "--data").map_or_else(|| base("XDG_DATA_HOME", ".local/share", "reiwand"), Ok)?;
    let config = value(args, "--config").map_or_else(|| base("XDG_CONFIG_HOME", ".config", "reiwand"), Ok)?;
    let check = flag(args, "--check").is_some();
    let copies = client::copies_of(&data);
    let mut settings = if check { Sink::checking() } else { Sink::copying(&copies) };
    reiwand_upgrade::settings::run(&config, &mut settings)?;
    report(&settings, check);
    let also = args.iter().enumerate().filter(|(_, a)| *a == "--also").filter_map(|(at, _)| args.get(at + 1));
    let options = Options {
        skip_download: flag(args, "--skip-download").is_some(),
        check,
        also: also.map(PathBuf::from).collect(),
        copies: Some(copies),
    };
    let mut vault: Box<dyn reiwand_upgrade::Vault> = match check {
        true => Box::new(reiwand_upgrade::NoVault),
        false => Box::new(reiwand_upgrade::vault::KeyringVault::open(&data)?),
    };
    let done = client::convert(&data, vault.as_mut(), &options)?;
    for (path, reason) in &done.skipped {
        eprintln!("not converted {}: {reason}", path.display());
    }
    for path in &done.changed {
        println!("{} {}", if check { "would change" } else { "changed" }, path.display());
    }
    say_unsigned(&done.unsigned, check);
    println!("{}: {}", data.display(), done.outcome);
    if !check && (!done.changed.is_empty() || !settings.changed().is_empty()) {
        println!("the originals are in {}", done.copies.display());
    }
    then(args)
}

fn files(args: &[String]) -> anyhow::Result<()> {
    let check = flag(args, "--check").is_some();
    let data = base("XDG_DATA_HOME", ".local/share", "reiwand")?;
    let copies = value(args, "--copies").unwrap_or_else(|| client::copies_of(&data));
    let mut sink = if check { Sink::checking() } else { Sink::copying(&copies) };
    let mut password = match value(args, "--password-file") {
        Some(file) => Some(std::fs::read_to_string(file)?.trim_end_matches(['\r', '\n']).to_owned()),
        None => None,
    };
    let skip: Vec<usize> = ["--password-file", "--copies"].iter().filter_map(|n| flag(args, n)).collect();
    let mut total = 0;
    for (at, path) in args.iter().enumerate().skip(1) {
        if path.starts_with("--") || skip.iter().any(|s| *s + 1 == at) {
            continue;
        }
        let path = PathBuf::from(path);
        if password.is_none() && reiwand_upgrade::carry::sealed(&path) {
            eprint!("password of {}: ", path.display());
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            password = Some(line.trim_end_matches(['\r', '\n']).to_owned());
        }
        total += reiwand_upgrade::file::convert(&path, password.as_deref(), &mut sink)?;
    }
    report(&sink, check);
    println!("{total} converted");
    Ok(())
}

fn servers(args: &[String]) -> anyhow::Result<()> {
    let data = value(args, "--data").context("server needs --data DIR")?;
    let done = reiwand_upgrade::server::run_checked(&data, flag(args, "--check").is_some())?;
    println!("{}: {done}", data.display());
    then(args)
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("client") => clients(&args),
        Some("file") => files(&args),
        Some("server") => servers(&args),
        Some(other) => bail!("unknown command {other:?}: use client, file or server"),
        None => {
            println!("{HELP}");
            Ok(())
        }
    }
}
