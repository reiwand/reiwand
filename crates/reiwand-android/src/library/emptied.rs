use crate::{Error, settings};
use reiwand_core::config::OutputConfig;
use reiwand_library::Library;
use serde_json::{Value, json};

/// The lists that hold any of these items, before they leave the library.
pub(super) fn lists_of(lib: &Library, uids: &[&str]) -> Vec<String> {
    let mut lists: Vec<String> =
        uids.iter().flat_map(|uid| lib.item_playlists(uid).unwrap_or_default()).map(|p| p.uid).collect();
    lists.sort();
    lists.dedup();
    lists
}

/// A list that was left empty stops being the list of any screen, so no screen points at a list
/// that has nothing to show.
pub(super) fn release(lib: &Library, lists: &[String]) -> Result<(), Error> {
    let now = super::now();
    let empty: Vec<&String> =
        lists.iter().filter(|uid| lib.playlist_items(uid, now).is_ok_and(|items| items.is_empty())).collect();
    let used = |o: &OutputConfig| o.playlist.as_ref().is_some_and(|uid| empty.contains(&uid));
    let config = settings::config();
    if !used(&config.default) && !config.outputs.values().any(used) {
        return Ok(());
    }
    settings::update(|config| {
        let sections = std::iter::once(&mut config.default).chain(config.outputs.values_mut());
        sections.filter(|o| used(o)).for_each(|o| o.playlist = None);
        config.outputs.retain(|_, o| *o != Default::default());
        Ok(())
    })
}

/// What removing an item takes along: whether a screen shows it (on its own or as the only item of
/// its list) and the presets of the library that need it as their base.
pub(crate) fn impact(lib: &Library, uid: &str) -> Result<Value, Error> {
    let item = lib.item(uid)?;
    let now = super::now();
    let alone =
        &|list: &String| lib.playlist_items(list, now).is_ok_and(|items| items.iter().all(|i| i.uid == uid));
    let shows =
        |o: &OutputConfig| o.image.as_ref() == Some(&item.path) || o.playlist.as_ref().is_some_and(alone);
    let config = settings::config();
    let in_use = shows(&config.default) || config.outputs.values().any(shows);
    let own = item.package.as_ref().filter(|p| p.kind != "preset").map(|p| p.id.clone());
    let presets = own.map(|id| lib.variants(&id)).transpose()?.unwrap_or_default();
    let presets: Vec<String> = presets.into_iter().filter_map(|p| p.package.map(|p| p.id)).collect();
    Ok(json!({ "in_use": in_use, "presets": presets }))
}
