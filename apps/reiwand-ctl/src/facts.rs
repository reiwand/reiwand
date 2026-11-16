use crate::show::{self, list, text};
use crate::told::{fill, told, tr};
use anyhow::Result;
use reiwand_editor::{Project, review};
use reiwand_format::{Error, Facts, Limits, Manifest, features, inspect, open};
use serde_json::{Value, json};
use std::path::Path;

pub fn check(path: &Path, json: bool) -> Result<()> {
    let (reviewed, cautions) = reviewed(path);
    let opened = match path.is_dir() {
        true => inspect(path),
        false => open(path, &Limits::default()).and_then(|mut package| {
            package.look_closer()?;
            Ok((package.manifest, package.facts))
        }),
    };
    let (manifest, facts) = match opened {
        Ok(opened) => opened,
        Err(e) if reviewed.is_empty() => return Err(e.into()),
        Err(_) => {
            reviewed.iter().for_each(|line| println!("{line}"));
            anyhow::bail!("{}", crate::authoring::problems_of(reviewed.len()));
        }
    };
    match features::check(&manifest, &facts) {
        Err(Error::Invalid { field, .. }) if field == "requires.features" => {}
        checked => checked?,
    }
    let mut problems: Vec<String> =
        crate::scriptfaults::of(path, &manifest).iter().map(|f| f.said().local()).collect();
    problems.extend(crate::scriptfaults::passages(path, &manifest));
    problems.extend(reviewed);
    let listed = manifest.features();
    let (needed, missing) = review::needs(&manifest, &facts);
    let platforms = review::platforms(&manifest, &facts);
    let report = json!({
        "path": path, "schema": manifest.schema, "id": manifest.id, "version": manifest.version,
        "kind": manifest.kind, "title": manifest.title_in(&reiwand_core::locale::languages()),
        "rating": manifest.content.as_ref().map(|c| text(&c.rating)),
        "reasons": manifest.content.iter().flat_map(|c| &c.descriptors).map(text).collect::<Vec<_>>(),
        "photosensitive": manifest.photosensitive, "origin": manifest.origin.as_ref().map(|o| text(&o.kind)),
        "inputs": manifest.inputs.iter().map(|i| i.name()).collect::<Vec<_>>(),
        "features": needed, "declared": listed, "missing": missing,
        "facets": facts.facets, "languages": manifest.locales(), "platforms": platforms,
        "warnings": features::warnings(&manifest, &facts).into_iter().chain(cautions).collect::<Vec<_>>(),
        "problems": problems,
    });
    match json {
        true => println!("{report}"),
        false => print!("{}", human(&manifest, &facts, &report)),
    }
    anyhow::ensure!(problems.is_empty(), "{}", crate::authoring::problems_of(problems.len()));
    Ok(())
}

pub(crate) fn reviewed(dir: &Path) -> (Vec<String>, Vec<String>) {
    let Some(project) = Some(dir).filter(|d| d.is_dir()).and_then(|d| Project::open(d).ok()) else {
        return (Vec::new(), Vec::new());
    };
    if reiwand_editor::pages::scened(&project.doc) && project.doc["manifest"]["photosensitive"] != true {
        reiwand_editor::flashes::measure(&project.dir, &project.doc, (0.0, 8.0, 15)).ok();
    }
    let all = reiwand_editor::package::problems(&project.dir, &project.doc);
    let said = |p: &reiwand_editor::Problem| format!("{}: {}", p.pointer, p.said.local());
    let (cautions, errors): (Vec<_>, Vec<_>) = all.iter().partition(|p| p.warning);
    (errors.into_iter().map(said).collect(), cautions.into_iter().map(said).collect())
}

fn human(manifest: &Manifest, facts: &Facts, report: &Value) -> String {
    let words = |key: &str| list(report[key].as_array().into_iter().flatten().filter_map(Value::as_str));
    let named =
        |f: &Value| format!("{} ({})", f["name"].as_str().unwrap_or(""), f["why"].as_str().unwrap_or(""));
    let needs = |key: &str| list(report[key].as_array().into_iter().flatten().map(named));
    let flag = |on: Option<bool>| on.map_or("unknown", |on| if on { "yes" } else { "no" }).to_string();
    let rating = report["rating"].as_str().unwrap_or("unrated").to_string();
    let plays = |on: &Value| match (on["refused"].as_str(), on["without"].as_array()) {
        (Some(reason), _) => format!("refused: {reason}"),
        (None, Some(without)) if !without.is_empty() => {
            fill(told("plays without %1"), &[&list(without.iter().filter_map(Value::as_str))])
        }
        _ => "plays".into(),
    };
    let reasons = Some(words("reasons")).filter(|r| !r.is_empty()).map(|r| format!(" ({r})"));
    let problems = report["problems"].as_array().into_iter().flatten().filter_map(Value::as_str);
    let problems: String = problems.map(|p| fill(told("problem: %1\n"), &[&p])).collect();
    let state = if problems.is_empty() { "ok" } else { "problems" };
    let head =
        format!("{state} {} {} {} ({})\n", manifest.kind, manifest.id, manifest.version, manifest.schema);
    let pairs = [
        ("title", report["title"].as_str().unwrap_or("").to_string()),
        ("rating", rating + &reasons.unwrap_or_default()),
        ("photosensitive", flag(manifest.photosensitive)),
        ("origin", report["origin"].as_str().unwrap_or("-").to_string()),
        ("inputs", words("inputs")),
        ("needs", needs("features")),
        (&tr(told("build adds")), needs("missing")),
        ("facets", list(&facts.facets)),
        ("desktop", plays(&report["platforms"]["desktop"])),
        ("android", plays(&report["platforms"]["android"])),
        ("languages", words("languages")),
    ];
    let shown = pairs.into_iter().filter(|(_, v)| !v.is_empty());
    let warnings = report["warnings"].as_array().into_iter().flatten().filter_map(Value::as_str);
    head + &problems
        + &show::lines(shown)
        + &warnings.map(|w| format!("{}\n", fill(told("warning: %1"), &[&w]))).collect::<String>()
}
