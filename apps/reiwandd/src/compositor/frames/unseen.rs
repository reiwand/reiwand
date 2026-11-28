//! A screen that is shown and not covered but whose stream does not play stays black on the desktop:
//! the consumer (the Plasma wallpaper) stopped reading it. After a while that is said once in the
//! log, with the state PipeWire gives the stream, so a black wallpaper leaves a trace.

use super::super::output::{Output, Sink};
use super::super::state::State;
use std::time::{Duration, Instant};

pub(super) const UNSEEN: Duration = Duration::from_secs(10);

/// Follows one output: `Some(since)` the first time it is due to be said, once.
pub(super) fn follow(output: &mut Output, streaming: bool, now: Instant) -> Option<Instant> {
    let shown = matches!(output.sink, Sink::Stream(_)) && output.active && !output.covered;
    if !shown || streaming {
        output.unseen = None;
        return None;
    }
    let (since, told) = output.unseen.get_or_insert((now, false));
    if *told || now < *since + UNSEEN {
        return None;
    }
    *told = true;
    Some(*since)
}

impl State {
    pub(super) fn check_unseen(&mut self, now: Instant) {
        let screen =
            |name: &str| !crate::drafts::virtual_output(name) && !name.starts_with(crate::previews::PREVIEW);
        for (name, output) in self.outputs.iter_mut().filter(|(name, _)| screen(name)) {
            if let Some(since) = follow(output, self.exporter.is_streaming(name), now) {
                let state = self.exporter.stream_state(name).unwrap_or_else(|| "none".into());
                let node = self.exporter.stream_info(name).map(|i| i.node_id);
                tracing::warn!(
                    output = %name,
                    stream = %state,
                    node = ?node,
                    "the screen is shown but its stream has not played for {} s: nothing reads it, so the wallpaper stays black",
                    now.duration_since(since).as_secs()
                );
            }
        }
    }

    pub(super) fn unseen_due(&self) -> Option<Instant> {
        let waiting = self.outputs.values().filter_map(|o| o.unseen.filter(|(_, told)| !told));
        waiting.map(|(since, _)| since + UNSEEN).min()
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Content;
    use super::super::super::output::Shown;
    use super::*;
    use reiwand_core::Fit;

    fn screen() -> Output {
        let (content, fit, background) = (Content::Color, Fit::Stretch, [0; 3]);
        let (image, transition, uv, relief, scene) = (None, None, None, None, None);
        let shown = Shown {
            content,
            fit,
            background,
            image,
            transition,
            uv,
            relief,
            scene,
            tone: Default::default(),
        };
        Output::new((16, 9), shown, Sink::Stream(None), 60)
    }

    #[test]
    fn a_shown_screen_whose_stream_does_not_play_is_told_once_after_a_while() {
        let (mut output, start) = (screen(), Instant::now());
        assert_eq!(follow(&mut output, false, start), None, "not yet");
        assert_eq!(follow(&mut output, false, start + UNSEEN / 2), None, "not yet");
        assert_eq!(follow(&mut output, false, start + UNSEEN), Some(start), "told");
        assert_eq!(follow(&mut output, false, start + UNSEEN * 3), None, "told once");
        assert_eq!(follow(&mut output, true, start + UNSEEN * 4), None, "it plays");
        assert_eq!(output.unseen, None, "playing forgets it");
        output.covered = true;
        assert_eq!(follow(&mut output, false, start + UNSEEN * 5), None);
        assert_eq!(follow(&mut output, false, start + UNSEEN * 9), None, "a covered screen is not told");
        output.covered = false;
        assert_eq!(follow(&mut output, false, start + UNSEEN * 10), None, "counts again from when it shows");
        assert_eq!(follow(&mut output, false, start + UNSEEN * 11), Some(start + UNSEEN * 10));
    }
}
