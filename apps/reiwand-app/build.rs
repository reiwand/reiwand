use cxx_qt_build::{CxxQtBuilder, QmlFile, QmlModule};
use std::path::{Path, PathBuf};
use std::{env, fs};

#[path = "qt-shim/setup.rs"]
mod shim;

#[path = "look/resources.rs"]
mod look;

fn files(dir: &str) -> Vec<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "qml"))
        .collect();
    files.sort();
    files
}

fn walk(dir: &Path, base: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()) {
        match entry.is_dir() {
            true => walk(&entry, base, found),
            false => found.push(entry.strip_prefix(base).unwrap().to_owned()),
        }
    }
}

fn builtin(out: &Path) {
    let mut scenes: Vec<_> = fs::read_dir("../../scenes").unwrap().map(|e| e.unwrap().path()).collect();
    scenes.retain(|p| p.join("manifest.json").is_file());
    scenes.sort();
    let mut text = String::from("&[");
    for scene in scenes {
        let mut files = Vec::new();
        walk(&scene, &scene, &mut files);
        files.sort();
        let name = scene.file_name().unwrap().to_string_lossy().into_owned();
        text += &format!("({name:?}, &[");
        for file in files {
            let full = scene.join(&file).canonicalize().unwrap();
            text += &format!("({:?}, include_bytes!({full:?}) as &[u8]),", file.to_string_lossy());
        }
        text += "] as Files),";
    }
    fs::write(out.join("builtin.rs"), text + "]").unwrap();
}

fn own_qt() {
    println!("cargo::rerun-if-env-changed=QMAKE");
    let Some(qmake) = env::var_os("QMAKE") else { return };
    let out = std::process::Command::new(qmake).args(["-query", "QT_INSTALL_LIBS"]).output();
    let libs = out.ok().and_then(|o| String::from_utf8(o.stdout).ok()).map(|t| t.trim().to_owned());
    let system = |dir: &str| ["/usr/", "/lib"].iter().any(|root| dir.starts_with(root));
    if let Some(libs) = libs.filter(|l| l.starts_with('/') && !system(l)) {
        println!("cargo::rustc-link-arg=-Wl,-rpath,{libs}");
    }
}

fn main() {
    own_qt();
    let mut langs: Vec<_> = fs::read_dir("../../po").unwrap().map(|e| e.unwrap().path()).collect();
    langs.sort();
    let catalogs: String =
        (langs.iter().map(|dir| (dir, dir.join("reiwand-app.po"))).filter(|(_, po)| po.is_file()))
            .map(|(dir, po)| {
                format!("({:?}, include_str!({:?})),", dir.file_name().unwrap(), po.canonicalize().unwrap())
            })
            .collect();
    fs::write(PathBuf::from(env::var("OUT_DIR").unwrap()).join("catalogs.rs"), format!("&[{catalogs}]"))
        .unwrap();
    println!("cargo::rerun-if-changed=../../po");
    println!("cargo::rerun-if-changed=qml");
    println!("cargo::rerun-if-changed=starters");
    println!("cargo::rerun-if-changed=../../scenes");
    builtin(&PathBuf::from(env::var("OUT_DIR").unwrap()));
    shim::install();
    let look = look::write(
        &PathBuf::from("look").canonicalize().unwrap(),
        &PathBuf::from(env::var("OUT_DIR").unwrap()),
    );
    let pages = files("qml").into_iter().map(QmlFile::from);
    let singletons = files("qml/facade").into_iter().map(|p| QmlFile::from(p).singleton(true));
    CxxQtBuilder::new_qml_module(QmlModule::new("io.github.reiwand.app").qml_files(pages.chain(singletons)))
        .qrc(look)
        .qrc("starters/starters.qrc")
        .files([
            "src/backend.rs",
            "src/benchdeskqt.rs",
            "src/benchqt.rs",
            "src/connectcode.rs",
            "src/depth.rs",
            "src/depthfollow.rs",
            "src/desknote.rs",
            "src/editor.rs",
            "src/items.rs",
            "src/keyboard.rs",
            "src/looks.rs",
            "src/packer.rs",
            "src/relief.rs",
            "src/scene.rs",
            "src/scenerecent.rs",
            "src/shadercheck.rs",
            "src/shelf.rs",
            "src/signing.rs",
            "src/strips.rs",
            "src/stall.rs",
        ])
        .build();
}
