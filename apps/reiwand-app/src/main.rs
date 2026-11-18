mod addlink;
mod app;
mod args;
mod backend;
mod builtin;
mod bus;
mod carrying;
mod checkrun;
mod choices;
mod chooser;
mod connectcode;
mod covers;
mod crashtrace;
mod crowd;
mod depth;
mod depthfollow;
mod depthproc;
mod depthview;
mod depthwork;
mod desknote;
mod diff;
mod editing;
mod editor;
mod editsteps;
mod events;
mod fetch;
mod grid;
mod items;
mod keyboard;
mod landing;
mod library;
mod looks;
mod methods;
mod packer;
mod picture;
mod preview;
mod recentstore;
mod relief;
mod reliefjobs;
mod reliefland;
mod reliefshow;
mod reliefwork;
mod rows;
mod scene;
mod scenecode;
mod scenecopy;
mod scenecross;
mod scenedoc;
mod scenefile;
mod scenehear;
mod scenelive;
mod scenemedia;
mod scenerecent;
mod scenerender;
mod scenesettle;
mod sceneshow;
mod scenesplit;
mod scenesteps;
mod scenesweep;
mod scenewave;
mod scenework;
mod secret;
mod service;
mod shadercheck;
mod shards;
mod shelf;
mod signing;
mod stall;
mod stallcheck;
mod stallwork;
mod strips;
#[cfg(test)]
mod tests;
mod texts;
mod textual;
mod tones;
mod tr;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};
use reiwand_core::dbus::launcher;
use std::process::ExitCode;

const MAIN: &str = "qrc:/qt/qml/io/github/reiwand/app/qml/Main.qml";

fn main() -> ExitCode {
    reiwand_render::serve_script_host();
    reiwand_scene::serve_check();
    reiwand_scene::check_out_of_process(true);
    if bus::checking() {
        crashtrace::install();
    }
    reiwand_core::stopped_effects().inspect(|p| reiwand_scene::keep_strikes(p));
    let code = start();
    bus::part(std::time::Duration::from_secs(2));
    reiwand_scene::end_trials();
    let status = if code == ExitCode::SUCCESS { 0 } else { 1 };
    reiwand_render::finish_builds(status);
    if bus::checking() {
        leave(status);
    }
    code
}

fn leave(status: i32) {
    use std::io::Write;
    drop(std::io::stdout().flush());
    drop(std::io::stderr().flush());
    unsafe { libc::_exit(status) }
}

fn start() -> ExitCode {
    let args: Vec<String> = std::env::args_os().skip(1).map(|a| a.to_string_lossy().into_owned()).collect();
    match args.first().map(String::as_str) {
        Some("--version" | "-V") => println!("reiwand-app {}", reiwand_core::VERSION),
        Some("--help" | "-h") => {
            println!(
                "usage: reiwand-app [--check | --version | --page PAGE | --browse OUTPUT] [FILE | URI]..."
            );
            println!("pages: {}", app::PAGES.join(", "));
        }
        Some("--check") if !bus::checking() => return checkrun::run(),
        Some("--depth") => return depthproc::child(&args[1..]),
        Some("--browse") => return preview::start(),
        Some("--page") => match app::page(args.get(1).map(String::as_str)) {
            Some(page) => return open(&args[2..], Some(page)),
            None => {
                eprintln!("reiwand-app: --page takes one of {}", app::PAGES.join(", "));
                return ExitCode::from(2);
            }
        },
        _ => return open(&args, None),
    }
    ExitCode::SUCCESS
}

fn open(args: &[String], page: Option<&str>) -> ExitCode {
    app::start(args.iter().filter_map(|a| file(a)).collect(), page).unwrap_or_else(run)
}

fn file(arg: &str) -> Option<String> {
    if arg.contains("://") {
        return Some(arg.to_owned());
    }
    let path = std::path::absolute(arg).ok().filter(|p| p.is_file())?;
    Some(bus::url(path.to_str()?))
}

fn run() -> ExitCode {
    show(MAIN)
}

fn show(url: &str) -> ExitCode {
    ExitCode::from(engine(url))
}

fn engine(url: &str) -> u8 {
    if !bus::checking() {
        chooser::ask_the_desktop();
    }
    let mut owned_app = QGuiApplication::new();
    QGuiApplication::set_desktop_file_name(&QString::from(
        launcher(std::env::var("FLATPAK_ID").ok()).as_str(),
    ));
    let mut owned_engine = QQmlApplicationEngine::new();
    let (Some(mut app), Some(mut engine)) = (owned_app.as_mut(), owned_engine.as_mut()) else { return 1 };
    app.as_mut().set_application_name(&QString::from("reiwand"));
    engine.as_mut().add_import_path(&QString::from("qrc:/qt/qml"));
    looks::install();
    if bus::checking() && shards::claim("compiled") {
        looks::qobject::compile_qml();
    }
    if bus::checking() || url == preview::PREVIEW {
        engine.as_mut().on_object_creation_failed(|_, _| std::process::exit(1)).release();
    }
    engine.load(&QUrl::from(url));
    let code = app.exec() as u8;
    if bus::checking() {
        std::mem::forget((owned_engine, owned_app));
    }
    code
}
