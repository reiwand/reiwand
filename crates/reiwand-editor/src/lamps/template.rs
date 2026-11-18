use anyhow::{Result, anyhow};
use reiwand_core::message::said;
use reiwand_scene::Beam;
use serde_json::{Value, json};

pub fn template(doc: &Value, id: &str, source: Option<&str>) -> Result<Value> {
    let beam: Beam = match source {
        None => Beam::Point,
        Some(s) => serde_json::from_value(json!(s)).map_err(|_| {
            anyhow!(said(
                "editor.light_type",
                "%1 is not a light type; use point, spot, tube or directional",
                &[&s]
            ))
        })?,
    };
    let size = crate::seats::canvas_of(doc);
    let middle = size.map(|s| s / 2.0);
    let own = json!({ "id": id, "type": "light", "light": beam, "transform": { "position": middle } });
    let name = match beam {
        Beam::Point => "point",
        Beam::Spot => "spot",
        Beam::Tube => "tube",
        Beam::Directional => "directional",
    };
    let mut l = own;
    let row = reiwand_scene::prefill::offer(&format!("light.{name}"), "", json!({}));
    if let Some(into) = l.as_object_mut() {
        for (key, put) in row.as_object().into_iter().flatten().filter(|(k, _)| k.as_str() != "transform") {
            into.insert(key.clone(), put.clone());
        }
    }
    if let Some(tilt) = row.pointer("/transform/tilt") {
        l["transform"]["tilt"] = tilt.clone();
    }
    Ok(l)
}
