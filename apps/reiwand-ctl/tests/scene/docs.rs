use reiwand_editor::{ACTIONS, manual};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURES: [(&str, &str); 7] = [
    ("picture.png", "scenes/pulse/assets/cover.png"),
    ("sheet.png", "tests/fixtures/bands-640x180.png"),
    ("sound.ogg", "scenes/pond/assets/water.ogg"),
    ("model.glb", "tests/fixtures/rig.glb"),
    ("figure.glb", "tests/fixtures/toy.glb"),
    ("pieces.glb", "tests/fixtures/parts.glb"),
    ("video.webm", "tests/fixtures/magenta-64x32.webm"),
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

// The pages of docs/authoring whose name starts with `prefix`: each page and, since the long
// pages were split, the pages in the folder of the same name, named by their path from there.
fn pages(prefix: &str) -> Vec<(String, String)> {
    let base = root().join("docs/authoring");
    let mut found: Vec<_> = std::fs::read_dir(&base)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.file_name().unwrap().to_string_lossy().starts_with(prefix))
        .flat_map(|p| markdown(&p))
        .map(|p| {
            let name = p.strip_prefix(&base).unwrap().to_string_lossy().into_owned();
            (name, std::fs::read_to_string(&p).unwrap())
        })
        .collect();
    found.sort();
    found
}

// A page with the pages of its folder after it, as one text (`docs/<page>` and `docs/<page stem>/`).
pub fn whole(page: &str) -> String {
    let path = root().join("docs").join(page);
    let mut parts = markdown(&path.with_extension(""));
    parts.sort();
    std::iter::once(path).chain(parts).map(|p| std::fs::read_to_string(p).unwrap()).collect()
}

fn markdown(path: &Path) -> Vec<PathBuf> {
    match std::fs::read_dir(path) {
        Ok(dir) => dir.flat_map(|e| markdown(&e.unwrap().path())).collect(),
        Err(_) if path.extension().is_some_and(|e| e == "md") => vec![path.to_owned()],
        Err(_) => Vec::new(),
    }
}

fn blocks(text: &str) -> Vec<String> {
    let mut all = Vec::new();
    let mut open: Option<String> = None;
    for line in text.lines() {
        match (&mut open, line.trim_end()) {
            (None, "```sh") => open = Some(String::new()),
            (Some(_), "```") => all.extend(open.take()),
            (Some(block), _) => {
                block.push_str(line);
                block.push('\n');
            }
            _ => {}
        }
    }
    all
}

// An entry of the reference: a heading of any level (the split pages raised some) with the name alone.
fn headed(text: &str, name: &str) -> bool {
    text.lines().any(|l| l.starts_with('#') && l.trim_start_matches('#') == format!(" `{name}`"))
}

#[test]
fn every_action_and_query_has_an_entry_and_an_example_in_the_reference() {
    let text: String = pages("actions").into_iter().map(|(_, t)| t).collect();
    let mut missing = Vec::new();
    for name in ACTIONS {
        let called = text.lines().any(|l| {
            l.strip_prefix(&format!("reiwand-ctl scene act my-scene {name} ")).is_some()
                || l == format!("reiwand-ctl scene act my-scene {name}")
        });
        if !called || !headed(&text, name) {
            missing.push(format!("action {name}"));
        }
    }
    for (name, ..) in manual::ASKS {
        let called = text.lines().any(|l| {
            l.strip_prefix(&format!("reiwand-ctl scene ask my-scene {name}"))
                .is_some_and(|r| r.is_empty() || r.starts_with(' '))
        });
        if !called || !headed(&text, name) {
            missing.push(format!("query {name}"));
        }
    }
    assert!(missing.is_empty(), "docs/authoring/actions*.md lacks: {missing:?}");
    let index = std::fs::read_to_string(root().join("docs/authoring/actions.md")).unwrap();
    let unnamed: BTreeSet<_> = ACTIONS
        .iter()
        .copied()
        .chain(manual::ASKS.iter().map(|(n, ..)| *n))
        .filter(|n| !index.contains(&format!("[{n}](")))
        .collect();
    assert!(unnamed.is_empty(), "docs/authoring/actions.md does not index {unnamed:?}");
}

fn run_block(block: &str, bin: &Path) -> Result<(), String> {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    for (name, from) in FIXTURES {
        std::fs::copy(root().join(from), dir.join(name)).unwrap();
    }
    std::fs::create_dir(dir.join("bin")).unwrap();
    std::os::unix::fs::symlink(bin, dir.join("bin/reiwand-ctl")).unwrap();
    let path = format!("{}:{}", dir.join("bin").display(), std::env::var("PATH").unwrap_or_default());
    let script = if block.contains("scene new ") {
        block.to_owned()
    } else {
        format!("reiwand-ctl scene new my-scene >/dev/null\n{block}")
    };
    let out = Command::new("bash")
        .args(["-e", "-o", "pipefail", "-c", &script])
        .current_dir(dir)
        .env("PATH", path)
        .env("LC_ALL", "C")
        .env("XDG_STATE_HOME", dir.join("state"))
        .env("DBUS_SESSION_BUS_ADDRESS", format!("unix:path={}/no-bus", dir.display()))
        .output()
        .unwrap();
    match out.status.success() {
        true => Ok(()),
        false => Err(format!("{block}\n{}", String::from_utf8_lossy(&out.stderr))),
    }
}

#[test]
fn every_example_line_of_the_creator_pages_works() {
    let bin = PathBuf::from(env!("CARGO_BIN_EXE_reiwand-ctl"));
    let mut failed = Vec::new();
    let mut count = 0;
    let creator = pages("editor").into_iter().filter(|(page, _)| !page.starts_with("editor-features"));
    for (page, text) in pages("actions").into_iter().chain(creator) {
        for block in blocks(&text) {
            count += 1;
            if let Err(why) = run_block(&block, &bin) {
                failed.push(format!("{page}: {why}"));
            }
        }
    }
    assert!(count > 100, "only {count} example blocks found");
    assert!(failed.is_empty(), "{} blocks fail:\n{}", failed.len(), failed.join("\n---\n"));
}
