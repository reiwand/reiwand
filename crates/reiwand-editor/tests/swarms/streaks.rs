use super::*;

#[test]
fn handles_report_the_stretched_sprite_and_the_oscillation_curves() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    edit(&mut p, "start", json!({ "ranges": { "speed": [100, 100], "size": [10, 10] } })).unwrap();
    assert!(handles(&p)["stretch"].is_null());
    for (mode, long) in [("add", 13.0), ("relative", 30.0)] {
        let stretch = json!({ "mode": mode, "time": 0.03, "min": 0, "max": 3 });
        edit(&mut p, "render", json!({ "stretch": stretch })).unwrap();
        let s = &handles(&p)["stretch"];
        assert_eq!((s["mode"].clone(), s["long"].as_f64().unwrap()), (json!(mode), long), "{s}");
    }
    assert!(edit(&mut p, "render", json!({ "stretch": { "time": 1 } })).is_err());
    let wave = |what: &str, from: &str| json!({ "oscillate": { "what": what, "from": from, "freq": [1, 1], "scale": [0.2, 1] } });
    edit(&mut p, "force", json!({ "force": wave("alpha", "floor") })).unwrap();
    edit(&mut p, "force", json!({ "force": wave("size", "top") })).unwrap();
    let h = handles(&p);
    let floor = &h["waves"][0];
    let lows: Vec<f64> = floor["curves"][0].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    let highs: Vec<f64> =
        floor["curves"][2].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    assert_eq!(floor["from"], "floor");
    assert!(lows.iter().all(|v| (v - 0.2).abs() < 1e-9), "every particle shares the floor: {lows:?}");
    assert!(highs.iter().cloned().fold(0.0, f64::max) > 0.99, "{highs:?}");
    assert_eq!((h["waves"][1]["what"].clone(), h["waves"][1]["from"].clone()), (json!("size"), json!("top")));
    assert_eq!(h["waves"].as_array().unwrap().len(), 2);
}
