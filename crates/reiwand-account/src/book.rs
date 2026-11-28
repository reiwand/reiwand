use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub url: String,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    pub repo_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Rec {
    pub id: String,
    pub origin: String,
    pub user: String,
    pub user_id: i64,
    pub label: String,
    pub pending: bool,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub accounts: Vec<Rec>,
}

impl File {
    pub fn read(path: &std::path::Path) -> Result<File, reiwand_core::stored::StoredError> {
        Ok(reiwand_core::stored::read(path)?.unwrap_or_default())
    }

    pub fn text(&self) -> serde_json::Result<Vec<u8>> {
        reiwand_core::stored::text(self)
    }
}

pub fn clean(url: &str) -> String {
    let url = url.split(['?', '#']).next().unwrap_or_default();
    format!("{}/", url.trim_end_matches('/'))
}

pub fn origin(url: &str) -> Option<String> {
    let url = reqwest::Url::parse(url).ok()?;
    matches!(url.scheme(), "http" | "https").then(|| url.origin().ascii_serialization())
}
