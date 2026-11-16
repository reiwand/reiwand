const THEME: &str = "QT_QPA_PLATFORMTHEME";
const PORTAL: &str = "xdgdesktopportal";

pub fn theme(current: Option<&str>) -> Option<&'static str> {
    match current.map(str::trim) {
        None | Some("") => Some(PORTAL),
        Some(name) if name.eq_ignore_ascii_case("kde") => Some(PORTAL),
        Some(_) => None,
    }
}

pub fn ask_the_desktop() {
    if let Some(name) = theme(std::env::var(THEME).ok().as_deref()) {
        unsafe { std::env::set_var(THEME, name) };
    }
}
