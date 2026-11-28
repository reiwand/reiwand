use crate::shards::{self, Census};
use crate::{bus, checkpick};
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

const CHECK_TIME: Duration = Duration::from_secs(900);
const TAIL: usize = 40;
const FONT_DIRS: [&str; 1] = ["/usr/share/fonts"];
const SCREEN: &str = r#"{"screens": [{"name": "check", "x": 0, "y": 0, "width": 1600, "height": 900,
    "logicalDpi": 96, "logicalBaseDpi": 96, "dpr": 1}], "windowFrameMargins": false}"#;

fn fonts(state: &Path) -> Option<PathBuf> {
    let dirs: String = FONT_DIRS
        .iter()
        .filter(|dir| Path::new(dir).is_dir())
        .map(|dir| format!("<dir>{dir}</dir>\n"))
        .collect();
    (!dirs.is_empty()).then_some(())?;
    std::fs::create_dir_all(state).ok()?;
    let cache = state.join("fonts-cache");
    let file = state.join("fonts.conf");
    let text = format!(
        "<?xml version=\"1.0\"?>\n<fontconfig>\n{dirs}<cachedir>{}</cachedir>\n</fontconfig>\n",
        cache.display()
    );
    std::fs::write(&file, text).ok()?;
    Some(file)
}

fn spawn(
    state: &Path,
    shard: usize,
    shards: usize,
    font_file: Option<&Path>,
    only: Option<&str>,
) -> std::io::Result<Child> {
    let mut platform = std::env::var("QT_QPA_PLATFORM").unwrap_or_else(|_| "offscreen".into());
    if platform == "offscreen" {
        let screen = state.join("screen.json");
        std::fs::write(&screen, SCREEN)?;
        platform = format!("offscreen:configfile={}", screen.display());
    }
    let own = state.join(format!("shard-{shard}"));
    let mut command = Command::new(std::env::current_exe()?);
    command.env(bus::CHECK, "1").env("QT_QPA_PLATFORM", platform).env("QT_FORCE_STDERR_LOGGING", "1");
    command.env("XDG_STATE_HOME", own.join("state"));
    if shards > 1 {
        command.env("XDG_DATA_HOME", own.join("data")).env("XDG_CACHE_HOME", own.join("cache"));
        command.env(shards::CLAIMS, state.join("claims"));
    }
    if let Some(file) = font_file {
        command.env("FONTCONFIG_FILE", file);
    }
    if let Some(only) = only {
        command.env(checkpick::ONLY, only);
    }
    command.args(["--page", "settings"]);
    command.stderr(Stdio::piped()).spawn()
}

fn read(
    stderr: std::process::ChildStderr,
    census: Arc<Mutex<Census>>,
    problems: Arc<Mutex<usize>>,
) -> VecDeque<String> {
    let mut last = VecDeque::with_capacity(TAIL);
    for line in BufReader::new(stderr).lines().map_while(Result::ok) {
        eprintln!("{line}");
        census.lock().unwrap().note(&line);
        let bad = [".qml", "QQmlApplicationEngine", "call failed"].iter().any(|p| line.contains(p));
        *problems.lock().unwrap() += usize::from(bad);
        if last.len() == TAIL {
            last.pop_front();
        }
        last.push_back(line);
    }
    last
}

pub fn run(args: &[String]) -> ExitCode {
    let (patterns, list) = match checkpick::parse(args) {
        Ok(chosen) => chosen,
        Err(said) => {
            eprintln!("{said}");
            return ExitCode::from(2);
        }
    };
    let only = checkpick::only(&patterns, list);
    let state = std::env::temp_dir().join(format!("reiwand-app-check-state-{}", std::process::id()));
    let font_file = std::env::var_os("FONTCONFIG_FILE").is_none().then(|| fonts(&state)).flatten();
    let shards = shards::count();
    std::fs::create_dir_all(state.join("claims")).ok();
    let spawned = (0..shards).map(|n| spawn(&state, n, shards, font_file.as_deref(), only.as_deref()));
    let Ok(mut children) = spawned.collect::<Result<Vec<_>, _>>() else { return ExitCode::FAILURE };
    let ids: Vec<u32> = children.iter().map(Child::id).collect();
    let census = Arc::new(Mutex::new(Census::default()));
    let problems = Arc::new(Mutex::new(0usize));
    let readers: Vec<_> = children
        .iter_mut()
        .map(|child| {
            let stderr = child.stderr.take().unwrap();
            let (census, problems) = (census.clone(), problems.clone());
            std::thread::spawn(move || read(stderr, census, problems))
        })
        .collect();
    let children: Vec<_> = children.into_iter().map(|child| Arc::new(Mutex::new(child))).collect();
    let (done, waited) = mpsc::channel();
    let watched = children.clone();
    let watch = std::thread::spawn(move || {
        let late = waited.recv_timeout(CHECK_TIME).is_err();
        if late {
            eprintln!("reiwand-app: the QML check did not finish within {} s", CHECK_TIME.as_secs());
            watched.iter().for_each(|child| drop(child.lock().unwrap().kill()));
        }
        late
    });
    let last: Vec<VecDeque<String>> =
        readers.into_iter().map(|reader| reader.join().unwrap_or_default()).collect();
    done.send(()).ok();
    let late = watch.join().unwrap_or(true);
    let ended: Vec<_> = children.iter().map(|child| child.lock().unwrap().wait()).collect();
    for (n, (end, last)) in ended.iter().zip(&last).enumerate() {
        match end {
            Ok(status) if status.success() => {}
            Ok(status) => {
                eprintln!("reiwand-app: check process {n} ended with {status}; its last lines:");
                last.iter().for_each(|line| eprintln!("reiwand-app: [{n}] {line}"));
            }
            Err(error) => eprintln!("reiwand-app: check process {n} could not be waited for: {error}"),
        }
    }
    let failed = ended.iter().any(|end| !end.as_ref().is_ok_and(|s| s.success()));
    for id in ids {
        std::fs::remove_dir_all(std::env::temp_dir().join(format!("reiwand-app-check-{id}"))).ok();
    }
    std::fs::remove_dir_all(&state).ok();
    let census = census.lock().unwrap();
    if list {
        census.listed().iter().for_each(|unit| println!("{unit}"));
        return if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS };
    }
    let mut missing = if shards > 1 { census.problems() } else { Vec::new() };
    if !patterns.is_empty() && census.units() == 0 {
        missing.push(format!("no check unit matches {}; --check --list names them", patterns.join(" ")));
    }
    missing.iter().for_each(|m| eprintln!("reiwand-app: {m}"));
    let found = *problems.lock().unwrap() + missing.len() + usize::from(late);
    if shards > 1 || only.is_some() {
        eprintln!("reiwand-app: {} check units in {shards} processes", census.units());
    }
    if failed || found > 0 {
        eprintln!("reiwand-app: the QML check found {found} problems");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
