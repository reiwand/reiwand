mod authors;
mod carry;
mod enroll;
mod fetching;
mod glue;
mod publish;
mod signin;
mod store;
#[cfg(test)]
mod tests;
mod token;
mod using;
mod votes;

use crate::keyring::Keyring;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use zeroize::Zeroizing;

pub use glue::{block_on, hostname, random, start, sync_error};

#[derive(Default)]
pub struct Book {
    gate: Mutex<()>,
    tokens: Mutex<HashMap<String, Zeroizing<String>>>,
    keyring: OnceLock<Keyring>,
}
