use crate::daemon::job;
use crate::input::path_arg;
use crate::providers::patient;
use crate::server::Server;
use anyhow::Result;
use futures_util::TryFutureExt;
use reiwand_client::Client;

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
        Server::Packages { json } => {
            let found: serde_json::Value = serde_json::from_str(&patient(sync.list_packages()).await?)?;
            if json {
                println!("{found}");
            }
            for v in found["items"].as_array().into_iter().flatten().filter(|_| !json) {
                let text = |k: &str| v[k].as_str().unwrap_or("-").to_owned();
                println!("{} {} {} {}", text("package"), text("version"), text("state"), text("visibility"));
            }
        }
        Server::Uploads { json, cancel } => crate::keyed::uploads(client, json, cancel).await?,
        Server::AuthorKey(command) => crate::keyed::run(client, command).await?,
        Server::Visibility { id, visibility, version } => {
            patient(sync.set_visibility(&id, version.as_deref().unwrap_or(""), &visibility)).await?
        }
    }
    Ok(())
}
