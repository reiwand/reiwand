mod folders;
mod gbm;
mod host;
mod qt;
mod quit;
mod replay;
mod ring;
mod run;
mod said;
mod scheme;
mod script;
mod state;
mod watch;

use reiwand_web::{FromWeb, Link, Start, ToWeb};
use serde_json::json;
use std::os::fd::{FromRawFd, OwnedFd};

const SOCKET: i32 = 3;
const FLAGS: &str = "--autoplay-policy=no-user-gesture-required --disable-breakpad --disable-sync \
    --disable-background-networking --no-pings --disable-features=Translate,MediaRouter";

fn sandbox_usable() -> bool {
    match unsafe { libc::fork() } {
        0 => {
            let flags = libc::CLONE_NEWUSER | libc::CLONE_NEWPID | libc::CLONE_NEWNET;
            unsafe { libc::_exit(i32::from(libc::unshare(flags) != 0)) }
        }
        pid if pid > 0 => {
            let mut status = 0;
            unsafe { libc::waitpid(pid, &mut status, 0) };
            libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0
        }
        _ => false,
    }
}

fn proxy(network: &[String]) -> String {
    let bypass: Vec<&str> = std::iter::once("<-loopback>").chain(network.iter().map(String::as_str)).collect();
    format!("--proxy-server=http://127.0.0.1:1 --proxy-bypass-list={}", bypass.join(";"))
}

fn environment(start: &Start) {
    let off = std::env::var_os("QTWEBENGINE_DISABLE_SANDBOX").is_some() || !sandbox_usable();
    unsafe {
        if off {
            std::env::set_var("QTWEBENGINE_DISABLE_SANDBOX", "1");
        }
        std::env::set_var("QTWEBENGINE_CHROMIUM_FLAGS", format!("{FLAGS} {}", proxy(&start.network)));
        if start.device.is_none() {
            std::env::set_var("LIBGL_ALWAYS_SOFTWARE", "1");
        }
    }
}

fn nonce() -> String {
    use std::io::Read;
    let mut bytes = [0u8; 16];
    let _ = std::fs::File::open("/dev/urandom").and_then(|mut f| f.read_exact(&mut bytes));
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn config(start: &Start, nonce: &str) -> String {
    json!({
        "own": format!("wpak://{}/", start.id.to_ascii_lowercase()),
        "prefix": script::prefix(nonce),
        "policy": reiwand_web::csp(&start.network),
        "background": start.background,
        "network": start.network,
        "storage": start.storage,
        "languages": start.languages,
        "modifiers": start.modifiers.iter().map(u64::to_string).collect::<Vec<_>>(),
        "copy": false,
    })
    .to_string()
}

fn main() {
    let mut args = std::env::args().skip(1);
    if let (Some(reiwand_web::PROBE), Some(node)) = (args.next().as_deref(), args.next()) {
        std::process::exit(if gbm::allocates(&node) { 0 } else { 3 });
    }
    if std::env::args().nth(1).as_deref() == Some("--check") {
        let (code, said) = match qt::check() {
            Ok(version) => (0, format!("Qt {version}")),
            Err(reason) => (2, reason),
        };
        println!("{said}");
        std::process::exit(code);
    }
    let link = Link::from_fd(unsafe { OwnedFd::from_raw_fd(SOCKET) });
    let start = match link.recv::<ToWeb>(true) {
        Ok(Some((ToWeb::Start(start), _))) => *start,
        _ => std::process::exit(1),
    };
    let (entries, preload) = match scheme::package(&start) {
        Ok(found) => found,
        Err(reason) => {
            state::say(&link, &FromWeb::Failed(reason), &[]);
            std::process::exit(1);
        }
    };
    state::live(|l| {
        (l.settings, l.allowed, l.properties) =
            (start.settings.clone(), start.allowed, start.properties.clone());
        l.files.extend(start.files.iter().cloned());
        for folder in &start.folders {
            l.files.extend(folder.files.iter().cloned());
            l.folders.insert(folder.prop.clone(), folder.clone());
        }
    });
    environment(&start);
    let (raw, nonce) = (link.raw(), nonce());
    let config = config(&start, &nonce);
    state::set_engine(state::Engine { link, start, entries, preload, nonce });
    let code = qt::run(&config, &host::HOST, raw);
    if code == 2 {
        state::send(&FromWeb::Unavailable(host::last()));
    }
    quit::finish(code);
}
