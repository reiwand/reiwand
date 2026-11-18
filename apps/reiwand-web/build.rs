use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const MODULES: [&str; 12] = [
    "Core",
    "Gui",
    "Qml",
    "QmlIntegration",
    "QmlMeta",
    "QmlModels",
    "Quick",
    "Network",
    "Positioning",
    "WebChannel",
    "WebEngineCore",
    "WebEngineQuick",
];
const LIBS: [&str; 6] = ["Core", "Gui", "Qml", "Quick", "WebEngineCore", "WebEngineQuick"];

fn qmake() -> Option<PathBuf> {
    if let Some(qmake) = env::var_os("REIWAND_WEB_QMAKE") {
        return Some(PathBuf::from(qmake));
    }
    if let Some(qmake) = env::var_os("QMAKE") {
        return Some(PathBuf::from(qmake));
    }
    let dirs: Vec<_> = env::split_paths(&env::var_os("PATH")?).collect();
    ["qmake6", "qmake-qt6", "qmake"]
        .iter()
        .flat_map(|n| dirs.iter().map(move |d| d.join(n)))
        .find(|p| p.is_file())
}

fn query(qmake: &Path, name: &str) -> Option<String> {
    let out = Command::new(qmake).args(["-query", name]).output().ok()?;
    let text = String::from_utf8(out.stdout).ok()?.trim().to_owned();
    (!text.is_empty() && !text.contains("**Unknown**")).then_some(text)
}

struct Qt {
    version: String,
    headers: PathBuf,
    libs: PathBuf,
    moc: PathBuf,
}

fn find() -> Result<Qt, String> {
    let qmake = qmake().ok_or("qmake was not found")?;
    let version = query(&qmake, "QT_VERSION").ok_or("qmake does not answer")?;
    let headers = PathBuf::from(query(&qmake, "QT_INSTALL_HEADERS").ok_or("no Qt headers")?);
    let libs = PathBuf::from(query(&qmake, "QT_INSTALL_LIBS").ok_or("no Qt libraries")?);
    let mut parts = version.split('.').filter_map(|p| p.parse::<u32>().ok());
    let (major, minor) = (parts.next().unwrap_or(0), parts.next().unwrap_or(0));
    if (major, minor) < (6, 5) {
        return Err(format!("Qt {version} is older than 6.5"));
    }
    if !headers.join("QtWebEngineQuick/qtwebenginequickglobal.h").is_file() {
        return Err(format!("Qt {version} has no Qt WebEngine headers"));
    }
    let moc = ["QT_HOST_LIBEXECS", "QT_INSTALL_LIBEXECS", "QT_HOST_BINS"]
        .iter()
        .filter_map(|n| query(&qmake, n))
        .map(|d| PathBuf::from(d).join("moc"))
        .find(|p| p.is_file())
        .ok_or("moc was not found")?;
    Ok(Qt { version, headers, libs, moc })
}

fn compile(qt: &Qt) {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let moc = out.join("moc_bridge.cpp");
    let mut includes = vec![qt.headers.clone()];
    includes.extend(MODULES.iter().map(|m| qt.headers.join(format!("Qt{m}"))));
    let mut command = Command::new(&qt.moc);
    for include in &includes {
        command.arg(format!("-I{}", include.display()));
    }
    let status = command.arg("cpp/bridge.h").arg("-o").arg(&moc).status().expect("moc cannot run");
    assert!(status.success(), "moc failed");
    let mut build = cc::Build::new();
    build.cpp(true).std("c++17").warnings(true).flag_if_supported("-fPIC");
    for include in &includes {
        build.include(include);
    }
    build.include("cpp").include(&out);
    let qml = fs::read_to_string("qml/View.qml").expect("the view is missing");
    fs::write(out.join("view.inc"), format!("R\"QML({qml})QML\"")).expect("cannot write the view");
    let mut files: Vec<PathBuf> = fs::read_dir("cpp")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "cpp"))
        .collect();
    files.push(moc);
    files.sort();
    build.files(files).compile("reiwand_web_engine");
    println!("cargo::rustc-link-search=native={}", qt.libs.display());
    if env::var_os("REIWAND_WEB_QMAKE").is_some() {
        println!("cargo::rustc-link-arg=-Wl,-rpath,{}", qt.libs.display());
    }
    LIBS.iter().for_each(|l| println!("cargo::rustc-link-lib=Qt6{l}"));
    println!("cargo::rustc-link-lib=EGL");
    println!("cargo::rustc-link-lib=GLESv2");
}

fn main() {
    println!("cargo::rustc-check-cfg=cfg(no_engine)");
    println!("cargo::rerun-if-env-changed=QMAKE");
    println!("cargo::rerun-if-env-changed=REIWAND_WEB_QMAKE");
    println!("cargo::rerun-if-changed=cpp");
    println!("cargo::rerun-if-changed=qml");
    match find() {
        Ok(qt) => {
            println!("cargo::rustc-env=REIWAND_WEB_QT={}", qt.version);
            compile(&qt);
        }
        Err(reason) => {
            println!("cargo::warning=reiwand-web is built without Qt WebEngine: {reason}");
            println!("cargo::rustc-cfg=no_engine");
            println!("cargo::rustc-env=REIWAND_WEB_QT={reason}");
        }
    }
}
