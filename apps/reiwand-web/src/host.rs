use crate::qt::Host;
use crate::state::{engine, say, send, view};
use reiwand_web::{FromWeb, ToWeb};
use std::ffi::{CStr, c_char, c_int};
use std::os::fd::{AsFd, FromRawFd, OwnedFd};

thread_local! {
    static LAST: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

pub fn last() -> String {
    LAST.with(|l| l.borrow().clone())
}

pub static HOST: Host = Host {
    ready,
    readable,
    loaded,
    crashed,
    message,
    answered,
    buffer,
    frame,
    forget,
    slot,
    pixels,
    serve,
    said,
};

unsafe fn string(text: *const c_char) -> String {
    unsafe { CStr::from_ptr(text) }.to_string_lossy().into_owned()
}

unsafe extern "C" fn ready(version: *const c_char) {
    let sandboxed = std::env::var_os("QTWEBENGINE_DISABLE_SANDBOX").is_none();
    crate::qt::muted(crate::state::live(|l| l.settings.volume <= 0.0));
    send(&FromWeb::Ready { version: unsafe { string(version) }, sandboxed });
    if !sandboxed {
        send(&FromWeb::Said("sandbox: off".into()));
    }
}

unsafe extern "C" fn readable() {
    let Some(engine) = engine() else { return };
    loop {
        match engine.link.recv::<ToWeb>(false) {
            Ok(Some((message, _))) => crate::run::handle(message),
            Ok(None) => return crate::qt::quit(),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return,
            Err(_) => return crate::qt::quit(),
        }
    }
}

unsafe extern "C" fn loaded(id: u32) {
    crate::run::loaded(id);
}

unsafe extern "C" fn crashed(id: u32, status: c_int) {
    crate::run::crashed(id, status);
}

unsafe extern "C" fn message(id: u32, json: *const c_char) {
    crate::script::message(id, &unsafe { string(json) });
}

unsafe extern "C" fn answered(id: u32) {
    crate::watch::answered(id);
}

#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn buffer(
    id: u32,
    buffer: u32,
    width: u32,
    height: u32,
    fourcc: u32,
    modifier: u64,
    offset: u32,
    stride: u32,
    fd: c_int,
) {
    let fd = unsafe { OwnedFd::from_raw_fd(fd) };
    if let Some(engine) = engine() {
        let message =
            FromWeb::Buffer { view: id, id: buffer, size: (width, height), fourcc, modifier, offset, stride };
        say(&engine.link, &message, &[fd.as_fd()]);
    }
}

unsafe extern "C" fn frame(id: u32, buffer: u32) {
    send(&FromWeb::Frame { view: id, buffer });
}

unsafe extern "C" fn forget(id: u32) {
    send(&FromWeb::Forget { view: id });
}

unsafe extern "C" fn slot(id: u32, width: u32, height: u32, stride: u32, index: *mut u32) -> *mut u8 {
    let Some(view) = view(id) else { return std::ptr::null_mut() };
    let mut ring = view.ring.borrow_mut();
    let size = (width, height);
    if !ring.as_ref().is_some_and(|r| r.fits(size, stride)) {
        *ring = crate::ring::Ring::new(size, stride).ok();
    }
    match ring.as_mut().and_then(crate::ring::Ring::claim) {
        Some((slot, at)) => {
            unsafe { *index = slot };
            at
        }
        None => std::ptr::null_mut(),
    }
}

unsafe extern "C" fn pixels(id: u32, slot: u32, width: u32, height: u32, stride: u32) {
    let (Some(view), Some(engine)) = (view(id), engine()) else { return };
    let mut ring = view.ring.borrow_mut();
    let Some(ring) = ring.as_mut() else { return };
    let fresh = !std::mem::replace(&mut ring.sent, true);
    let message = FromWeb::Pixels { view: id, slot, size: (width, height), stride, fresh };
    let fds = if fresh { vec![ring.fd.as_fd()] } else { Vec::new() };
    say(&engine.link, &message, &fds);
}

unsafe extern "C" fn serve(uri: *const c_char, size: *mut u64, mime: *mut c_char, room: u32) -> c_int {
    match crate::scheme::serve(&unsafe { string(uri) }) {
        Some((fd, length, kind)) => {
            let bytes = kind.as_bytes();
            let n = bytes.len().min(room as usize - 1);
            unsafe {
                *size = length;
                std::ptr::copy_nonoverlapping(bytes.as_ptr().cast(), mime, n);
                *mime.add(n) = 0;
            }
            std::os::fd::IntoRawFd::into_raw_fd(fd)
        }
        None => -1,
    }
}

unsafe extern "C" fn said(text: *const c_char) {
    let text = unsafe { string(text) };
    LAST.with(|l| l.borrow_mut().clone_from(&text));
    send(&FromWeb::Said(text));
}
