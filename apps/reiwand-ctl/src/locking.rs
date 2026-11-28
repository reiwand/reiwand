use crate::cli::{ITEM, JSON, Out};
use crate::input::one;
use crate::told::{fill, told, tr};
use anyhow::Result;
use clap::Subcommand;
use reiwand_client::Client;
use reiwand_core::model::{LockChoice, LockReply};

#[derive(Subcommand, Debug)]
pub enum Lock {
    #[command(about = "Show what the lock screen shows on every screen and on each one")]
    Show {
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(about = "Show a wallpaper of the library on the lock screen instead of the desktop's")]
    Set {
        #[arg(value_name = "ITEM", help = ITEM)]
        item: String,
        #[command(flatten)]
        out: Out,
    },
    #[command(about = "Play a playlist on the lock screen instead of the desktop's wallpaper")]
    Playlist {
        #[arg(value_name = "PL", help = "Playlist uid or name")]
        playlist: String,
        #[command(flatten)]
        out: Out,
    },
    #[command(
        about = "Show on the lock screen what the desktop shows; a screen without its own choice takes the one for every screen"
    )]
    Follow(Out),
    #[command(
        about = "Show the desktop on the lock screen of one screen even while every lock screen has a choice"
    )]
    Desktop(Out),
}

fn choice(choice: Option<&LockChoice>) -> String {
    match choice {
        None => tr(told("like the desktop")),
        Some(c) if c.desktop => tr(told("the desktop, whatever every screen shows")),
        Some(c) if c.playlist.is_some() => fill(told("playlist %1"), &[&c.name]),
        Some(c) => c.name.clone(),
    }
}

fn shown(reply: LockReply) -> String {
    let mut lines = vec![fill(told("every screen: %1"), &[&choice(reply.every.as_ref())])];
    lines.extend(reply.outputs.iter().map(|(name, c)| format!("{name}: {}", choice(Some(c)))));
    lines.join("\n") + "\n"
}

pub(crate) fn said(command: &Lock) -> Option<String> {
    let on = crate::confirming::on;
    Some(match command {
        Lock::Show { .. } => return None,
        Lock::Set { item, out } => fill(told("lock screen of %1: %2"), &[&on(&out.output), item]),
        Lock::Playlist { playlist, out } => {
            fill(told("lock screen of %1: playlist %2"), &[&on(&out.output), playlist])
        }
        Lock::Follow(out) => fill(told("the lock screen of %1 follows the desktop"), &[&on(&out.output)]),
        Lock::Desktop(out) => fill(told("the lock screen of %1 shows the desktop"), &[&on(&out.output)]),
    })
}

pub async fn run(client: &Client, command: Lock) -> Result<()> {
    let daemon = &client.daemon;
    match command {
        Lock::Show { json } => crate::show::reply(daemon.get_lock().await?, json, shown)?,
        Lock::Set { item, out } => daemon.set_lock_wallpaper(&one(client, &item).await?, &out.output).await?,
        Lock::Playlist { playlist, out } => daemon.set_lock_playlist(&playlist, &out.output).await?,
        Lock::Follow(out) => daemon.follow_desktop_on_lock(&out.output).await?,
        Lock::Desktop(out) => daemon.show_desktop_on_lock(&out.output).await?,
    }
    Ok(())
}
