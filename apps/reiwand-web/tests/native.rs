//! C++ of the helper that can be checked without a GPU or Qt, built with AddressSanitizer.

use std::path::{Path, PathBuf};
use std::process::Command;

fn compiler() -> Option<PathBuf> {
    let named = std::env::var_os("CXX").map(PathBuf::from);
    let found = ["c++", "g++", "clang++"].iter().map(PathBuf::from);
    named
        .into_iter()
        .chain(found)
        .find(|c| Command::new(c).arg("--version").output().is_ok_and(|o| o.status.success()))
}

#[test]
fn a_dmabuf_query_with_several_planes_stays_in_its_array() {
    let Some(cxx) = compiler() else { return eprintln!("skipped: no C++ compiler") };
    if !Path::new("/usr/include/EGL/eglext.h").is_file() {
        return eprintln!("skipped: no EGL headers");
    }
    let dir = tempfile::tempdir().unwrap();
    let binary = dir.path().join("planes");
    let built = Command::new(&cxx)
        .args(["-std=c++17", "-g", "-O0", "-fsanitize=address", "-fno-omit-frame-pointer", "-Icpp"])
        .arg("tests/native/planes.cpp")
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    if !built.status.success() && String::from_utf8_lossy(&built.stderr).contains("asan") {
        return eprintln!("skipped: the compiler has no AddressSanitizer runtime");
    }
    assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
    let ran = Command::new(&binary).env("ASAN_OPTIONS", "detect_leaks=0").output().unwrap();
    assert!(ran.status.success(), "{}", String::from_utf8_lossy(&ran.stderr));
}
