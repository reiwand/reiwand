use crate::args::Wait;
use crate::carrying_show::{show_preview, show_report};
use crate::daemon::finished;
use crate::told::{told, tr};
use anyhow::{Context, Result, ensure};
use clap::{Args, Subcommand};
use futures_util::TryFutureExt;
use reiwand_client::{Client, explain};
use serde_json::{Value, json};
use std::io::{IsTerminal, Read};
use std::path::{Path, PathBuf};

const MAGIC: &[u8] = b"REIWANDE";

#[derive(Args, Debug)]
pub struct PasswordFile {
    #[arg(
        long,
        value_name = "FILE",
        help = "Read the password of the file from FILE instead of asking on the terminal"
    )]
    pub password_file: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Carrying {
    #[command(
        about = "Write the configuration of this computer to one file that import brings to another",
        long_about = "Write the configuration of this computer to one file that import brings to another: added repositories, photo providers, lists, favourites, ratings, tags, installed wallpapers (as references, downloaded again on import) and the settings that belong to a person rather than to a screen.\n\nWith --secrets the file also carries the keys of the photo providers and is encrypted with a password that is asked for twice on the terminal, or read from --password-file."
    )]
    Export {
        #[arg(help = "File to write; it must not exist")]
        file: PathBuf,
        #[arg(long, help = "Also carry the files of your own wallpapers (imported with a copy)")]
        own: bool,
        #[arg(
            long,
            help = "Also carry the keys of the photo providers; the file is encrypted with a password"
        )]
        secrets: bool,
        #[command(flatten)]
        password: PasswordFile,
        #[arg(long, help = "Print the finished job as JSON")]
        json: bool,
    },
    #[command(
        about = "Show what an exported file holds, or add what you choose from it; nothing is replaced",
        long_about = "Show what an exported file holds, or add what you choose from it. Importing only adds: a repository, provider, favourite or tag you already have is skipped, and a rating is set only on an item without one. Without --take every section is added."
    )]
    Import {
        #[arg(help = "File written by export")]
        file: PathBuf,
        #[arg(long, help = "Only show what the file holds and what is new here; change nothing")]
        list: bool,
        #[arg(long, value_name = "SECTION", help = SECTIONS_HELP)]
        take: Vec<String>,
        #[arg(
            long,
            value_parser = ["join", "both"],
            default_value = "join",
            help = "A list with a name you already have: join adds the missing items to yours, both keeps the imported one beside it"
        )]
        lists: String,
        #[arg(
            long,
            help = "Text added to the name of an imported list kept beside yours; by default ' (imported)'"
        )]
        suffix: Option<String>,
        #[command(flatten)]
        password: PasswordFile,
        #[arg(long, help = "Print the result as JSON")]
        json: bool,
    },
}

const SECTIONS_HELP: &str = "Section to add (repeat it): repositories, providers, lists, favourites, ratings, tags, installed, settings, own, secrets";

fn encrypted(file: &Path) -> Result<bool> {
    let mut head = [0u8; 8];
    let mut open = std::fs::File::open(file).with_context(|| file.display().to_string())?;
    Ok(open.read_exact(&mut head).is_ok() && head == MAGIC)
}

fn password(given: &PasswordFile, twice: bool) -> Result<String> {
    if let Some(path) = &given.password_file {
        let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
        return Ok(text.trim_end_matches(['\n', '\r']).to_owned());
    }
    ensure!(
        std::io::stdin().is_terminal(),
        "{}",
        tr(told("a password is needed and there is no terminal to ask for it: use --password-file"))
    );
    let first = rpassword::prompt_password(tr(told("Password of the file: ")))?;
    ensure!(
        !twice || rpassword::prompt_password(tr(told("Repeat the password: ")))? == first,
        "{}",
        tr(told("the passwords differ"))
    );
    Ok(first)
}

fn absolute(file: &Path) -> Result<String> {
    Ok(std::path::absolute(file)?.to_string_lossy().into_owned())
}

pub async fn run(client: &Client, command: Carrying) -> Result<()> {
    let carry = &client.carry;
    match command {
        Carrying::Export { file, own, secrets, password: given, json } => {
            let secret = if secrets || given.password_file.is_some() {
                password(&given, true)?
            } else {
                String::new()
            };
            ensure!(!secrets || !secret.is_empty(), "{}", tr(told("the password is empty")));
            let options = json!({"own": own, "secrets": secrets, "password": secret}).to_string();
            let path = absolute(&file)?;
            let wait = Wait { wait: true, json };
            let start = carry.export(&path, &options).map_ok(|id| ((), id));
            finished(client, start, &wait).await?;
            Ok(())
        }
        Carrying::Import { file, list, take, lists, suffix, password: given, json } => {
            let secret = if encrypted(&file)? { password(&given, false)? } else { String::new() };
            let path = absolute(&file)?;
            if list {
                let found: Value =
                    serde_json::from_str(&carry.preview_import(&path, &secret).await.map_err(explain)?)?;
                return show_preview(&found, json);
            }
            let suffix = suffix.unwrap_or_else(|| tr(" (imported)"));
            let take = (!take.is_empty()).then_some(take);
            let choices = json!({"take": take, "lists": lists, "suffix": suffix}).to_string();
            let wait = Wait { wait: true, json };
            let start = carry.import_chosen(&path, &secret, &choices).map_ok(|id| ((), id));
            let (_, job) = finished(client, start, &wait).await?;
            match job.and_then(|job| job.output).filter(|_| !json) {
                Some(output) => show_report(&output),
                None => Ok(()),
            }
        }
    }
}
