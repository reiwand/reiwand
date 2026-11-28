use reiwand_script::syntax;

fn source(path: &str, text: &str) -> (String, String) {
    (path.to_owned(), text.to_owned())
}

#[test]
fn a_syntax_error_names_its_script_and_line() {
    let good = "export function update(frame) {\n  return frame.time;\n}\n";
    let bad = "export function update(frame) {\n  return frame.time;\n\nlet = = 3;\n}\n";
    let faults = syntax(&[source("scripts/a.js", good), source("scripts/b.js", bad)]);
    assert_eq!(faults.len(), 1, "{faults:?}");
    assert_eq!(faults[0].0, 1);
    assert_eq!(faults[0].1.line, Some(4), "{:?}", faults[0].1);
    assert!(!faults[0].1.message.is_empty());
}

#[test]
fn modules_that_import_each_other_compile_and_nothing_runs() {
    let a =
        "import { b } from './b.js';\nthrow new Error('never');\nexport function update() { return b; }\n";
    let b = "export const b = 1;\n";
    let faults = syntax(&[source("scripts/a.js", a), source("scripts/b.js", b)]);
    assert!(faults.is_empty(), "{faults:?}");
}

#[test]
fn hostile_scripts_are_refused_quickly_without_breaking_the_checker() {
    let quantified = "a{65535}".to_owned();
    let mut regex = quantified;
    for _ in 0..4 {
        regex = format!("({regex}){{65535}}");
    }
    let atoms: String = (0..400_000).map(|i| format!("var a{i};")).collect();
    let hostile = [
        "(".repeat(200_000),
        "[".repeat(200_000),
        "{".repeat(200_000),
        "function f(){".repeat(100_000),
        "`${".repeat(100_000),
        format!("/{}/", "(".repeat(100_000)),
        format!("/{regex}/"),
        format!("let a = 1{};", "+1".repeat(2_000_000)),
        format!("let s = '{}';", "x".repeat(48 << 20)),
        atoms,
    ];
    for text in hostile {
        let start = std::time::Instant::now();
        let faults = syntax(&[source("scripts/a.js", &text)]);
        let spent = start.elapsed();
        assert!(spent < std::time::Duration::from_secs(5), "{spent:?} for {}", &text[..40.min(text.len())]);
        assert!(text.len() <= 1 << 20 || !faults.is_empty(), "{faults:?}");
    }
    let after = syntax(&[source("scripts/a.js", "let = = 3;")]);
    assert_eq!(after.len(), 1, "{after:?}");
}
