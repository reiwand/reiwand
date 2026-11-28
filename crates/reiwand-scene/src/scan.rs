use crate::{Script, ScriptApi};

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Word(&'a str),
    Text(String),
    Mark(char),
}

fn tokens(source: &str) -> Vec<Token<'_>> {
    let (bytes, mut out, mut i) = (source.as_bytes(), Vec::new(), 0);
    let until =
        |from: usize, end: &str| source[from..].find(end).map_or(source.len(), |at| from + at + end.len());
    while i < bytes.len() {
        let c = bytes[i];
        if source[i..].starts_with("//") {
            i = until(i, "\n");
        } else if source[i..].starts_with("/*") {
            i = until(i + 2, "*/");
        } else if matches!(c, b'"' | b'\'' | b'`') {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] != c {
                j += if bytes[j] == b'\\' { 2 } else { 1 };
            }
            out.push(Token::Text(source[i + 1..j.min(bytes.len())].to_owned()));
            i = j + 1;
        } else if c.is_ascii_alphanumeric() || c == b'_' || c == b'$' {
            let end = source[i..].find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '$'));
            let end = end.map_or(source.len(), |e| i + e);
            out.push(Token::Word(&source[i..end]));
            i = end;
        } else {
            if !c.is_ascii_whitespace() {
                out.push(Token::Mark(c as char));
            }
            i += source[i..].chars().next().map_or(1, char::len_utf8);
        }
    }
    out
}

fn api(name: &str) -> Option<ScriptApi> {
    Some(match name {
        "audio" => ScriptApi::Audio,
        "input" => ScriptApi::Pointer,
        "particles" => ScriptApi::Particles,
        "setTimeout" | "setInterval" | "clearTimeout" | "clearInterval" => ScriptApi::Timers,
        "console" => ScriptApi::Console,
        "shared" => ScriptApi::Shared,
        "sound" => ScriptApi::Sounds,
        "storage" => ScriptApi::Storage,
        "folder" => ScriptApi::Folders,
        "openShortcut" => ScriptApi::Shortcuts,
        "media" => ScriptApi::Media,
        _ => return None,
    })
}

fn braced<'a>(list: &[Token<'a>]) -> Vec<(&'a str, &'a str)> {
    let inner = list.iter().skip_while(|t| **t != Token::Mark('{')).skip(1);
    let inner: Vec<&Token> = inner.take_while(|t| **t != Token::Mark('}')).collect();
    let words = |part: &[&Token<'a>]| -> Vec<&'a str> {
        part.iter().filter_map(|t| if let Token::Word(w) = t { Some(*w) } else { None }).collect()
    };
    let parts = inner.split(|t| **t == Token::Mark(','));
    parts.map(words).filter_map(|w| Some((*w.first()?, *w.last()?))).collect()
}

fn modules<'a, 'b>(all: &'b [Token<'a>]) -> Vec<(&'b [Token<'a>], &'b str)> {
    let mut found = Vec::new();
    for (i, t) in all.iter().enumerate() {
        let (Token::Word("from"), Some(Token::Text(m))) = (t, all.get(i + 1)) else { continue };
        let start = all[..i].iter().rposition(|t| matches!(t, Token::Word("import" | "export")));
        found.extend(start.filter(|s| i - s <= 256).map(|s| (&all[s + 1..i], m.as_str())));
    }
    found
}

pub fn imported(source: &str) -> Vec<ScriptApi> {
    let all = tokens(source);
    let mut found = Vec::new();
    for (names, module) in modules(&all) {
        match module {
            "reiwand/math" => found.push(ScriptApi::Math),
            "reiwand" if names.contains(&Token::Mark('*')) => {
                found.extend([ScriptApi::Audio, ScriptApi::Pointer])
            }
            "reiwand" => found.extend(braced(names).into_iter().filter_map(|(name, _)| api(name))),
            _ => found.push(ScriptApi::Shared),
        }
    }
    let loose = all
        .windows(2)
        .any(|w| w[0] == Token::Word("import") && matches!(w[1], Token::Text(_) | Token::Mark('(')));
    found.extend(loose.then_some(ScriptApi::Shared));
    found.sort();
    found.dedup();
    found
}

impl Script {
    pub fn declares(&self) -> bool {
        !self.uses.is_empty() || !self.events.is_empty() || !self.params.is_empty()
    }

    pub fn effective(&self, source: &str) -> Vec<ScriptApi> {
        if self.declares() { self.uses.clone() } else { imported(source) }
    }
}
