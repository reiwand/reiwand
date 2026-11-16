use crate::told::{told, tr};
use anyhow::Result;
use clap::{Args, ValueEnum};
use reiwand_client::Client;

#[derive(Args, Debug)]
pub struct Private {
    #[arg(
        value_enum,
        default_value = "status",
        help = "on cuts all network, off gives it back, status tells which"
    )]
    pub state: State,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum State {
    On,
    Off,
    Status,
}

pub async fn run(client: &Client, args: Private) -> Result<()> {
    let on = match args.state {
        State::On | State::Off => {
            let on = matches!(args.state, State::On);
            client.daemon.set_private_mode(on).await?;
            on
        }
        State::Status => client.daemon.private_mode().await?,
    };
    let said = if on { told("private mode: on, the network is cut") } else { told("private mode: off") };
    println!("{}", tr(said));
    Ok(())
}
