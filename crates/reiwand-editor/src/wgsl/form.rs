use anyhow::{Result, anyhow, ensure};
use reiwand_core::message::said;
use serde_json::{Map, Value, json};

pub const KINDS: [&str; 12] = [
    "float", "vec2", "color", "bool", "texture", "lut", "int", "enum", "blend", "position", "mask",
    "emitters",
];
pub const FIELDS: [&str; 19] = [
    "name",
    "category",
    "description",
    "uses",
    "mask_mode",
    "cost",
    "fuse",
    "reach",
    "bounded",
    "min_fps",
    "velocity",
    "state",
    "solve",
    "pace",
    "quality",
    "targets",
    "options",
    "compose",
    "lighter",
];
pub(super) const PARAM_KEYS_ALL: [&str; 14] = [
    "type",
    "prefill",
    "min",
    "max",
    "unit",
    "label",
    "help",
    "values",
    "channels",
    "paint",
    "uses",
    "yields_to",
    "waits_for",
    "tools",
];
pub(super) const PASS_KEYS: [&str; 10] =
    ["entry", "scale", "inputs", "output", "target", "copy", "blend", "vertex", "grid", "swap"];

fn default_of(kind: &str) -> Value {
    match kind {
        "vec2" => json!([0.0, 0.0]),
        "position" => json!([0.5, 0.5]),
        "color" => json!("#ffffff"),
        "bool" => json!(false),
        "texture" | "mask" => json!("white"),
        "lut" => json!(""),
        "emitters" => json!([]),
        _ => json!(0),
    }
}

pub(super) fn keyed(key: &str, allowed: &[&str]) -> Result<()> {
    ensure!(allowed.contains(&key), said("editor.shader_key", "%1 is not a field this form edits", &[&key]));
    Ok(())
}

pub(super) fn list<'a>(json: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    if !json[key].is_array() {
        json[key] = json!([]);
    }
    json[key].as_array_mut().expect("made above")
}

pub(crate) struct ParamChange<'a> {
    pub id: &'a str,
    pub set: &'a Map<String, Value>,
    pub clear: &'a [String],
    pub to: Option<&'a str>,
    pub at: Option<usize>,
    pub remove: bool,
}

pub(crate) fn param(json: &mut Value, c: &ParamChange) -> Result<()> {
    let params = list(json, "params");
    let found = params.iter().position(|p| p["id"] == c.id);
    if c.remove {
        let at = found
            .ok_or_else(|| anyhow!(said("editor.no_parameter", "%1 has no parameter %2", &[&"", &c.id])))?;
        params.remove(at);
        return Ok(());
    }
    let at = match found {
        Some(at) => at,
        None => {
            crate::clips::valid(c.id)?;
            let kind = c.set.get("type").and_then(Value::as_str).map_or_else(
                || {
                    reiwand_scene::prefill::offered_row("wgsl.param")["type"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned()
                },
                str::to_owned,
            );
            let kind = kind.as_str();
            params.push(super::whole::param(c.id, kind, default_of(kind)));
            params.len() - 1
        }
    };
    let p = &mut params[at];
    let kind_before = p["type"].clone();
    for (k, v) in c.set {
        keyed(k, &PARAM_KEYS_ALL)?;
        p[k] = v.clone();
    }
    if p["type"] != kind_before && !c.set.contains_key("prefill") {
        p["prefill"] = default_of(p["type"].as_str().unwrap_or_default());
    }
    for k in c.clear {
        keyed(k, &PARAM_KEYS_ALL[2..])?;
        p[k] = super::whole::cleared_param(k);
    }
    if let Some(to) = c.to {
        crate::clips::valid(to)?;
        ensure!(
            !params.iter().any(|q| q["id"] == to),
            said("editor.parameter_taken", "there is already a parameter %1", &[&to])
        );
        params[at]["id"] = json!(to);
    }
    if let Some(to) = c.at {
        let moved = params.remove(at);
        params.insert(to.min(params.len()), moved);
    }
    Ok(())
}
