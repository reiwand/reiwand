use crate::bus::{self, Event};
use crate::crowd::UID;
use serde_json::{Map, Value, json};
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};
use std::time::Duration;

#[derive(Default)]
struct Maker {
    made: HashSet<String>,
    queue: Vec<(String, u32)>,
    running: bool,
}

static MAKER: LazyLock<Mutex<Maker>> = LazyLock::new(Mutex::default);

fn ours(uid: &str) -> bool {
    uid.strip_prefix(UID).is_some_and(|n| n.len() == 5 && n.bytes().all(|b| b.is_ascii_digit()))
}

fn picture() -> Option<String> {
    static PATH: LazyLock<Option<String>> = LazyLock::new(|| {
        let path = std::path::Path::new(&std::env::var_os("XDG_CACHE_HOME")?).join("crowd-cover.png");
        image::RgbImage::from_pixel(4, 4, image::Rgb([40, 80, 120])).save(&path).ok()?;
        Some(path.to_string_lossy().into_owned())
    });
    PATH.clone()
}

pub fn thumbs(args: &[Value]) -> Option<String> {
    let size = u32::try_from(args.get(1)?.as_u64()?).ok()?;
    let uids = args.first()?.as_array()?.iter().filter_map(Value::as_str).filter(|uid| ours(uid));
    let uids: Vec<String> = uids.map(String::from).collect();
    if uids.is_empty() {
        return None;
    }
    let path = picture()?;
    let mut maker = MAKER.lock().ok()?;
    let (made, waiting): (Vec<String>, Vec<String>) =
        uids.into_iter().partition(|uid| maker.made.contains(uid));
    maker.queue = waiting.into_iter().map(|uid| (uid, size)).collect();
    if !maker.running && !maker.queue.is_empty() {
        maker.running = true;
        bus::spawn(make(path.clone()));
    }
    let found: Map<String, Value> = made.into_iter().map(|uid| (uid, json!(path))).collect();
    Some(json!([found]).to_string())
}

async fn make(path: String) {
    loop {
        tokio::time::sleep(Duration::from_millis(15)).await;
        let Some((uid, size)) = next() else { return };
        bus::publish(Event::Changed("library".into(), vec![uid.clone()], 0));
        bus::publish(Event::Thumb(uid, size, path.clone()));
    }
}

fn next() -> Option<(String, u32)> {
    let mut maker = MAKER.lock().ok()?;
    if maker.queue.is_empty() {
        maker.running = false;
        return None;
    }
    let (uid, size) = maker.queue.remove(0);
    maker.made.insert(uid.clone());
    Some((uid, size))
}
