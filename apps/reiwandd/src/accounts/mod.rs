mod carry;
mod fetching;
mod signin;
mod store;
#[cfg(test)]
mod tests;
mod token;
mod using;

use std::collections::HashMap;
use std::sync::Mutex;
use zeroize::Zeroizing;

#[derive(Default)]
pub struct Book {
    gate: Mutex<()>,
    tokens: Mutex<HashMap<String, Zeroizing<String>>>,
}
