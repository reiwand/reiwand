use crate::show::{short, table};
use crate::told::{fill, told, tr};
use reiwand_core::model::{Away, AwayReply};

const WHERE: &str = told("“%1” comes from your other computers, but its file is not on this computer.");
const SEND: &str = told("On the computer that has it, use “Send files to my other computers”.");
const REPOSITORY: &str = told(
    "“%1”, synced from your other computers, comes from the repository “%2”, which is not added on this computer.",
);

fn name(away: &Away) -> String {
    if away.title.is_empty() { tr(told("Untitled wallpaper")) } else { away.title.clone() }
}

pub fn note(title: &str) -> String {
    format!("{} {}", fill(WHERE, &[&title]), tr(SEND))
}

fn line(away: &Away) -> String {
    match &away.repo {
        Some(repo) => fill(REPOSITORY, &[&name(away), repo]),
        None => note(&name(away)),
    }
}

pub fn notes(away: &[Away]) -> String {
    away.iter().map(|a| format!("\n{}\n", line(a))).collect()
}

pub fn listing(reply: AwayReply) -> String {
    let join = |list: Vec<String>| if list.is_empty() { "-".to_owned() } else { list.join(",") };
    let rows: Vec<Vec<String>> = reply
        .items
        .iter()
        .map(|a| {
            let playlists = a.playlists.iter().map(|p| short(p)).collect();
            let screens = a.profiles.iter().map(|p| p.output.clone()).collect();
            let repo = a.repo.clone().unwrap_or_else(|| "-".to_owned());
            vec![short(&a.key), name(a), repo, join(playlists), join(screens)]
        })
        .collect();
    let empty = rows.is_empty();
    let text = table(&["RECORD", "TITLE", "REPOSITORY", "PLAYLISTS", "SCREENS"], rows);
    if empty { text } else { text + "\n" + &tr(SEND) + "\n" }
}
