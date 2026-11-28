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
        type Workbench = super::WorkbenchRust;

        #[qinvokable]
        fn show(self: &Workbench, root: &QString, panel: &QString) -> QString;
        #[qinvokable]
        fn stack(self: &Workbench, root: &QString, panel: &QString, beside: &QString) -> QString;
        #[qinvokable]
        fn step(self: &Workbench, root: &QString, panel: &QString, side: &QString) -> QString;
        #[qinvokable]
        fn beside(
            self: &Workbench,
            root: &QString,
            panel: &QString,
            target: &QString,
            side: &QString,
        ) -> QString;
        #[qinvokable]
        fn edge(self: &Workbench, root: &QString, panel: &QString, side: &QString) -> QString;
        #[qinvokable]
        fn close(self: &Workbench, root: &QString, panel: &QString) -> QString;
        #[qinvokable]
        fn without(self: &Workbench, root: &QString, panel: &QString) -> QString;
        #[qinvokable]
        fn reopen(self: &Workbench, root: &QString, usual: &QString, panel: &QString) -> QString;
        #[qinvokable]
        fn closed(self: &Workbench, root: &QString) -> QString;
        #[qinvokable]
        fn places(self: &Workbench, root: &QString, measure: &QString, width: f64, height: f64) -> QString;
        #[qinvokable]
        fn resize(
            self: &Workbench,
            root: &QString,
            measure: &QString,
            size: &QString,
            edge: &QString,
            by: f64,
        ) -> QString;
        #[qinvokable]
        fn even(self: &Workbench, root: &QString, usual: &QString, edge: &QString) -> QString;
        #[qinvokable]
        fn blueprint(self: &Workbench, root: &QString, width: i32, height: i32) -> QString;
    }
}

use crate::bench::{Node, Panel, Path};
use crate::benchops::{self as ops, Side};
use crate::benchplace as place;
use crate::benchplan::blueprint;
use crate::benchsize::{Measure, even};
use cxx_qt_lib::QString;
use qobject::Workbench;
use serde::de::DeserializeOwned;
use serde_json::json;

#[derive(Default)]
pub struct WorkbenchRust;

pub fn of<T: DeserializeOwned>(text: &QString) -> Option<T> {
    serde_json::from_str(&text.to_string()).ok()
}

pub fn word<T: DeserializeOwned>(text: &QString) -> Option<T> {
    serde_json::from_value(json!(text.to_string())).ok()
}

pub fn told<T: serde::Serialize>(value: Option<T>) -> QString {
    QString::from(&value.and_then(|v| serde_json::to_string(&v).ok()).unwrap_or_default())
}

fn line(text: &QString) -> Option<(Path, usize)> {
    let value: serde_json::Value = of(text)?;
    Some((serde_json::from_value(value["path"].clone()).ok()?, value["index"].as_u64()? as usize))
}

impl Workbench {
    pub fn show(&self, root: &QString, panel: &QString) -> QString {
        told(of(root).zip(word::<Panel>(panel)).and_then(|(r, p)| ops::show(&r, p)))
    }

    pub fn stack(&self, root: &QString, panel: &QString, beside: &QString) -> QString {
        let found = of(root).zip(word::<Panel>(panel)).zip(word::<Panel>(beside));
        told(found.and_then(|((r, p), b)| ops::stack(&r, p, b)))
    }

    pub fn step(&self, root: &QString, panel: &QString, side: &QString) -> QString {
        let found = of(root).zip(word::<Panel>(panel)).zip(word::<Side>(side));
        told(found.and_then(|((r, p), s)| ops::step(&r, p, s)))
    }

    pub fn beside(&self, root: &QString, panel: &QString, target: &QString, side: &QString) -> QString {
        let found = of(root).zip(word::<Panel>(panel)).zip(word::<Panel>(target)).zip(word::<Side>(side));
        told(found.and_then(|(((r, p), t), s)| place::beside(&r, p, t, s)))
    }

    pub fn edge(&self, root: &QString, panel: &QString, side: &QString) -> QString {
        let found = of(root).zip(word::<Panel>(panel)).zip(word::<Side>(side));
        told(found.and_then(|((r, p), s)| place::edge(&r, p, s)))
    }

    pub fn close(&self, root: &QString, panel: &QString) -> QString {
        told(of(root).zip(word::<Panel>(panel)).and_then(|(r, p)| place::close(&r, p)))
    }

    /// The tree drawn while a panel is in a window of its own, the canvas too; never saved.
    pub fn without(&self, root: &QString, panel: &QString) -> QString {
        told(of(root).zip(word::<Panel>(panel)).and_then(|(r, p)| crate::benchops::emptied(&r, p)))
    }

    pub fn reopen(&self, root: &QString, usual: &QString, panel: &QString) -> QString {
        let found = of::<Node>(root).zip(of::<Node>(usual)).zip(word::<Panel>(panel));
        told(found.and_then(|((r, u), p)| place::reopen(&r, &u, p)))
    }

    pub fn closed(&self, root: &QString) -> QString {
        let closed = of::<Node>(root).map(|r| place::closed(&r)).unwrap_or_default();
        QString::from(&serde_json::to_string(&closed).unwrap_or_default())
    }

    pub fn places(&self, root: &QString, measure: &QString, width: f64, height: f64) -> QString {
        let Some((root, measure)) = of::<Node>(root).zip(of::<Measure>(measure)) else {
            return QString::default();
        };
        let (boxes, edges) = measure.boxes(&root, width, height);
        let groups: Vec<_> = (boxes.iter())
            .filter_map(|(path, at)| match root.at(path) {
                Some(Node::Group(g)) => Some(json!({ "path": path, "panels": g.panels, "shown": g.shown,
                    "x": at.x, "y": at.y, "width": at.width, "height": at.height })),
                _ => None,
            })
            .collect();
        QString::from(&json!({ "groups": groups, "edges": edges }).to_string())
    }

    pub fn resize(
        &self,
        root: &QString,
        measure: &QString,
        size: &QString,
        edge: &QString,
        by: f64,
    ) -> QString {
        let found = of::<Node>(root).zip(of::<Measure>(measure)).zip(of::<(f64, f64)>(size)).zip(line(edge));
        told(found.and_then(|(((r, m), size), (path, index))| m.resize(&r, size, &path, index, by)))
    }

    pub fn even(&self, root: &QString, usual: &QString, edge: &QString) -> QString {
        let found = of::<Node>(root).zip(of::<Node>(usual)).zip(line(edge));
        told(found.and_then(|((r, u), (path, index))| even(&r, &u, &path, index)))
    }

    pub fn blueprint(&self, root: &QString, width: i32, height: i32) -> QString {
        let plan = of::<Node>(root).map(|r| blueprint(&r, width, height)).unwrap_or_default();
        QString::from(&serde_json::to_string(&plan).unwrap_or_default())
    }
}
