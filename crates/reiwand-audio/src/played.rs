use crate::{Analyzer, AudioFrame};
use std::sync::Arc;

pub const WARM_UP: f64 = 60.0;
const CHUNK: usize = 4096;
const MARGIN: f64 = 1.0;

#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub rate: u32,
    pub first: usize,
    pub stereo: Vec<i16>,
}

impl Track {
    pub fn new(rate: u32, first: usize, channels: usize, samples: &[i16]) -> Track {
        let stereo = match channels {
            1 => samples.iter().flat_map(|&s| [s, s]).collect(),
            _ => samples.as_chunks::<2>().0.iter().flatten().copied().collect(),
        };
        Track { rate: rate.max(1), first, stereo }
    }

    pub fn span(start: f64, end: f64) -> (f64, f64) {
        let start = start.max(0.0);
        ((start - WARM_UP - MARGIN).max(0.0), end.max(start))
    }

    fn sample(&self, frame: usize) -> [f32; 2] {
        let held = frame.checked_sub(self.first).and_then(|i| self.stereo.get(2 * i..2 * i + 2));
        held.map_or([0.0; 2], |p| [p[0], p[1]].map(|s| f32::from(s) / 32768.0))
    }
}

pub struct Played {
    track: Arc<Track>,
    analyzer: Analyzer,
    heard: usize,
    last: Option<f64>,
    scratch: Vec<f32>,
}

impl Played {
    pub fn new(track: Arc<Track>, start: f64, step: f64) -> Played {
        let (start, step) = (start.max(0.0), step.max(1e-3));
        let ticks = (start.min(WARM_UP) / step).floor() as usize;
        let first = start - ticks as f64 * step;
        let heard = ((first * f64::from(track.rate)).round() as usize).saturating_sub(crate::WINDOW);
        let analyzer = Analyzer::new(track.rate);
        let scratch = Vec::with_capacity(2 * CHUNK);
        let mut played = Played { track, analyzer, heard, last: None, scratch };
        for k in 0..ticks {
            played.at(first + k as f64 * step);
        }
        played
    }

    pub fn at(&mut self, t: f64) -> &AudioFrame {
        let t = t.max(0.0);
        let until = (t * f64::from(self.track.rate)).round() as usize;
        while self.heard < until {
            let end = until.min(self.heard + CHUNK);
            self.scratch.clear();
            self.scratch.extend((self.heard..end).flat_map(|f| self.track.sample(f)));
            self.analyzer.push(&self.scratch);
            self.heard = end;
        }
        let dt = self.last.replace(t).map_or(0.0, |last| (t - last).max(0.0) as f32);
        self.analyzer.update(dt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(rate: u32, seconds: f64, hz: f64) -> Track {
        let n = (seconds * f64::from(rate)) as usize;
        let wave =
            |i: usize| ((std::f64::consts::TAU * hz * i as f64 / f64::from(rate)).sin() * 16_000.0) as i16;
        Track::new(rate, 0, 1, &(0..n).map(wave).collect::<Vec<_>>())
    }

    #[test]
    fn a_tone_is_heard_and_silence_after_it_is_not() {
        let track = Arc::new(tone(48_000, 1.0, 100.0));
        let mut played = Played::new(track, 0.0, 1.0 / 30.0);
        let frames: Vec<AudioFrame> = (0..120).map(|i| played.at(f64::from(i) / 30.0).clone()).collect();
        assert!(frames[0].silent && frames[0].bass == 0.0);
        assert!(frames[20].bass > 0.3 && !frames[20].silent, "{}", frames[20].bass);
        assert!(frames[119].silent && frames[119].bass < 0.01);
    }

    #[test]
    fn the_same_track_gives_the_same_frames() {
        let track = Arc::new(tone(44_100, 2.0, 440.0));
        let run = || {
            let mut played = Played::new(track.clone(), 0.5, 1.0 / 24.0);
            (0..24).map(|i| played.at(0.5 + f64::from(i) / 24.0).clone()).collect::<Vec<_>>()
        };
        assert_eq!(run(), run());
    }

    #[test]
    fn a_late_start_hears_what_came_before() {
        let track = Arc::new(tone(48_000, 3.0, 100.0));
        let mut late = Played::new(track.clone(), 2.0, 1.0 / 30.0);
        let mut early = Played::new(track, 0.0, 1.0 / 30.0);
        let from_zero = (0..=60).map(|i| early.at(f64::from(i) / 30.0).clone()).last().expect("frames");
        let late = late.at(2.0);
        assert!((late.bass - from_zero.bass).abs() < 1e-3 && (late.level - from_zero.level).abs() < 1e-3);
        assert!(late.bass > 0.3);
    }

    #[test]
    fn mono_is_heard_on_both_sides_and_the_span_keeps_a_minute_before() {
        assert_eq!(Track::new(8, 0, 1, &[1, 2]).stereo, [1, 1, 2, 2]);
        assert_eq!(Track::span(100.0, 110.0), (39.0, 110.0));
        assert_eq!(Track::span(0.0, 4.0), (0.0, 4.0));
    }
}
