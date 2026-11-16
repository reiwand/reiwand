use reiwand_editor::actions::visible;
use serde_json::{Value, json};
use std::path::Path;

pub(crate) fn document(doc: &Value, dir: &Path, look: &Value) -> Value {
    if reiwand_editor::pages::paged(doc) {
        reiwand_editor::pages::preview_document(doc, look)
    } else if reiwand_editor::reels::media(doc) {
        reiwand_editor::reels::preview_document(dir, doc, look)
    } else {
        let mut scene = visible(doc);
        reiwand_editor::swarms::pin_seed(&mut scene);
        reiwand_editor::rigs::preview(&mut scene, look);
        reiwand_editor::scripts::stamped(&mut scene, dir, doc);
        let mut document = json!({"manifest": doc["manifest"], "scene": scene});
        if let Some(staged) = reiwand_editor::rigs::staged(look) {
            document["preview"] = staged;
        }
        reiwand_editor::tried::stage(doc, look, &mut document);
        document
    }
}
