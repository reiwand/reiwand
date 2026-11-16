use crate::author::Author;
use crate::capturing::Capture;
use crate::steps::edit;
use crate::told::{fill, told, tr};
use crate::usage;
use anyhow::{Context, Result};
use reiwand_editor::actions::{Action, ShotEdit, ShotFormat};
use reiwand_editor::starters::{default_id, starter};
use reiwand_editor::{Project, inspect, package};
use std::fs;
use std::path::Path;

fn describe(project: &Project, json: bool) -> Result<()> {
    let problems = package::problems(&project.dir, &project.doc);
    let errors = problems.iter().filter(|p| !p.warning).count();
    let m = &project.doc["manifest"];
    let layers = reiwand_editor::layers::walk(&project.doc).len();
    let pending = project.pending.is_some();
    if json {
        let files = reiwand_editor::check::check_files(&project.dir, &project.doc);
        println!("{}", reiwand_editor::outline::outline(project, 0.0, &files));
        return Ok(());
    }
    let text = |k: &str| m[k].as_str().unwrap_or("?").to_owned();
    let title = reiwand_editor::meta::title(&project.doc, &reiwand_core::locale::languages());
    let (layers, errors) = (layers_of(layers), problems_of(errors));
    if !reiwand_editor::pages::scened(&project.doc) {
        let kind = reiwand_editor::pages::kind(&project.doc);
        println!("{title} ({} {}, {kind}): {errors}", text("id"), text("version"));
        for file in reiwand_editor::pages::files(&project.dir, &project.doc) {
            let (path, role) = (file["path"].as_str().unwrap_or(""), file["role"].as_str().unwrap_or(""));
            println!(
                "{}",
                fill(
                    told("  %1: %2, %3, %4 bytes"),
                    &[&path, &role, &file["kind"].as_str().unwrap_or(""), &file["size"]]
                )
            );
        }
    } else {
        println!("{title} ({} {}): {layers}, {errors}", text("id"), text("version"));
    }
    if pending {
        println!(
            "{}",
            tr(told("the app kept unsaved changes; reiwand-ctl scene recover restores or drops them"))
        );
    }
    Ok(())
}

pub(crate) fn layers_of(n: usize) -> String {
    match n {
        1 => tr(told("1 layer")).to_owned(),
        n => fill(told("%1 layers"), &[&n]),
    }
}

pub(crate) fn problems_of(n: usize) -> String {
    match n {
        1 => tr(told("1 problem")).to_owned(),
        n => fill(told("%1 problems"), &[&n]),
    }
}

pub fn run(command: Author) -> Result<()> {
    if let Author::Thumbnail { dir, animated, shot } = &command
        && shot.capture
        && crate::webshot::wanted(dir, shot)
    {
        return crate::webshot::run(dir, *animated, shot);
    }
    match command {
        Author::New { dir, from, id, title, locale, preset } => {
            preset.allowed(&from)?;
            let name = fs::canonicalize(
                dir.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new(".")),
            )?
            .join(dir.file_name().context(tr(told("name the new folder")))?);
            let folder = name.file_name().unwrap_or_default().to_string_lossy().into_owned();
            let id = id.unwrap_or_else(|| default_id(&folder));
            let locale = locale.unwrap_or_else(reiwand_editor::starters::locale);
            let doc = starter(&from, &id, &title.unwrap_or(folder), &locale)
                .map_err(|e| usage(reiwand_editor::told(&e)))?;
            Project::create(&dir, doc)?;
            preset.apply(&dir)?;
        }
        Author::Open { dir, from, copying, json } => {
            if let Some(file) = from {
                if copying.facts {
                    println!("{}", reiwand_editor::copying::facts(&file)?);
                    return Ok(());
                }
                reiwand_editor::copying::extract(&file, &dir, copying.id.as_deref(), copying.mine)?;
            }
            describe(&Project::open(&dir)?, json)?;
        }
        Author::Show { dir, pointer } => {
            let project = Project::open(&dir)?;
            let shown = project
                .doc
                .pointer(pointer.as_deref().unwrap_or(""))
                .context(tr(told("nothing is at that pointer")))?;
            println!("{}", serde_json::to_string_pretty(shown)?);
        }
        Author::Fields { dir, pointer } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&{
                    let project = Project::open(&dir)?;
                    inspect::fields_in(Some(&project.dir), &project.doc, &pointer, &project.effects())
                })?
            );
        }
        Author::Import { dir, file, id, linear, atlas } => {
            let file = fs::canonicalize(&file).unwrap_or(file);
            edit(&dir, Action::Import { file, id, linear, atlas })?
        }
        Author::Thumbnail { dir, animated, shot } => edit(&dir, thumbnail(animated, shot))?,
        Author::Texturing(command) => crate::texturing::run(command)?,
        Author::Folder(command) => crate::folder::run(command)?,
        Author::Acting(command) => crate::acting::run(command)?,
        Author::Rendering(command) => crate::rendering::run(command)?,
    }
    Ok(())
}

fn thumbnail(animated: bool, shot: Capture) -> Action {
    if let Some(file) = shot.file {
        return Action::Thumbnail { file: fs::canonicalize(&file).unwrap_or(file), animated };
    }
    if !animated {
        return Action::Shot(ShotEdit::Still { time: shot.time, base: shot.base });
    }
    let format = match shot.format.as_deref() {
        Some("webp") => ShotFormat::Webp,
        _ => ShotFormat::Gif,
    };
    let (from, seconds, fps) = (shot.from, shot.seconds, shot.fps);
    Action::Shot(ShotEdit::Recording { from, seconds, fps, format, base: shot.base })
}
