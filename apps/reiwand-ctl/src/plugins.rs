use crate::cli::JSON;
use crate::show::{self, list, table};
use crate::told::{fill, told, tr};
use crate::usage;
use anyhow::{Result, bail};
use clap::Subcommand;
use reiwand_client::{Client, parse};
use reiwand_core::model::{PluginOffer, PluginsReply};
use std::io::{self, BufRead, IsTerminal};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Plugin {
    #[command(
        about = "Install a provider plugin from a folder or zip, after showing what it contacts and asking",
        long_about = "Install a provider plugin from a folder or zip. It first shows the hosts the plugin \
                      contacts and the secret it asks for, and installs only after you say yes. The plugin \
                      then is the provider kind plugin:ID of reiwand-ctl provider add."
    )]
    Install {
        #[arg(help = "Folder or zip of the plugin")]
        path: PathBuf,
        #[arg(long, help = "Install without asking, for scripts that already showed what it contacts")]
        yes: bool,
    },
    #[command(about = "List the installed provider plugins")]
    Ls {
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(about = "Remove a provider plugin that no provider uses")]
    Rm {
        #[arg(help = "Id of the plugin")]
        id: String,
    },
}

pub async fn run(client: &Client, command: Plugin) -> Result<()> {
    let providers = &client.providers;
    match command {
        Plugin::Install { path, yes } => {
            let path = std::fs::canonicalize(&path).unwrap_or(path);
            let path = path.to_string_lossy();
            let offer: PluginOffer = parse(&providers.inspect_plugin(&path).await?)?;
            eprint!("{}", shown(&offer));
            if !yes {
                ask()?;
            }
            let kind = providers.install_plugin(&path, &offer.token).await?;
            eprintln!("{}", fill(told("add a provider with: reiwand-ctl provider add ID %1"), &[&kind]));
            println!("{kind}");
        }
        Plugin::Ls { json } => show::reply(providers.list_plugins().await?, json, listed)?,
        Plugin::Rm { id } => providers.remove_plugin(&id).await?,
    }
    Ok(())
}

pub fn shown(offer: &PluginOffer) -> String {
    let p = &offer.plugin;
    let mut lines = vec![fill(told("%1 %2 (%3), license %4"), &[&p.title, &p.version, &p.id, &p.license])];
    lines.push(fill(told("contacts: %1"), &[&list(&p.hosts)]));
    if let Some(before) = &offer.installed {
        let added = if offer.added_hosts.is_empty() { tr(told("none")) } else { list(&offer.added_hosts) };
        lines.push(fill(told("replaces version %1; new hosts: %2"), &[before, &added]));
        match offer.step.as_str() {
            "same" => lines.push(tr(told("warning: it is the version already installed"))),
            "older" => {
                lines.push(fill(told("warning: it is older than the installed %1: it goes back"), &[before]))
            }
            _ => {}
        }
    }
    lines.push(match &p.secret {
        Some(s) if s.required => {
            fill(told("asks for a secret: %1, sent only to %2"), &[&s.label, &list(&s.hosts)])
        }
        Some(s) => {
            fill(told("asks for an optional secret: %1, sent only to %2"), &[&s.label, &list(&s.hosts)])
        }
        None => tr(told("asks for no secret")),
    });
    if offer.secret_changed {
        lines.push(tr(told("the secret it asks for changed since the installed version")));
    }
    lines.iter().map(|l| format!("{l}\n")).collect()
}

fn ask() -> Result<()> {
    if !io::stdin().is_terminal() {
        return Err(usage(tr(told("add --yes to install it without asking"))));
    }
    eprint!("{}", tr(told("Install it? [y/N] ")));
    let mut answer = String::new();
    io::stdin().lock().read_line(&mut answer)?;
    match crate::consent::agreed(&answer) {
        true => Ok(()),
        false => bail!("{}", tr(told("nothing was installed"))),
    }
}

pub fn listed(reply: PluginsReply) -> String {
    let rows = reply.plugins.into_iter().map(|p| {
        let secret = p.secret.map_or_else(String::new, |s| s.label);
        let error = p.error.unwrap_or_default();
        vec![p.id, p.version, p.title, list(&p.hosts), secret, list(&p.sources), error]
    });
    table(&["PLUGIN", "VERSION", "TITLE", "HOSTS", "SECRET", "PROVIDERS", "ERROR"], rows)
}
