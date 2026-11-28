use serde_json::{Value, json};

pub const MARK: &str = "crowd:yes";
const SIZE: usize = 356;
const SOURCED: usize = 341;
const FAVORITES: [usize; 2] = [3, 350];

pub const UID: &str = "01J9Z0X9Z6Q7X4F3K5E1V";

fn item(at: usize) -> Value {
    let sourced = at < SOURCED;
    json!({
        "uid": format!("{UID}{at:05}"),
        "kind": if at % 9 == 4 { "scene" } else { "image" },
        "title": format!("Crowd {at}"),
        "width": 1920,
        "height": 1080,
        "duration_ms": null,
        "color": (at as u32).wrapping_mul(2_654_435_761) & 0xff_ffff,
        "tags": [],
        "rating": 0,
        "favorite": FAVORITES.contains(&at),
        "missing": false,
        "error": null,
        "revoked": null,
        "removable": false,
        "imported": false,
        "sound_muted": null,
        "sound_volume": null,
        "thumb": null,
        "package": null,
        "provider": if sourced { Some("example") } else { None },
    })
}

fn keeps(at: usize, word: &str) -> bool {
    let sourced = at < SOURCED;
    let scene = at % 9 == 4;
    match word {
        MARK => true,
        "fav:yes" => FAVORITES.contains(&at),
        "provider:yes" => sourced,
        "provider:no" | "local:yes" => !sourced,
        "kind:scene" => scene,
        "kind:image" => !scene,
        w if w.starts_with("kind:") => false,
        w if w.starts_with("uid:") => w[4..].split(',').any(|uid| uid == format!("{UID}{at:05}")),
        w => format!("crowd {at}").contains(&w.to_lowercase()),
    }
}

pub fn search(args: &[Value]) -> Option<String> {
    let query = args.first()?.as_str()?;
    if !query.split_whitespace().any(|w| w == MARK) {
        return None;
    }
    let offset = args.get(1).and_then(Value::as_u64).unwrap_or(0) as usize;
    let limit = args.get(2).and_then(Value::as_u64).unwrap_or(0) as usize;
    let found: Vec<usize> =
        (0..SIZE).filter(|&at| query.split_whitespace().all(|word| keeps(at, word))).collect();
    let items: Vec<Value> = found.iter().skip(offset).take(limit).map(|&at| item(at)).collect();
    let page = json!({"model": 1, "serial": 1, "query": query, "total": found.len(), "offset": offset, "items": items});
    Some(page.to_string())
}
