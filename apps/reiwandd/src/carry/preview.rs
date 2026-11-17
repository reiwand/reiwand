use super::local::local_of;
use super::stage::open;
use crate::daemon::Daemon;
use crate::error::Result;
use reiwand_carry::{Content, preview};
use serde_json::{Value, json};

impl Daemon {
    pub fn carry_preview(&self, path: &str, password: &str) -> Result<String> {
        let opened = open(path, password)?;
        let content: &Content = &opened.content;
        let local =
            local_of(&self.carry_state()?.content, self.carry_keys_held(), self.carry_sessions_held());
        let rows: Vec<Value> = preview(content, &local)
            .rows
            .iter()
            .map(|r| {
                json!({"section": r.section, "held": r.held, "new": r.new, "skipped": r.skipped, "conflicts": r.conflicts})
            })
            .collect();
        let m = &opened.manifest;
        let file = json!({"program": m.program, "version": m.version, "created": m.created,
            "encrypted": m.encrypted, "sections": m.sections});
        let lists: Vec<&str> = content.lists.iter().map(|l| l.name.as_str()).collect();
        let notes: Vec<&str> = Vec::new();
        Ok(json!({"file": file, "rows": rows, "lists": lists, "notes": notes}).to_string())
    }
}
