use crate::Error;
use reiwand_scene::{SCHEMA, Scene};

pub(crate) fn blank() -> Result<Scene, Error> {
    let canvas = serde_json::json!({"ambient": null, "background": "#000000", "blend_space": "linear",
        "bloom": null, "camera": null, "clear": true, "fit": "fill", "gusts": null, "hdr": false,
        "parallax": null, "perspective": null, "size": [1, 1], "sky": null, "tonemap": null, "wind": null});
    let clock = serde_json::json!({"max_fps": "display", "poster_time": 0.0, "seed": 0, "step": 60});
    Ok(serde_json::from_value(serde_json::json!({"assets": {}, "audio": null, "camera3d": null,
        "canvas": canvas, "clock": clock, "effects": {}, "ext": {}, "layers": [], "listens": [],
        "particles": {}, "pools": {}, "schema": SCHEMA, "scripts": [], "templates": {}, "timelines": [],
        "transition": null}))?)
}
