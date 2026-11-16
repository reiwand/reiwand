use reiwand_library::{Library, Rules};
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
fn many_programs_opening_and_writing_one_new_database_never_see_it_locked() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("library.db");
    let gate = Arc::new(Barrier::new(8));
    let workers: Vec<_> = (0..8)
        .map(|n| {
            let (path, gate) = (path.clone(), gate.clone());
            thread::spawn(move || -> Result<(), String> {
                gate.wait();
                let mut library = Library::open(&path).map_err(|e| format!("open {n}: {e}"))?;
                for i in 0..60 {
                    library
                        .create_playlist(&format!("p{n}-{i}"), None, &Rules::default())
                        .map_err(|e| format!("write {n}/{i}: {e}"))?;
                    library.playlists().map_err(|e| format!("read {n}/{i}: {e}"))?;
                }
                Ok(())
            })
        })
        .collect();
    let failures: Vec<String> = workers.into_iter().filter_map(|w| w.join().unwrap().err()).collect();
    assert!(failures.is_empty(), "{failures:?}");
    assert_eq!(Library::open(&path).unwrap().playlists().unwrap().len(), 480);
}
