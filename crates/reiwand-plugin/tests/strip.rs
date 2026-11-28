use reiwand_plugin::{Error, strip};

#[test]
fn types_are_removed_and_positions_kept() {
    let source = "import type { Item } from \"./item.ts\";\ninterface Page { items: Item[] }\n\
        type Id = string;\nexport async function list<T>(args: { page: Id | null }): Promise<Page> {\n\
        const items = [] as Item[];\n  return { items, next: args.page! };\n}\n";
    let js = strip("main.ts", source).unwrap();
    assert_eq!(js.lines().count(), source.lines().count());
    for gone in ["interface", "type Id", "Promise<Page>", "as Item", "<T>", "page!"] {
        assert!(!js.contains(gone), "{gone} is left in {js}");
    }
    assert!(js.contains("export async function list"));
}

#[test]
fn syntax_that_needs_more_than_removing_types_is_refused_with_its_place() {
    let cases = [
        ("const a = 1;\nenum Mode { A, B }", 2, "enum"),
        ("namespace N { export const a = 1; }", 1, "namespace"),
        ("class A {\n  constructor(private x: number) {}\n}", 2, "parameter property"),
        ("@sealed class A {}", 1, ""),
        ("const x = <number>y;", 1, "as"),
    ];
    for (source, line, word) in cases {
        match strip("lib/odd.ts", source) {
            Err(Error::Strip { file, line: at, message, .. }) => {
                assert_eq!((file.as_str(), at), ("lib/odd.ts", line), "{source}: {message}");
                assert!(message.contains(word), "{source}: {message}");
                assert!(!message.contains("strip-only"), "{message}");
            }
            other => panic!("{source} was not refused: {other:?}"),
        }
    }
}
