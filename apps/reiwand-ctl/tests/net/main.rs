mod counted;
mod kinds;
mod net;
mod scenes;

use net::Net;
use reiwand_render::Renderer;

const SIZE: (u32, u32) = (160, 90);
const TIMES: [f64; 3] = [0.0, 1.0, 5.0];

fn renderer() -> Option<&'static Renderer> {
    if std::env::var_os("WGPU_BACKEND").is_none() {
        unsafe { std::env::set_var("WGPU_BACKEND", "vulkan") };
    }
    match Renderer::shared() {
        Ok(r) if r.software() => Some(r),
        Ok(r) => {
            eprintln!("skipping the regression net: {} is not the software renderer", r.adapter());
            None
        }
        Err(e) if std::env::var_os("REIWAND_REQUIRE_GPU").is_none() => {
            eprintln!("skipping the regression net: {e}");
            None
        }
        Err(e) => panic!("the software renderer is required but unavailable: {e}"),
    }
}

fn main() {
    unsafe { std::env::set_var("RUST_LIB_BACKTRACE", "0") };
    reiwand_render::serve_script_host();
    if std::env::args().skip(1).any(|a| a == "--list") {
        return println!("net: test");
    }
    if std::env::var_os("REIWAND_NET").is_none() {
        return eprintln!("skipping the regression net: scripts/regression.sh runs it with its own build");
    }
    let Some(r) = renderer() else { return };
    reiwand_render::set_step_budget(None);
    let mut net = Net::new(r.adapter());
    scenes::check(r, &mut net);
    kinds::check(r, &mut net);
    net.finish();
}
