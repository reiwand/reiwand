use crate::{Error, store, vault};
use reiwand_account::File;
use std::sync::Mutex;
use zeroize::Zeroizing;

const FILE: &str = "accounts.json";

static GATE: Mutex<()> = Mutex::new(());

fn ring(id: &str) -> String {
    format!("account {id}")
}

pub(crate) fn file() -> File {
    store::dir().ok().and_then(|dir| File::read(&dir.join(FILE)).ok()).unwrap_or_default()
}

pub(crate) fn edit<T>(change: impl FnOnce(&mut File) -> Result<T, Error>) -> Result<T, Error> {
    let _gate = GATE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut file = file();
    let done = change(&mut file)?;
    store::write(&store::dir()?.join(FILE), &file.text()?)?;
    Ok(done)
}

pub(crate) fn tokens() -> std::collections::BTreeMap<String, Zeroizing<String>> {
    let kept = vault::kept();
    let found = file().accounts.into_iter().filter_map(|r| {
        let token = kept.get(&ring(&r.id))?.clone();
        Some((r.id, Zeroizing::new(token)))
    });
    found.collect()
}

pub(crate) fn keep_token(id: &str, token: &str) -> Result<(), Error> {
    vault::keep(&ring(id), token)
}

pub(crate) fn drop_token(id: &str) -> Result<(), Error> {
    vault::keep(&ring(id), "")
}

pub(crate) fn random_id() -> Result<String, Error> {
    let mut bytes = [0u8; 5];
    getrandom::fill(&mut bytes).map_err(|e| Error::Setting(e.to_string()))?;
    Ok(format!("a{}", reiwand_account::hex(&bytes)))
}
