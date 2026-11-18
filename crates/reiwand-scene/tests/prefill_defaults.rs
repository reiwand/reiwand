use reiwand_scene::prefill::{completed, of};
use serde_json::json;

#[test]
fn a_new_mask_keeps_its_full_strength() {
    let mask =
        of("Layer", json!({"mask": {"shape": {"rect": {"size": [4, 4], "radius": 0}}}}))["mask"].clone();
    assert_eq!(mask["strength"], json!(1.0));
}

#[test]
fn hues_written_alone_cover_the_full_saturation_and_value() {
    let system = completed(json!({"capacity": 1, "emitters": [], "init": {"hsv": {"hue": [180, 240]}}}));
    assert_eq!(system["init"]["hsv"]["sat"], json!([1.0, 1.0]));
    assert_eq!(system["init"]["hsv"]["val"], json!([1.0, 1.0]));
}
