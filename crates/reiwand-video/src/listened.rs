use crate::Error;
use crate::feed::decoder;
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app::AppSink;
use reiwand_mix::{Clip, RATE};
use std::path::Path;
use std::time::{Duration, Instant};

const WAIT: Duration = Duration::from_secs(120);

pub fn listen(path: &Path, (from, to): (f64, f64)) -> Result<(usize, Clip), Error> {
    let [first, last] = [from, to].map(|s| (s.max(0.0) * f64::from(RATE)).round() as usize);
    let (pipeline, sink) = decoder(path, Some(8))?;
    pipeline.set_state(gst::State::Playing).map_err(|e| Error::Decode(e.to_string()))?;
    let bus = pipeline.bus().expect("a pipeline has a bus");
    let pulled = pull(&sink, &bus, (first, last.max(first)));
    let _ = pipeline.set_state(gst::State::Null);
    pulled.map(|clip| (first, clip)).map_err(Error::Decode)
}

fn pull(sink: &AppSink, bus: &gst::Bus, (first, last): (usize, usize)) -> Result<Clip, String> {
    let deadline = Instant::now() + WAIT;
    let (mut clip, mut at, mut heard) = (Clip { channels: 2, samples: Vec::new() }, 0, false);
    while !sink.is_eos() && at < last {
        if let Some(sample) = sink.try_pull_sample(gst::ClockTime::from_mseconds(100)) {
            let channels = sample.caps().and_then(|c| c.structure(0)?.get::<i32>("channels").ok());
            clip.channels = channels.unwrap_or(1).clamp(1, 2) as usize;
            let map = sample.buffer().and_then(|b| b.map_readable().ok());
            let values = map.as_ref().map_or(&[][..], |m| m.as_chunks::<2>().0);
            let frames = values.len() / clip.channels;
            let (lo, hi) = (first.clamp(at, at + frames) - at, last.clamp(at, at + frames) - at);
            let kept = &values[lo * clip.channels..hi * clip.channels];
            clip.samples.extend(kept.iter().map(|b| i16::from_le_bytes(*b)));
            (at, heard) = (at + frames, true);
        }
        if let Some(gst::MessageView::Error(e)) =
            bus.pop_filtered(&[gst::MessageType::Error]).as_ref().map(|m| m.view())
        {
            return Err(e.error().to_string());
        }
        if Instant::now() > deadline {
            return Err(format!("the sound was not decoded within {WAIT:?}"));
        }
    }
    heard.then_some(clip).ok_or_else(|| "the file holds no sound".into())
}
