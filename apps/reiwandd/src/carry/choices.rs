use reiwand_carry::{ListMode, Pick};
use serde::Deserialize;

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

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Choices {
    pub take: Option<Vec<String>>,
    pub lists: Option<String>,
    pub suffix: Option<String>,
}

impl Choices {
    pub fn parse(json: &str) -> Result<Choices, String> {
        let choices: Choices = if json.trim().is_empty() {
            Choices::default()
        } else {
            serde_json::from_str(json).map_err(|e| format!("the choices are not valid: {e}"))?
        };
        for name in choices.take.iter().flatten() {
            if !SECTIONS.contains(&name.as_str()) {
                return Err(format!("{name} is not a section; the sections are {}", SECTIONS.join(", ")));
            }
        }
        match choices.lists.as_deref() {
            None | Some("join" | "both") => Ok(choices),
            Some(other) => Err(format!("lists is join or both, not {other}")),
        }
    }

    pub fn pick(&self) -> Pick {
        let mode = if self.lists.as_deref() == Some("both") { ListMode::Both } else { ListMode::Join };
        let mut pick = Pick::all(mode, self.suffix.as_deref().unwrap_or(" (imported)"));
        if let Some(take) = &self.take {
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
        pick
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choices_name_sections_and_a_list_mode() {
        let choices = Choices::parse(r#"{"take":["lists","own"],"lists":"both"}"#).unwrap();
        let pick = choices.pick();
        assert!(pick.lists && pick.own && !pick.settings && !pick.secrets);
        assert_eq!(pick.list_mode, ListMode::Both);
        assert!(Choices::parse(r#"{"take":["nothing"]}"#).is_err());
        assert!(Choices::parse(r#"{"lists":"x"}"#).is_err());
        assert!(Choices::parse("").unwrap().pick().secrets);
    }
}
