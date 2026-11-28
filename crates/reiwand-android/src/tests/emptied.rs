use crate::tests::ask;
use crate::{settings, store};
use serde_json::{Value, json};
use std::io::Cursor;

fn picture() -> String {
    let mut bytes = Cursor::new(Vec::new());
    let image = image::RgbImage::from_pixel(24, 12, image::Rgb([11, 222, 33]));
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    store::store(&mut Cursor::new(bytes.into_inner()), "lonely.png").unwrap().0
}

pub(super) fn a_list_left_empty_stops_being_in_use() {
    let uid = picture();
    let list = ask(json!({ "op": "create", "name": "Lonely" })).as_str().unwrap().to_owned();
    ask(json!({ "op": "add", "playlist": list, "items": [uid] }));
    ask(json!({ "op": "pick", "choice": { "playlist": list } }));
    crate::choice::commit(false, crate::choice::Target::Both).unwrap();
    assert_eq!(settings::config().resolve("").playlist.as_deref(), Some(list.as_str()));
    let impact = crate::library::with(|lib| crate::library::impact(lib, &uid)).unwrap();
    assert_eq!(
        impact,
        json!({ "in_use": true, "presets": [] }),
        "the only item of the list in use is in use"
    );
    assert_eq!(ask(json!({ "op": "remove", "uids": [uid] })), Value::Null);
    assert_eq!(settings::config().resolve("").playlist, None, "a list left empty is no screen's list");
    ask(json!({ "op": "delete", "playlist": list }));
}
