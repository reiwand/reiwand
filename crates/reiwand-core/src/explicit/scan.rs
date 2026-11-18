use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    SerdeDefault,
    SkipSerializing,
    UnwrapOr,
    DeriveDefault,
    OptionWithoutNull,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub file: PathBuf,
    pub line: usize,
    pub rule: Rule,
}

fn attribute_block(lines: &[&str], at: usize) -> String {
    let mut block = String::new();
    for line in lines[..at].iter().rev() {
        let t = line.trim();
        if !t.starts_with("#[") && !t.starts_with("///") && !t.starts_with("#![") {
            break;
        }
        block.push_str(t);
    }
    block
}

fn option_field(t: &str) -> bool {
    let t = t.strip_prefix("pub ").unwrap_or(t);
    let t = t.strip_prefix("pub(crate) ").unwrap_or(t);
    t.split_once(": Option<")
        .is_some_and(|(name, _)| !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
}

fn check(file: &Path, text: &str, rules: &[Rule], out: &mut Vec<Violation>) {
    let lines: Vec<&str> = text.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        if t.starts_with("#[cfg(test)]") {
            break;
        }
        let mut hit = |rule: Rule, yes: bool| {
            if yes && rules.contains(&rule) {
                out.push(Violation { file: file.to_owned(), line: i + 1, rule });
            }
        };
        let marked = t.contains("explicit::block") || t.contains("deserialize_with = \"block\"");
        hit(
            Rule::SerdeDefault,
            !marked && (t.contains("serde(default") || t.contains("serde(alias") && t.contains("default")),
        );
        hit(Rule::SkipSerializing, !marked && t.contains("skip_serializing_if"));
        hit(
            Rule::UnwrapOr,
            t.contains(".unwrap_or(") || t.contains(".unwrap_or_default(") || t.contains(".unwrap_or_else("),
        );
        hit(
            Rule::DeriveDefault,
            t.starts_with("#[derive(") && t.split([',', '(', ')', ' ']).any(|w| w == "Default"),
        );
        hit(Rule::DeriveDefault, t.starts_with("impl Default for "));
        let block = attribute_block(&lines, i);
        let written = [
            "deserialize_with = \"null\"",
            "explicit::null",
            "deserialize_with = \"block\"",
            "explicit::block",
        ]
        .iter()
        .any(|m| block.contains(m));
        let loose = option_field(t) && !written;
        hit(Rule::OptionWithoutNull, loose);
    }
}

fn walk(dir: &Path, rules: &[Rule], out: &mut Vec<Violation>) {
    if dir.is_file() {
        if let Ok(text) = fs::read_to_string(dir) {
            check(dir, &text, rules, out);
        }
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else { return };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| Some(e.ok()?.path())).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            walk(&path, rules, out);
        } else if path.extension().is_some_and(|e| e == "rs")
            && let Ok(text) = fs::read_to_string(&path)
        {
            check(&path, &text, rules, out);
        }
    }
}

pub fn scan(dirs: &[&Path], rules: &[Rule]) -> Vec<Violation> {
    let mut out = Vec::new();
    for dir in dirs {
        walk(dir, rules, &mut out);
    }
    out
}
