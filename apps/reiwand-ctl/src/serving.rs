use crate::daemon::job;
use crate::input::path_arg;
use crate::providers::patient;
use crate::server::Server;
use crate::told::{fill, told, tr};
use crate::usage;
use anyhow::Result;
use anyhow::bail;
use futures_util::TryFutureExt;
use reiwand_client::Client;
use std::io::{self, BufRead, IsTerminal};

pub async fn run(client: &Client, command: Server) -> Result<()> {
    let sync = &client.publish;
    match command {
        Server::Upload { file, unlisted, repo, wait } => {
            let visibility = if unlisted { "unlisted" } else { "public" };
            job(
                client,
                sync.upload_as(repo.as_deref().unwrap_or(""), &path_arg(&file)?, visibility)
                    .map_ok(|id| ((), id)),
                &wait,
            )
            .await?
        }
        Server::Packages { json, at } => {
            let found: serde_json::Value =
                serde_json::from_str(&patient(sync.list_packages(at.text())).await?)?;
            if json {
                println!("{found}");
            }
            for v in found["items"].as_array().into_iter().flatten().filter(|_| !json) {
                let text = |k: &str| v[k].as_str().unwrap_or("-").to_owned();
                println!("{} {} {} {}", text("package"), text("version"), text("state"), text("visibility"));
            }
        }
        Server::Uploads { json, cancel, at } => crate::keyed::uploads(client, json, cancel, at).await?,
        Server::AuthorKey(command) => crate::keyed::run(client, command).await?,
        Server::Visibility { id, visibility, version, at } => {
            patient(sync.set_visibility(at.text(), &id, version.as_deref().unwrap_or(""), &visibility))
                .await?
        }
        Server::Withdraw { id, yes, at } => {
            if !yes {
                ask(&id)?;
            }
            patient(sync.withdraw_package(at.text(), &id)).await?;
            eprintln!("{}", fill(told("%1 is withdrawn"), &[&id]));
        }
    }
    Ok(())
}

fn ask(id: &str) -> Result<()> {
    eprintln!(
        "{}",
        fill(
            told(
                "%1 is withdrawn for good: every version is revoked and its files are deleted from the server"
            ),
            &[&id]
        )
    );
    if !io::stdin().is_terminal() {
        return Err(usage(tr(told("add --yes to withdraw it without asking"))));
    }
    eprint!("{}", tr(told("Withdraw it? [y/N] ")));
    let mut answer = String::new();
    io::stdin().lock().read_line(&mut answer)?;
    match crate::consent::agreed(&answer) {
        true => Ok(()),
        false => bail!("{}", tr(told("nothing was withdrawn"))),
    }
}
