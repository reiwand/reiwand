use crate::findings::{Problem, problem};
use reiwand_core::message::said;
use reiwand_scene::Scene;
use reiwand_script::Fault;
use std::sync::{Mutex, PoisonError};

type Compiled = (Vec<(String, String)>, Vec<(usize, Fault)>);

static LAST: Mutex<Option<Compiled>> = Mutex::new(None);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptFault {
    pub index: usize,
    pub path: String,
    pub line: Option<u32>,
    pub message: String,
    pub missing: bool,
}

impl ScriptFault {
    pub fn said(&self) -> reiwand_core::message::Message {
        match (self.missing, self.line) {
            (true, _) => said(
                "editor.script_missing",
                "%1 is listed in scripts but is not in the folder",
                &[&self.path],
            ),
            (false, Some(line)) => {
                said("editor.script_syntax_at", "%1:%2: %3", &[&self.path, &line, &self.message])
            }
            (false, None) => said("editor.script_syntax", "%1: %2", &[&self.path, &self.message]),
        }
    }
}

pub fn faults(scene: &Scene, read: impl Fn(&str) -> Option<String>) -> Vec<ScriptFault> {
    let mut out = Vec::new();
    let mut sources = Vec::new();
    let mut owners = Vec::new();
    for (index, script) in scene.scripts.iter().enumerate() {
        match read(&script.path) {
            Some(text) => {
                sources.push((script.path.clone(), text));
                owners.push(index);
            }
            None => out.push(ScriptFault {
                index,
                path: script.path.clone(),
                line: None,
                message: String::new(),
                missing: true,
            }),
        }
    }
    for (at, fault) in compiled(sources) {
        let index = owners[at];
        let path = scene.scripts[index].path.clone();
        out.push(ScriptFault { index, path, line: fault.line, message: fault.message, missing: false });
    }
    out.sort_by_key(|f| f.index);
    out
}

fn compiled(sources: Vec<(String, String)>) -> Vec<(usize, Fault)> {
    let mut last = LAST.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some((seen, found)) = last.as_ref()
        && *seen == sources
    {
        return found.clone();
    }
    let found = reiwand_script::syntax(&sources);
    *last = Some((sources, found.clone()));
    found
}

pub fn problems(scene: &Scene, dir: &std::path::Path) -> Vec<Problem> {
    let read = |path: &str| std::fs::read_to_string(dir.join(path)).ok();
    let found = faults(scene, read);
    found.iter().map(|f| problem(format!("/scene/scripts/{}/path", f.index), f.said())).collect()
}
