mod common;
use common::{corpus, first_diff, same};
use reiwand_upgrade::fill::{Customs, Format, complete_with};
use reiwand_upgrade::scenes::customs;
use serde_json::Value;
use std::collections::BTreeMap;

fn kept(original: &Value, done: &Value, path: &str) -> Option<String> {
    match (original, done) {
        (Value::Object(a), Value::Object(b)) => a.iter().find_map(|(k, v)| match b.get(k) {
            Some(w) => kept(v, w, &format!("{path}/{k}")),
            None => Some(format!("{path}/{k} was dropped")),
        }),
        (Value::Array(a), Value::Array(b)) if a.len() == b.len() => {
            a.iter().zip(b).enumerate().find_map(|(i, (v, w))| kept(v, w, &format!("{path}/{i}")))
        }
        _ if same(original, done) => None,
        _ => Some(format!("{path} changed")),
    }
}

#[test]
fn scenes_keep_what_they_wrote_after_step_a() {
    let (mut read, mut changed, mut fields) = (0, 0, BTreeMap::<String, usize>::new());
    for path in corpus("scene.json") {
        let text = std::fs::read_to_string(&path).unwrap();
        let mut doc: Value = serde_json::from_str(&text).unwrap();
        reiwand_upgrade::gain::fixed(&mut doc);
        let original = doc.clone();
        let custom = customs(path.parent().unwrap(), &doc);
        let report = complete_with(Format::Scene, &mut doc, &custom);
        let again = serde_json::to_string_pretty(&doc).unwrap();
        if let Err(e) = reiwand_scene::Scene::from_json(&again) {
            panic!("{}: not readable after step A: {e}", path.display());
        }
        let lost = kept(&original, &doc, "");
        assert!(lost.is_none(), "{}: step A altered the scene: {lost:?}", path.display());
        let mut twice = doc.clone();
        assert_eq!(complete_with(Format::Scene, &mut twice, &custom).written, 0, "{}", path.display());
        read += 1;
        changed += usize::from(report.written > 0);
        report.fields.iter().for_each(|(k, n)| *fields.entry(k.clone()).or_default() += n);
    }
    println!("scenes: {read} read, {changed} completed, {} kinds of field written", fields.len());
    assert!(read >= 30, "the corpus is too small: {read}");
}

#[test]
fn manifests_read_the_same_after_step_a() {
    let mut read = 0;
    for path in corpus("manifest.json") {
        let text = std::fs::read_to_string(&path).unwrap();
        let Ok(before) = serde_json::from_str::<reiwand_format::Manifest>(&text) else { continue };
        let mut doc: Value = serde_json::from_str(&text).unwrap();
        complete_with(Format::Manifest, &mut doc, &Customs::new());
        let after: reiwand_format::Manifest = serde_json::from_value(doc.clone())
            .unwrap_or_else(|e| panic!("{}: not readable after step A: {e}", path.display()));
        let (x, y) = (serde_json::to_value(&before).unwrap(), serde_json::to_value(&after).unwrap());
        let at = first_diff(&x, &y, "");
        let (d, e) = (format!("{before:?}"), format!("{after:?}"));
        let cut = d.chars().zip(e.chars()).take_while(|(a, b)| a == b).count().saturating_sub(40);
        let show = |t: &str| t.chars().skip(cut).take(120).collect::<String>();
        assert!(
            before == after,
            "{}: differs after step A at {at:?}: {} vs {}",
            path.display(),
            show(&d),
            show(&e)
        );
        read += 1;
    }
    assert!(read >= 20, "the corpus is too small: {read}");
}
