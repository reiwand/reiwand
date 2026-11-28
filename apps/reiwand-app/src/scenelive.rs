use crate::bus;
use crate::scene::qobject::SceneEditor;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use serde_json::{Value, json};
use std::pin::Pin;
use std::time::Duration;

const THROTTLE: Duration = Duration::from_millis(50);
const REFRESH: Duration = Duration::from_secs(30);

pub(crate) fn previewable(doc: &Value) -> bool {
    reiwand_editor::pages::scened(doc)
        || reiwand_editor::pages::paged(doc)
        || reiwand_editor::reels::media(doc)
}

impl SceneEditor {
    pub fn cover(self: Pin<&mut Self>, covered: bool) {
        let outputs: Vec<String> = [self.output.to_string(), self.halved().unwrap_or_default()]
            .into_iter()
            .filter(|o| !o.is_empty())
            .collect();
        bus::spawn(async move {
            for output in outputs {
                bus::call("Daemon1", "SetOutputCovered", vec![json!(output), json!(covered)]).await.ok();
            }
        });
    }

    pub fn touch(self: Pin<&mut Self>, x: f64, y: f64, buttons: u32) {
        let outputs: Vec<String> = [self.output.to_string(), self.halved().unwrap_or_default()]
            .into_iter()
            .filter(|o| !o.is_empty())
            .collect();
        bus::spawn(async move {
            for output in outputs {
                let args = vec![json!(output), json!(x), json!(y), json!(buttons)];
                bus::call("Inputs1", "SimulateTouch", args).await.ok();
            }
        });
    }

    pub fn seek(mut self: Pin<&mut Self>, time: f64, playing: bool) {
        self.as_mut().set_time(time.max(0.0));
        self.as_mut().set_playing(playing);
        self.as_mut().refresh();
        self.send();
    }

    pub fn preview(mut self: Pin<&mut Self>, live: bool, on: &QString) {
        if live && self.desk != on.to_string() {
            let old = std::mem::replace(&mut self.as_mut().rust_mut().desk, on.to_string());
            if self.shown {
                crate::scenefile::unshow(old, self.project.as_ref().map(|p| p.dir.clone()));
            }
        }
        self.as_mut().set_live(live);
        self.send();
    }

    pub fn send(mut self: Pin<&mut Self>) {
        let Some(project) = self.project.as_ref().filter(|p| previewable(&p.doc)) else {
            return;
        };
        let streamed = !self.output.is_empty() && self.serial > 0.0;
        if !streamed && !self.live && !self.shown {
            return;
        }
        if self.sending {
            self.as_mut().rust_mut().queued = true;
            return;
        }
        let look: Value = serde_json::from_str(&self.look.to_string()).unwrap_or_default();
        let document = crate::scenedoc::document(&project.doc, &project.dir, &look);
        let plain =
            self.halved().map(|output| (output, crate::scenesplit::without_glow(&document).to_string()));
        let document = document.to_string();
        let dir = project.dir.to_string_lossy().into_owned();
        let time = if self.playing { -1.0 } else { self.time };
        let mut targets: Vec<(String, String)> = Vec::new();
        targets.extend(streamed.then(|| (self.output.to_string(), document.clone())));
        targets.extend(plain);
        match (self.live, self.shown) {
            (true, _) => targets.push((self.desk.clone(), document)),
            (false, true) => targets.push((self.desk.clone(), String::new())),
            _ => {}
        }
        let (qt, live) = (self.qt_thread(), self.live);
        let mut this = self.as_mut().rust_mut();
        (this.sending, this.shown, this.sent) = (true, live, this.sent + 1);
        let sent = this.sent;
        bus::spawn(async move {
            let mut failed = None;
            for (output, document) in targets {
                let args = vec![json!(output), json!(dir), json!(document), json!(time)];
                if let Err(e) = bus::call("Daemon1", "PreviewScene", args).await {
                    failed = failed.or((!e.name.ends_with("InvalidArgs")).then_some(e.message));
                }
            }
            tokio::time::sleep(THROTTLE).await;
            qt.queue(move |mut editor| {
                editor.as_mut().rust_mut().sending = false;
                if let Some(message) = failed {
                    editor.as_mut().set_error(QString::from(&message));
                }
                match std::mem::take(&mut editor.as_mut().rust_mut().queued) {
                    true => editor.send(),
                    false => editor.keep(sent),
                }
            })
            .ok();
        });
    }

    fn keep(self: Pin<&mut Self>, sent: u64) {
        let qt = self.qt_thread();
        bus::spawn(async move {
            tokio::time::sleep(REFRESH).await;
            qt.queue(move |editor| {
                if editor.sent == sent && !editor.sending && (editor.live || !editor.output.is_empty()) {
                    editor.send();
                }
            })
            .ok();
        });
    }
}
