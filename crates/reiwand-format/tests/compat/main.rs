#[path = "../fixture/mod.rs"]
mod fixture;

use fixture::{DAWN, DUSK, NOW, Repo};
use reiwand_format::{Entry, FileFetch, KINDS_FORMAT, Kind, Shard, add_repo, browse};
use std::fs;

mod listing;

#[test]
fn every_kind_is_listed_in_one_format() {
    let repo = Repo::new();
    repo.add_v2(DUSK, "2.0.0");
    repo.add_page(DAWN, "1.0.0");
    repo.publish(NOW);
    let entry: Entry = serde_json::from_slice(&fs::read(repo.dir.join("entry/1.json")).unwrap()).unwrap();
    let read =
        |b: &reiwand_format::Blob| fs::read(repo.dir.join(format!("index/{}.json", b.sha256))).unwrap();
    let every = entry.shards.iter().map(|b| Shard::parse(&read(b)).unwrap());
    assert!(every.clone().all(|s| s.format == KINDS_FORMAT));
    assert_eq!(every.map(|s| s.packages.len()).sum::<usize>(), 2);
    assert!(!repo.dir.join("details").exists());
    add_repo(&repo.data, &FileFetch, &repo.url(), NOW).unwrap();
    let mut found: Vec<_> =
        browse(&repo.data, "").unwrap().into_iter().map(|f| (f.id, f.release.kind)).collect();
    found.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(found, [(DAWN.to_string(), Kind::Web), (DUSK.to_string(), Kind::Image)]);
}
