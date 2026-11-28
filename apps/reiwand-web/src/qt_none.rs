use super::{Back, Host};
use std::ffi::{c_char, c_int, c_void};

pub const WHY: &str = env!("REIWAND_WEB_QT");

pub unsafe fn rw_check(message: *mut c_char, room: usize) -> c_int {
    let text = format!("Qt WebEngine is missing: {WHY}");
    let bytes = text.as_bytes();
    let n = bytes.len().min(room.saturating_sub(1));
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr().cast(), message, n);
        *message.add(n) = 0;
    }
    2
}

pub unsafe fn rw_run(_: *const c_char, host: *const Host, _: c_int) -> c_int {
    let text = std::ffi::CString::new(format!("Qt WebEngine is missing: {WHY}")).unwrap_or_default();
    unsafe { ((*host).said)(text.as_ptr()) };
    2
}
pub unsafe fn rw_quit() {}
pub unsafe fn rw_open(_: u32, _: u32, _: u32, _: f64) -> c_int {
    0
}
pub unsafe fn rw_close(_: u32) {}
pub unsafe fn rw_resize(_: u32, _: u32, _: u32, _: f64) {}
pub unsafe fn rw_visible(_: u32, _: c_int) {}
pub unsafe fn rw_fps(_: u32, _: u32) {}
pub unsafe fn rw_scripts(_: u32, _: *const c_char) {}
pub unsafe fn rw_load(_: u32, _: *const c_char) {}
pub unsafe fn rw_eval(_: u32, _: *const c_char) {}
pub unsafe fn rw_ping(_: u32) {}
pub unsafe fn rw_renderer(_: u32) -> i64 {
    0
}
pub unsafe fn rw_muted(_: c_int) {}
pub unsafe fn rw_copy() {}
pub unsafe fn rw_released(_: u32, _: u32) {}
pub unsafe fn rw_shot(_: u32) {}
pub unsafe fn rw_pointer(_: u32, _: c_int, _: f64, _: f64) {}
pub unsafe fn rw_press(_: u32, _: f64, _: f64, _: u32) {}
pub unsafe fn rw_later(_: u32, _: Back, _: *mut c_void) {}
