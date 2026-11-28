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
        #[qproperty(bool, moving)]
        #[qproperty(QString, error)]
        type DepthFollow = super::FollowRust;

        #[qinvokable]
        fn open(self: Pin<&mut DepthFollow>, path: &QString);
        #[qinvokable]
        fn follow(self: Pin<&mut DepthFollow>, on: bool);
    }
}

use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use qobject::DepthFollow;
use reiwand_core::edit::{self, Action, Edit, History, Source};
use std::path::{Path, PathBuf};
use std::pin::Pin;

#[derive(Default)]
pub struct FollowRust {
    pub moving: bool,
    pub error: QString,
    pub file: PathBuf,
}

pub(crate) fn current(file: &Path) -> Result<Edit, String> {
    if !file.is_file() {
        return Err(edit::missing(file).local());
    }
    match edit::load(file, &edit::dir()).map_err(|e| e.local())? {
        Some((_, edit)) => Ok(edit),
        None => {
            let (w, h) = reiwand_render::image_size(file).map_err(|e| e.to_string())?;
            let source = Source::read(file, [w, h]).map_err(|e| e.to_string())?;
            Ok(Edit { source, ..Edit::default() })
        }
    }
}

pub(crate) fn follow(file: &Path, on: bool) -> Result<Edit, String> {
    let before = current(file)?;
    let mut after = before.clone();
    let source = on.then(|| "pointer".to_owned());
    let action = Action::Parallax {
        source,
        strength: None,
        focus: None,
        steps: None,
        smoothing: None,
        overscan: None,
        fill: None,
        layers: None,
        off: !on,
    };
    after.act(action).map_err(|e| e.message().local())?;
    edit::commit(&after, file, &edit::dir()).map_err(|e| e.local())?;
    let mut history = History::load(file);
    history.record(&before, &after);
    history.save(file).map_err(|e| e.to_string())?;
    Ok(after)
}

fn moving(edit: &Edit) -> bool {
    edit.parallax.as_ref().is_some_and(|p| p.enabled)
}

impl DepthFollow {
    pub fn open(mut self: Pin<&mut Self>, path: &QString) {
        let file = crate::app::path(&path.to_string()).map(PathBuf::from).unwrap_or_default();
        let on = current(&file).is_ok_and(|e| moving(&e));
        self.as_mut().rust_mut().file = file;
        self.as_mut().set_error(QString::default());
        self.set_moving(on);
    }

    pub fn follow(mut self: Pin<&mut Self>, on: bool) {
        let done = follow(&self.file, on);
        self.as_mut().set_error(QString::from(done.as_ref().err().map(String::as_str).unwrap_or_default()));
        let on = done.map_or(self.moving, |e| moving(&e));
        self.set_moving(on);
    }
}
