use crate::signal::Smooth;
use crate::validate::{Check, Issue};
use crate::value::{AudioKind, Band, Binding};

impl Check<'_> {
    pub(super) fn signal<T>(&mut self, path: &str, b: &Binding<T>) {
        let of = b.of.unwrap_or(64);
        if let Some(of) = b.of {
            self.uses(true, "audio_ranges");
            if ![16, 32, 64].contains(&of) {
                self.issues.push(Issue::Range {
                    path: format!("{path}/of"),
                    value: of as f32,
                    min: 16.0,
                    max: 64.0,
                });
            }
        }
        let top = of.saturating_sub(1) as f32;
        match b.audio {
            Some(Band::Index(i)) => self.range(&format!("{path}/audio"), i as f32, 0.0, top),
            Some(Band::Range([lo, hi])) => {
                self.uses(true, "audio_ranges");
                self.range(&format!("{path}/audio/0"), lo as f32, 0.0, top);
                self.range(&format!("{path}/audio/1"), hi as f32, lo as f32, top);
            }
            _ => {}
        }
        if b.audio.is_some() {
            for (name, given) in [("kind", b.kind.is_some()), ("unit", b.unit.is_some())] {
                if !given {
                    self.issues.push(Issue::Required(format!("{path}/{name}")));
                }
            }
        }
        let surging = b.kind == Some(AudioKind::Surge);
        let extras = [("window", b.window.is_some()), ("reference", b.reference.is_some())];
        let extras = extras.into_iter().chain([("hold_silence", b.hold_silence.is_some())]);
        let stray = b.audio.is_none() && (b.kind.is_some() || b.unit.is_some());
        let mut wrong = stray;
        for (name, given) in extras {
            if surging && !given {
                self.issues.push(Issue::Required(format!("{path}/{name}")));
            }
            wrong |= !surging && given;
        }
        if wrong {
            self.issues.push(Issue::Type(path.into()));
        }
        if let Some(window) = b.window {
            self.range(&format!("{path}/window"), window, 0.05, 60.0);
        }
        let ranged = matches!(b.audio, Some(Band::Range(_)));
        if (b.of.is_some() && b.audio.is_none()) || (b.reduce.is_some() && !ranged) {
            self.issues.push(Issue::Type(path.into()));
        }
        self.uses(b.reduce.is_some(), "audio_ranges");
        if let Some([lo, hi]) = b.bounds {
            self.uses(true, "shaping");
            self.range(&format!("{path}/bounds/0"), lo, -1e6, 1e6);
            self.range(&format!("{path}/bounds/1"), hi, lo.next_up(), 1e6);
        }
        if let Some(e) = b.exponent {
            self.uses(true, "shaping");
            self.range(&format!("{path}/exponent"), e, 0.1, 10.0);
        }
        if let Some(smooth) = b.smooth {
            self.uses(matches!(smooth, Smooth::Apart { .. }), "shaping");
            let [up, down] = smooth.times();
            self.range(&format!("{path}/smooth"), up, 0.0, 60.0);
            self.range(&format!("{path}/smooth"), down, 0.0, 60.0);
        }
    }
}
