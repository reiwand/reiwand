mod common;
mod fixture;

use common::{image, v2};
use fixture::{key, pair, signer};
use reiwand_format::{
    Error, Kind, Limits, build, build_with, manifest_comment, open, sign_source, source_digest,
    verify_package,
};
use std::fs;
use std::path::Path;

fn folder(dir: &Path) {
    fs::create_dir_all(dir).unwrap();
    let mut saved = v2(Kind::Web);
    saved.requires.get_or_insert_default().features.push("web".into());
    fs::write(dir.join("manifest.json"), saved.to_json().unwrap()).unwrap();
    fs::write(dir.join("index.html"), b"<!doctype html>").unwrap();
    fs::write(dir.join("preview.png"), image(16, 16, ::image::ImageFormat::Png)).unwrap();
}

#[test]
fn a_matching_signature_is_built_in_and_verifies() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("page"), tmp.path().join("page.wpak"));
    folder(&src);
    let author = key();
    sign_source(&src, &signer(&author)).unwrap();
    build(&src, &out, &Limits::default()).unwrap();
    let mut package = open(&out, &Limits::default()).unwrap();
    assert_eq!(verify_package(&mut package, &[pair(&author)]).unwrap(), author.fingerprint);
}

fn old_form(dir: &Path, author: &reiwand_format::Keypair) {
    let json = fs::read(dir.join("manifest.json")).unwrap();
    let manifest = reiwand_format::Manifest::from_json(&json).unwrap();
    let files = source_digest(dir).unwrap();
    let comment = manifest_comment(&manifest.id, &manifest.version, &files);
    fs::write(dir.join("manifest.json.minisig"), signer(author).sign(&json, &comment).unwrap()).unwrap();
}

fn edit(dir: &Path) {
    let mut edited = fs::read_to_string(dir.join("manifest.json")).unwrap();
    edited.push('\n');
    fs::write(dir.join("manifest.json"), edited).unwrap();
}

fn refused(result: reiwand_format::Result<String>) -> bool {
    matches!(&result, Err(Error::Invalid { field, .. }) if field == "manifest.json.minisig")
}

#[test]
fn a_new_form_signature_keeps_the_trusted_comment_of_the_old_form() {
    let tmp = tempfile::tempdir().unwrap();
    folder(tmp.path());
    let author = key();
    sign_source(tmp.path(), &signer(&author)).unwrap();
    let text = fs::read_to_string(tmp.path().join("manifest.json.minisig")).unwrap();
    assert!(text.lines().nth(1).is_some() && text.contains("trusted comment: wpak manifest "), "{text}");
    assert!(!text.lines().nth(2).unwrap().contains("sha256"), "{text}");
    assert!(text.lines().next().unwrap().contains("sha256:"), "{text}");
}

#[test]
fn a_stale_new_form_signature_stops_the_build() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("page"), tmp.path().join("page.wpak"));
    folder(&src);
    sign_source(&src, &signer(&key())).unwrap();
    edit(&src);
    assert!(refused(build(&src, &out, &Limits::default())));
    assert!(!out.exists());
}

#[test]
fn an_old_form_signature_verifies_and_builds_with_its_key() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("page"), tmp.path().join("page.wpak"));
    folder(&src);
    let author = key();
    old_form(&src, &author);
    assert!(refused(build(&src, &out, &Limits::default())), "no key at hand");
    let keys = [pair(&author)];
    build_with(&src, &out, &Limits::default(), &keys).unwrap();
    let mut package = open(&out, &Limits::default()).unwrap();
    assert_eq!(verify_package(&mut package, &keys).unwrap(), author.fingerprint);
}

#[test]
fn a_stale_old_form_signature_is_refused_by_build() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("page"), tmp.path().join("page.wpak"));
    folder(&src);
    let author = key();
    old_form(&src, &author);
    edit(&src);
    assert!(refused(build_with(&src, &out, &Limits::default(), &[pair(&author)])));
    assert!(!out.exists());
}

#[test]
fn a_signature_by_another_key_names_the_key() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("page"), tmp.path().join("page.wpak"));
    folder(&src);
    sign_source(&src, &signer(&key())).unwrap();
    let other = key();
    let keys = [pair(&other)];
    let error = build_with(&src, &out, &Limits::default(), &keys).unwrap_err().to_string();
    assert!(error.contains("not signed by any of the given keys"), "{error}");
    assert!(!error.contains("another version"), "{error}");
}

#[test]
fn a_signature_covers_every_file_and_not_only_the_manifest() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("page"), tmp.path().join("page.wpak"));
    folder(&src);
    let author = key();
    sign_source(&src, &signer(&author)).unwrap();
    fs::write(src.join("index.html"), b"<!doctype html><script src=other.js></script>").unwrap();
    let error = build(&src, &out, &Limits::default()).unwrap_err().to_string();
    assert!(error.contains("signs other files"), "{error}");
    let signature = fs::read(src.join("manifest.json.minisig")).unwrap();
    fs::remove_file(src.join("manifest.json.minisig")).unwrap();
    build(&src, &out, &Limits::default()).unwrap();
    let swapped = tmp.path().join("swapped.wpak");
    let mut zip = zip::ZipWriter::new(fs::File::create(&swapped).unwrap());
    let mut original = zip::ZipArchive::new(fs::File::open(&out).unwrap()).unwrap();
    for index in 0..original.len() {
        zip.raw_copy_file(original.by_index(index).unwrap()).unwrap();
    }
    zip.start_file("manifest.json.minisig", zip::write::SimpleFileOptions::default()).unwrap();
    std::io::Write::write_all(&mut zip, &signature).unwrap();
    zip.finish().unwrap();
    let mut package = open(&swapped, &Limits::default()).unwrap();
    let result = verify_package(&mut package, &[pair(&author)]);
    assert!(matches!(result, Err(Error::Signature(_))), "{result:?}");
}
