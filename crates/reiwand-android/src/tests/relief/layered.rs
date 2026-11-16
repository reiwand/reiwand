use crate::power::Power;
use crate::{depth, settings};
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn layered(uid: &str, image: &Path, deep: &dyn Fn(Value) -> Value) {
    let item = || deep(json!({ "op": "item", "uid": uid }));
    let own = |fill: &str, layers: u8| {
        deep(json!({ "op": "parallax", "uid": uid, "fill": fill, "layers": layers }));
    };
    assert!(settings::setting("parallax.fill", Some("simple")).is_err());
    own("simple", 4);
    assert_eq!(deep(json!({ "op": "layers", "uid": uid })), Value::Null);
    assert_eq!(item()["layers"], json!(true));
    assert_eq!(item()["fill"]["count"], json!(4));
    assert!(depth::raised(image).is_some_and(|raise| raise.deck.is_some()));
    own("lama", 5);
    let saver = |on: bool| Power { saver: on, ..Power::now() }.set();
    saver(true);
    assert_eq!(item()["fill"]["chosen"], json!("lama"), "an explicit choice is kept on battery saver");
    saver(false);
    assert_eq!(item()["fill"]["chosen"], json!("lama"));
    own("simple", 5);
}

#[test]
fn a_picture_that_never_chose_gets_the_simple_fill() {
    use reiwand_core::Fill;
    let mut own = reiwand_core::edit::Parallax::default();
    assert_eq!(depth::layers::chosen(&own), Fill::Simple);
    own.fill = Some(Fill::Lama);
    assert_eq!(depth::layers::chosen(&own), Fill::Lama);
    assert_eq!(reiwand_core::edit::Parallax::default().layers(), 5);
}
