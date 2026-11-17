use crate::told::{told, tr};
use anyhow::{Result, bail};
use futures_util::StreamExt;
use reiwand_client::Client;

pub async fn preview(client: &Client, spec: &str, animated: bool) -> Result<String> {
    let mut ready = client.repos.receive_preview_ready().await?;
    let cached = match animated {
        true => client.repos.request_animated_previews(&[spec]).await?,
        false => client.repos.request_previews(&[spec]).await?,
    };
    if let Some(path) = cached.get(spec) {
        return Ok(path.clone());
    }
    let wanted = if animated { format!("animated:{spec}") } else { spec.to_owned() };
    while let Some(signal) = ready.next().await {
        if let Ok(args) = signal.args()
            && args.spec == wanted
        {
            return Ok(args.path.into());
        }
    }
    bail!("{}", tr(told("the connection to the session bus was closed")))
}
