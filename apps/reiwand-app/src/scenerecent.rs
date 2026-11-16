#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[auto_cxx_name]
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type Recents = super::RecentsRust;

        #[qinvokable]
        fn projects(self: &Recents) -> QString;
        #[qinvokable]
        fn forget_project(self: &Recents, folder: &QString);
        #[qinvokable]
        fn hide_project(self: &Recents, folder: &QString);
        #[qinvokable]
        fn clear_recents(self: &Recents);
        #[qinvokable]
        fn snapshot(self: &Recents) -> QString;
        #[qinvokable]
        fn restore(self: &Recents, text: &QString);
        #[qinvokable]
        fn is_project(self: &Recents, folder: &QString) -> bool;
        #[qinvokable]
        fn shown(self: &Recents, folder: &QString) -> QString;
        #[qinvokable]
        fn homed(self: &Recents, folder: &QString) -> QString;
        #[qinvokable]
        fn expanded(self: &Recents, text: &QString) -> QString;
    }
}

use crate::recentstore as store;
use cxx_qt_lib::QString;
use qobject::Recents;
use reiwand_editor::project;
use std::path::Path;

#[derive(Default)]
pub struct RecentsRust;

impl Recents {
    pub fn projects(&self) -> QString {
        QString::from(&serde_json::to_string(&store::list()).unwrap_or_default())
    }

    pub fn forget_project(&self, folder: &QString) {
        store::forget(Path::new(&folder.to_string()));
    }

    pub fn hide_project(&self, folder: &QString) {
        store::hide_one(Path::new(&folder.to_string()));
    }

    pub fn clear_recents(&self) {
        store::clear();
    }

    pub fn snapshot(&self) -> QString {
        QString::from(&store::snapshot())
    }

    pub fn restore(&self, text: &QString) {
        store::restore(&text.to_string());
    }

    pub fn is_project(&self, folder: &QString) -> bool {
        project::read(Path::new(&folder.to_string())).is_ok()
    }

    pub fn shown(&self, folder: &QString) -> QString {
        QString::from(&short_path(&folder.to_string(), &home(), 56))
    }

    pub fn homed(&self, folder: &QString) -> QString {
        QString::from(&short_path(&folder.to_string(), &home(), usize::MAX))
    }

    pub fn expanded(&self, text: &QString) -> QString {
        QString::from(&full_path(&text.to_string(), &home()))
    }
}

fn home() -> String {
    dirs::home_dir().map(|h| h.to_string_lossy().into_owned()).unwrap_or_default()
}

pub fn full_path(text: &str, home: &str) -> String {
    match text.strip_prefix('~') {
        Some(rest) if !home.is_empty() && (rest.is_empty() || rest.starts_with('/')) => {
            format!("{}{rest}", home.trim_end_matches('/'))
        }
        _ => text.to_owned(),
    }
}

pub fn short_path(path: &str, home: &str, max: usize) -> String {
    let home = home.trim_end_matches('/');
    let path = match path.strip_prefix(home) {
        Some(rest) if !home.is_empty() && (rest.is_empty() || rest.starts_with('/')) => format!("~{rest}"),
        _ => path.to_owned(),
    };
    let chars: Vec<char> = path.chars().collect();
    if chars.len() <= max || max < 3 {
        return path;
    }
    let head = (max - 1) / 3;
    let tail = max - 1 - head;
    let mut out: String = chars[..head].iter().collect();
    out.push('…');
    out.extend(&chars[chars.len() - tail..]);
    out
}
