pub(super) fn renderer() -> Option<&'static reiwand_render::Renderer> {
    match reiwand_render::Renderer::shared() {
        Ok(r) => Some(r),
        Err(e) if std::env::var_os("REIWAND_REQUIRE_GPU").is_none() => {
            eprintln!("skipping the scene render: {e}");
            None
        }
        Err(e) => panic!("GPU required but unavailable: {e}"),
    }
}
