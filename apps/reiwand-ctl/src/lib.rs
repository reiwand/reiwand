mod about;
mod accounts;
mod acting;
mod address;
mod applying;
mod args;
pub mod author;
mod author_words;
mod authoring;
mod bench;
mod benching;
mod blanks;
mod browsing;
mod calls;
mod calls_accounts;
mod calls_carry;
mod capturing;
mod carrying;
mod carrying_show;
mod catalog;
mod caution;
pub mod cli;
mod confirming;
mod connecting;
mod consent;
mod console;
mod copying;
mod cropping;
mod daemon;
mod depth;
pub mod edit;
mod editing;
mod facts;
mod filtering;
#[cfg(test)]
mod fixture;
mod folder;
mod foreign;
mod gpus;
mod hearing;
mod held;
pub mod input;
pub mod inputs;
pub mod items;
mod keyed;
pub mod keys;
mod levels;
pub mod library;
pub mod licenses;
mod looking;
pub mod media;
pub mod models;
pub mod moderate;
pub mod network;
pub mod pack;
mod passing;
mod paths;
pub mod pkg;
pub mod pkg_import;
pub mod pkg_preview;
pub mod playlists;
pub mod presets;
mod presetting;
pub mod profiles;
pub mod props;
pub mod propsets;
pub mod providers;
mod quiet;
pub mod refusing;
mod regioning;
pub mod remote;
mod rendering;
pub mod repo;
mod rgb;
mod scoped;
mod scriptfaults;
pub mod server;
pub mod serving;
pub mod shelving;
pub mod show;
mod sourcing;
pub mod spanning;
mod stacking;
mod steps;
mod stillness;
pub mod storing;
mod texturing;
pub mod told;
mod touching;
mod unkept;
mod usage;
mod wallclock;
pub mod wallhaven;
mod wallhaven_filters;
mod watching;
mod webshot;
mod wording;

pub use calls::CALLS;
pub use calls_accounts::ACCOUNT_CALLS;
pub use calls_carry::CARRY_CALLS;
pub use cli::{Cli, Command};
pub use usage::{Usage, usage};

use reiwand_client::{error_name, explain};
use std::process::ExitCode;

fn quiet_on_closed_pipe() {
    let shown = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let text = info.payload_as_str().unwrap_or_default();
        if text.starts_with("failed printing to stdout") && text.contains("Broken pipe") {
            std::process::exit(0);
        }
        shown(info);
    }));
}

pub fn main() -> ExitCode {
    quiet::libraries();
    reiwand_render::serve_script_host();
    quiet_on_closed_pipe();
    reiwand_scene::serve_check();
    reiwand_scene::check_out_of_process(true);
    reiwand_core::stopped_effects().inspect(|p| reiwand_scene::keep_strikes(p));
    let result = match told::parse().command {
        Command::Pack(command) => pack::run(command),
        Command::ConnectCode(command) => connecting::run(command),
        Command::Scene(command) => scoped::run(command),
        Command::Edit(command) => edit::run(command),
        Command::Models(command) => models::run(command),
        Command::Repo(command) => blocking(repo::run(command)),
        Command::Moderate(command) => blocking(moderate::run(command)),
        Command::Licenses => licenses::run(),
        command => blocking(daemon::run(command)),
    };
    reiwand_scene::end_trials();
    let Err(error) = result else {
        reiwand_render::finish_gpu(0);
        return ExitCode::SUCCESS;
    };
    let error = error.downcast::<zbus::Error>().map_or_else(|e| e, explain);
    eprintln!("Error: {}", show::clean(&foreign::said(&reiwand_editor::told(&error))));
    let code = if misused(&error) { 2 } else { 1 };
    reiwand_render::finish_gpu(i32::from(code));
    ExitCode::from(code)
}

fn blocking<T>(work: impl Future<Output = anyhow::Result<T>>) -> anyhow::Result<T> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(anyhow::Error::from)
        .and_then(|runtime| runtime.block_on(work))
}

fn misused(error: &anyhow::Error) -> bool {
    error.is::<Usage>() || error_name(error) == Some("org.freedesktop.DBus.Error.InvalidArgs")
}
