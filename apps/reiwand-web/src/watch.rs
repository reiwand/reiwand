use crate::state::{View, view};
use reiwand_web::FromWeb;
use std::rc::Rc;
use std::time::{Duration, Instant};

const BEAT: u32 = 5000;
const DEAF: Duration = Duration::from_secs(10);

pub fn beat(view: Rc<View>) {
    crate::qt::later(BEAT, move || tick(view));
}

fn tick(watched: Rc<View>) {
    if view(watched.id).is_none_or(|v| !Rc::ptr_eq(&v, &watched)) {
        return;
    }
    let now = Instant::now();
    let waiting = watched.answered.get() > now;
    match (waiting, now.duration_since(watched.answered.get()) > DEAF) {
        (true, _) => {}
        (false, true) => {
            watched.answered.set(now);
            crate::state::send(&FromWeb::Said("the page stopped answering and loads again".into()));
            let pid = crate::qt::renderer(watched.id);
            if pid > 1 {
                unsafe { libc::kill(pid as i32, libc::SIGKILL) };
            }
        }
        (false, false) => crate::qt::ping(watched.id),
    }
    beat(watched);
}

pub fn answered(id: u32) {
    if let Some(view) = view(id).filter(|v| v.answered.get() <= Instant::now()) {
        view.answered.set(Instant::now());
    }
}
