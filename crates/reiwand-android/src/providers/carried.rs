use super::kinds::{Options, kind, secret_of};
use super::options::effective;
use super::saved::{self, Entry, held, load, save};
use crate::{Error, said};
use std::collections::BTreeSet;

pub(crate) struct Listed {
    pub(crate) id: String,
    pub(crate) kind: String,
    pub(crate) options: Options,
}

pub(crate) fn all() -> Result<Vec<Listed>, Error> {
    let entry = |e: Entry| Listed { id: e.id, kind: e.kind, options: e.options };
    Ok(load()?.into_iter().map(entry).collect())
}

fn wants_secret(kind_name: &str) -> bool {
    kind(kind_name).ok().as_ref().and_then(secret_of).is_some()
}

pub(crate) fn secret_ids() -> Result<BTreeSet<String>, Error> {
    Ok(load()?.into_iter().filter(|e| wants_secret(&e.kind)).map(|e| e.id).collect())
}

pub(crate) fn keys_held() -> Result<BTreeSet<String>, Error> {
    Ok(secret_ids()?.into_iter().filter(|id| saved::secret(id).is_some()).collect())
}

pub(crate) fn secrets() -> Result<Vec<(String, String)>, Error> {
    Ok(secret_ids()?.into_iter().filter_map(|id| Some((id.clone(), saved::secret(&id)?))).collect())
}

fn plain_name(hint: &str) -> bool {
    !hint.is_empty()
        && hint.len() <= 40
        && hint.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

pub(crate) fn import(hint: &str, kind_name: &str, given: &Options) -> Result<String, Error> {
    let known = kind(kind_name)?;
    let options = effective(&known, given)?;
    let _held = held();
    let mut entries = load()?;
    let taken = |id: &str| entries.iter().any(|e| e.id == id);
    let id =
        if plain_name(hint) && !taken(hint) { hint.to_owned() } else { super::fresh_id(&entries, kind_name) };
    entries.push(Entry::new(id.clone(), kind_name.to_owned(), options, false));
    save(&entries)?;
    Ok(id)
}

pub(crate) fn keep_secret(id: &str, secret: &str) -> Result<(), Error> {
    let _held = held();
    if !load()?.iter().any(|e| e.id == id && wants_secret(&e.kind)) {
        return Err(said("android.provider_missing", "there is no picture source %1", &[&id]));
    }
    saved::keep(id, secret.trim())
}
