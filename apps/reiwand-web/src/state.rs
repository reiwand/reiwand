use crate::ring::Ring;
use reiwand_web::{Allowed, Folder, FromWeb, Link, Settings, Start};
use std::cell::{Cell, OnceCell, RefCell};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Instant;

pub struct Engine {
    pub link: Link,
    pub start: Start,
    pub entries: HashSet<String>,
    pub preload: Option<String>,
    pub nonce: String,
}

pub struct View {
    pub id: u32,
    pub visible: Cell<bool>,
    pub hidden_at: Cell<Option<Instant>>,
    pub crashes: Cell<u32>,
    pub loaded: Cell<bool>,
    pub answered: Cell<Instant>,
    pub ring: RefCell<Option<Ring>>,
    pub ring_failed: Cell<bool>,
}

#[derive(Default)]
pub struct Live {
    pub settings: Settings,
    pub allowed: Allowed,
    pub lights: std::collections::BTreeSet<u32>,
    pub properties: serde_json::Map<String, serde_json::Value>,
    pub views: BTreeMap<u32, serde_json::Map<String, serde_json::Value>>,
    pub folders: BTreeMap<String, Folder>,
    pub files: HashMap<String, PathBuf>,
    pub audio: bool,
    pub media: bool,
    pub heard: Option<serde_json::Value>,
    pub said: crate::said::Said,
}

impl Live {
    pub fn file(&self, uri: &str) -> Option<PathBuf> {
        let fold = |u: &str| match u.split_once("://") {
            Some((scheme, rest)) => {
                let (host, tail) = rest.split_once('/').unwrap_or((rest, ""));
                format!("{scheme}://{}/{tail}", host.to_ascii_lowercase())
            }
            None => u.to_owned(),
        };
        self.files
            .get(uri)
            .or_else(|| {
                let wanted = fold(uri);
                self.files.iter().find(|(k, _)| fold(k) == wanted).map(|(_, v)| v)
            })
            .cloned()
    }
}

thread_local! {
    static ENGINE: OnceCell<&'static Engine> = const { OnceCell::new() };
    pub static VIEWS: RefCell<BTreeMap<u32, Rc<View>>> = const { RefCell::new(BTreeMap::new()) };
    pub static LIVE: RefCell<Live> = RefCell::new(Live::default());
}

pub fn set_engine(engine: Engine) {
    let leaked: &'static Engine = Box::leak(Box::new(engine));
    ENGINE.with(|e| {
        let _ = e.set(leaked);
    });
}

pub fn engine() -> Option<&'static Engine> {
    ENGINE.with(|e| e.get().copied())
}

pub fn send(message: &FromWeb) {
    if let Some(engine) = engine() {
        say(&engine.link, message, &[]);
    }
}

/// Whether the message reached the daemon.
pub fn say(link: &Link, message: &FromWeb, fds: &[std::os::fd::BorrowedFd<'_>]) -> bool {
    let sent = link.send(message, fds);
    if let Err(e) = &sent {
        eprintln!("reiwand-web: cannot reach the daemon: {e}");
    }
    sent.is_ok()
}

pub fn view(id: u32) -> Option<Rc<View>> {
    VIEWS.with(|v| v.borrow().get(&id).cloned())
}

pub fn all() -> Vec<Rc<View>> {
    VIEWS.with(|v| v.borrow().values().cloned().collect())
}

pub fn live<T>(f: impl FnOnce(&mut Live) -> T) -> T {
    LIVE.with(|l| f(&mut l.borrow_mut()))
}
