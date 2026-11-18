mod aim;
mod budget;
mod check;
mod emits;
mod ends;
mod fields;
pub(crate) mod glyphs;
mod handles;
mod looks;
mod named;
mod proportion;
mod pulls;
mod reach;
mod salt;
mod samples;
mod scopes;
mod shapes;
mod sow;
mod spots;
mod steers;
mod streaks;
mod tend;
mod view;
mod walls;

use crate::action::ParticleEdit;
use crate::ops::{Op, apply, escape, set};
use anyhow::{Result, anyhow, bail, ensure};
use reiwand_core::message::said;
use serde_json::{Value, json};
use std::path::Path;

pub(crate) use check::checked;
use check::parsed;

pub use crate::curves::{Curve, plot};
pub use budget::budget;
pub use handles::{handles, heard};
pub use looks::{controls, sources};
pub use salt::{pin_seed, preview_seed};
pub use walls::response;

pub(crate) fn rename_system(doc: &Value, at: &str, old: &str, to: &str) -> Result<Vec<crate::ops::Op>> {
    emits::rename(doc, at, old, to)
}

pub(crate) fn system(doc: &Value, id: &str) -> Result<String> {
    let path = format!("/scene/particles/{}", escape(id));
    ensure!(
        doc.pointer(&path).is_some(),
        said("editor.no_system", "there is no particle system \"%1\"", &[&id])
    );
    Ok(path)
}

pub(crate) fn render(
    doc: &Value,
    sys: &str,
    layer: Option<&str>,
    change: impl Fn(&Value, &str) -> Vec<Op>,
) -> Result<Vec<Op>> {
    let Some(id) = layer else {
        let own = format!("{sys}/render");
        let mut ops = squared(doc, &own);
        let mut probe = doc.clone();
        apply(&mut probe, &ops)?;
        ops.extend(change(&probe, &own));
        return Ok(ops);
    };
    let (path, found) = crate::actions::layer(doc, id)?;
    let own = crate::ops::unescape(sys.rsplit('/').next().unwrap_or_default());
    let fits = found["type"] == "particles" && found["system"] == own.as_str();
    ensure!(fits, said("editor.not_particles", "the layer \"%1\" draws no particles of this system", &[&id]));
    let own = format!("{path}/render");
    let copy = doc.pointer(&format!("{sys}/render")).cloned().unwrap_or(json!({}));
    let ops =
        if found.get("render").is_some_and(|r| !r.is_null()) { Vec::new() } else { set(doc, &own, copy) };
    let mut probe = doc.clone();
    let mut ops = ops;
    apply(&mut probe, &ops)?;
    let filled = crate::completing::ops(&probe);
    apply(&mut probe, &filled)?;
    ops.extend(filled);
    let squares = squared(&probe, &own);
    apply(&mut probe, &squares)?;
    ops.extend(squares);
    ops.extend(change(&probe, &own));
    Ok(ops)
}

fn squared(doc: &Value, at: &str) -> Vec<Op> {
    match doc.pointer(&format!("{at}/aspect")) {
        Some(_) => Vec::new(),
        None => set(doc, &format!("{at}/aspect"), json!(1)),
    }
}

pub(crate) fn put(doc: &Value, path: &str, value: Option<Value>) -> Vec<Op> {
    match value {
        Some(v) => set(doc, path, v),
        None if doc.pointer(path).is_some() => vec![Op::Remove { path: path.into() }],
        None => Vec::new(),
    }
}

pub(crate) fn edit(dir: &Path, doc: &Value, e: &ParticleEdit) -> Result<Vec<Op>> {
    if let ParticleEdit::Sow(crate::action::SowEdit::Scripts { from, to }) = e {
        return named::rewrite(dir, doc, from, to);
    }
    let ops = match e {
        ParticleEdit::Shape { system: s, emitter, shape, remove } => {
            spots::shape(doc, &system(doc, s)?, *emitter, shape, *remove)?
        }
        ParticleEdit::Point { system: s, point, at, follow, angle, remove } => {
            spots::point(doc, &system(doc, s)?, *point, (*at, *follow, *angle), *remove)?
        }
        ParticleEdit::Remap { system: s, index, remap, remove } => {
            looks::remap(doc, &system(doc, s)?, *index, remap.as_ref(), *remove)?
        }
        ParticleEdit::Cutout { system: s, layer, range } => {
            render(doc, &system(doc, s)?, layer.as_deref(), |d, at| {
                put(d, &format!("{at}/cutout"), range.map(|r| json!(r)))
            })?
        }
        ParticleEdit::Aspect { system: s, layer, aspect, from_texture } => {
            proportion::aspect((dir, doc), &system(doc, s)?, layer.as_deref(), (*aspect, *from_texture))?
        }
        ParticleEdit::Orient { system: s, layer, orient, angle } => {
            render(doc, &system(doc, s)?, layer.as_deref(), |d, at| {
                let fixed = *orient == Some(reiwand_scene::Orient::Fixed);
                let mut ops = put(d, &format!("{at}/orient"), orient.map(|o| json!(o)));
                ops.extend(put(d, &format!("{at}/angle"), angle.filter(|_| fixed).map(|a| json!(a))));
                ops
            })?
        }
        ParticleEdit::Perspective { system: s, focal, off } => {
            looks::perspective(doc, &system(doc, s)?, *focal, *off)
        }
        ParticleEdit::Layer { system: s, emitter, id, threshold, color, inherit, jitter, refresh } => {
            let mut source = json!({ "id": id, "color": color, "inherit": inherit });
            threshold.iter().for_each(|t| source["threshold"] = json!(t));
            jitter.iter().for_each(|j| source["jitter"] = json!(j));
            refresh.iter().for_each(|r| source["refresh"] = json!(r));
            spots::shape(doc, &system(doc, s)?, *emitter, &json!({ "layer": source }), false)?
        }
        ParticleEdit::Inherit { system: s, child, inherit, follow } => {
            let at = format!("{}/children/{child}", system(doc, s)?);
            ensure!(
                doc.pointer(&at).is_some(),
                said("editor.no_child", "the system has no child %1", &[child])
            );
            let mut ops = put(doc, &format!("{at}/inherit"), (!inherit.is_empty()).then(|| json!(inherit)));
            ops.extend(put(doc, &format!("{at}/follow_inherit"), follow.filter(|f| *f).map(|_| json!(true))));
            ops
        }
        ParticleEdit::Turbulent { system: s, turbulent, quiet, off } => {
            fields::turbulent(doc, &system(doc, s)?, turbulent.as_ref(), (*quiet, *off))?
        }
        ParticleEdit::Offset { system: s, offset, off } => {
            fields::offset(doc, &system(doc, s)?, offset.as_ref(), *off)?
        }
        ParticleEdit::Pace { system: s, emitter, max_per_step } => {
            fields::pace(doc, &system(doc, s)?, *emitter, *max_per_step)?
        }
        ParticleEdit::Aim { system: s, target, tilt, turn } => {
            aim::aim(doc, &system(doc, s)?, target, *tilt, *turn)?
        }
        ParticleEdit::Tend(t) => tend::edit(doc, t)?,
        ParticleEdit::Sow(s) => sow::edit(doc, s)?,
        ParticleEdit::Light(l) => return crate::lamps::glow(dir, doc, l),
        ParticleEdit::Shader { system: s, layer, shader } => {
            render(doc, &system(doc, s)?, layer.as_deref(), |d, at| {
                put(d, &format!("{at}/shader"), shader.as_ref().map(|id| json!(id)))
            })?
        }
    };
    checked(dir, doc, ops)
}
