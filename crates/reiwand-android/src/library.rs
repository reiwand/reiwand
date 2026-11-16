use crate::choice::{self, Choice};
use crate::said;
use crate::{Error, framing, pack, settings, store};
use chrono::Utc;
use reiwand_library::{Library, PlaylistInfo, Rules};
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::{Mutex, PoisonError};

mod asked;
mod labels;
mod lists;

pub(crate) use asked::request;

pub(crate) use lists::{drop_playlist, remove as drop_items};
use lists::{listed, named};

const DATABASE: &str = "library.db";

static LIBRARY: Mutex<Option<Library>> = Mutex::new(None);
static LANGUAGES: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub(crate) fn languages() -> Vec<String> {
    LANGUAGES.lock().unwrap_or_else(PoisonError::into_inner).clone()
}

fn heard(languages: &Option<Vec<String>>) {
    if let Some(languages) = languages.as_ref().filter(|l| !l.is_empty()) {
        *LANGUAGES.lock().unwrap_or_else(PoisonError::into_inner) = languages.clone();
    }
}

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum Request {
    Items { query: String, languages: Option<Vec<String>> },
    Count { query: String },
    Item { uid: String, languages: Option<Vec<String>> },
    Confirm { uid: String, what: String },
    Unconfirm { uid: String, what: String },
    ClearData { uid: String },
    Cleared,
    Thumb { uid: String },
    Favorite { uids: Vec<String>, favorite: bool },
    Remove { uids: Vec<String> },
    Playlists,
    Entries { playlist: String },
    Create { name: String },
    Save { playlist: PlaylistInfo },
    Delete { playlist: String },
    Add { playlist: String, items: Vec<String> },
    Sound { uids: Vec<String>, muted: Option<String>, volume: Option<String> },
    Heard { path: String },
    Rotate { name: String, items: Vec<String>, every: String },
    Take { playlist: String, items: Vec<String> },
    Move { playlist: String, from: usize, to: usize },
    Pick { choice: Option<Choice> },
    Unpick { choice: Choice },
    Current,
}

pub(crate) fn hidden() -> bool {
    settings::config().library.hide_mature
}

pub(crate) fn now() -> i64 {
    Utc::now().timestamp()
}

pub(crate) fn with<T>(work: impl FnOnce(&mut Library) -> Result<T, Error>) -> Result<T, Error> {
    let mut library = LIBRARY.lock().unwrap_or_else(PoisonError::into_inner);
    match library.as_mut() {
        Some(library) => work(library),
        None => work(library.insert(Library::open(store::dir()?.join(DATABASE))?)),
    }
}

fn answer(lib: &mut Library, request: Request, on_battery: bool) -> Result<Value, Error> {
    let (now, shown) = (now(), settings::config().resolve(""));
    let reads = matches!(
        request,
        Request::Items { .. }
            | Request::Count { .. }
            | Request::Item { .. }
            | Request::Thumb { .. }
            | Request::Playlists
            | Request::Entries { .. }
            | Request::Current
            | Request::Cleared
            | Request::Heard { .. }
    );
    let value = match request {
        Request::Items { query, languages } => {
            heard(&languages);
            let query = reiwand_library::shown(&query, hidden());
            let mut items = lib.search(&query, now, 0, u32::MAX)?;
            lib.localize(&mut items, &languages.unwrap_or_default())?;
            listed(items)?
        }
        Request::Count { query } => json!(lib.count(&reiwand_library::shown(&query, hidden()), now)?),
        Request::Item { uid, languages } => {
            heard(&languages);
            let (mut items, languages) = (vec![lib.item(&uid)?], languages.unwrap_or_default());
            lib.localize(&mut items, &languages)?;
            let about = items[0].package.as_ref().and_then(|_| pack::about(&items[0].path, &languages));
            let animated = about.as_ref().and_then(|_| pack::animated(&uid, &items[0].path));
            let about = about.map(|about| reiwand_core::model::About { animated, ..about });
            let lacks = items[0].package.as_ref().map(|p| pack::lacks(&items[0].path, &p.facets));
            let mut value = listed(items)?[0].take();
            (value["about"], value["lacks"]) = (json!(about), json!(lacks.unwrap_or_default()));
            value
        }
        Request::Confirm { uid, what } => {
            lib.confirm(&[&uid], &what)?;
            store::changed();
            Value::Null
        }
        Request::Unconfirm { uid, what } => {
            lib.unconfirm(&[&uid], &what)?;
            store::changed();
            Value::Null
        }
        Request::ClearData { uid } => {
            let package = lib.item(&uid)?.package;
            package.filter(|p| p.kind == "web").map_or(Ok(()), |p| crate::page::clear(&p.id))?;
            Value::Null
        }
        Request::Cleared => crate::page::cleared()?,
        Request::Thumb { uid } => json!(store::thumb(lib, &lib.item(&uid)?)?),
        Request::Favorite { uids, favorite } => json!(lib.set_favorite_many(&known(lib, &uids), favorite)?),
        Request::Remove { uids } => lists::remove(lib, &uids, now)?,
        Request::Playlists => lists::playlists(lib, now)?,
        Request::Entries { playlist } => listed(named(lib, lib.playlist_items(&playlist, now)?)?)?,
        Request::Create { name } => json!(lib.create_playlist(&name, None, &Rules::default())?),
        Request::Save { playlist } => json!(lib.update_playlist(&playlist)?),
        Request::Delete { playlist } => {
            drop_playlist(lib, &playlist)?;
            Value::Null
        }
        Request::Add { playlist, items } => lists::add(lib, &playlist, &items)?,
        Request::Sound { uids, muted, volume } => lists::sound(lib, &uids, (muted, volume))?,
        Request::Heard { path } => lists::heard(lib, &path)?,
        Request::Rotate { name, items, every } => lists::rotate(lib, &name, &items, &every)?,
        Request::Take { playlist, items } => {
            json!(lib.remove_from_playlist_many(&playlist, &known(lib, &items))?)
        }
        Request::Move { playlist, from, to } => json!(lib.move_in_playlist(&playlist, from, to)?),
        Request::Pick { choice } => {
            if let Some(Choice::Item(uid)) = &choice
                && !settings::playback().allow_flashing
                && lib.item(uid)?.package.is_some_and(|p| p.photosensitive == Some(true))
            {
                return Err(said("android.flashing_disabled", "this wallpaper flashes and is disabled", &[]));
            }
            json!(choice::pick(choice))
        }
        Request::Unpick { choice } => {
            if let Choice::Item(uid) = &choice {
                framing::discard(uid);
            }
            json!(choice::unpick(&choice))
        }
        Request::Current => {
            let image = shown.image.and_then(|path| lib.resolve_item(path.to_str()?).ok());
            json!({ "image": image, "playlist": shown.playlist, "on": choice::on(lib) })
        }
    };
    if !reads {
        choice::retime(lib, on_battery);
        store::changed();
    }
    Ok(value)
}

pub(crate) fn known<'a>(lib: &Library, uids: &'a [String]) -> Vec<&'a str> {
    uids.iter().map(String::as_str).filter(|uid| lib.item(uid).is_ok()).collect()
}
