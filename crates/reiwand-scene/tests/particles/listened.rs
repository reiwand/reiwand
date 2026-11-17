use super::*;

fn problems(init: Json) -> Vec<Issue> {
    issues(with(sparks(), json!({ "init": init })))
}

#[test]
fn turbulent_and_remap_audio_state_their_kind_and_surge_fields() {
    let ok = json!({ "turbulent": { "forward": 5, "audio": "bass", "kind": "relative", "unit": "db" } });
    assert_eq!(problems(ok), []);
    let surge =
        json!({ "turbulent": { "forward": 5, "audio": "bass", "kind": "surge", "unit": "amplitude" } });
    assert!(problems(surge).iter().any(|i| matches!(i, Issue::Required(_))));
    let full = json!({ "turbulent": { "forward": 5, "audio": "bass", "kind": "surge", "unit": "amplitude",
        "window": 1, "reference": "band", "hold_silence": true } });
    assert_eq!(problems(full), []);
    let stray = json!({ "turbulent": { "forward": 5, "audio": "bass", "kind": "relative", "unit": "db", "window": 1 } });
    assert!(problems(stray).iter().any(|i| matches!(i, Issue::Type(_))));
    let remap = json!({ "remap": [{ "input": "audio:3", "in": [0, 1], "output": "size", "out": [1, 8],
        "kind": "relative", "unit": "db" }] });
    assert_eq!(problems(remap), []);
}
