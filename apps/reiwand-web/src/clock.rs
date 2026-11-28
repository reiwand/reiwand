//! The frame clock of a rendered page (`Start::clock`): the page's time stands still until the host steps it.

const SOURCE: &str = include_str!("clock.js");

/// The script that puts the frame clock in the page, before any script of the page runs; `wall` is the wall
/// clock at the first frame in milliseconds.
pub fn script(post: &str, key: &str, wall: f64) -> String {
    format!(
        "{}({}, {}, {}, {});",
        SOURCE.trim_end(),
        "globalThis",
        post,
        serde_json::json!(key),
        serde_json::json!(wall)
    )
}

/// The call that moves the clock of the page to `at` seconds and paints it.
pub fn step(key: &str, at: f64) -> String {
    let at = if at.is_finite() { at.max(0.0) * 1000.0 } else { 0.0 };
    format!(
        "window.__reiwandStep && window.__reiwandStep({}, {})",
        serde_json::json!(key),
        serde_json::json!(at)
    )
}
