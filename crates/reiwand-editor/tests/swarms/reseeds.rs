use serde_json::json;

#[test]
fn a_new_seed_reaches_the_preview_and_never_a_scene_without_a_clock() {
    let mut fixed = json!({ "clock": { "seed": 3 } });
    reiwand_editor::swarms::preview_reseed(&mut fixed, &json!({ "seed": 77 }));
    assert_eq!(fixed["clock"]["seed"], json!(77), "New seed did not reach the preview");
    reiwand_editor::swarms::preview_reseed(&mut fixed, &json!({}));
    assert_eq!(fixed["clock"]["seed"], json!(77), "a preview without a new seed changed it");
    let mut bare = json!({});
    reiwand_editor::swarms::preview_reseed(&mut bare, &json!({ "seed": 5 }));
    assert_eq!(bare, json!({}), "a scene without a clock got half a clock");
}
