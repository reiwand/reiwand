use crate::actions::Action;
use crate::history::Step;
use crate::ops::Op;
use crate::project::Project;
use anyhow::{Result, ensure};
use reiwand_core::message::{Message, said};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn files(dir: &Path, at: &Path, out: &mut BTreeSet<PathBuf>) {
    for entry in std::fs::read_dir(at).into_iter().flatten().flatten() {
        let path = entry.path();
        match entry.file_type() {
            Ok(t) if t.is_dir() => files(dir, &path, out),
            Ok(_) => drop(out.insert(path.strip_prefix(dir).unwrap_or(&path).to_path_buf())),
            Err(_) => {}
        }
    }
}

fn listed(dir: &Path) -> BTreeSet<PathBuf> {
    let mut out = BTreeSet::new();
    files(dir, dir, &mut out);
    out
}

fn folded(steps: Vec<Step>) -> Option<Step> {
    let at = steps.last()?.at;
    let mut label: Vec<&str> = Vec::new();
    for step in &steps {
        if !label.contains(&step.label.as_str()) {
            label.push(&step.label);
        }
    }
    let label = match label[..] {
        [one] if steps.len() > 1 => format!("{one}*{}", steps.len()),
        [one] => one.to_owned(),
        _ => format!("batch:{}", label.join(",")),
    };
    let undo = steps.iter().rev().flat_map(|s| s.undo.iter().cloned()).collect();
    let redo = steps.iter().flat_map(|s| s.redo.iter().cloned()).collect();
    let size = steps.iter().map(|s| s.size).sum();
    Some(Step { label, undo, redo, at, size, gesture: false })
}

pub(crate) fn named(names: &str) -> Message {
    match names.split_once(',') {
        None => crate::history::step(names),
        Some((first, rest)) => said("editor.step.batch_then", "%1, %2", &[&"", &""])
            .with(1, crate::history::step(first))
            .with(2, named(rest)),
    }
}

impl Project {
    pub fn act_all(&mut self, actions: &[Action]) -> Result<Vec<Op>> {
        ensure!(
            !actions.iter().any(|a| matches!(a, Action::Undo {} | Action::Redo {})),
            said("editor.no_undo_in_step", "undo and redo cannot be part of a step with other actions", &[])
        );
        let (start, saved) = (self.history.done.len(), self.history.saved);
        let undone = std::mem::take(&mut self.history.undone);
        let existed = listed(&self.dir);
        let before = crate::meta::detected(&self.doc, Some(&self.dir));
        self.history.batching = true;
        let (mut ops, mut failed) = (Vec::new(), None);
        for action in actions {
            match self.act(action) {
                Ok(done) => ops.extend(done),
                Err(e) => {
                    failed = Some(e);
                    break;
                }
            }
        }
        // The manifest follows the whole batch at once: a sync per action read the whole scene
        // each time, so a batch of n actions took about n^1.5.
        for _ in 0..2 {
            if failed.is_some() || self.history.done.len() == start {
                break;
            }
            let follow = crate::meta::sync(&self.doc, &before, Some(&self.dir));
            if follow.is_empty() {
                break;
            }
            match self.history.extend(&mut self.doc, follow) {
                Ok(done) => ops.extend(done),
                Err(e) => failed = Some(e),
            }
        }
        if let Some(e) = failed {
            let mut back = Vec::new();
            while self.history.done.len() > start {
                match self.history.undo(&mut self.doc) {
                    Ok(Some((_, ops))) => back.extend(ops),
                    _ => break,
                }
            }
            // like Undo, scripts go back to the code their recorded version names, and the
            // files the batch's actions created leave the folder
            crate::scripts::restore(&self.dir, &self.doc, &back);
            for made in listed(&self.dir).difference(&existed) {
                drop(std::fs::remove_file(self.dir.join(made)));
            }
            (self.history.undone, self.history.saved, self.history.batching) = (undone, saved, false);
            return Err(e);
        }
        self.history.batching = false;
        if let Some(step) = folded(self.history.done.split_off(start)) {
            self.history.done.push(step);
            self.history.undone.clear();
            if self.history.saved.is_some_and(|s| s > self.history.done.len()) {
                self.history.saved = None;
            }
            self.history.trim();
        } else {
            self.history.undone = undone;
        }
        Ok(ops)
    }
}
