use reiwand_format::{Content, Descriptor, Kind, Limits, Manifest, Origin, Previews, Rating, SCHEMA, Text};
use reiwand_library::{Error, Library};
use rusqlite::Connection;
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

mod shared;
mod upgrades;

const NOW: i64 = 1_700_000_000;
const PNG: &[u8] = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR\0\0\0\x40\0\0\0\x24\x08\x02\0\0\0";

fn package(root: &Path, name: &str, rating: Rating, flashing: bool) {
    build(root, name, rating, flashing, name.into());
}

fn build(root: &Path, name: &str, rating: Rating, flashing: bool, title: Text) {
    let descriptors = if rating == Rating::Mature { vec![Descriptor::Violence] } else { Vec::new() };
    let manifest = Manifest {
        schema: SCHEMA.into(),
        id: format!("org.example.{name}"),
        version: "1.0.0".into(),
        default_locale: matches!(title, Text::Localized(_)).then(|| "en".into()),
        title,
        license: "CC0-1.0".into(),
        kind: Kind::Image,
        entry: "image.png".into(),
        content: Some(Content { rating, descriptors }),
        photosensitive: Some(flashing),
        origin: Some(Origin::default()),
        previews: Some(Previews { thumbnail: Some("preview.png".into()), animated: None }),
        ..Default::default()
    };
    let source = root.parent().unwrap().join(name);
    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("manifest.json"), manifest.to_json().unwrap()).unwrap();
    [("image.png", PNG), ("preview.png", PNG)]
        .iter()
        .for_each(|(f, b)| fs::write(source.join(f), b).unwrap());
    fs::create_dir_all(root).unwrap();
    reiwand_format::build(&source, root.join(format!("{name}.wpak")), &Limits::default()).unwrap();
}

#[test]
fn rated_packages_are_searchable_and_confirmed_once_per_package() {
    let dir = TempDir::new().unwrap();
    let mut library = Library::open(dir.path().join("library.db")).unwrap();
    let root = dir.path().join("pictures");
    package(&root, "storm", Rating::Mature, true);
    package(&root, "meadow", Rating::General, false);
    package(&root, "calm", Rating::General, false);
    let source = library.add_folder(&root, true).unwrap();
    library.scan(&source, NOW).unwrap();
    let count = |library: &Library, q: &str| library.count(q, NOW).unwrap();
    let counts =
        ["content:mature", "content:general", "content:unrated", "facet:photosensitive", "-content:mature"];
    assert_eq!(counts.map(|q| count(&library, q)), [1, 2, 0, 1, 2]);
    assert!(matches!(library.count("content:pg", NOW), Err(Error::Query(_))));
    let hidden = ["", "content:mature"].map(|q| count(&library, &reiwand_library::shown(q, true)));
    assert_eq!((hidden, reiwand_library::shown("sea", false)), ([2, 1], "sea".into()));
    let storm = library.search("content:mature", NOW, 0, 1).unwrap().remove(0);
    let info = storm.package.clone().unwrap();
    assert_eq!((info.content.as_deref(), info.photosensitive), (Some("mature"), Some(true)));
    assert_eq!(
        (info.facets, info.confirmed),
        (
            ["android", "desktop", "landscape", "photosensitive"].map(String::from).to_vec(),
            Vec::<String>::new()
        )
    );
    assert_eq!(library.flashing().unwrap(), std::slice::from_ref(&storm.uid));
    library.confirm(&[&storm.uid, &storm.uid], "mature").unwrap();
    let confirmed = library.item(&storm.uid).unwrap().package.unwrap().confirmed;
    assert_eq!(confirmed, ["mature"]);
    assert!(matches!(library.confirm(&[&storm.uid], "photosensitive"), Err(Error::Invalid(..))));
    assert!(matches!(library.confirm(&[&storm.uid], "spoilers"), Err(Error::Invalid(..))));
    assert!(matches!(library.confirm(&["01NOPE"], "mature"), Err(Error::NotFound(..))));
    fs::copy(root.join("storm.wpak"), root.join("storm-copy.wpak")).unwrap();
    library.scan(&source, NOW).unwrap();
    let mut both: Vec<_> =
        library.search("content:mature", NOW, 0, 5).unwrap().into_iter().map(|i| i.uid).collect();
    both.sort();
    let copy = both.iter().find(|uid| **uid != storm.uid).unwrap();
    assert_eq!(library.item(copy).unwrap().package.unwrap().confirmed, ["mature"]);
    assert_eq!(library.confirm(&[&storm.uid], "mature").unwrap(), both);
    let calm = library.search("calm", NOW, 0, 1).unwrap().remove(0).uid;
    assert_eq!(library.confirm(&[&calm, &calm], "mature").unwrap(), std::slice::from_ref(&calm));
    let id = info_id(&library, &storm.uid);
    assert!(matches!(library.confirm(&[&calm, &storm.uid], "motion"), Err(Error::Invalid("motion", _))));
    assert!(!library.decided(&id, "motion").unwrap() && !library.moves(&id).unwrap());
    let gentle = info_id(&library, &calm);
    assert!(!library.decided(&gentle, "motion").unwrap());
    library.confirm(&[&calm], "motion").unwrap();
    assert!(library.decided(&gentle, "motion").unwrap() && library.moves(&gentle).unwrap());
    assert_eq!(library.unconfirm(&[&calm], "motion").unwrap(), std::slice::from_ref(&calm));
    assert!(!library.moves(&gentle).unwrap() && library.decided(&gentle, "mature").unwrap());
    assert_eq!(library.unconfirm(&[&storm.uid], "motion").unwrap(), both);
    assert!(matches!(library.unconfirm(&["01NOPE"], "motion"), Err(Error::NotFound(..))));
}

fn info_id(library: &Library, uid: &str) -> String {
    library.item(uid).unwrap().package.unwrap().id
}
