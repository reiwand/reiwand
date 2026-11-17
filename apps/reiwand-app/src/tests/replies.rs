use super::ifaces::iface;
use super::{literals, qml, qml_files};
use crate::methods::METHODS;
use crate::service::SERVICE;
use std::path::Path;

const RETIRED: &str = "Sync1";

#[test]
fn qml_calls_name_documented_methods() {
    for line in qml().iter().flat_map(|file| file.lines().map(String::from).collect::<Vec<_>>()) {
        for member in literals(&line, ".query(").into_iter().chain(literals(&line, ".act(")) {
            let iface = iface(&line);
            if iface == RETIRED {
                continue;
            }
            let known = METHODS.contains_key(&format!("{iface}.{member}"))
                || SERVICE.iter().any(|(i, m, _)| (*i, *m) == (iface, member));
            assert!(known, "{iface}.{member} in {line}");
        }
    }
}

#[test]
fn tray_calls_carry_their_signatures() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../integrations/plasma-tray/contents/ui");
    let files = std::fs::read_dir(dir).unwrap().map(|f| std::fs::read_to_string(f.unwrap().path()).unwrap());
    let text: String = files.collect();
    for line in text.lines() {
        for member in literals(line, "act(").into_iter().chain(literals(line, "query(")) {
            let iface = iface(line);
            let call = METHODS.get(&format!("{iface}.{member}"));
            let args = &call.unwrap_or_else(|| panic!("{iface}.{member} in {line}")).args;
            let signature =
                text.split(&format!("\"{iface}.{member}\": \"(")).nth(1).and_then(|s| s.split(')').next());
            assert_eq!(
                signature.unwrap_or_default().replace("as", "s").len(),
                args.len(),
                "{iface}.{member}"
            );
        }
    }
}

fn close(text: &str, open: usize) -> usize {
    let (mut depth, mut quote, mut escaped) = (0, None, false);
    for (i, c) in text.char_indices().skip_while(|(i, _)| *i < open) {
        match (quote, c) {
            (Some(_), _) if escaped => escaped = false,
            (Some(_), '\\') => escaped = true,
            (Some(q), c) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'' | '`') => quote = Some(c),
            (None, '(' | '[' | '{') => depth += 1,
            (None, ')' | ']' | '}') => {
                depth -= 1;
                if depth == 0 {
                    return i;
                }
            }
            _ => {}
        }
    }
    panic!("unbalanced call at {open}")
}

fn split(text: &str) -> Vec<&str> {
    let (mut parts, mut start, mut at) = (Vec::new(), 0, 0);
    while let Some(i) = text[at..].find([',', '(', '[', '{', '"', '\'']).map(|i| i + at) {
        match text.as_bytes()[i] {
            b',' => {
                parts.push(&text[start..i]);
                (start, at) = (i + 1, i + 1);
            }
            b'"' | b'\'' => at = i + 1 + text[i + 1..].find(text.as_bytes()[i] as char).unwrap_or(0) + 1,
            _ => at = close(text, i) + 1,
        }
    }
    parts.push(&text[start..]);
    parts
}

fn mentions(body: &str, id: &str) -> bool {
    body.match_indices(id).any(|(i, _)| {
        let before =
            body[..i].chars().next_back().is_none_or(|c| !c.is_alphanumeric() && c != '_' && c != '.');
        before && body[i + id.len()..].starts_with(['.', '[', '('])
    })
}

#[test]
fn replies_check_that_their_owner_is_still_there() {
    for (name, text) in qml_files() {
        let ids: Vec<&str> = text.lines().filter_map(|l| l.trim().strip_prefix("id: ")).collect();
        for call in [".query(", ".act("] {
            for (at, _) in text.match_indices(call) {
                let open = at + call.len() - 1;
                for arg in split(&text[open + 1..close(&text, open)]).into_iter().skip(3) {
                    let Some((params, body)) = arg.split_once("=>").filter(|(p, _)| !p.contains('"')) else {
                        continue;
                    };
                    let (params, body) = (params.trim(), body.trim());
                    let used = ids.iter().filter(|id| mentions(body, id) && !params.contains(**id));
                    let guarded = |id: &&&str| {
                        body.starts_with(&format!("{id} && "))
                            || body.strip_prefix('{').is_some_and(|b| {
                                let b = b.trim_start();
                                b.starts_with(&format!("if (!{id})"))
                                    || b.starts_with(&format!("if (!{id} ||"))
                            })
                    };
                    assert!(
                        used.clone().next().is_none() || used.clone().any(|id| guarded(&id)),
                        "{name}: {arg}"
                    );
                }
            }
        }
    }
}

#[test]
fn texts_take_their_arguments_in_one_pass() {
    for (name, text) in qml_files() {
        assert!(!text.contains(".arg("), "{name} replaces arguments one after another");
    }
    let app = qml_files().into_iter().find(|(name, _)| name == "App.qml").unwrap().1;
    assert!(app.contains("const args = arguments;") && app.contains(".replace(/%([1-9])/g"));
}

#[test]
fn text_areas_keep_a_frame_in_every_style() {
    for (name, text) in qml_files().into_iter().filter(|(name, _)| name != "Area.qml") {
        assert!(!text.contains("TextArea {"), "{name} uses a text area without a frame");
    }
}

#[test]
fn every_documented_method_can_be_sent() {
    let doc = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/specs/dbus.md"));
    let doc = doc.unwrap();
    let sections = doc.split("\n## `io.github.reiwand.").skip(1).map(|s| s.split("\n## ").next().unwrap());
    let headings = sections.map(|s| s.matches("\n### `").count()).sum::<usize>();
    assert_eq!(METHODS.len(), headings);
    let sent = ["s", "b", "u", "i", "t", "d", "as", "au", "ay", "a{ss}"];
    for (name, method) in METHODS.iter() {
        assert!(method.args.iter().all(|(s, _)| sent.contains(&s.as_str())), "{name}");
        assert!(method.reply.iter().all(|(s, n)| !s.is_empty() && !n.is_empty()), "{name}");
    }
    assert!(METHODS.len() > 150, "{}", METHODS.len());
}

#[test]
fn animated_previews_play_only_when_shown_allowed_and_on_mains() {
    let tray = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../integrations/plasma-tray/contents/ui");
    let app = |name: &str| qml_files().into_iter().find(|(n, _)| n == name).unwrap().1;
    for (text, shown) in [
        (app("Thumbnail.qml"), "thumbnail.animated && !thumbnail.mature"),
        (std::fs::read_to_string(tray.join("Thumbnail.qml")).unwrap(), "thumbnail.animated !== \"\""),
    ] {
        assert!(text.contains("AnimatedImage {") && text.contains(shown));
        assert!(text.contains(
            "playing: visible && thumbnail.moving && thumbnail.Window.visibility !== Window.Hidden"
        ));
    }
    for text in [app("OutputRow.qml"), std::fs::read_to_string(tray.join("ScreenRow.qml")).unwrap()] {
        assert!(text.contains("animated: row.pkg.photosensitive === true && !row.allowed ? \"\" :"));
        assert!(text.contains("moving: !(row.daemon.state?.on_battery ?? false)"));
    }
    assert!(app("ItemHost.qml").contains("readonly property string animated: flashing ? \"\" :"));
    assert!(app("LibraryPanel.qml").contains("playing: visible && !(App.state.on_battery ?? false)"));
}

#[test]
fn the_installed_applications_fixture_follows_the_data_model() {
    let found: reiwand_core::model::ApplicationsReply =
        reiwand_client::parse(&crate::args::fixture("Daemon1", "ListApplications")).unwrap();
    assert!(found.available && found.applications.len() == 2);
}

#[test]
fn the_fixtures_refuse_a_page_the_service_refuses() {
    use serde_json::json;
    let get = |limit: u32| {
        let args = vec![json!("Sea"), json!(0), json!(limit)];
        crate::bus::block_on(crate::methods::dispatch(
            &crate::bus::Daemon::Fixtures,
            "Playlists1",
            "Get",
            args,
        ))
    };
    assert!(get(500).is_ok());
    assert!(get(501).unwrap_err().message.contains("at most 500"));
}
