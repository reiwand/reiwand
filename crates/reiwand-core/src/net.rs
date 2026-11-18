#[cfg(feature = "http")]
mod gate;

#[cfg(feature = "http")]
pub use gate::{blocking, client};
use serde::{Deserialize, Serialize};
use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicU8, Ordering::Relaxed};
use std::time::Duration;

pub const REFUSED: &str = "private mode is on: the network is cut (turn it off with `reiwand-ctl private off` or in the app's settings)";
pub const KEY: &str = "network.private";

static MODE: AtomicU8 = AtomicU8::new(0);

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Network {
    pub private: bool,
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("{}", REFUSED)]
pub struct Refused;

pub fn set_private(on: bool) {
    MODE.store(if on { 2 } else { 1 }, Relaxed);
}

pub fn private() -> bool {
    match MODE.load(Relaxed) {
        0 => stored(),
        mode => mode == 2,
    }
}

fn stored() -> bool {
    let config = crate::Config::default_path().and_then(|p| crate::Config::load(&p));
    config.is_ok_and(|c| c.network.private)
}

pub fn open() -> Result<(), Refused> {
    match private() {
        true => Err(Refused),
        false => Ok(()),
    }
}

pub fn refused(error: &(dyn std::error::Error + 'static)) -> bool {
    std::iter::successors(Some(error), |e| e.source())
        .any(|e| e.is::<Refused>() || e.to_string().contains(REFUSED))
}

pub fn tcp(address: &str, timeout: Duration) -> io::Result<TcpStream> {
    open().map_err(io::Error::other)?;
    let mut last = io::Error::new(io::ErrorKind::NotFound, format!("{address} has no address"));
    for found in address.to_socket_addrs()? {
        match TcpStream::connect_timeout(&found, timeout) {
            Ok(stream) => return Ok(stream),
            Err(e) => last = e,
        }
    }
    Err(last)
}

#[cfg(test)]
mod tests;
