use crate::{Error, said};
use reiwand_carry::{ListMode, Pick};
use serde::Deserialize;
use serde_json::Value;

mod apply;
mod export;
mod gather;
mod libapply;
mod local;
mod preview;
mod report;
mod stage;

const SECTIONS: [&str; 10] = [
    "repositories",
    "providers",
    "lists",
    "favourites",
    "ratings",
    "tags",
    "installed",
    "settings",
    "own",
    "secrets",
];

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub(crate) enum Request {
    #[serde(rename = "carry_preview")]
    Preview {
        path: String,
        #[serde(default)]
        password: String,
    },
    #[serde(rename = "carry_export")]
    Export {
        path: String,
        #[serde(default)]
        own: bool,
        #[serde(default)]
        secrets: bool,
        #[serde(default)]
        password: String,
    },
    #[serde(rename = "carry_import")]
    Import {
        path: String,
        #[serde(default)]
        password: String,
        #[serde(default)]
        take: Option<Vec<String>>,
        #[serde(default)]
        lists: Option<String>,
        #[serde(default)]
        suffix: Option<String>,
    },
}

pub(crate) fn request(request: Request) -> Result<Value, Error> {
    match request {
        Request::Preview { path, password } => preview::preview(&path, &password),
        Request::Export { path, own, secrets, password } => export::export(&path, own, secrets, &password),
        Request::Import { path, password, take, lists, suffix } => {
            let pick = pick(take.as_deref(), lists.as_deref(), suffix.as_deref())?;
            apply::import(&path, &password, &pick)
        }
    }
}

pub(crate) fn pick(
    take: Option<&[String]>,
    lists: Option<&str>,
    suffix: Option<&str>,
) -> Result<Pick, Error> {
    let wrong = |what: &str| said("android.carry_choice", "%1 is not a choice of the import", &[&what]);
    for name in take.into_iter().flatten() {
        if !SECTIONS.contains(&name.as_str()) {
            return Err(wrong(name));
        }
    }
    let mode = match lists {
        None | Some("join") => ListMode::Join,
        Some("both") => ListMode::Both,
        Some(other) => return Err(wrong(other)),
    };
    let mut pick = Pick::all(mode, suffix.unwrap_or(" (imported)"));
    if let Some(take) = take {
        let on = |name: &str| take.iter().any(|t| t == name);
        pick.repositories = on("repositories");
        pick.providers = on("providers");
        pick.lists = on("lists");
        pick.favourites = on("favourites");
        pick.ratings = on("ratings");
        pick.tags = on("tags");
        pick.installed = on("installed");
        pick.settings = on("settings");
        pick.own = on("own");
        pick.secrets = on("secrets");
    }
    Ok(pick)
}
