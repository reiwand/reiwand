use gstreamer as gst;
use gstreamer::prelude::*;

#[test]
fn a_huge_video_gives_a_capped_still() {
    if reiwand_video::init().is_err() {
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("huge.mkv");
    let made = gst::parse::launch(&format!(
        "videotestsrc num-buffers=2 pattern=black ! video/x-raw,width=16384,height=8192,framerate=1/1 ! \
         jpegenc ! matroskamux ! filesink location={}",
        file.display()
    ));
    let Ok(pipeline) = made else {
        return assert!(std::env::var_os("REIWAND_REQUIRE_VIDEO").is_none(), "no encoder to make the file");
    };
    pipeline.set_state(gst::State::Playing).unwrap();
    let bus = pipeline.bus().unwrap();
    let kinds = [gst::MessageType::Eos, gst::MessageType::Error];
    let ended = bus.timed_pop_filtered(gst::ClockTime::from_seconds(60), &kinds).unwrap();
    assert!(matches!(ended.view(), gst::MessageView::Eos(_)), "{ended:?}");
    pipeline.set_state(gst::State::Null).unwrap();
    let still = reiwand_video::still(&file).unwrap();
    assert_eq!((still.width, still.height), (8192, 4096));
    assert_eq!(still.rgba.len(), 8192 * 4096 * 4);
}
