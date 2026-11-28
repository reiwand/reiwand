use reiwand_render::{Renderer, set_step_budget};

pub fn renderer() -> Option<&'static Renderer> {
    set_step_budget(None);
    match Renderer::shared() {
        Ok(r) => Some(r),
        Err(e) if std::env::var_os("REIWAND_REQUIRE_GPU").is_none() => {
            eprintln!("skipping GPU test: {e}");
            None
        }
        Err(e) => panic!("GPU required but unavailable: {e}"),
    }
}

include!("whole.rs");
