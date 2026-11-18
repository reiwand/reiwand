use crate::{Error, store};
use std::collections::BTreeMap;

const FILE: &str = "repo-pins.json";

pub(crate) fn all() -> BTreeMap<String, String> {
    let found = store::dir().ok().and_then(|dir| reiwand_core::stored::read(&dir.join(FILE)).ok());
    found.flatten().unwrap_or_default()
}

pub(crate) fn pin(origin: &str, print: &str) -> Result<(), Error> {
    let mut pins = all();
    pins.insert(origin.to_owned(), print.to_ascii_lowercase());
    store::write(&store::dir()?.join(FILE), &reiwand_core::stored::text(&pins)?)
}
