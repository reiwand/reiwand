use crate::backend::qobject::{Backend, reap_deleted};
use cxx_qt_lib::QString;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub const CLAIMS: &str = "REIWAND_APP_CHECK_CLAIMS";
pub const SHARDS: &str = "REIWAND_APP_CHECK_SHARDS";
const MARK: &str = "reiwand-check ";
const MOST: usize = 3;

pub fn count() -> usize {
    let cores = std::thread::available_parallelism().map_or(1, usize::from);
    let wanted = std::env::var(SHARDS).ok().and_then(|n| n.parse::<usize>().ok());
    wanted.unwrap_or(cores / 2).clamp(1, MOST)
}

pub fn claim(unit: &str) -> bool {
    if !crate::checkpick::allowed(unit) {
        return false;
    }
    let dir = std::env::var_os(CLAIMS);
    let Some(taken) = claim_in(dir.as_deref().map(Path::new), unit) else {
        // Alone, a process runs every unit it may; a chosen walk still counts them.
        if std::env::var_os(crate::checkpick::ONLY).is_some() {
            eprintln!("{MARK}ran {unit}");
        }
        return true;
    };
    eprintln!("{MARK}{} {unit}", if taken { "ran" } else { "seen" });
    taken
}

pub fn claim_in(dir: Option<&Path>, unit: &str) -> Option<bool> {
    Some(std::fs::create_dir(dir?.join(unit)).is_ok())
}

#[derive(Default)]
pub struct Census {
    ran: BTreeMap<String, usize>,
    seen: BTreeSet<String>,
    listed: BTreeSet<String>,
}

impl Census {
    pub fn note(&mut self, line: &str) {
        let Some(rest) = line.strip_prefix(MARK) else { return };
        match rest.split_once(' ') {
            Some(("ran", unit)) => {
                *self.ran.entry(unit.to_owned()).or_default() += 1;
                self.seen.insert(unit.to_owned());
            }
            Some(("seen", unit)) => {
                self.seen.insert(unit.to_owned());
            }
            Some(("listed", unit)) => {
                self.listed.insert(unit.to_owned());
            }
            _ => {}
        }
    }

    pub fn problems(&self) -> Vec<String> {
        let skipped = self.seen.iter().filter(|unit| !self.ran.contains_key(*unit));
        let twice = self.ran.iter().filter(|(_, n)| **n > 1).map(|(unit, _)| unit);
        let skipped = skipped.map(|unit| format!("no process ran the check unit {unit}"));
        skipped.chain(twice.map(|unit| format!("two processes ran the check unit {unit}"))).collect()
    }

    pub fn units(&self) -> usize {
        self.seen.len()
    }

    /// The units a `--check --list` walk reached, by name.
    pub fn listed(&self) -> &BTreeSet<String> {
        &self.listed
    }
}

impl Backend {
    pub fn claim(&self, unit: &QString) -> bool {
        claim(&unit.to_string())
    }

    pub fn reap(&self) {
        reap_deleted();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn census(lines: &[&str]) -> Census {
        let mut census = Census::default();
        lines.iter().for_each(|line| census.note(line));
        census
    }

    #[test]
    fn a_unit_run_once_and_seen_by_the_others_is_complete() {
        let census = census(&[
            "reiwand-check ran scene.empty",
            "reiwand-check seen scene.empty",
            "reiwand-check seen scene.clock",
            "reiwand-check ran scene.clock",
            "a line that is not a mark",
        ]);
        assert!(census.problems().is_empty());
        assert_eq!(census.units(), 2);
    }

    #[test]
    fn a_unit_nobody_ran_is_reported() {
        let census = census(&["reiwand-check seen repo", "reiwand-check seen repo"]);
        assert_eq!(census.problems(), ["no process ran the check unit repo"]);
    }

    #[test]
    fn a_unit_run_twice_is_reported() {
        let census = census(&["reiwand-check ran image", "reiwand-check ran image"]);
        assert_eq!(census.problems(), ["two processes ran the check unit image"]);
    }

    #[test]
    fn only_the_first_claim_of_a_unit_wins() {
        let dir = std::env::temp_dir().join(format!("reiwand-app-claims-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(claim_in(Some(&dir), "scene.empty"), Some(true));
        assert_eq!(claim_in(Some(&dir), "scene.empty"), Some(false));
        assert_eq!(claim_in(Some(&dir), "scene.clock"), Some(true));
        assert_eq!(claim_in(None, "scene.empty"), None);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_scene_check_reaps_only_after_the_page_that_opened_it_returned() {
        let qml = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("qml");
        for entry in std::fs::read_dir(&qml).unwrap() {
            let path = entry.unwrap().path();
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            let named = path.file_name().unwrap() == "SceneCheck.qml";
            assert!(named || !text.contains("backend.reap("), "{}", path.display());
        }
        let page = std::fs::read_to_string(qml.join("ScenePage.qml")).unwrap();
        assert!(!page.contains("}).exercise()"));
        assert!(page.contains("interval: 0") && page.contains("check.exercise()"));
    }
}
