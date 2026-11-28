mod bundle;
#[cfg(not(target_os = "android"))]
mod child;
mod files;
mod guard;
mod install;
mod limits;
mod manifest;
mod modules;
mod net;
mod page;
mod plugin;
mod prefill;
mod provider;
mod runner;
mod setting;
mod strip;

pub use bundle::{Bundle, MAIN};
#[cfg(not(target_os = "android"))]
pub use child::{HOST_ARG, serve, serve_if_asked};
pub use files::Files;
pub use guard::public;
pub use install::{Installed, install, remove, step};
pub use limits::Limits;
pub use manifest::{FILE, Kind, Manifest, SCHEMA, Secret, Setting};
pub use net::{Answer, Fault, Net, Request, declared};
pub use page::{Item, Media, Page};
pub use plugin::{Isolation, Plugin, parse};
pub use prefill::{OPTIONAL, TABLE as PREFILL, schema};
pub use provider::{Reach, build};
pub use runner::{Call, Fetcher, run};
pub use strip::strip;

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("{0}")]
    Format(String),
    #[error("{file}:{line}:{column}: {message}")]
    Strip { file: String, line: usize, column: usize, message: String },
    #[error("{0}")]
    Setting(String),
    #[error("the plugin used more than {0:?} of processor time in one call")]
    Cpu(Duration),
    #[error("the plugin took more than {0:?} in one call")]
    Time(Duration),
    #[error("the plugin used more than its memory limit")]
    Memory,
    #[error("refused: {0}")]
    Refused(String),
    #[error("limit reached: {0}")]
    Limit(String),
    #[error("the plugin failed: {0}")]
    Script(String),
    #[error("the plugin returned a page that is not valid: {0}")]
    Page(String),
    #[error("the plugin host failed: {0}")]
    Host(String),
    #[error("i/o error: {0}")]
    Io(String),
}
