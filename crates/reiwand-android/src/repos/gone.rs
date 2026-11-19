use super::{ITEMS, dir, installed};
use crate::{Error, library, store};
use serde_json::Value;

pub(crate) fn forget(uids: &[String]) -> Result<(), Error> {
    let mut items = installed();
    let gone: Vec<(String, String)> = uids.iter().filter_map(|uid| items.remove(uid)).collect();
    if gone.is_empty() {
        return Ok(());
    }
    let data = dir()?;
    store::write(&data.join(ITEMS), &serde_json::to_vec(&items)?)?;
    gone.iter().for_each(|(_, id)| drop(reiwand_format::remove(&data, id, None)));
    Ok(())
}

pub(crate) fn uninstall(repo_id: &str, id: &str) -> Result<Value, Error> {
    let found = installed().into_iter().find(|(_, (repo, name))| repo == repo_id && name == id);
    let (uid, _) = found.ok_or_else(|| reiwand_library::Error::NotFound("package", id.into()))?;
    library::with(|lib| library::drop_items(lib, &[uid]))?;
    store::changed();
    Ok(Value::Null)
}
