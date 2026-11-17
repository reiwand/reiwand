use super::gather::gather;
use super::local::local_of;
use super::stage::open;
use crate::Error;
use crate::providers::carried::keys_held;
use reiwand_carry::preview as rows_of;
use serde_json::{Value, json};

pub(super) fn preview(path: &str, password: &str) -> Result<Value, Error> {
    let opened = open(path, password)?;
    let content = &opened.content;
    let local = local_of(&gather()?.content, keys_held());
    let rows: Vec<Value> = (rows_of(content, &local).rows.iter())
        .map(|r| json!({"section": r.section, "held": r.held, "new": r.new, "skipped": r.skipped, "conflicts": r.conflicts}))
        .collect();
    let m = &opened.manifest;
    let file = json!({"program": m.program, "version": m.version, "created": m.created, "encrypted": m.encrypted, "sections": m.sections});
    let lists: Vec<&str> = content.lists.iter().map(|l| l.name.as_str()).collect();
    let sessions = content.secrets.as_ref().map_or(0, |s| s.sessions.len());
    let notes: Vec<&str> = if sessions > 0 { vec!["android.carry_sessions"] } else { Vec::new() };
    Ok(json!({"file": file, "rows": rows, "lists": lists, "notes": notes}))
}
