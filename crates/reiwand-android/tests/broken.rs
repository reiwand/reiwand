mod common;
use common::*;
use reiwand_android::host;
use std::fs;
use std::path::Path;

#[test]
fn corrupt_and_truncated_files_are_refused_not_crashed() {
    let work = tempfile::tempdir().unwrap();
    let files = work.path().join("files");
    fs::create_dir(&files).unwrap();
    host::prepare(files.to_str().unwrap()).unwrap();
    let fixtures = Path::new(ROOT).join("../../tests/fixtures");
    let mut sources: Vec<(String, Vec<u8>)> = [
        "green-64x32.png",
        "blink-16x8.gif",
        "magenta-64x32.webm",
        "green-64x32.ktx2",
        "blue-64x32.avif",
        "blue-64x32.heic",
    ]
    .iter()
    .map(|n| (n.to_string(), fs::read(fixtures.join(n)).unwrap()))
    .collect();
    for scene in ["orbits", "dusk"] {
        let path = pack(scene, work.path());
        sources.push((format!("{scene}.wpak"), fs::read(path).unwrap()));
    }
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let mut bad = Vec::new();
    let cases = work.path().join("cases");
    fs::create_dir(&cases).unwrap();
    let before = (items(), leftovers(&files));
    for (name, bytes) in &sources {
        let mut variants: Vec<(String, Vec<u8>)> = vec![("empty".into(), Vec::new())];
        let step = (bytes.len() / 40).max(1);
        for cut in (1..bytes.len()).step_by(step) {
            variants.push((format!("cut{cut}"), bytes[..cut].to_vec()));
        }
        for n in 0..60 {
            let mut copy = bytes.clone();
            for _ in 0..1 + n % 4 {
                let at = (rng.next() as usize) % copy.len();
                copy[at] = rng.next() as u8;
            }
            variants.push((format!("flip{n}"), copy));
        }
        for (label, data) in variants {
            let file = cases.join(name);
            fs::write(&file, &data).unwrap();
            let (reply, alive) = attempt(file);
            if !alive {
                bad.push(format!("{name} {label}: {reply}"));
            }
        }
    }
    assert!(bad.is_empty(), "{} broke: {:#?}", bad.len(), &bad[..bad.len().min(20)]);
    let after = (items(), leftovers(&files));
    println!("items {before:?} -> {after:?}");
}
