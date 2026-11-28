use super::*;
use crate::error::Error;
use crate::repos::newer;
use reiwand_core::dbus::error::{EXISTS, NOT_FOUND, STALE};
use reiwand_format::{Keypair, Limits, Settings, Signer, add, build, init, keygen, publish, sign_root};
use reiwand_providers::HttpFetch;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn signer(key: &Keypair) -> Signer {
    Signer::new(&key.secret, None).unwrap()
}

fn repository(dir: &Path) -> String {
    let (root, online) = (keygen(None).unwrap(), keygen(None).unwrap());
    let (root_keys, online_keys) = (vec![root.public.clone()], vec![online.public.clone()]);
    let settings = Settings { name: "Example".into(), root_keys, online_keys, ..Settings::default() };
    let site = dir.join("site");
    init(&site, &settings, now()).unwrap();
    sign_root(&site, &signer(&root)).unwrap();
    for version in ["1.0.0", "1.1.0"] {
        let src = dir.join(version);
        fs::create_dir_all(&src).unwrap();
        let mut manifest = json!({"schema": reiwand_format::SCHEMA, "id": "org.example.dawn", "version": version,
            "title": "Dawn", "license": "CC0-1.0", "kind": "image", "entry": "image.png",
            "previews": {"thumbnail": "preview.png"}, "content": {"rating": "general"}, "photosensitive": false,
            "origin": {"kind": "original"}});
        reiwand_fill::manifests::complete(&mut manifest);
        fs::write(src.join("manifest.json"), manifest.to_string()).unwrap();
        let green = image::RgbaImage::from_pixel(8, 4, image::Rgba([0, 255, 0, 255]));
        green.save(src.join("image.png")).unwrap();
        green.save(src.join("preview.png")).unwrap();
        let wpak = dir.join(format!("{version}.wpak"));
        build(&src, &wpak, &Limits::default()).unwrap();
        add(&site, &wpak, None, now()).unwrap();
    }
    publish(&site, &signer(&online), 7, now()).unwrap();
    root.fingerprint
}

async fn serve(dir: PathBuf) -> MockServer {
    let server = MockServer::start().await;
    let files = move |request: &Request| match fs::read(dir.join(request.url.path().trim_start_matches('/')))
    {
        Ok(bytes) => ResponseTemplate::new(200).set_body_bytes(bytes),
        Err(_) => ResponseTemplate::new(404),
    };
    Mock::given(any()).respond_with(files).mount(&server).await;
    server
}

fn name(error: reiwand_format::Error) -> String {
    Error::from(error).parts().0
}

#[test]
fn specs_and_errors_are_checked() {
    assert_eq!(parse_spec("org.example.dawn").unwrap(), (None, "org.example.dawn", None));
    let full = parse_spec("0123abcd/org.example.dawn@1.0.0").unwrap();
    assert_eq!(full, (Some("0123abcd"), "org.example.dawn", Some("1.0.0")));
    for bad in ["", "dawn", "0123/org.example.dawn", "org.example.dawn@", "../org.x", "org.example.dawn@1/x"]
    {
        assert!(parse_spec(bad).is_err(), "{bad}");
    }
    assert_eq!(name(reiwand_format::Error::Stale("0123".into())), STALE);
    let interrupted = reiwand_format::Error::Io(io::ErrorKind::Interrupted.into());
    assert!(matches!(Error::from(interrupted), Error::Cancelled));
    assert!(newer("1.10.0", "1.9.0") && !newer("1.0.0", "1.0.0"));
}

#[tokio::test(flavor = "multi_thread")]
async fn repositories_install_over_http_with_progress() {
    let tmp = tempfile::tempdir().unwrap();
    repository(tmp.path());
    let server = serve(tmp.path().join("site")).await;
    let (url, data) = (format!("{}/", server.uri()), tmp.path().join("data"));
    let dest = tmp.path().join("preview");
    tokio::task::spawn_blocking(move || {
        let fetch = HttpFetch::new(Arc::default()).unwrap();
        let info = reiwand_format::add_repo(&data, &fetch, &url, now()).unwrap();
        assert_eq!((info.packages, info.hosts.len()), (1, 1));
        assert_eq!(name(reiwand_format::add_repo(&data, &fetch, &url, now()).unwrap_err()), EXISTS);
        let needs = |version: &str| reiwand_format::Dependency {
            id: "org.example.dawn".into(),
            version: version.into(),
        };
        let older = crate::variants::matching(&data, &info.repo_id, &needs("~1.0")).unwrap();
        assert_eq!(
            older,
            format!("{}/org.example.dawn@1.0.0", info.repo_id),
            "the newest release that matches"
        );
        assert!(crate::variants::matching(&data, &info.repo_id, &needs("^2")).is_err());
        let dry = reiwand_format::install(&data, &Dry, "org.example.dawn", now(), false, false);
        assert!(matches!(dry, Err(reiwand_format::Error::Io(e)) if e.kind() == io::ErrorKind::WouldBlock));
        let missing = reiwand_format::install(&data, &Dry, "org.example.dusk", now(), false, false);
        assert_eq!(name(missing.unwrap_err()), NOT_FOUND);
        let seen = AtomicU64::new(0);
        let progress = Progress(fetch, |done| seen.store(done, Ordering::Relaxed));
        let spec = format!("{}/org.example.dawn@1.0.0", info.repo_id);
        let record = reiwand_format::install(&data, &progress, &spec, now(), false, false).unwrap();
        assert_eq!((seen.load(Ordering::Relaxed), record.version.as_str()), (record.size, "1.0.0"));
        reiwand_format::install(&data, &progress, "org.example.dawn", now(), false, false).unwrap();
        let older = reiwand_format::install(&data, &Dry, &spec, now(), false, false).unwrap_err();
        assert!(Error::from(older).to_string().contains("confirm the downgrade"));
        assert!(
            reiwand_format::preview(&data, &progress.0, &info.repo_id, "org.example.dawn", "1.1.0", &dest)
                .unwrap()
        );
        assert!(image::ImageReader::open(&dest).unwrap().with_guessed_format().unwrap().decode().is_ok());
    })
    .await
    .unwrap();
}

#[test]
fn installing_a_damaged_package_again_clears_its_error() {
    let tmp = tempfile::tempdir().unwrap();
    repository(tmp.path());
    let file = tmp.path().join("dawn.wpak");
    fs::write(&file, b"not a package").unwrap();
    let mut lib = reiwand_library::Library::open_in_memory().unwrap();
    let uid = lib.import_file(&file, now()).unwrap();
    assert!(lib.item(&uid).unwrap().meta.error.is_some(), "a damaged package is marked");
    fs::copy(tmp.path().join("1.0.0.wpak"), &file).unwrap();
    let (again, old) = place(&mut lib, &file, &[]).unwrap();
    assert_eq!((again.as_str(), old.is_none()), (uid.as_str(), true));
    assert_eq!(lib.item(&uid).unwrap().meta.error, None);
}
