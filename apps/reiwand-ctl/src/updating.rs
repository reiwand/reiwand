use crate::told::{told, tr};
use anyhow::{Result, bail};
use clap::{Subcommand, ValueEnum};
use reiwand_client::Client;
use reiwand_core::{Config, net};

#[derive(Subcommand, Debug)]
pub enum Update {
    #[command(
        about = "Turn the daily check for a new version of the AppImage on or off, or tell whether it is on (on by default; it asks GitHub for the latest release)"
    )]
    Auto {
        #[arg(value_enum, default_value = "status", help = "on, off, or status to tell which")]
        state: State,
    },
    #[command(
        about = "Tell whether the updater of the AppImage may contact GitHub now: exits with 0 when it may, 1 when private mode is on or, with --auto, when the daily check is off"
    )]
    Allowed {
        #[arg(long, help = "Ask for the daily check, which also needs its setting on")]
        auto: bool,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum State {
    On,
    Off,
    Status,
}

const ON: &str = told("daily check for a new version: on (it asks GitHub for the latest release)");
const OFF: &str = told("daily check for a new version: off");
const PRIVATE: &str = told("private mode is on: no check for a new version is made");
const STOPPED: &str =
    told("the daily check for a new version is off: turn it on with `reiwand-ctl update auto on`");

/// Read from the configuration file, so the updater can ask without the service running.
fn stored() -> Result<Config> {
    Ok(Config::load(&Config::default_path()?)?)
}

pub async fn run(command: Update) -> Result<()> {
    match command {
        Update::Auto { state: State::Status } => said(stored()?.auto_updates()),
        Update::Auto { state } => {
            let on = matches!(state, State::On);
            Client::connect().await?.daemon.set_option(net::UPDATES, &on.to_string()).await?;
            said(on)
        }
        Update::Allowed { auto } => {
            let config = stored()?;
            if config.network.private {
                bail!(tr(PRIVATE));
            }
            if auto && !config.auto_updates() {
                bail!(tr(STOPPED));
            }
            Ok(())
        }
    }
}

fn said(on: bool) -> Result<()> {
    println!("{}", tr(if on { ON } else { OFF }));
    if on && net::private() {
        println!("{}", tr(PRIVATE));
    }
    Ok(())
}
