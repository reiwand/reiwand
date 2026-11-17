mod common;
use common::{corpus, first_diff, same};
use reiwand_upgrade::fill::{Customs, Format, complete_required, complete_with};
use reiwand_upgrade::scenes::customs;
use serde_json::Value;
use std::collections::BTreeMap;

fn strip(v: &mut Value, defaults: &Customs) {
    match v {
        Value::Object(map) => {
            let wanted = map.get("effect").and_then(Value::as_str).and_then(|e| defaults.get(e)).cloned();
            if let (Some(wanted), Some(Value::Object(params))) = (wanted, map.get_mut("params")) {
                params.retain(|id, now| !wanted.iter().any(|(k, d)| k == id && same(d, now)));
            }
            map.values_mut().for_each(|inner| strip(inner, defaults));
        }
        Value::Array(list) => list.iter_mut().for_each(|inner| strip(inner, defaults)),
        _ => {}
    }
}

fn all_defaults(custom: Customs) -> Customs {
    let mut all = Customs::new();
    for d in reiwand_scene::builtin() {
        let list = d.params.iter().map(|p| (p.id.clone(), serde_json::to_value(&p.default).unwrap()));
        all.insert(d.id.clone(), list.collect());
    }
    all.extend(custom);
    all
}

#[test]
fn scenes_read_the_same_after_step_a() {
    let (mut read, mut old, mut unreadable, mut changed, mut fields) =
        (0, 0, Vec::new(), 0, BTreeMap::<String, usize>::new());
    for path in corpus("scene.json") {
        let text = std::fs::read_to_string(&path).unwrap();
        let mut doc: Value = serde_json::from_str(&text).unwrap();
        reiwand_upgrade::gain::fixed(&mut doc);
        let before = match reiwand_scene::Scene::from_json(&text) {
            Ok(before) => before,
            Err(e) => {
                let mut required = doc.clone();
                complete_required(Format::Scene, &mut required);
                let fixed = serde_json::to_string(&required).unwrap();
                let Ok(before) = reiwand_scene::Scene::from_json(&fixed) else {
                    unreadable.extend([format!("{}: {e}", path.display())]);
                    continue;
                };
                old += 1;
                before
            }
        };
        let custom = customs(path.parent().unwrap(), &doc);
        let report = complete_with(Format::Scene, &mut doc, &custom);
        let again = serde_json::to_string_pretty(&doc).unwrap();
        let after = reiwand_scene::Scene::from_json(&again).unwrap_or_else(|e| {
            panic!(
                "{}: not readable after step A: {e}\n{}",
                path.display(),
                again.lines().skip(125).take(8).collect::<Vec<_>>().join("\n")
            )
        });
        let touched_params = report.fields.keys().any(|k| k.starts_with("EffectUse.params"));
        if !touched_params {
            let (x, y) = (serde_json::to_value(&before).unwrap(), serde_json::to_value(&after).unwrap());
            let at = first_diff(&x, &y, "");
            assert!(before == after, "{}: differs after step A at {at:?}", path.display());
        }
        let defaults = all_defaults(custom.clone());
        let (mut a, mut b) = (serde_json::to_value(&before).unwrap(), serde_json::to_value(&after).unwrap());
        strip(&mut a, &defaults);
        strip(&mut b, &defaults);
        let at = first_diff(&a, &b, "");
        assert!(at.is_none(), "{}: parsed values differ after step A at {at:?}", path.display());
        let mut twice = doc.clone();
        assert_eq!(complete_with(Format::Scene, &mut twice, &custom).written, 0, "{}", path.display());
        read += 1;
        changed += usize::from(report.written > 0);
        report.fields.iter().for_each(|(k, n)| *fields.entry(k.clone()).or_default() += n);
    }
    println!(
        "scenes: {read} read identical ({old} after required fields only), {changed} completed, {} unreadable",
        unreadable.len()
    );
    unreadable.iter().for_each(|u| println!("unreadable before: {u}"));
    println!("fields written: {}", fields.len());
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
    assert!(read >= 30, "the corpus is too small: {read}");
}
