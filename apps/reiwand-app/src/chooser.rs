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

const TABLES: [(&str, &str); 2] =
    [("XKB_CONFIG_ROOT", "/usr/share/X11/xkb"), ("XLOCALEDIR", "/usr/share/X11/locale")];

pub fn tables(
    given: impl Fn(&str) -> bool,
    exists: impl Fn(&std::path::Path) -> bool,
) -> Vec<(&'static str, &'static str)> {
    TABLES.into_iter().filter(|(var, dir)| !given(var) && exists(std::path::Path::new(dir))).collect()
}

pub fn use_the_system_keyboard() {
    let exe = std::env::current_exe().ok();
    let carried = exe.and_then(|e| Some(e.parent()?.join("../qt/lib/libxkbcommon.so.0")));
    if carried.is_some_and(|lib| lib.is_file()) {
        for (var, dir) in tables(|var| std::env::var_os(var).is_some(), std::path::Path::is_dir) {
            unsafe { std::env::set_var(var, dir) };
        }
    }
}
