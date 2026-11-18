use reiwand_scene::prefill::{of, offer};
use serde_json::json;

#[test]
fn the_offered_objects_come_whole_from_the_table() {
    let boids = of("Boids", json!({}));
    assert_eq!(
        boids,
        json!({ "radius": 40.0, "separation": 1.0, "alignment": 1.0, "cohesion": 0.5, "max_speed": 200.0 })
    );
    assert_eq!(offer("perspective", "", json!({}))["focal"], 800.0);
    assert_eq!(offer("trail", "", json!({}))["length"], 32);
    let camera = offer("camera", "Camera", json!({ "center": [1.0, 2.0] }));
    assert_eq!((&camera["zoom"], &camera["rotation"]), (&json!(1.0), &json!(0.0)));
    let pool = of("Pool", json!({ "type": "mesh", "max": 16, "max_vertices": 1024 }));
    assert!(pool["texture"].is_null() && pool["size"].is_null());
}

#[test]
fn every_prefill_name_the_editor_asks_for_is_in_the_table() {
    let mut asked = Vec::new();
    let mut stack = vec![std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src"))];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path).unwrap();
                for part in text.split("prefill::").skip(1) {
                    let call = part.split(['(', ' ']).next().unwrap_or_default();
                    let rest = part.trim_start_matches(|c: char| {
                        c.is_alphanumeric() || c == '_' || c == ':' || c == '<' || c == '>'
                    });
                    if let (true, Some(args)) =
                        (matches!(call.split("::").next(), Some("arg" | "offer")), rest.strip_prefix('('))
                    {
                        asked.push(args.split('"').nth(1).map(|n| (call.to_owned(), n.to_owned())));
                    }
                }
            }
        }
    }
    let rows: Vec<String> = reiwand_scene::prefill::rows().map(|(r, _)| r.clone()).collect();
    for (call, name) in asked.into_iter().flatten().filter(|(_, n)| !n.contains('{')) {
        let key = if call.starts_with("arg") { format!("Action.{name}") } else { format!("Offer.{name}") };
        assert!(rows.contains(&key), "{key} is not in the prefill table");
    }
}
