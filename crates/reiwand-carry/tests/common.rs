#![allow(dead_code)]
use reiwand_carry::*;
use serde_json::json;
use std::path::Path;

pub const COST: Cost = Cost { memory_kib: 64, passes: 1 };

pub fn item(id: &str) -> Item {
    Item { repo_id: Some("r1".into()), id: id.into() }
}

pub fn content() -> Content {
    Content {
        repositories: vec![Repository {
            address: "https://example.org/r/one".into(),
            repo_id: "r1".into(),
            origin: "https://example.org".into(),
            name: "One".into(),
        }],
        providers: vec![Provider {
            kind: "photos".into(),
            name: "Gallery".into(),
            config: json!({"query": "sea"}),
        }],
        lists: vec![List { name: "Calm".into(), items: vec![item("a"), item("b")] }],
        favourites: vec![item("a")],
        ratings: vec![Rating { item: item("a"), rating: 5 }],
        tags: vec![Tagging { item: item("b"), tags: vec!["blue".into()] }],
        installed: vec![Installed { repo_id: "r1".into(), id: "a".into(), version: "1.2.0".into() }],
        settings: [("playback.fps".to_string(), json!(30)), ("playback.volume".to_string(), json!(0.5))]
            .into(),
        ..Content::default()
    }
}

pub fn export(content: Content, own: Vec<(OwnFile, std::path::PathBuf)>, password: Option<&str>) -> Export {
    Export {
        program: "reiwand".into(),
        version: "0.1.0".into(),
        created: "2026-11-16T00:00:00Z".into(),
        content,
        own,
        password: password.map(String::from),
        cost: COST,
    }
}

pub fn temp(dir: &Path, name: &str) -> std::path::PathBuf {
    dir.join(name)
}
