use super::Kind;
use crate::show::list;
use crate::told::{fill, told, tr};
use crate::usage;
use anyhow::{Result, bail};
use reiwand_client::{Client, error_name, parse};
use reiwand_core::dbus::error::UNCONFIRMED;
use reiwand_core::model::InputsReply;
use std::collections::BTreeSet;

pub(super) async fn grant(client: &Client, scene: &str, input: &str, output: Option<&str>) -> Result<()> {
    let reply: InputsReply = parse(&client.inputs.get_inputs().await?)?;
    let shown = reply.shown.iter().filter(|(name, s)| s.scene == scene && output.is_none_or(|o| o == *name));
    let keys: BTreeSet<&str> = shown.map(|(_, s)| s.key.as_str()).filter(|k| !k.is_empty()).collect();
    let sent = match (Vec::from_iter(keys).as_slice(), output) {
        ([key], _) => client.inputs.grant_package(scene, key, input).await,
        ([], Some(output)) => bail!("{}", fill(told("%1 does not show %2"), &[&output, &scene])),
        ([], None) => client.inputs.grant(scene, input).await,
        _ => {
            let outputs = reply.shown.iter().filter(|(_, s)| s.scene == scene).map(|(name, _)| name);
            let many = fill(
                told("more than one package with the id %1 is shown; choose one with --output"),
                &[&scene],
            );
            return Err(usage(format!("{many} ({})", list(outputs))));
        }
    };
    let done = fill(told("%1 may use %2"), &[&scene, &input]);
    settle(sent.map_err(reiwand_client::explain), done)
}

fn settle(sent: Result<()>, done: String) -> Result<()> {
    match sent {
        Err(e) if error_name(&e) == Some(UNCONFIRMED) => {
            eprintln!(
                "{}",
                tr(told(
                    "Asked in a desktop notification: the scene gets the input once you allow it there."
                ))
            );
            Ok(())
        }
        sent => {
            sent?;
            println!("{done}");
            Ok(())
        }
    }
}

pub(super) async fn decide(
    client: &Client,
    scene: &str,
    (allow, without): (&[Kind], &[Kind]),
    output: Option<&str>,
) -> Result<()> {
    let reply: InputsReply = parse(&client.inputs.get_inputs().await?)?;
    let shown = reply.shown.iter().filter(|(name, s)| s.scene == scene && output.is_none_or(|o| o == *name));
    let keys: BTreeSet<&str> = shown.map(|(_, s)| s.key.as_str()).filter(|k| !k.is_empty()).collect();
    let key = match Vec::from_iter(keys).as_slice() {
        [key] => key.to_string(),
        [] => bail!("{}", fill(told("%1 is not shown"), &[&scene])),
        _ => {
            let outputs = reply.shown.iter().filter(|(_, s)| s.scene == scene).map(|(name, _)| name);
            let many = fill(
                told("more than one package with the id %1 is shown; choose one with --output"),
                &[&scene],
            );
            return Err(usage(format!("{many} ({})", list(outputs))));
        }
    };
    let names = |kinds: &[Kind]| kinds.iter().map(|k| k.name()).collect::<Vec<_>>();
    let sent = client.inputs.decide(scene, &key, &names(allow), &names(without)).await;
    settle(sent.map_err(reiwand_client::explain), fill(told("%1 answered"), &[&scene]))
}
