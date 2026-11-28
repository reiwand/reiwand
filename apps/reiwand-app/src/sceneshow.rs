use crate::scene::qobject::SceneEditor;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use reiwand_editor::actions::LayerEdit;
use reiwand_editor::{Action, outline};
use serde_json::{Value, json};
use std::pin::Pin;
use std::time::Duration;

const QUIET: Duration = Duration::from_millis(120);
const FRAME: Duration = Duration::from_millis(16);

impl SceneEditor {
    pub fn pass_time(&self, ms: i32) {
        reiwand_editor::project::skip_clock(u64::try_from(ms).unwrap_or(0));
    }

    pub fn hold(mut self: Pin<&mut Self>, on: bool) {
        if let Some(project) = self.as_mut().rust_mut().project.as_mut() {
            project.hold(on);
        }
        if !on && self.stale > 0 {
            self.settled();
        }
    }

    pub(crate) fn scoped(mut self: Pin<&mut Self>) {
        let scope =
            reiwand_editor::scope::of(&serde_json::from_str(&self.look.to_string()).unwrap_or_default());
        if let Some(project) = self.as_mut().rust_mut().project.as_mut() {
            project.scope = scope;
        }
    }

    pub fn rig(&self, id: &QString, query: &QString) -> QString {
        let q = serde_json::from_str(&query.to_string()).unwrap_or_default();
        let found = self.project.as_ref().map(|p| p.ask(&id.to_string(), &q));
        QString::from(&found.unwrap_or_default().to_string())
    }

    pub fn select(mut self: Pin<&mut Self>, pointer: &QString) {
        let fields = self.inspect(pointer);
        let (moved, changed) = (self.selected != *pointer, self.fields != fields);
        let mut this = self.as_mut().rust_mut();
        (this.selected, this.fields) = (pointer.clone(), fields);
        if changed {
            self.as_mut().fields_changed();
        }
        if moved {
            self.as_mut().selected_changed();
        }
    }

    pub fn show_fields(mut self: Pin<&mut Self>) {
        let fields = self.inspect(&self.selected.clone());
        self.as_mut().set_fields(fields);
    }

    pub fn run(mut self: Pin<&mut Self>, action: &QString, args: &QString, gesture: bool) {
        let args = serde_json::from_str::<Value>(&args.to_string()).unwrap_or(json!({}));
        let action = match serde_json::from_value::<Action>(json!({ action.to_string(): args.clone() })) {
            Err(_) if action.to_string() == "steps" => return self.steps(args),
            Ok(action) if crate::scenework::queued(&action) => return self.import(action),
            Ok(action) => action,
            Err(e) => return self.fail(e),
        };
        self.as_mut().scoped();
        let before = matches!(action, Action::ImportScript(_)).then(|| self.scripts_now());
        let done = {
            let mut this = self.as_mut().rust_mut();
            let Some(project) = this.project.as_mut() else { return };
            if gesture { project.drag(&action) } else { project.act(&action) }
        };
        let Ok(_) = done.map_err(|e| self.as_mut().fail(e)) else { return };
        match gesture && matches!(action, Action::Layer(LayerEdit::Transform { .. } | LayerEdit::Grip { .. }))
        {
            true => self.as_mut().moved(),
            false => self.as_mut().changed(),
        }
        if let Some(before) = before {
            self.as_mut().said(&before);
        }
        if gesture && let Some(project) = self.rust_mut().project.as_mut() {
            project.held();
        }
    }

    fn moved(mut self: Pin<&mut Self>) {
        self.as_mut().set_error(QString::default());
        let Some(project) = self.project.as_ref() else { return };
        let (gizmos, dirty) = (outline::moving(project, self.time).to_string(), project.dirty());
        let (stale, qt) = (self.stale + 1, self.qt_thread());
        self.as_mut().rust_mut().stale = stale;
        crate::bus::spawn(async move {
            tokio::time::sleep(QUIET).await;
            let _ = qt.queue(move |editor| {
                if editor.stale == stale {
                    editor.settled();
                }
            });
        });
        self.as_mut().set_gizmos(QString::from(&gizmos));
        self.as_mut().set_dirty(dirty);
        self.as_mut().attach(0);
        self.send();
    }

    pub fn changed(mut self: Pin<&mut Self>) {
        self.as_mut().set_error(QString::default());
        self.as_mut().refresh();
        self.as_mut().attach(0);
        self.send();
    }

    pub fn settled(mut self: Pin<&mut Self>) {
        self.as_mut().outlined();
        let qt = self.qt_thread();
        crate::bus::spawn(async move {
            tokio::time::sleep(FRAME).await;
            let _ = qt.queue(|editor| editor.show_fields());
        });
    }

    pub fn refresh(mut self: Pin<&mut Self>) {
        self.as_mut().outlined();
        self.show_fields();
    }

    fn outlined(mut self: Pin<&mut Self>) {
        self.as_mut().scoped();
        let Some(project) = self.project.as_ref() else { return };
        let mut view = outline::outline(project, self.time, &self.files);
        let gizmos = view["gizmos"].take().to_string();
        let dirty = project.dirty();
        self.as_mut().rust_mut().stale = 0;
        self.as_mut().set_gizmos(QString::from(&gizmos));
        self.as_mut().set_view(QString::from(&view.to_string()));
        self.as_mut().set_dirty(dirty);
    }
}
