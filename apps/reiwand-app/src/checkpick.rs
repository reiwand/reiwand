//! Which check units a `--check` walk runs: `reiwand-app --check PATTERN…` runs only the units whose
//! name matches one of the patterns (`*` for any run of characters, `?` for one), and
//! `--check --list` names the units the walk reaches without running them. With neither, every
//! unit runs, as the full walk always did. The choice reaches the check processes in `ONLY`.

pub const ONLY: &str = "REIWAND_APP_CHECK_ONLY";
pub const LIST: &str = "--list";
const MARK: &str = "reiwand-check listed ";

/// What the arguments after `--check` ask for: the patterns, and whether to only list.
pub fn parse(args: &[String]) -> Result<(Vec<String>, bool), String> {
    let list = args.iter().any(|a| a == LIST);
    let patterns: Vec<String> = args.iter().filter(|a| *a != LIST).cloned().collect();
    match patterns.iter().find(|p| p.is_empty() || p.starts_with('-') || p.contains(char::is_whitespace)) {
        Some(bad) => Err(format!("reiwand-app: --check takes unit names or patterns, not “{bad}”")),
        None => Ok((patterns, list)),
    }
}

/// The value of `ONLY` for the check processes: the patterns, or `LIST` alone to run none.
pub fn only(patterns: &[String], list: bool) -> Option<String> {
    if list { Some(LIST.to_owned()) } else { (!patterns.is_empty()).then(|| patterns.join(" ")) }
}

/// Whether this process may run `unit`: always without a choice; when listing, never (the unit is
/// named instead); else when a pattern matches it.
pub fn allowed(unit: &str) -> bool {
    let Ok(only) = std::env::var(ONLY) else { return true };
    if only == LIST {
        eprintln!("{MARK}{unit}");
        return false;
    }
    only.split(' ').any(|pattern| matches(pattern, unit))
}

pub fn matches(pattern: &str, name: &str) -> bool {
    let (p, n): (Vec<char>, Vec<char>) = (pattern.chars().collect(), name.chars().collect());
    let (mut at, mut on, mut star, mut back) = (0, 0, None, 0);
    while on < n.len() {
        if at < p.len() && (p[at] == '?' || p[at] == n[on]) {
            (at, on) = (at + 1, on + 1);
        } else if at < p.len() && p[at] == '*' {
            (star, back, at) = (Some(at), on, at + 1);
        } else if let Some(s) = star {
            (at, back) = (s + 1, back + 1);
            on = back;
        } else {
            return false;
        }
    }
    p[at..].iter().all(|c| *c == '*')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patterns_match_whole_unit_names() {
        assert!(matches("scene.*", "scene.particles"));
        assert!(matches("scene.clock", "scene.clock"));
        assert!(!matches("scene", "scene.clock"));
        assert!(matches("*clock", "scene.clock"));
        assert!(matches("page.?ettings", "page.settings"));
        assert!(matches("*", "image"));
        assert!(!matches("keys.*", "scene.keys"));
        assert!(matches("a*b*c", "a-x-b-y-c"));
    }

    #[test]
    fn the_arguments_choose_patterns_or_a_list() {
        let args = |list: &[&str]| list.iter().map(|a| (*a).to_owned()).collect::<Vec<_>>();
        assert_eq!(parse(&args(&[])), Ok((vec![], false)));
        assert_eq!(parse(&args(&["scene.*", "image"])), Ok((args(&["scene.*", "image"]), false)));
        assert_eq!(parse(&args(&["--list"])), Ok((vec![], true)));
        assert!(parse(&args(&["--quick"])).is_err());
        assert_eq!(only(&[], false), None);
        assert_eq!(only(&args(&["scene.*", "image"]), false).as_deref(), Some("scene.* image"));
        assert_eq!(only(&args(&["image"]), true).as_deref(), Some("--list"));
    }
}
