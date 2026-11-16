mod fixture;

use fixture::{NOW, Repo};
use reiwand_format::{Error, FileFetch, add_repo};

#[test]
fn an_address_with_a_fingerprint_is_refused_and_says_to_use_the_plain_one() {
    let repo = Repo::with(|_, _| {});
    repo.publish(NOW);
    let old = format!("{}?fingerprint={}", repo.url(), repo.root.fingerprint);
    let refused = add_repo(&repo.data, &FileFetch, &old, NOW).unwrap_err();
    assert!(matches!(refused, Error::Invalid { .. }), "{refused}");
    assert!(refused.to_string().contains("plain address"), "{refused}");
    assert!(add_repo(&repo.data, &FileFetch, &repo.url(), NOW).is_ok());
}
