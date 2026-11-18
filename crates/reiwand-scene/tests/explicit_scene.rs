use reiwand_core::explicit::{all_required, omission, read, round_trip};
use reiwand_scene::{Scene, schema};
use serde_json::Value;

fn sample() -> Value {
    let text = reiwand_upgrade::fill::whole(include_str!("newer.json"));
    serde_json::from_str(&text).unwrap()
}

#[test]
fn every_scene_property_is_required() {
    let loose = all_required(&schema());
    assert!(loose.is_empty(), "{loose:#?}");
}

#[test]
fn a_full_scene_is_refused_key_by_key_when_it_loses_one() {
    let load = |v: &Value| read::<Scene>("scene.json", v).map(|_| ());
    load(&sample()).unwrap();
    let wrong = omission(&sample(), |_| false, load);
    let untagged = |line: &&String| {
        let (path, rest) = line.split_once(": ").unwrap();
        let reported = rest.strip_prefix("reported \"").and_then(|r| r.split('"').next());
        let entry = ["assets.", "effects.", "templates.", "particles.", "pools.", "ext."]
            .iter()
            .any(|m| path.strip_prefix(m).is_some_and(|r| !r.contains(['.', '['])));
        let flat = rest.ends_with("(Missing)")
            && reported.is_some_and(|r| r.rsplit('.').next() == path.rsplit('.').next());
        let held = path.rsplit('.').nth(1).is_some_and(|m| ["text", "params", "strings", "map"].contains(&m));
        flat || held && rest == "accepted without it"
            || entry && rest == "accepted without it"
            || reported.is_some_and(|r| path.starts_with(r) && path.len() > r.len())
    };
    let wrong: Vec<&String> = wrong.iter().filter(|l| !untagged(l)).collect();
    assert!(wrong.is_empty(), "{:#?}", &wrong[..wrong.len().min(40)]);
}

#[test]
fn a_full_scene_written_again_gives_the_same_bytes() {
    let text = serde_json::to_string_pretty(&sample()).unwrap();
    let again = |t: &str| {
        let s: Scene = serde_json::from_str(t).map_err(|e| e.to_string())?;
        serde_json::to_string_pretty(&s).map_err(|e| e.to_string())
    };
    let canonical = again(&text).unwrap();
    round_trip(&canonical, again).unwrap();
}
