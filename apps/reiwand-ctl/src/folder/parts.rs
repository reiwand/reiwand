use crate::told::{fill, told, tr};
use anyhow::{Context, Result, anyhow, bail, ensure};
use reiwand_client::Client;
use reiwand_editor::{Project, applying, package};
use serde_json::Value;
use std::path::Path;
use std::time::Instant;

const SETTLE_MS: u64 = 15_000;
const TICK: std::time::Duration = std::time::Duration::from_secs(5);

fn say(text: &'static str, args: &[&dyn std::fmt::Display]) {
    eprintln!("{}", fill(text, args));
}

pub(super) async fn apply(path: &str, output: &str, fit: &str, doc: &Value) -> Result<()> {
    let client = Client::connect().await?;
    say(told("Adding the package to the library…"), &[]);
    let started = Instant::now();
    let waiting = tokio::spawn(async move {
        loop {
            tokio::time::sleep(TICK).await;
            say(
                told("Still adding it to the library (%1 s); the daemon may be busy with other jobs."),
                &[&started.elapsed().as_secs()],
            );
        }
    });
    let imported =
        client.run_job(async { Ok(((), client.library.import(&[path], false).await?)) }, |_, _| {}).await;
    waiting.abort();
    let report = imported?.1.output.unwrap_or_default();
    let uid = report["imported"][0]["uid"].as_str().map(String::from);
    if let (None, Some(kept)) = (&uid, report["kept"][0]["uid"].as_str()) {
        let version = report["kept"][0]["version"].as_str().unwrap_or_default();
        bail!(fill(
            told(
                "version %1 of this package is already in the library, maybe with other files: raise the version in manifest.json, or remove the old one with reiwand-ctl lib rm %2"
            ),
            &[&version, &kept]
        ));
    }
    let failed = report["failed"][0]["error"]
        .as_str()
        .unwrap_or(&tr(told("the library refused the package")))
        .to_owned();
    let uid = uid.ok_or_else(|| anyhow!(failed))?;
    say(told("Showing it on the screen…"), &[]);
    client.library.apply(&uid, output, fit, "").await?;
    let shown = settle(&client, &uid, output).await?;
    let state: reiwand_core::model::State = serde_json::from_str(&client.daemon.get_state().await?)?;
    let ours = |name: &String| output.is_empty() || name == output;
    crate::levels::sandbox_lines(&state.outputs, ours).iter().for_each(|line| eprintln!("{line}"));
    println!("{}", applying::done(doc, &shown).local());
    if let Some(why) = applying::failed(&shown) {
        bail!(why.local());
    }
    Ok(())
}

async fn settle(client: &Client, uid: &str, output: &str) -> Result<Vec<applying::Screen>> {
    let ends = Instant::now() + std::time::Duration::from_millis(SETTLE_MS);
    let mut serial = client.daemon.serial().await?;
    loop {
        let state: Value = serde_json::from_str(&client.daemon.get_state().await?)?;
        let shown = applying::screens(&state, uid, output);
        if applying::settled(&shown) || Instant::now() >= ends {
            return Ok(shown);
        }
        (serial, _) = client.daemon.wait_changed(serial, 1000).await?;
    }
}

pub(super) async fn drafted(client: &Client) -> Result<Vec<String>> {
    let state: Value = serde_json::from_str(&client.daemon.get_state().await?)?;
    let outputs = state["outputs"].as_object().into_iter().flatten();
    Ok(outputs.filter(|(_, o)| o["drafted"] == true).map(|(name, _)| name.clone()).collect())
}

pub(super) async fn previewed(
    client: &Client,
    (before, output): (&[String], &str),
    stop: bool,
    doc: &Value,
) -> Result<()> {
    if stop {
        let ended = if output.is_empty() { before.join(", ") } else { output.to_owned() };
        match ended.is_empty() {
            true => say(told("No preview was running on any screen."), &[]),
            false => say(told("Ended the preview on %1."), &[&ended]),
        }
        return Ok(());
    }
    let title = applying::title(doc);
    let on = if output.is_empty() { drafted(client).await?.join(", ") } else { output.to_owned() };
    say(told("Previewing %1 on %2 for 60 seconds. End it with scene preview --stop."), &[&title, &on]);
    Ok(())
}

pub(super) fn apply_folder(dir: &Path, output: &str, span: bool) -> Result<()> {
    let mut project = Project::open(dir)?;
    ensure!(!project.dirty() && project.pending.is_none(), "{}", tr(told("save the scene first")));
    let out = package::applied(&project.doc)?;
    project.settle()?;
    eprintln!("{}", fill(told("Building the package…"), &[]));
    applying::build(&project.dir, &project.doc, &out)?;
    let path = out.to_str().context(tr(told("the path is not UTF-8")))?.to_owned();
    let fit = if span { "span" } else { "" };
    super::runtime()?.block_on(apply(&path, output, fit, &project.doc))
}
