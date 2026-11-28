use super::*;

#[test]
fn an_empty_database_gets_the_schema_and_reopening_keeps_data() {
    let dir = TempDir::new().unwrap();
    let uid = open(&dir).create_playlist("Evening", None, &Rules::default()).unwrap();
    let raw = Connection::open(dir.path().join("data/library.db")).unwrap();
    let version: u32 = raw.pragma_query_value(None, "user_version", |r| r.get(0)).unwrap();
    let journal: String = raw.pragma_query_value(None, "journal_mode", |r| r.get(0)).unwrap();
    let fts5: bool =
        raw.query_row("SELECT sqlite_compileoption_used('ENABLE_FTS5')", [], |r| r.get(0)).unwrap();
    assert_eq!((version, journal.as_str(), fts5), (SCHEMA_VERSION, "wal", true));
    let reopened = open(&dir);
    assert_eq!(reopened.playlist(&uid).unwrap().name, "Evening");
    assert_eq!(reopened.playlist("evening").unwrap().uid, uid);
}

#[test]
fn a_database_of_another_version_is_refused_and_left_untouched() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("data/library.db");
    let uid = open(&dir).create_playlist("Evening", None, &Rules::default()).unwrap();
    let raw = Connection::open(&path).unwrap();
    for other in [SCHEMA_VERSION - 1, SCHEMA_VERSION + 1] {
        raw.pragma_update(None, "user_version", other).unwrap();
        let error = Library::open(&path).unwrap_err();
        assert!(matches!(error, Error::Foreign(version) if version == other), "{error}");
        assert!(error.to_string().contains("another version of the program"), "{error}");
    }
    raw.pragma_update(None, "user_version", SCHEMA_VERSION).unwrap();
    assert_eq!(open(&dir).playlist(&uid).unwrap().name, "Evening");
}

#[test]
fn a_database_with_tables_but_no_version_is_refused() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("library.db");
    Connection::open(&path).unwrap().execute_batch("CREATE TABLE other (x);").unwrap();
    assert!(matches!(Library::open(&path), Err(Error::Foreign(0))));
}

#[test]
fn the_schema_is_the_one_earlier_builds_wrote() {
    let dir = TempDir::new().unwrap();
    drop(open(&dir));
    let raw = Connection::open(dir.path().join("data/library.db")).unwrap();
    let sql =
        "SELECT sql FROM sqlite_master WHERE sql IS NOT NULL AND name NOT LIKE 'items_fts_%' ORDER BY rowid";
    let mut statement = raw.prepare(sql).unwrap();
    let found: Vec<String> = statement.query_map([], |r| r.get(0)).unwrap().map(Result::unwrap).collect();
    let written = include_str!("../fixtures/library-schema-8.sql");
    let expected: Vec<&str> = written.split(";\n\n").map(|s| s.trim().trim_end_matches(';')).collect();
    assert_eq!(found, expected);
}

#[test]
fn the_database_files_and_their_folder_are_private() {
    let dir = TempDir::new().unwrap();
    let library = open(&dir);
    let data = dir.path().join("data");
    assert_eq!(mode(&data), 0o700);
    for name in ["library.db", "library.db-wal", "library.db-shm"] {
        assert_eq!(mode(data.join(name)), 0o600, "{name}");
    }
    drop(library);
}

#[test]
fn scanning_refinds_moved_files_and_marks_vanished_ones_missing() {
    let dir = TempDir::new().unwrap();
    let mut library = open(&dir);
    let root = dir.path().join("pictures");
    write(&root.join("waves.jpg"), b"waves");
    write(&root.join("dunes.PNG"), b"dunes");
    write(&root.join("clips/rain.webm"), b"rain");
    write(&root.join("notes.txt"), b"skipped");
    write(&root.join(".hidden.jpg"), b"skipped");
    let source = library.add_folder(&root, true).unwrap();
    assert!(matches!(library.add_folder(&root, false), Err(Error::Exists("source", _))));
    assert_eq!(counts(&mut library, &source, NOW), [3, 0, 0, 0]);
    let waves = find(&library, "waves");
    assert_eq!(
        (waves.kind, waves.source.as_deref(), waves.added_at),
        (Kind::Image, Some(source.as_str()), NOW)
    );
    assert_eq!(find(&library, "rain").kind, Kind::Video);
    library.add_tag(&waves.uid, "sea").unwrap();

    fs::create_dir_all(root.join("moved")).unwrap();
    fs::rename(root.join("waves.jpg"), root.join("moved/renamed.jpg")).unwrap();
    fs::remove_file(root.join("dunes.PNG")).unwrap();
    write(&root.join("clips/rain.webm"), b"heavier rain");
    assert_eq!(counts(&mut library, &source, NOW + 60), [0, 1, 1, 1]);
    let moved = library.item(&waves.uid).unwrap();
    assert_eq!(
        (moved.path, moved.tags, moved.added_at),
        (fs::canonicalize(root.join("moved/renamed.jpg")).unwrap(), vec!["sea".to_string()], NOW)
    );
    assert!(find(&library, "dunes").missing);
    assert_eq!(find(&library, "renamed").uid, waves.uid);

    write(&root.join("dunes.PNG"), b"dunes");
    assert_eq!(counts(&mut library, &source, NOW + 120), [0, 1, 0, 0]);
    assert!(!find(&library, "dunes").missing);
    assert_eq!(library.sources().unwrap()[0].scanned_at, Some(NOW + 120));
}

#[test]
fn shallow_sources_skip_subfolders_and_loose_files_have_no_source() {
    let dir = TempDir::new().unwrap();
    let mut library = open(&dir);
    let root = dir.path().join("pictures");
    write(&root.join("top.jpg"), b"top");
    write(&root.join("deep/below.jpg"), b"below");
    let source = library.add_folder(&root, false).unwrap();
    assert_eq!(counts(&mut library, &source, NOW), [1, 0, 0, 0]);
    let loose = write(&dir.path().join("loose.mkv"), b"loose");
    let uid = library.import_file(&loose, NOW).unwrap();
    assert_eq!(library.import_file(&loose, NOW).unwrap(), uid);
    let item = library.item(&uid).unwrap();
    assert_eq!((item.kind, item.source, item.path), (Kind::Video, None, loose));
    let text = write(&dir.path().join("readme.txt"), b"text");
    assert!(matches!(library.import_file(&text, NOW), Err(Error::Invalid("file", _))));

    fs::remove_dir_all(&root).unwrap();
    assert!(matches!(library.scan(&source, NOW), Err(Error::Io(_))));
    assert!(library.sources().unwrap()[0].error.is_some());
    assert!(!find(&library, "top").missing);
    library.remove_source(&source).unwrap();
    assert!(library.sources().unwrap().is_empty());
    assert_eq!(library.count("", NOW).unwrap(), 1);
    let raw = Connection::open(dir.path().join("data/library.db")).unwrap();
    let indexed: u32 = raw
        .query_row("SELECT count(*) FROM items_fts WHERE items_fts MATCH 'top'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(indexed, 0);
}

#[test]
fn an_empty_folder_that_had_items_is_unreachable_and_keeps_them() {
    let dir = TempDir::new().unwrap();
    let (mut library, source, [a, b, c]) = folder_with_three_items(&dir);
    let root = dir.path().join("pictures");
    [&a, &b, &c].iter().for_each(|item| fs::remove_file(&item.path).unwrap());
    assert!(matches!(library.scan(&source, NOW + 60), Err(Error::Io(_))));
    let error = library.sources().unwrap()[0].error.clone().unwrap_or_default();
    assert!(error.contains("empty"), "{error}");
    assert!([&a, &b, &c].iter().all(|item| !library.item(&item.uid).unwrap().missing));
    write(&root.join("a.jpg"), b"a.jpg");
    assert_eq!(counts(&mut library, &source, NOW + 120), [0, 0, 0, 2]);
    assert_eq!(library.sources().unwrap()[0].error, None);
    fs::remove_file(root.join("a.jpg")).unwrap();
    write(&root.join(".hidden"), b"kept");
    assert_eq!(counts(&mut library, &source, NOW + 180), [0, 0, 0, 1]);
}
