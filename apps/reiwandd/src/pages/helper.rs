use crate::daemon::Daemon;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const NAME: &str = "reiwand-web";

static PROBED: OnceLock<Result<String, String>> = OnceLock::new();

pub fn helper() -> Option<PathBuf> {
    let given = std::env::var_os("REIWAND_WEB").map(PathBuf::from);
    let beside = std::env::current_exe().ok().and_then(|exe| Some(exe.parent()?.join(NAME)));
    let path = std::env::var_os("PATH").unwrap_or_default();
    let searched = std::env::split_paths(&path).map(|dir| dir.join(NAME));
    given.into_iter().chain(beside).chain(searched).find(|p| p.is_file())
}

fn probe() -> Result<String, String> {
    let Some(path) = helper() else { return Err(crate::words::web_missing()) };
    let output = std::process::Command::new(&path).arg("--check").stdin(std::process::Stdio::null()).output();
    let output = output.map_err(|e| crate::words::web_broken(&e.to_string()))?;
    let said = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let error = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    match output.status.success() {
        true => Ok(said),
        false => Err(refusal(&said, &error, output.status.code(), crate::flatpak::app_id().is_some())),
    }
}

const ANSWERED: i32 = 2;
const MISSING: &str = "Qt WebEngine is missing:";

fn refusal(said: &str, error: &str, code: Option<i32>, flatpak: bool) -> String {
    if code != Some(ANSWERED) || said.is_empty() {
        let detail = if error.is_empty() { said } else { error };
        let detail = detail.lines().next().unwrap_or_default();
        let engine = crate::words::web_engine(flatpak);
        return match (detail.is_empty(), code) {
            (true, Some(code)) => format!("{engine} (exit {code})"),
            (true, None) => engine,
            (false, _) => format!("{engine} ({detail})"),
        };
    }
    let reason = said.strip_prefix(MISSING).unwrap_or(said).trim();
    match reason {
        r if r.contains("older than") => crate::words::web_old(r),
        _ if said.starts_with(MISSING) => crate::words::web_unbuilt(reason),
        _ => crate::words::web_broken(said),
    }
}

pub fn available() -> Result<String, String> {
    PROBED.get_or_init(probe).clone()
}

pub fn unavailable() -> Option<String> {
    available().err()
}

impl Daemon {
    pub(crate) fn page_problem(&self, path: &Path) -> Option<String> {
        let status = self.compositor.status();
        let shown = status.values().filter(|s| s.package.as_deref() == Some(path));
        let shown: Vec<_> = shown.collect();
        let last =
            shown.iter().flat_map(|s| s.problems.last().map(|l| (s.kind == "scene", l))).next_back()?;
        Some(if last.0 {
            crate::words::scene_script_error(last.1)
        } else {
            crate::words::web_page_error(last.1)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::refusal;

    #[test]
    fn a_helper_that_does_not_load_is_a_missing_engine_with_its_detail_apart() {
        let loader = "libQt6WebEngineQuick.so.6: cannot open shared object file";
        let (flatpak, other) = (refusal("", loader, Some(127), true), refusal("", loader, Some(127), false));
        assert!(flatpak.contains("Flatpak") && flatpak.contains(loader), "{flatpak}");
        assert!(other.contains("Ubuntu 24.04") && other.ends_with(&format!("({loader})")), "{other}");
        let killed = refusal("", "", None, false);
        assert!(killed.contains("Qt WebEngine 6.8") && !killed.contains("()"), "{killed}");
        assert!(refusal("", "", Some(127), false).contains("exit 127"));
    }

    #[test]
    fn the_answers_of_the_helper_itself_keep_their_own_reason() {
        let old = refusal("Qt WebEngine is missing: Qt 6.4.2 is older than 6.8", "", Some(2), false);
        assert!(old.contains("6.4.2") && !old.contains("missing"), "{old}");
        let unbuilt = refusal("Qt WebEngine is missing: qmake was not found", "", Some(2), false);
        assert!(unbuilt.contains("built without") && unbuilt.contains("qmake was not found"), "{unbuilt}");
        assert!(refusal("boom", "", Some(2), true).contains("boom"));
    }
}
