use std::path::Path;

fn sources(dir: &Path, out: &mut Vec<String>) {
    if dir.is_file() {
        out.extend(std::fs::read_to_string(dir));
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.extend(std::fs::read_to_string(&path));
        }
    }
}

fn body<'a>(text: &'a str, ty: &str) -> Vec<&'a str> {
    let mut found = Vec::new();
    for (at, _) in text.match_indices(ty) {
        let line_start = text[..at].rfind('\n').map_or(0, |n| n + 1);
        let head = text[line_start..at].trim_start();
        let after = text[at + ty.len()..].chars().next();
        let named = matches!(head, "pub struct " | "struct " | "pub enum " | "enum ")
            || head.ends_with("struct ")
            || head.ends_with("enum ");
        if !named || after.is_some_and(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let indent = &text
            [line_start..line_start + (text[line_start..].len() - text[line_start..].trim_start().len())];
        let Some(open) = text[at..].find('{') else { continue };
        let start = at + open;
        let Some(close) = text[start..].find(&format!("\n{indent}}}")) else { continue };
        found.push(&text[start..start + close]);
    }
    found
}

fn class_of(attributes: &str) -> Option<&'static str> {
    let block = attributes.contains("explicit::block")
        || attributes.contains("deserialize_with = \"block\"")
        || (attributes.contains("default")
            && attributes.contains("skip_serializing_if = \"Option::is_none\""));
    let value = attributes.contains("explicit::null")
        || attributes.contains("deserialize_with = \"null\"")
        || ["general", "empty", "none", "auto", "namespace"]
            .iter()
            .any(|w| attributes.contains(&format!("with = \"crate::explicit::{w}\"")));
    match (block, value) {
        (true, false) => Some("bloque"),
        (false, true) => Some("valor"),
        _ => None,
    }
}

fn field_class(chunk: &str, field: &str) -> Vec<Option<&'static str>> {
    let lines: Vec<&str> = chunk.lines().collect();
    let mut found = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim_start().trim_start_matches("pub(super) ").trim_start_matches("pub(crate) ");
        let t = t.trim_start_matches("pub ");
        if !t.starts_with(&format!("{field}:")) || !t.contains("Option<") {
            continue;
        }
        let mut attributes = String::new();
        let mut j = i;
        let mut open = false;
        while j > 0 {
            let above = lines[j - 1].trim_start();
            if above.starts_with(")]") {
                open = true;
            } else if !open && !above.starts_with("#[") {
                break;
            }
            if above.starts_with("#[") {
                open = false;
            }
            j -= 1;
            attributes.push_str(lines[j]);
        }
        found.push(class_of(&attributes));
    }
    found
}

pub fn declared(dir: &Path, ty: Option<&str>, field: &str) -> Option<&'static str> {
    let mut texts = Vec::new();
    sources(dir, &mut texts);
    let mut classes = Vec::new();
    for text in &texts {
        match ty {
            Some(ty) => body(text, ty).iter().for_each(|b| classes.extend(field_class(b, field))),
            None => classes.extend(field_class(text, field)),
        }
    }
    let first = *classes.first()?;
    classes.iter().all(|c| *c == first).then_some(first).flatten()
}
