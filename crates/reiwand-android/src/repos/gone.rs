use super::{ITEMS, dir, installed};
use crate::{Error, library, store};
use serde_json::{Value, json};
use std::path::Path;

fn unregister(data: &Path, repo_id: &str, id: &str) -> Result<(), Error> {
    match reiwand_format::remove_from(data, repo_id, id) {
        Ok(_) | Err(reiwand_format::Error::NotFound(_)) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub(crate) fn forget(uids: &[String]) -> Result<(), Error> {
    let mut items = installed();
    let gone: Vec<(String, String)> = uids.iter().filter_map(|uid| items.remove(uid)).collect();
    if gone.is_empty() {
        return Ok(());
    }
    let data = dir()?;
    gone.iter().try_for_each(|(repo, id)| unregister(&data, repo, id))?;
    store::write(&data.join(ITEMS), &serde_json::to_vec(&items)?)?;
    Ok(())
}

pub(crate) fn uninstall(repo_id: &str, id: &str) -> Result<Value, Error> {
    let data = dir()?;
    let uid = installed().into_iter().find(|(_, (repo, name))| repo == repo_id && name == id).map(|(u, _)| u);
    let recorded = reiwand_format::installed(&data)?.iter().any(|i| i.repo_id == repo_id && i.id == id);
    if uid.is_none() && !recorded {
        return Err(reiwand_library::Error::NotFound("package", id.into()).into());
    }
    if let Some(uid) = uid {
        library::with(|lib| library::drop_items(lib, std::slice::from_ref(&uid)))?;
        forget(&[uid])?;
    }
    unregister(&data, repo_id, id)?;
    store::changed();
    Ok(Value::Null)
}

pub(crate) fn impact(repo_id: &str, id: &str) -> Result<Value, Error> {
    let uid = installed().into_iter().find(|(_, (repo, name))| repo == repo_id && name == id).map(|(u, _)| u);
    let Some(uid) = uid else { return Ok(json!({ "in_use": false, "presets": [] })) };
    library::with(|lib| library::impact(lib, &uid))
}
