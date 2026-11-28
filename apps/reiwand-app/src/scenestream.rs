//! The stream of the canvas: the editor asks the service for a picture of the shown size, and asks again
//! when the service did not answer.
use crate::bus;
use crate::scene::qobject::SceneEditor;
use crate::scenelive::previewable;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use serde_json::{Value, json};
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

const EDGE: u32 = 1280;
const STEP: u32 = 640;
const RETRY: Duration = Duration::from_secs(2);
static STREAMS: AtomicU32 = AtomicU32::new(0);

pub(crate) fn steps(edge: u32) -> u32 {
    match edge {
        0 => EDGE,
        e => e.div_ceil(STEP) * STEP,
    }
}

impl SceneEditor {
    pub fn attach(mut self: Pin<&mut Self>, edge: u32) {
        if edge > 0 {
            self.as_mut().rust_mut().edge = edge;
        }
        let Some(project) = self.project.as_ref().filter(|p| previewable(&p.doc)) else {
            return;
        };
        let paged = reiwand_editor::pages::paged(&project.doc) || reiwand_editor::reels::media(&project.doc);
        let shown = serde_json::from_str::<Value>(&self.look.to_string()).is_ok_and(|l| l["show"] == true);
        if paged && !shown {
            return;
        }
        let size = &project.doc["scene"]["canvas"]["size"];
        let fallback = if paged { (1280.0, 720.0) } else { (1920.0, 1080.0) };
        let (w, h) = (
            size[0].as_f64().unwrap_or(fallback.0).max(1.0),
            size[1].as_f64().unwrap_or(fallback.1).max(1.0),
        );
        let scale = (f64::from(steps(self.edge)) / w.max(h)).min(1.0);
        let pixels = (((w * scale).round() as u32).max(16), ((h * scale).round() as u32).max(16));
        if pixels == self.attached {
            return;
        }
        let output = format!("preview:{}-{}", std::process::id(), STREAMS.fetch_add(1, Ordering::Relaxed));
        let (old, qt) = (self.output.to_string(), self.qt_thread());
        self.as_mut().rust_mut().attached = pixels;
        self.as_mut().set_output(QString::from(&output));
        self.as_mut().pair();
        bus::spawn(async move {
            if !old.is_empty() {
                bus::call("Daemon1", "ReleaseStream", vec![json!(old)]).await.ok();
            }
            let reply =
                bus::call("Daemon1", "RequestStream", vec![json!(output), json!(pixels.0), json!(pixels.1)])
                    .await;
            let reply = reply.map(|r| serde_json::from_str::<Value>(&r).unwrap_or_default());
            let asked = output.clone();
            qt.queue(move |mut editor| match reply {
                // A newer stream was asked for meanwhile (the canvas changed size twice in a row):
                // this one no longer gets the scene, so the daemon can let it go.
                Ok(_) if editor.output.to_string() != asked => {
                    bus::spawn(async move {
                        bus::call("Daemon1", "ReleaseStream", vec![json!(asked)]).await.ok();
                    });
                }
                Ok(r) => {
                    editor.as_mut().clear_stream_error();
                    editor.as_mut().set_node(r[0].as_u64().unwrap_or(0) as u32);
                    editor.as_mut().set_serial(r[1].as_f64().unwrap_or(0.0));
                    editor.send();
                }
                // An older stream that a newer one replaced is refused as removed: not an error of this canvas.
                Err(_) if editor.output.to_string() != asked => {}
                // The service may not answer yet (it starts after the app, or restarts): ask again shortly.
                Err(e) => {
                    editor.as_mut().rust_mut().attached = (0, 0);
                    editor.as_mut().rust_mut().stream_failed = true;
                    editor.as_mut().fail(e.message);
                    let qt = editor.qt_thread();
                    bus::spawn(async move {
                        tokio::time::sleep(RETRY).await;
                        qt.queue(|editor| editor.attach(0)).ok();
                    });
                }
            })
            .ok();
        });
    }

    /// The failure of an earlier stream is gone once a stream comes: the canvas shows it again.
    fn clear_stream_error(mut self: Pin<&mut Self>) {
        if std::mem::take(&mut self.as_mut().rust_mut().stream_failed) {
            self.as_mut().set_error(QString::default());
        }
    }
}

#[test]
fn the_preview_is_drawn_at_the_shown_size_in_steps() {
    assert_eq!([steps(0), steps(1), steps(1300), steps(2560)], [1280, 640, 1920, 2560]);
}
