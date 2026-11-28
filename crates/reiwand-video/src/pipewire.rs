use gstreamer as gst;
use gstreamer::prelude::*;
use std::path::Path;

pub(crate) const SINK: &str = "reiwand-audio";
const FIXED_POOL: (u32, u32) = (1, 2);

pub(crate) const SINKS: [&str; 2] = ["pulsesink", "pipewiresink"];

fn pulse_reachable() -> bool {
    let set = std::env::var_os("PULSE_SERVER").is_some_and(|v| !v.is_empty());
    set || std::env::var_os("XDG_RUNTIME_DIR")
        .is_some_and(|dir| Path::new(&dir).join("pulse/native").exists())
}

fn element_for(pulse: bool, pulsesink: bool) -> &'static str {
    if pulse && pulsesink { SINKS[0] } else { SINKS[1] }
}

fn period(latency: &str) -> Option<i64> {
    let (frames, rate) = latency.split_once('/')?;
    let (frames, rate): (i64, i64) = (frames.parse().ok()?, rate.parse().ok()?);
    (rate > 0).then(|| frames * 1_000_000 / rate)
}

fn pulse(factory: &gst::ElementFactory, latency: Option<&str>) -> Option<gst::Element> {
    let props = gst::Structure::builder("props").field("media.role", "Movie").build();
    let sink = factory.create().name(SINK).build().ok()?;
    sink.set_property("client-name", "reiwand");
    sink.set_property("stream-properties", props);
    if let Some(period) = latency.and_then(period) {
        sink.set_property("latency-time", period);
        sink.set_property("buffer-time", period * 2);
    }
    Some(sink)
}

pub(crate) fn sink(latency: Option<&str>) -> Option<gst::Element> {
    let pulsesink = gst::ElementFactory::find(SINKS[0]);
    if element_for(pulse_reachable(), pulsesink.is_some()) == SINKS[0] {
        return pulse(&pulsesink?, latency);
    }
    let factory = gst::ElementFactory::find("pipewiresink")?;
    let version = factory.plugin().map(|p| p.version()).unwrap_or_default();
    if !fixed_pool(&version) {
        tracing::info!(%version, "the PipeWire GStreamer plugin is older than 1.2, using the default audio output");
        return None;
    }
    let props = gst::Structure::builder("props").field("media.role", "Movie");
    let props = match latency {
        Some(latency) => props.field("node.latency", latency),
        None => props,
    }
    .build();
    let sink = factory.create().name(SINK).build().ok()?;
    sink.set_property("client-name", "reiwand");
    sink.set_property("stream-properties", props);
    Some(sink)
}

fn fixed_pool(version: &str) -> bool {
    let mut parts = version.split('.').map(|part| part.parse().unwrap_or(0));
    (parts.next().unwrap_or(0), parts.next().unwrap_or(0)) >= FIXED_POOL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pulse_interface_is_preferred_when_reachable() {
        assert_eq!(element_for(true, true), "pulsesink");
        assert_eq!(element_for(false, true), "pipewiresink");
        assert_eq!(element_for(true, false), "pipewiresink");
        assert_eq!(period("960/48000"), Some(20_000));
        assert_eq!(period("x"), None);
    }

    #[test]
    fn only_plugins_from_1_2_send_the_sound_to_pipewire() {
        let versions = ["0.3.80", "1.0.5", "1.1.82", "", "1.2.0", "1.4.7", "2.0"];
        let fixed: Vec<_> = versions.iter().map(|v| fixed_pool(v)).collect();
        assert_eq!(fixed, [false, false, false, false, true, true, true]);
    }
}
