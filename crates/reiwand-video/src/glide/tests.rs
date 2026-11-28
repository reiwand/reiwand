use super::*;
use gstreamer_app::AppSink;

const RATE: u64 = 48_000;
const FRAMES: u64 = 480;

/// The glide's filter fed by hand, as playbin feeds it: buffers of 10 ms at full scale.
struct Feed {
    pipeline: gst::Pipeline,
    input: gst::Pad,
    sink: AppSink,
    at: u64,
}

impl Feed {
    fn new(filter: &gst::Element) -> Feed {
        let sink = AppSink::builder().sync(false).build();
        let pipeline = gst::Pipeline::new();
        pipeline.add_many([filter, sink.upcast_ref()]).unwrap();
        filter.link(&sink).unwrap();
        pipeline.set_state(gst::State::Playing).unwrap();
        let input = filter.static_pad("sink").unwrap();
        let caps = gst::Caps::builder("audio/x-raw")
            .field("format", "F32LE")
            .field("layout", "interleaved")
            .field("rate", RATE as i32)
            .field("channels", 1)
            .build();
        input.send_event(gst::event::StreamStart::new("glide"));
        input.send_event(gst::event::Caps::new(&caps));
        let mut feed = Feed { pipeline, input, sink, at: 0 };
        feed.restart();
        feed
    }

    /// Starts again from 0, as a loop of the video does: a flush and a new segment.
    fn restart(&mut self) {
        self.input.send_event(gst::event::FlushStart::new());
        self.input.send_event(gst::event::FlushStop::new(true));
        self.input.send_event(gst::event::Segment::new(&gst::FormattedSegment::<gst::ClockTime>::new()));
        self.at = 0;
    }

    /// Plays `n` buffers and gives back the levels that came out.
    fn play(&mut self, n: u64) -> Vec<f32> {
        let mut heard = Vec::new();
        for _ in 0..n {
            let mut buffer = gst::Buffer::from_mut_slice(vec![0u8; FRAMES as usize * 4]);
            let data = buffer.get_mut().unwrap();
            data.set_pts(gst::ClockTime::from_nseconds(self.at * 1_000_000_000 / RATE));
            data.set_duration(gst::ClockTime::from_nseconds(FRAMES * 1_000_000_000 / RATE));
            data.map_writable().unwrap().copy_from_slice(&1f32.to_le_bytes().repeat(FRAMES as usize));
            self.at += FRAMES;
            self.input.chain(buffer).unwrap();
            let sample = self.sink.try_pull_sample(gst::ClockTime::from_seconds(2)).unwrap();
            let map = sample.buffer().unwrap().map_readable().unwrap();
            heard.extend(map.as_chunks::<4>().0.iter().map(|b| f32::from_le_bytes(*b)));
        }
        heard
    }
}

#[test]
fn a_muted_video_stays_silent_when_it_loops_past_where_it_was_muted() {
    if crate::init().is_err() || gst::ElementFactory::find("volume").is_none() {
        return eprintln!("skipped: no GStreamer volume element");
    }
    let (filter, glide) = Glide::new(1.0).unwrap();
    let mut feed = Feed::new(&filter);
    assert!(feed.play(20).iter().all(|&v| (v - 1.0).abs() < 1e-6), "full volume before the change");
    glide.set(0.0);
    let gliding = feed.play(10);
    assert!(gliding[0] > 0.9 && gliding.last().copied() == Some(0.0), "it glides down: {:?}", &gliding[..4]);
    let after = feed.play(10);
    assert!(after.iter().all(|&v| v == 0.0), "muted after the glide");
    for _ in 0..3 {
        feed.restart();
        let again = feed.play(40);
        let loud = again.iter().filter(|&&v| v != 0.0).count();
        assert_eq!(loud, 0, "a loop came back to the old volume where it was muted");
    }
    feed.pipeline.set_state(gst::State::Null).unwrap();
}
