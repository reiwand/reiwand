use crate::{Error, Limits, Manifest};
use reiwand_core::explicit::{self, read};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    pub items: Vec<Item>,
    #[serde(deserialize_with = "explicit::null")]
    pub next: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub media: Media,
    pub url: String,
    #[serde(deserialize_with = "explicit::null")]
    pub page: Option<String>,
    #[serde(deserialize_with = "explicit::null")]
    pub author: Option<String>,
    #[serde(deserialize_with = "explicit::null")]
    pub license: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Media {
    Image,
    Video,
}

fn short(value: &str, max: usize) -> bool {
    value.chars().count() <= max && !value.chars().any(char::is_control)
}

impl Page {
    pub fn read(manifest: &Manifest, value: &Value, limits: &Limits) -> Result<Page, Error> {
        let page: Page = read("list()", value).map_err(|e| Error::Page(e.said().to_string()))?;
        let wrong = |path: String, why: &str| Err(Error::Page(format!("\"{path}\" {why}")));
        if page.items.len() > limits.items {
            return wrong("items".into(), &format!("holds more than {} items", limits.items));
        }
        if page.next.as_ref().is_some_and(|n| n.is_empty() || !short(n, 2048)) {
            return wrong("next".into(), "is not a cursor of at most 2048 characters");
        }
        for (i, item) in page.items.iter().enumerate() {
            let at = |field: &str| format!("items[{i}].{field}");
            if item.id.is_empty() || !short(&item.id, 256) || page.items[..i].iter().any(|o| o.id == item.id)
            {
                return wrong(at("id"), "is not a unique text of at most 256 characters");
            }
            if !short(&item.title, 512) {
                return wrong(at("title"), "is longer than 512 characters");
            }
            if !crate::net::declared(manifest, &item.url) {
                return wrong(at("url"), "is not an https address on a host the plugin declares");
            }
            if item.page.as_ref().is_some_and(|p| !p.starts_with("https://") || !short(p, 2048)) {
                return wrong(at("page"), "is not an https address");
            }
            for (field, text) in [("author", &item.author), ("license", &item.license)] {
                if text.as_ref().is_some_and(|t| !short(t, 256)) {
                    return wrong(at(field), "is longer than 256 characters");
                }
            }
        }
        Ok(page)
    }
}
