//! The volume of a video glides: a change takes 20 ms, sample by sample, instead of one step, which
//! clicks. It starts where the last buffer the volume element let through ends, not at what is heard,
//! so the buffers already queued after it do not jump. Once the sound leaves the stretch of the glide
//! (past its end, or back before its start after a loop or a seek) the points go and the volume stays
//! where it went: points left in stream time would glide again from the old volume on every loop.

use crate::{Error, make};
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_controller as controller;
use gstreamer_controller::prelude::*;
use std::sync::{Arc, Mutex};

const GLIDE: gst::ClockTime = gst::ClockTime::from_mseconds(20);

/// Where the sound is (the stream time the next buffer starts at) and the glide under way: its
/// stretch of stream time and the volume it goes to.
#[derive(Default)]
struct Course {
    seen: Option<gst::ClockTime>,
    glide: Option<(gst::ClockTime, gst::ClockTime, f64)>,
}

pub(crate) struct Glide {
    volume: gst::Element,
    source: controller::InterpolationControlSource,
    course: Arc<Mutex<Course>>,
}

impl Glide {
    /// The audio filter of the player (scaletempo when there is one, then the gliding volume) and the
    /// handle that moves its volume, which starts at `volume`.
    pub(crate) fn new(volume: f64) -> Result<(gst::Element, Glide), Error> {
        let bin = gst::Bin::new();
        let level = make("volume")?;
        level.set_property("volume", volume.clamp(0.0, 1.0));
        let tempo = make("scaletempo").ok();
        let chain: Vec<&gst::Element> = tempo.iter().chain([&level]).collect();
        bin.add_many(chain.iter().copied()).map_err(|e| Error::State(e.to_string()))?;
        gst::Element::link_many(chain.iter().copied()).map_err(|e| Error::State(e.to_string()))?;
        let (first, last) = (chain[0], &level);
        for (element, pad) in [(first, "sink"), (last, "src")] {
            let inner = element.static_pad(pad).ok_or_else(|| Error::State(format!("no {pad} pad")))?;
            let ghost = gst::GhostPad::with_target(&inner).map_err(|e| Error::State(e.to_string()))?;
            bin.add_pad(&ghost).map_err(|e| Error::State(e.to_string()))?;
        }
        let source = controller::InterpolationControlSource::new();
        source.set_mode(controller::InterpolationMode::Linear);
        let binding = controller::DirectControlBinding::new_absolute(&level, "volume", &source);
        level.add_control_binding(&binding).map_err(|e| Error::State(e.to_string()))?;
        let glide = Glide { volume: level, source, course: Arc::default() };
        glide.watch();
        Ok((bin.upcast(), glide))
    }

    /// Glides to `volume`, 0 to 1, from where the sound is now.
    pub(crate) fn set(&self, volume: f64) {
        let aim = volume.clamp(0.0, 1.0);
        let mut course = crate::lock(&self.course);
        let now = self.volume.property::<f64>("volume");
        self.source.unset_all();
        course.glide = None;
        match course.seen {
            None => self.volume.set_property("volume", aim),
            Some(at) => {
                self.source.set(at, now);
                self.source.set(at + GLIDE, aim);
                course.glide = Some((at, at + GLIDE, aim));
            }
        }
    }

    /// Follows the stream time of the buffers that reach the volume element: the controller places
    /// its points there. A buffer outside the stretch of the glide ends it; a flush or a new segment
    /// forgets where the sound was.
    fn watch(&self) {
        let Some(pad) = self.volume.static_pad("sink") else { return };
        let (course, source, volume) = (self.course.clone(), self.source.clone(), self.volume.clone());
        pad.add_probe(gst::PadProbeType::BUFFER | gst::PadProbeType::EVENT_DOWNSTREAM, move |pad, info| {
            let mut course = crate::lock(&course);
            if let Some(event) = info.event() {
                if matches!(event.type_(), gst::EventType::FlushStop | gst::EventType::Segment) {
                    course.seen = None;
                }
                return gst::PadProbeReturn::Ok;
            }
            let Some(buffer) = info.buffer() else { return gst::PadProbeReturn::Ok };
            let segment = pad.sticky_event::<gst::event::Segment>(0);
            let time =
                segment.as_ref().and_then(|s| s.segment().downcast_ref::<gst::format::Time>().cloned());
            let stream =
                |at: Option<gst::ClockTime>| time.as_ref().zip(at).and_then(|(s, at)| s.to_stream_time(at));
            let start = stream(buffer.pts());
            let end = stream(buffer.pts().map(|pts| pts + buffer.duration().unwrap_or_default()));
            if let (Some(at), Some((from, to, aim))) = (start, course.glide)
                && !(from..to).contains(&at)
            {
                source.unset_all();
                volume.set_property("volume", aim);
                course.glide = None;
            }
            course.seen = end.or(course.seen);
            gst::PadProbeReturn::Ok
        });
    }
}

#[cfg(test)]
mod tests;
