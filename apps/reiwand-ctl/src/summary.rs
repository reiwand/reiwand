use crate::show::{clean, list, text};
use crate::told::{fill, told, tr};
use reiwand_core::model::{ImportReport, InstalledPackage, Job, JobKind, Repo, UploadOutput};
use serde_json::{Value, from_value};
use std::collections::BTreeMap;

pub fn summary(job: &Job) -> String {
    let head = format!("job {} {} {}", job.id, text(&job.kind), text(&job.state));
    let output = job.output.clone().unwrap_or_default();
    let detail = match job.kind {
        JobKind::RepoAdd => {
            from_value(output).ok().map(|r: Repo| fill(told("%1 (%2 packages)"), &[&r.name, &r.packages]))
        }
        JobKind::Install => from_value(output).ok().map(|p: InstalledPackage| {
            let needs = p.missing.join(", ");
            let note = if needs.is_empty() {
                String::new()
            } else {
                fill(told(" (incomplete: could not install %1)"), &[&needs])
            };
            format!("{} {}{note}", p.id, p.version)
        }),
        JobKind::Upload => from_value(output).ok().map(|u: UploadOutput| {
            format!("{} {} {}", u.package.unwrap_or_default(), u.version.unwrap_or_default(), u.state)
        }),
        JobKind::Import => {
            let lines = from_value(output).ok().map(|r: ImportReport| imported(&r));
            let first = lines.as_ref().map(|lines| format!("{head}: {}", lines[0])).unwrap_or(head);
            let rest = lines.into_iter().flatten().skip(1);
            return [first].into_iter().chain(rest).map(|line| clean(&line)).collect::<Vec<_>>().join("\n");
        }
        JobKind::Unknown => None,
        _ => Some(counts(&output)).filter(|counts| !counts.is_empty()),
    };
    clean(&detail.map_or(head.clone(), |detail| format!("{head}: {detail}")))
}

pub fn imported(report: &ImportReport) -> Vec<String> {
    let (added, kept, failed) = (report.imported.len(), report.kept.len(), report.failed.len());
    let mut lines =
        vec![fill(told("%1 imported, %2 already in the library, %3 failed"), &[&added, &kept, &failed])];
    for k in &report.kept {
        let path = k.path.display();
        lines.push(if k.newer {
            fill(
                told("already in the library: %1 (the library has the newer version %2)"),
                &[&path, &k.version],
            )
        } else {
            fill(told("already in the library: %1 (the same version %2)"), &[&path, &k.version])
        });
    }
    for f in &report.failed {
        lines.push(fill(told("not imported: %1: %2"), &[&f.path.display(), &f.error]));
    }
    if report.kept.iter().any(|k| k.newer) {
        lines.push(tr(told(
            "reiwand-ctl pkg import --allow-older replaces a newer version with the older one",
        )));
    }
    lines
}

fn counts(output: &Value) -> String {
    let parts = match output.get("reports").and_then(Value::as_array) {
        Some(reports) => reports.iter().filter_map(Value::as_object).collect(),
        None => output.as_object().into_iter().collect::<Vec<_>>(),
    };
    let mut sums = BTreeMap::<&str, u64>::new();
    for (key, value) in parts.into_iter().flatten() {
        if let Some(n) = value.as_u64().or(value.as_array().map(|a| a.len() as u64)) {
            *sums.entry(key).or_default() += n;
        }
    }
    list(sums.into_iter().map(|(key, n)| format!("{n} {key}")))
}
