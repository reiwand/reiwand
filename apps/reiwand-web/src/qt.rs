use std::ffi::{CStr, CString, c_char, c_int, c_void};

pub type Back = unsafe extern "C" fn(*mut c_void);

#[repr(C)]
pub struct Host {
    pub ready: unsafe extern "C" fn(*const c_char),
    pub readable: unsafe extern "C" fn(),
    pub loaded: unsafe extern "C" fn(u32),
    pub crashed: unsafe extern "C" fn(u32, c_int),
    pub message: unsafe extern "C" fn(u32, *const c_char),
    pub answered: unsafe extern "C" fn(u32),
    pub buffer: unsafe extern "C" fn(u32, u32, u32, u32, u32, u64, u32, u32, c_int),
    pub frame: unsafe extern "C" fn(u32, u32),
    pub forget: unsafe extern "C" fn(u32),
    pub slot: unsafe extern "C" fn(u32, u32, u32, u32, *mut u32) -> *mut u8,
    pub pixels: unsafe extern "C" fn(u32, u32, u32, u32, u32),
    pub serve: unsafe extern "C" fn(*const c_char, *mut u64, *mut c_char, u32) -> c_int,
    pub said: unsafe extern "C" fn(*const c_char),
}

#[cfg(not(no_engine))]
mod sys {
    use super::{Back, Host};
    use std::ffi::{c_char, c_int, c_void};
    unsafe extern "C" {
        pub fn rw_check(message: *mut c_char, room: usize) -> c_int;
        pub fn rw_run(config: *const c_char, host: *const Host, socket: c_int) -> c_int;
        pub fn rw_quit();
        pub fn rw_open(view: u32, width: u32, height: u32, scale: f64);
        pub fn rw_close(view: u32);
        pub fn rw_resize(view: u32, width: u32, height: u32, scale: f64);
        pub fn rw_visible(view: u32, visible: c_int);
        pub fn rw_fps(view: u32, fps: u32);
        pub fn rw_scripts(view: u32, json: *const c_char);
        pub fn rw_load(view: u32, url: *const c_char);
        pub fn rw_eval(view: u32, code: *const c_char);
        pub fn rw_ping(view: u32);
        pub fn rw_muted(muted: c_int);
        pub fn rw_copy();
        pub fn rw_released(view: u32, id: u32);
        pub fn rw_shot(view: u32);
        pub fn rw_pointer(view: u32, present: c_int, x: f64, y: f64);
        pub fn rw_press(view: u32, x: f64, y: f64, button: u32);
        pub fn rw_later(millis: u32, run: Back, data: *mut c_void);
    }
}

#[cfg(no_engine)]
#[path = "qt_none.rs"]
mod sys;

fn text(value: &str) -> CString {
    CString::new(value.replace('\0', "")).unwrap_or_default()
}

pub fn check() -> Result<String, String> {
    let mut buffer = [0 as c_char; 512];
    let code = unsafe { sys::rw_check(buffer.as_mut_ptr(), buffer.len()) };
    let said = unsafe { CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().into_owned();
    if code == 0 { Ok(said) } else { Err(said) }
}

pub fn run(config: &str, host: &'static Host, socket: c_int) -> i32 {
    unsafe { sys::rw_run(text(config).as_ptr(), host, socket) }
}

pub fn quit() {
    unsafe { sys::rw_quit() }
}

pub fn open(view: u32, size: (u32, u32), scale: f64) {
    unsafe { sys::rw_open(view, size.0, size.1, scale) }
}

pub fn close(view: u32) {
    unsafe { sys::rw_close(view) }
}

pub fn resize(view: u32, size: (u32, u32), scale: f64) {
    unsafe { sys::rw_resize(view, size.0, size.1, scale) }
}

pub fn visible(view: u32, visible: bool) {
    unsafe { sys::rw_visible(view, c_int::from(visible)) }
}

pub fn fps(view: u32, fps: u32) {
    unsafe { sys::rw_fps(view, fps) }
}

pub fn scripts(view: u32, json: &str) {
    unsafe { sys::rw_scripts(view, text(json).as_ptr()) }
}

pub fn load(view: u32, url: &str) {
    unsafe { sys::rw_load(view, text(url).as_ptr()) }
}

pub fn eval(view: u32, code: &str) {
    unsafe { sys::rw_eval(view, text(code).as_ptr()) }
}

pub fn ping(view: u32) {
    unsafe { sys::rw_ping(view) }
}

pub fn muted(muted: bool) {
    unsafe { sys::rw_muted(c_int::from(muted)) }
}

pub fn copy() {
    unsafe { sys::rw_copy() }
}

pub fn released(view: u32, id: u32) {
    unsafe { sys::rw_released(view, id) }
}

pub fn shot(view: u32) {
    unsafe { sys::rw_shot(view) }
}

pub fn pointer(view: u32, at: Option<(f64, f64)>) {
    let (x, y) = at.unwrap_or_default();
    unsafe { sys::rw_pointer(view, c_int::from(at.is_some()), x, y) }
}

pub fn press(view: u32, at: (f64, f64), button: u32) {
    unsafe { sys::rw_press(view, at.0, at.1, button) }
}

pub fn later(millis: u32, run: impl FnOnce() + 'static) {
    unsafe extern "C" fn fire(data: *mut c_void) {
        let run = unsafe { Box::from_raw(data.cast::<Box<dyn FnOnce()>>()) };
        run();
    }
    let boxed: Box<Box<dyn FnOnce()>> = Box::new(Box::new(run));
    unsafe { sys::rw_later(millis, fire, Box::into_raw(boxed).cast()) }
}
