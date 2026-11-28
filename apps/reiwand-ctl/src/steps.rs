use crate::told::{fill, told, tr};
use anyhow::{Result, ensure};
use reiwand_editor::actions::{Action, ParticleEdit, SowEdit};
use reiwand_editor::{Op, Project};
use serde_json::json;
use std::path::Path;

pub(crate) const NEW: &str = "Create a package folder from a starting point: the scenes empty, particles, parallax or clock, web for a web page or preset for the values of another package";
pub(crate) const OPEN: &str = "Describe a scene folder, or first extract a .wpak scene package into it as an unsigned copy with the id <id>.copy";

pub(crate) fn edit(dir: &Path, action: Action) -> Result<()> {
    edit_all(dir, vec![action], false)
}

pub(crate) fn edit_all(dir: &Path, actions: Vec<Action>, json: bool) -> Result<()> {
    let mut project = Project::open(dir)?;
    let single = actions.len() == 1;
    let action = actions.first().cloned().unwrap_or(Action::Undo {});
    let undoing = single && matches!(action, Action::Undo {} | Action::Redo {});
    let named = crate::unkept::named(&project, &actions);
    let scripts = project.doc.pointer("/scene/scripts").cloned().unwrap_or_default();
    let lost = crate::unkept::lost(&project.doc, &actions);
    let before = reiwand_editor::check::review(&project.doc, Some(&project.dir));
    let ops = if single { project.act(&action)? } else { project.act_all(&actions)? };
    let after = reiwand_editor::check::review(&project.doc, Some(&project.dir));
    let problems = reiwand_editor::check::introduced(&before, &after);
    ensure!(
        !(undoing && ops.is_empty()),
        "{}",
        tr(if matches!(action, Action::Redo {}) {
            told("there is nothing to redo")
        } else {
            told("there is nothing to undo")
        })
    );
    project.save()?;
    named.iter().for_each(|(path, id)| {
        eprintln!(
            "{}",
            fill(told("warning: %1 names %2; scripts are not rewritten"), &[&path, &format!("{id:?}")])
        )
    });
    if let Some(text) = crate::unkept::warning(&lost) {
        eprintln!("{text}");
    }
    for p in &problems {
        eprintln!("{}: {}: {}", if p.warning { "warning" } else { "problem" }, p.pointer, p.said.local());
    }
    if let (Action::Particles(ParticleEdit::Sow(SowEdit::Rename { system, to })), false) =
        (&action, named.is_empty())
    {
        eprintln!(
            "{}",
            fill(
                told("to rewrite the exact names: scene act DIR particles '%1'"),
                &[&json!({ "scripts": { "from": system, "to": to } })]
            )
        );
    }
    if matches!(action, Action::ImportScript(_)) {
        let now = project.doc.pointer("/scene/scripts").cloned().unwrap_or_default();
        reiwand_editor::scripts::changes(&scripts, &now).iter().for_each(|m| eprintln!("{}", m.local()));
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&json!({ "ops": ops, "problems": problems }))?);
        return Ok(());
    }
    let added = ops.iter().filter_map(|op| match op {
        Op::Add { path, value } => {
            Some(value["id"].as_str().unwrap_or(path.rsplit('/').next().unwrap_or_default()))
        }
        _ => None,
    });
    let added: Vec<&str> = added.collect();
    match action {
        _ if !single => ops
            .iter()
            .filter_map(|op| match op {
                Op::Add { value, .. } => value["id"].as_str(),
                _ => None,
            })
            .for_each(|id| println!("{id}")),
        Action::Import { .. } => added.last().iter().for_each(|id| println!("{id}")),
        Action::AddPreset(_) => {
            let last = project.doc["manifest"]["presets"].as_array().and_then(|l| l.last());
            last.and_then(|p| p["id"].as_str()).iter().for_each(|id| println!("{id}"))
        }
        Action::AddProperty { .. } => ops
            .iter()
            .filter_map(|op| match op {
                Op::Add { path, .. } => {
                    path.strip_prefix("/manifest/properties/").filter(|p| !p.contains('/'))
                }
                _ => None,
            })
            .for_each(|id| println!("{id}")),
        Action::Thumbnail { .. } | Action::Shot(_) | Action::Asset(_) => ops
            .iter()
            .filter_map(|op| match op {
                Op::Add { value, .. } | Op::Replace { value, .. } => value.as_str(),
                _ => None,
            })
            .for_each(|path| println!("{path}")),
        _ => ops
            .iter()
            .filter_map(|op| match op {
                Op::Add { value, .. } => value["id"].as_str(),
                _ => None,
            })
            .for_each(|id| println!("{id}")),
    }
    Ok(())
}
