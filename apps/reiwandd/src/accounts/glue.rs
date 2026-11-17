use super::Book;
use crate::daemon::Daemon;
use crate::error::{Error, Result, failed};
use crate::keyring::Keyring;
use reiwand_core::dbus::error::{KEYRING_UNAVAILABLE, SIGNED_OUT};
use std::sync::Arc;
use std::time::Duration;

pub fn block_on<T>(work: impl Future<Output = T>) -> T {
    crate::outside::block_on(work)
}

pub fn sync_error(e: reiwand_account::Error) -> Error {
    match e.code() {
        "client_too_old" => failed(crate::words::too_old()),
        "unauthorized" => Error::Named(SIGNED_OUT, crate::words::signed_out()),
        _ => e.into(),
    }
}

pub fn hostname() -> String {
    let name = rustix::system::uname().nodename().to_string_lossy().into_owned();
    if name.trim().is_empty() { "Linux computer".into() } else { name }
}

pub fn random<const N: usize>() -> [u8; N] {
    let mut bytes = [0; N];
    getrandom::fill(&mut bytes).expect("the system random source failed");
    bytes
}

impl Book {
    pub fn ring(&self) -> Option<&Keyring> {
        self.keyring.get()
    }
}

pub fn start(daemon: &Arc<Daemon>, bus: zbus::Connection) {
    daemon.accounts.keyring.set(Keyring::server(bus, daemon.secret_home())).ok();
    daemon.resolve_links();
}

impl Daemon {
    pub(crate) fn server_keyring(&self) -> Result<&Keyring> {
        self.accounts.ring().ok_or_else(|| failed("the keyring is not ready yet"))
    }

    pub fn keyring_ready(&self) -> Result<()> {
        let keyring = self.server_keyring()?;
        let reached = block_on(tokio::time::timeout(Duration::from_secs(10), keyring.reachable("probe")));
        match reached {
            Ok(true) => Ok(()),
            _ => Err(Error::Named(
                KEYRING_UNAVAILABLE,
                "no keyring answers to store the access of this computer".into(),
            )),
        }
    }
}
