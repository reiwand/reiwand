use reiwand_upgrade::scenes;
use reiwand_upgrade::sink::Sink;
use std::fs;
use std::path::Path;

#[test]
fn the_pruned_examples_go_through_the_conversion_unchanged_twice() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/crear/ejemplos");
    let mut seen = 0;
    for entry in fs::read_dir(&root).unwrap().flatten() {
        for _ in 0..2 {
            let mut check = Sink::checking();
            let changed = scenes::walk_in(&entry.path(), &mut check).unwrap();
            assert_eq!(changed, 0, "{} would change: {:?}", entry.path().display(), check.changed());
        }
        seen += 1;
    }
    assert!(seen >= 20, "{seen} examples");
}
