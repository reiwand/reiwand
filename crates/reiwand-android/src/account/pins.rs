use crate::{Error, store};
use std::collections::BTreeMap;

const FILE: &str = "repo-pins.json";

pub(crate) fn all() -> BTreeMap<String, String> {
    let bytes = store::dir().ok().and_then(|dir| std::fs::read(dir.join(FILE)).ok());
    bytes.and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default()
}

pub(crate) fn pin(origin: &str, print: &str) -> Result<(), Error> {
    let mut pins = all();
    pins.insert(origin.to_owned(), print.to_ascii_lowercase());
    store::write(&store::dir()?.join(FILE), &serde_json::to_vec(&pins)?)
}
