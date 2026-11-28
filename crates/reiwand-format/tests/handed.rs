mod fixture;

use fixture::{DAWN, NOW, Repo, key, signer};
use reiwand_format::{
    Error, FileFetch, Reason, add, add_repo, anchors, install, installed, publish, register_author, remove,
    revoke_key, rotate_root, settings, sign_root, update_repo,
};
use std::collections::BTreeMap;

fn reassign(repo: &Repo, author: &str) {
    let path = repo.dir.join("catalog.json");
    let mut catalog: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    catalog["packages"][0]["author"] = author.into();
    std::fs::write(&path, serde_json::to_vec_pretty(&catalog).unwrap()).unwrap();
}

#[test]
fn uninstalling_a_package_forgets_the_key_it_was_anchored_to() {
    let repo = Repo::with(|_, s| (s.online_authors, s.authors) = (true, BTreeMap::new()));
    let ana = repo.author.clone();
    register_author(&repo.dir, "ana", &ana.public).unwrap();
    add(&repo.dir, &repo.package(DAWN, "1.0.0", Some(&ana)), None, NOW).unwrap();
    repo.publish(NOW);
    let info = add_repo(&repo.data, &FileFetch, &repo.url(), NOW).unwrap();
    install(&repo.data, &FileFetch, DAWN, NOW, false, false).unwrap();
    assert_eq!(anchors(&repo.data).unwrap().len(), 1);

    let next = key();
    revoke_key(&repo.dir, &ana.fingerprint, Reason::Other).unwrap();
    register_author(&repo.dir, "ana", &next.public).unwrap();
    add(&repo.dir, &repo.package(DAWN, "1.1.0", Some(&next)), None, NOW).unwrap();
    publish(&repo.dir, &signer(&repo.online), 7, NOW + 60).unwrap();
    update_repo(&repo.data, &FileFetch, &info.repo_id, NOW + 120).unwrap();
    let refused = install(&repo.data, &FileFetch, DAWN, NOW + 120, false, false);
    assert!(matches!(refused, Err(Error::Signature(_))), "{refused:?}");

    remove(&repo.data, DAWN, None).unwrap();
    assert!(anchors(&repo.data).unwrap().is_empty(), "the anchor outlived the package");
    let again = install(&repo.data, &FileFetch, DAWN, NOW + 120, false, false).unwrap();
    assert_eq!(again.signer.as_deref(), Some(next.fingerprint.as_str()));
    assert_eq!(anchors(&repo.data).unwrap()[0].fingerprint, next.fingerprint);
}

#[test]
fn another_author_of_the_root_cannot_take_a_package_whose_author_changed_keys() {
    let other = key();
    let repo = Repo::with(|_, s| drop(s.authors.insert("other".into(), vec![other.public.clone()])));
    repo.add(DAWN, "1.0.0");
    repo.publish(NOW);
    let info = add_repo(&repo.data, &FileFetch, &repo.url(), NOW).unwrap();
    let first = install(&repo.data, &FileFetch, DAWN, NOW, false, false).unwrap();
    assert_eq!(first.author.as_deref(), Some("studio"));
    assert_eq!(installed(&repo.data).unwrap()[0].author.as_deref(), Some("studio"));

    let mut next = settings(&repo.dir).unwrap();
    next.authors.insert("studio".into(), vec![key().public]);
    rotate_root(&repo.dir, &next, NOW + 10).unwrap();
    sign_root(&repo.dir, &signer(&repo.root)).unwrap();
    reassign(&repo, "other");
    add(&repo.dir, &repo.package(DAWN, "1.1.0", Some(&other)), Some("other"), NOW + 20).unwrap();
    publish(&repo.dir, &signer(&repo.online), 7, NOW + 30).unwrap();
    update_repo(&repo.data, &FileFetch, &info.repo_id, NOW + 40).unwrap();
    let refused = install(&repo.data, &FileFetch, DAWN, NOW + 40, false, false);
    assert!(matches!(&refused, Err(Error::Signature(m)) if m.contains("another author")), "{refused:?}");
    assert!(!repo.data.join("packages").join(DAWN).join("1.1.0.wpak").exists());
}
