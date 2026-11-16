use std::collections::HashMap;
use std::sync::LazyLock;

pub const CATALOGS: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/catalogs.rs"));
static CATALOG: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| catalog(&reiwand_core::locale::languages()));

pub fn tr(text: &str) -> String {
    CATALOG.get(text).cloned().unwrap_or_else(|| {
        text.rsplit_once(reiwand_core::locale::CONTEXT_MARK).map_or(text, |(_, plain)| plain).to_owned()
    })
}

pub fn catalog(languages: &[String]) -> HashMap<String, String> {
    reiwand_core::locale::catalog(CATALOGS, languages)
}
