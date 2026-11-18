use super::*;
use serde_json::{Value, json};

fn release(version: &str, kind: &str, extra: Value) -> Value {
    let hash = "b".repeat(64);
    let mut release = json!({ "version": version, "kind": kind, "title": "Sky", "description": "", "tags": [],
        "license": "CC0-1.0", "authors": [], "size": 1, "sha256": hash, "manifest_sha256": hash,
        "preview": null, "published": "2027-01-15T08:00:00Z", "schema": "reiwand.wpak/3",
        "default_locale": null, "changes": null, "content": null, "photosensitive": null, "requires": null,
        "facets": [], "animated": null, "platforms": [], "base": null, "hidden": false, "listed": null,
        "asks": [] });
    release.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
    release
}

fn shard(packages: Vec<Value>) -> String {
    json!({ "format": KINDS_FORMAT, "repo_id": "a".repeat(64), "shard": 0, "packages": packages }).to_string()
}

fn listing(id: &str, versions: Vec<Value>) -> Value {
    json!({ "id": id, "author": null, "versions": versions })
}

#[test]
fn releases_from_later_publishers_still_parse() {
    let hash = "b".repeat(64);
    let later = release(
        "1.0.0",
        "image",
        json!({ "preview": { "sha256": hash, "size": 1, "ext": "png", "width": 64 },
            "content": { "rating": "questionable", "descriptors": ["drugs"], "note": "x" },
            "requires": { "engine": ">=9", "features": ["meshes"], "gpu": "compute", "packages": [] },
            "listed": true }),
    );
    let shard: Shard = serde_json::from_str(&shard(vec![listing(DAWN, vec![later])])).unwrap();
    shard.check(KINDS_FORMAT, &"a".repeat(64), 0, 0).unwrap();
    let release = &shard.packages[0].versions[0];
    let content = release.content.as_ref().unwrap();
    assert!(content.mature() && content.descriptors == ["drugs"]);
    assert_eq!(release.requires.as_ref().unwrap().features, ["meshes"]);
}

#[test]
fn clients_skip_releases_of_kinds_they_do_not_know() {
    let unknown = |version: &str, kind: &str| {
        release(
            version,
            kind,
            json!({ "requires": { "engine": null, "features": ["hologram"], "gpu": null, "packages": [] } }),
        )
    };
    let shard = shard(vec![
        listing(DAWN, vec![unknown("1.0.0", "image"), unknown("2.0.0", "hologram")]),
        listing(DUSK, vec![unknown("1.0.0", "hologram")]),
    ]);
    assert!(serde_json::from_str::<Shard>(&shard).is_err());
    let read = Shard::parse(shard.as_bytes()).unwrap();
    read.check(KINDS_FORMAT, &"a".repeat(64), 0, 0).unwrap();
    let ids: Vec<(&str, usize)> = read.packages.iter().map(|p| (p.id.as_str(), p.versions.len())).collect();
    assert_eq!(ids, [(DAWN, 1)]);
    let empty = shard.replace(
        &listing(DUSK, vec![unknown("1.0.0", "hologram")]).to_string(),
        &listing(DUSK, vec![]).to_string(),
    );
    assert!(Shard::parse(empty.as_bytes()).unwrap().check(KINDS_FORMAT, &"a".repeat(64), 0, 0).is_err());
}
