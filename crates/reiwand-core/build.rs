use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let mut langs: Vec<_> = fs::read_dir("../../po").unwrap().map(|e| e.unwrap().path()).collect();
    langs.sort();
    let catalogs: String =
        (langs.iter().map(|dir| (dir, dir.join("reiwand-messages.po"))).filter(|(_, po)| po.is_file()))
            .map(|(dir, po)| {
                format!("({:?}, include_str!({:?})),", dir.file_name().unwrap(), po.canonicalize().unwrap())
            })
            .collect();
    fs::write(PathBuf::from(env::var("OUT_DIR").unwrap()).join("catalogs.rs"), format!("&[{catalogs}]"))
        .unwrap();
    println!("cargo::rerun-if-changed=../../po");
    fs::write(PathBuf::from(env::var("OUT_DIR").unwrap()).join("dbus.md"), dbus_spec()).unwrap();
    println!("cargo::rerun-if-changed=../../docs/specs/dbus");
}

/// The D-Bus interfaces as one text, the way docs/specs/dbus.md held them before it was split into
/// pages: "## `io.github.reiwand.X`" for each interface, "### `Member(…)`" for each member, whatever
/// level a page heads them at; a page holds one interface under its title, or several under headings.
fn dbus_spec() -> String {
    let mut pages = Vec::new();
    // Builds from a copy without docs/ (the server image, the phone library) get an empty text;
    // only the checks of the source tree read it.
    let mut dirs = vec![PathBuf::from("../../docs/specs/dbus")];
    while let Some(dir) = dirs.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for path in entries.map(|e| e.unwrap().path()) {
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().is_some_and(|e| e == "md") {
                pages.push(path);
            }
        }
    }
    pages.sort();
    let mut whole = String::new();
    for page in pages {
        let text = fs::read_to_string(&page).unwrap();
        let Some(rest) = text.strip_prefix("# `io.github.reiwand.") else { continue };
        let (name, body) = rest.split_once('`').unwrap_or_default();
        let body = body.split_once('\n').map_or("", |(_, body)| body);
        whole += &format!("\n## `io.github.reiwand.{name}`\n");
        for line in body.lines() {
            let heading = line.trim_start_matches('#');
            let text = heading.trim_start();
            whole += &match () {
                _ if heading.len() == line.len() || !heading.starts_with(' ') => line.to_owned(),
                _ if text.starts_with("`io.github.reiwand.") => format!("## {text}"),
                _ if member(text) => format!("### {text}"),
                _ => format!("#### {text}"),
            };
            whole.push('\n');
        }
    }
    whole
}

/// A heading of a member: a method "`Name(…)`", a signal "Signal `Name(…)`" or a property
/// "Property `name`".
fn member(text: &str) -> bool {
    (text.starts_with('`') && text.contains('('))
        || text.starts_with("Signal `")
        || text.starts_with("Property `")
}
