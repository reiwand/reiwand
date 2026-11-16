use super::*;

#[test]
fn releases_from_later_publishers_still_parse() {
    let release = |extra: &str| {
        format!(
            r#"{{"version":"1.0.0","kind":"image","title":"Sky","description":"","tags":[],"license":"CC0-1.0",
            "authors":[],"size":1,"sha256":"{0}","manifest_sha256":"{0}","preview":{{"sha256":"{0}","size":1,"ext":"png","width":64}},
            "published":"2027-01-15T08:00:00Z","schema":"reiwand.wpak/2"{extra}}}"#,
            "b".repeat(64)
        )
    };
    let later = release(
        r#","content":{"rating":"questionable","descriptors":["drugs"],"note":"x"},
        "requires":{"engine":">=9","features":["meshes"],"gpu":"compute"},"listed":true"#,
    );
    let shard = format!(
        r#"{{"format":"{KINDS_FORMAT}","repo_id":"{}","shard":0,"packages":[{{"id":"{DAWN}","author":null,"versions":[{later}]}}]}}"#,
        "a".repeat(64)
    );
    let shard: Shard = serde_json::from_str(&shard).unwrap();
    shard.check(KINDS_FORMAT, &"a".repeat(64), 0, 0).unwrap();
    let release = &shard.packages[0].versions[0];
    let content = release.content.as_ref().unwrap();
    assert!(content.mature() && content.descriptors == ["drugs"]);
    assert_eq!(release.requires.as_ref().unwrap().features, ["meshes"]);
}

#[test]
fn clients_skip_releases_of_kinds_they_do_not_know() {
    let release = |version: &str, kind: &str| {
        format!(
            r#"{{"version":"{version}","kind":"{kind}","title":"Sky","description":"","tags":[],"license":"CC0-1.0",
            "authors":[],"size":1,"sha256":"{0}","manifest_sha256":"{0}","preview":null,"published":"2027-01-15T08:00:00Z",
            "schema":"reiwand.wpak/2","requires":{{"features":["hologram"]}}}}"#,
            "b".repeat(64)
        )
    };
    let listing = |id: &str, versions: &[String]| {
        format!(r#"{{"id":"{id}","author":null,"versions":[{}]}}"#, versions.join(","))
    };
    let packages = [
        listing(DAWN, &[release("1.0.0", "image"), release("2.0.0", "hologram")]),
        listing(DUSK, &[release("1.0.0", "hologram")]),
    ];
    let shard = format!(
        r#"{{"format":"{KINDS_FORMAT}","repo_id":"{}","shard":0,"packages":[{}]}}"#,
        "a".repeat(64),
        packages.join(",")
    );
    assert!(serde_json::from_str::<Shard>(&shard).is_err());
    let read = Shard::parse(shard.as_bytes()).unwrap();
    read.check(KINDS_FORMAT, &"a".repeat(64), 0, 0).unwrap();
    let ids: Vec<(&str, usize)> = read.packages.iter().map(|p| (p.id.as_str(), p.versions.len())).collect();
    assert_eq!(ids, [(DAWN, 1)]);
    let empty = shard.replace(&listing(DUSK, &[release("1.0.0", "hologram")]), &listing(DUSK, &[]));
    assert!(Shard::parse(empty.as_bytes()).unwrap().check(KINDS_FORMAT, &"a".repeat(64), 0, 0).is_err());
}
