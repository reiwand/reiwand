#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[auto_cxx_name]
    extern "RustQt" {
        /// The layouts of the editor (`benchdesk::Desk`), passed to and from QML as JSON text.
        #[qobject]
        #[qml_element]
        #[qproperty(QString, place)]
        type Layouts = super::LayoutsRust;

        #[qinvokable]
        fn fresh(self: &Layouts) -> QString;
        #[qinvokable]
        fn file(self: &Layouts) -> QString;
        #[qinvokable]
        fn load(self: &Layouts) -> QString;
        #[qinvokable]
        fn save(self: &Layouts, desk: &QString) -> QString;
        #[qinvokable]
        fn root(self: &Layouts, desk: &QString) -> QString;
        #[qinvokable]
        fn usual(self: &Layouts, desk: &QString) -> QString;
        #[qinvokable]
        fn put(self: &Layouts, desk: &QString, root: &QString) -> QString;
        #[qinvokable]
        fn choose(self: &Layouts, desk: &QString, chosen: &QString) -> QString;
        #[qinvokable]
        fn keyed(self: &Layouts, desk: &QString, number: i32) -> QString;
        #[qinvokable]
        fn name_fault(self: &Layouts, desk: &QString, name: &QString, except: i32) -> QString;
        #[qinvokable]
        fn save_as(self: &Layouts, desk: &QString, name: &QString) -> QString;
        #[qinvokable]
        fn rename(self: &Layouts, desk: &QString, index: i32, name: &QString) -> QString;
        #[qinvokable]
        fn remove(self: &Layouts, desk: &QString, index: i32) -> QString;
        #[qinvokable]
        fn reset(self: &Layouts, desk: &QString) -> QString;
        #[qinvokable]
        fn opening(self: &Layouts, desk: &QString, kind: &QString, fresh: bool) -> QString;
    }
}

#[path = "benchold.rs"]
pub(crate) mod old;

use crate::bench::Node;
use crate::benchdesk::{Chosen, Desk, Trees};
use crate::benchkits::{factory, full};
use crate::benchqt::{of, told};
use crate::benchstore::{self as store, Loaded};
use cxx_qt_lib::QString;
use qobject::Layouts;
use serde_json::json;

#[derive(Default)]
pub struct LayoutsRust {
    /// Whose layouts these are: empty for a scene, the kind of a package that is not one.
    place: QString,
}

fn index(number: i32) -> Option<usize> {
    usize::try_from(number).ok()
}

impl Layouts {
    fn path(&self) -> std::path::PathBuf {
        store::file_for(&self.place().to_string())
    }

    /// A scene's layouts before any file of layouts: the arrangement earlier versions kept.
    fn conf(&self) -> Option<std::path::PathBuf> {
        let scene = self.place().is_empty() && !crate::bus::checking();
        scene.then(|| Some(dirs::config_dir()?.join("reiwand").join("window.conf"))).flatten()
    }

    /// The Scene layout from the arrangement of earlier versions, saved at once in the file of
    /// layouts; only then is the old key removed, so it is never lost.
    fn carried(&self) -> Option<Desk> {
        let conf = self.conf()?;
        let desk = old::carried(&conf)?;
        if store::save_to(&self.path(), &desk).is_ok() {
            // a key that cannot be removed now is converted again, to the same layout, next time
            drop(old::forget(&conf));
        }
        Some(desk)
    }

    /// The trees these layouts come with: a package page that is not a scene keeps every panel
    /// it has in view; a scene starts from the trees of the design.
    fn trees(&self) -> Trees {
        if self.place().is_empty() { factory } else { full }
    }

    pub fn fresh(&self) -> QString {
        told(Some(Desk::from_trees(self.trees())))
    }

    pub fn file(&self) -> QString {
        QString::from(&self.path().display().to_string())
    }

    pub fn load(&self) -> QString {
        let (desk, refused, mended) = match store::load_from(&self.path()) {
            Loaded::Fresh => {
                (self.carried().and_then(|d| serde_json::to_string(&d).ok()), String::new(), String::new())
            }
            Loaded::Kept(desk) => (serde_json::to_string(&desk).ok(), String::new(), String::new()),
            Loaded::Mended(desk, said) => (serde_json::to_string(&desk).ok(), String::new(), said),
            Loaded::Refused(said) => (None, said, String::new()),
        };
        QString::from(&json!({ "desk": desk, "refused": refused, "mended": mended }).to_string())
    }

    pub fn save(&self, desk: &QString) -> QString {
        let Some(desk) = of::<Desk>(desk) else { return QString::from("not a set of layouts") };
        QString::from(&store::save_to(&self.path(), &desk).err().unwrap_or_default())
    }

    pub fn root(&self, desk: &QString) -> QString {
        told(of::<Desk>(desk).map(|d| d.root().clone()))
    }

    pub fn usual(&self, desk: &QString) -> QString {
        told(of::<Desk>(desk).map(|d| d.usual_in(self.trees())))
    }

    pub fn put(&self, desk: &QString, root: &QString) -> QString {
        told(of::<Desk>(desk).zip(of::<Node>(root)).and_then(|(d, r)| d.put(r)))
    }

    pub fn choose(&self, desk: &QString, chosen: &QString) -> QString {
        told(of::<Desk>(desk).zip(of::<Chosen>(chosen)).and_then(|(d, c)| d.choose(c)))
    }

    pub fn keyed(&self, desk: &QString, number: i32) -> QString {
        told(of::<Desk>(desk).zip(index(number)).and_then(|(d, n)| d.keyed(n)))
    }

    pub fn name_fault(&self, desk: &QString, name: &QString, except: i32) -> QString {
        let said = of::<Desk>(desk).and_then(|d| d.name_fault(&name.to_string(), index(except)));
        QString::from(&said.unwrap_or_default())
    }

    pub fn save_as(&self, desk: &QString, name: &QString) -> QString {
        told(of::<Desk>(desk).and_then(|d| d.save_as(&name.to_string())))
    }

    pub fn rename(&self, desk: &QString, number: i32, name: &QString) -> QString {
        told(of::<Desk>(desk).zip(index(number)).and_then(|(d, n)| d.rename(n, &name.to_string())))
    }

    pub fn remove(&self, desk: &QString, number: i32) -> QString {
        told(of::<Desk>(desk).zip(index(number)).and_then(|(d, n)| d.remove(n)))
    }

    pub fn reset(&self, desk: &QString) -> QString {
        told(of::<Desk>(desk).map(|d| d.reset_in(self.trees())))
    }

    pub fn opening(&self, desk: &QString, kind: &QString, fresh: bool) -> QString {
        told(of::<Desk>(desk).map(|d| d.opening(&kind.to_string(), fresh)))
    }
}
