use crate::qt;
use crate::script::tell;
use crate::state::{VIEWS, View, all, engine, live, send, view};
use reiwand_web::{FromWeb, ToWeb};
use serde_json::json;
use std::rc::Rc;
use std::time::{Duration, Instant};

const ASLEEP: Duration = Duration::from_secs(300);
const RELOADS: u32 = 3;
const STABLE: u32 = 60_000;

pub fn handle(message: ToWeb) {
    match message {
        ToWeb::Start(_) => {}
        ToWeb::Open { view, size, scale, properties } => open(view, size, scale, properties),
        ToWeb::Close(id) => close(id),
        ToWeb::Resize { view: id, size, scale } => qt::resize(id, size, scale),
        ToWeb::Visible { view: id, visible } => view(id).into_iter().for_each(|v| show(&v, visible)),
        ToWeb::Fps { view: id, fps } => rate(id, fps.max(1)),
        ToWeb::Settings(settings) => {
            qt::muted(settings.volume <= 0.0);
            tell(&json!({"settings": settings}));
            live(|l| l.settings = settings);
        }
        ToWeb::Properties { view, values } => {
            live(|l| l.views.entry(view).or_default().extend(values.clone()));
            crate::script::tell_view(view, &json!({"properties": values}));
        }
        ToWeb::Allowed(allowed) => live(|l| l.allowed = allowed),
        ToWeb::Lights { view, on } => {
            live(|l| if on { l.lights.insert(view) } else { l.lights.remove(&view) });
            crate::script::tell_view(view, &json!({"lights": on}));
        }
        ToWeb::Audio { left, right } => {
            if live(|l| l.audio && l.allowed.audio) {
                tell(&json!({"audio": [left, right]}));
            }
        }
        ToWeb::Media(heard) => crate::script::heard(heard),
        ToWeb::Pointer { view: id, at } => pointer(id, at),
        ToWeb::Press { view: id, at, button } => press(id, at, button),
        ToWeb::Folder(folder) => crate::folders::update(folder),
        ToWeb::Files(files) => live(|l| l.files.extend(files)),
        ToWeb::Action(id) => tell(&json!({"action": id})),
        ToWeb::Released { view: id, buffer } => qt::released(id, buffer),
        ToWeb::Freed { view: id, slot } => {
            view(id).into_iter().for_each(|v| v.ring.borrow_mut().iter_mut().for_each(|r| r.release(slot)));
            qt::released(id, 0);
        }
        ToWeb::Copy => qt::copy(),
        ToWeb::Shot { view: id } => qt::shot(id),
        ToWeb::Quit => qt::quit(),
    }
}

fn open(id: u32, size: (u32, u32), scale: f64, properties: serde_json::Map<String, serde_json::Value>) {
    live(|l| l.views.insert(id, properties));
    let view = View {
        id,
        visible: true.into(),
        hidden_at: None.into(),
        crashes: 0.into(),
        loaded: false.into(),
        answered: Instant::now().into(),
        ring: None.into(),
    };
    let view = Rc::new(view);
    VIEWS.with(|v| v.borrow_mut().insert(id, view.clone()));
    qt::open(id, size, scale);
    qt::fps(id, live(|l| l.settings.fps).max(1));
    load(id);
    crate::watch::beat(view);
}

fn rate(id: u32, fps: u32) {
    qt::fps(id, fps);
    if view(id).is_some_and(|v| v.loaded.get()) {
        let key = engine().map(|e| serde_json::json!(crate::script::prefix(&e.nonce))).unwrap_or_default();
        qt::eval(id, &format!("window.__rafCap && window.__rafCap({key}, {fps})"));
    }
}

pub fn load(id: u32) {
    let Some(view) = view(id) else { return };
    crate::script::install(id);
    let Some(engine) = engine() else { return };
    view.answered.set(Instant::now());
    qt::load(id, &format!("wpak://{}/{}", engine.start.id.to_ascii_lowercase(), engine.start.entry));
}

pub fn close(id: u32) {
    if VIEWS.with(|v| v.borrow_mut().remove(&id)).is_some() {
        qt::close(id);
    }
}

fn show(view: &Rc<View>, visible: bool) {
    if view.visible.replace(visible) == visible {
        return;
    }
    qt::visible(view.id, visible);
    if visible {
        view.hidden_at.set(None);
        return;
    }
    let at = Instant::now();
    view.hidden_at.set(Some(at));
    let view = view.clone();
    qt::later(ASLEEP.as_millis() as u32, move || {
        let idle = all().iter().all(|v| !v.visible.get());
        if view.hidden_at.get() == Some(at) && idle {
            send(&FromWeb::Idle);
        }
    });
}

pub fn loaded(id: u32) {
    let Some(view) = view(id) else { return };
    view.loaded.set(true);
    send(&FromWeb::Loaded { view: id });
    crate::script::replay(id);
    let crashes = view.crashes.get();
    if crashes > 0 {
        qt::later(STABLE, move || {
            view_stable(id, crashes);
        });
    }
}

fn view_stable(id: u32, crashes: u32) {
    view(id).filter(|v| v.crashes.get() == crashes).iter().for_each(|v| v.crashes.set(0));
}

pub fn crashed(id: u32, _status: i32) {
    let Some(view) = view(id) else { return };
    let crashes = view.crashes.get() + 1;
    view.crashes.set(crashes);
    view.loaded.set(false);
    send(&FromWeb::Crashed { view: id });
    let wait = if crashes <= RELOADS { 1000 * crashes.pow(2) } else { u32::MAX };
    view.answered.set(Instant::now() + Duration::from_millis(u64::from(wait)));
    match crashes <= RELOADS {
        true => qt::later(wait, move || load(id)),
        false => send(&FromWeb::Broken { view: id }),
    }
}

fn pointer(id: u32, at: Option<(f64, f64)>) {
    if live(|l| l.allowed.pointer || l.allowed.clicks) {
        qt::pointer(id, at);
    }
}

fn press(id: u32, at: (f64, f64), button: u32) {
    if live(|l| l.allowed.clicks) {
        qt::pointer(id, Some(at));
        qt::press(id, at, button);
    }
}
