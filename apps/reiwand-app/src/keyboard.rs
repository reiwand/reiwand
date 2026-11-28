#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("reiwand-app/keys.h");
        #[rust_name = "install_keys"]
        pub fn installKeys();
        #[rust_name = "send_key"]
        pub fn sendKey(key: i32, modifiers: i32);
        #[rust_name = "claims_key"]
        pub fn claimsKey(key: i32, modifiers: i32) -> bool;
        #[rust_name = "send_wheel"]
        pub fn sendWheel(x: f64, y: f64, delta: i32);
        #[rust_name = "send_mouse"]
        pub fn sendMouse(kind: i32, x: f64, y: f64);
    }
}

use crate::backend::qobject::Backend;

pub use qobject::install_keys;

impl Backend {
    pub fn press(&self, key: i32, modifiers: i32) {
        if crate::bus::checking() {
            qobject::send_key(key, modifiers);
        }
    }

    pub fn claims(&self, key: i32, modifiers: i32) -> bool {
        crate::bus::checking() && qobject::claims_key(key, modifiers)
    }

    pub fn wheel(&self, x: f64, y: f64, delta: i32) {
        if crate::bus::checking() {
            qobject::send_wheel(x, y, delta);
        }
    }

    pub fn pointer(&self, kind: i32, x: f64, y: f64) {
        if crate::bus::checking() {
            qobject::send_mouse(kind, x, y);
        }
    }
}
