use super::*;
use reiwand_video::Cut;
use serde_json::json;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/magenta-64x32.webm");

fn layers(count: usize) -> Vec<Reel> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/authoring/examples/video");
    let text = std::fs::read_to_string(format!("{dir}/scene.json")).unwrap();
    let mut doc: serde_json::Value = serde_json::from_str(&text).unwrap();
    let film = doc["layers"].as_array().unwrap().iter().find(|l| l["type"] == "video").unwrap().clone();
    let own = |n: usize| {
        let mut layer = film.clone();
        (layer["id"], layer["playback"]["own_clock"]) = (json!(format!("film{n}")), json!(true));
        layer
    };
    doc["layers"] = (0..count).map(own).collect();
    let scene = reiwand_scene::Scene::from_json(&doc.to_string()).unwrap();
    let mut videos = vec![("frames".to_owned(), Reel::plain(FIXTURE.into()))];
    crate::scenes::lanes(&scene, &Default::default(), &mut videos);
    videos.into_iter().map(|(_, reel)| reel).collect()
}

fn steer(at: f64, jumps: u32) -> Option<Steer> {
    Some(Steer { cut: Cut::default(), jumps, at, wants: true, volume: 1.0 })
}

fn wish(reel: &Reel, steer: Option<Steer>, volume: u32, decoding: bool) -> Wish<'_> {
    Wish { reel, steer, volume, decoding }
}

fn sources(planned: &HashMap<Reel, Source>) -> (usize, usize, usize) {
    let count = |f: fn(&Source) -> bool| planned.values().filter(|s| f(s)).count();
    (count(|s| *s == Source::Own), count(|s| matches!(s, Source::Follows(_))), count(|s| *s == Source::Still))
}

#[test]
fn a_scene_with_many_video_layers_runs_at_most_the_decoders_allowed() {
    let reels = layers(256);
    assert_eq!(reels.len(), 257, "one clock for the asset and one per layer");
    let wishes: Vec<Wish> =
        reels.iter().enumerate().map(|(n, reel)| wish(reel, steer(n as f64, 1), 0, false)).collect();
    assert_eq!(sources(&plan(&wishes)), (MAX_DECODERS, 0, 257 - MAX_DECODERS));
    let wishes: Vec<Wish> = reels.iter().map(|reel| wish(reel, steer(0.02, 0), 0, false)).collect();
    assert_eq!(sources(&plan(&wishes)), (1, 256, 0), "videos in step share one decoder");
}

#[test]
fn videos_that_decode_keep_their_decoder_and_sound_is_never_shared() {
    let reels = layers(MAX_DECODERS + 2);
    let decoding = |n: usize| (2..MAX_DECODERS + 2).contains(&n);
    let wishes: Vec<Wish> =
        reels.iter().enumerate().map(|(n, reel)| wish(reel, steer(n as f64, 0), 0, decoding(n))).collect();
    let planned = plan(&wishes);
    assert!(reels[2..MAX_DECODERS + 2].iter().all(|r| planned[r] == Source::Own));
    assert!(planned[&reels[0]] == Source::Still && planned[&reels[1]] == Source::Still);
    let loud = [wish(&reels[0], steer(0.0, 0), 0, false), wish(&reels[1], steer(0.0, 0), 80, false)];
    assert_eq!(sources(&plan(&loud)), (2, 0, 0));
}

#[test]
fn a_video_past_the_limit_shows_a_still_frame() {
    if let Err(e) = reiwand_video::init() {
        return assert!(std::env::var_os("REIWAND_REQUIRE_VIDEO").is_none(), "video required: {e}");
    }
    let (reel, (tx, rx)) = (Reel::plain(FIXTURE.into()), mpsc::channel());
    let mut slot = PlayerSlot::new(((64, 32), true, 60, 0), std::time::Instant::now());
    slot.take_source(Source::Still, &reel, &tx);
    assert!(slot.player.is_none() && slot.due.is_none(), "a still opens a decoder");
    rx.recv_timeout(std::time::Duration::from_secs(30)).unwrap();
    let still = lock(&slot.picture).take().expect("no still frame");
    assert_eq!((still.width, still.height), (64, 32));
}
