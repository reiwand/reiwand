use reiwand_scene::{Clip, Lut, Model, ModelFile, picture_size};
use serde_json::Value;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

const NUMBERS: [i64; 10] = [0, 1, -1, 3, 255, 65_535, 1 << 31, (1 << 32) - 1, 1 << 40, i64::MAX];
const WORDS: [&str; 7] = ["SCALAR", "VEC2", "VEC4", "MAT4", "", "POSITION", "image/png"];
const MODELS: [&str; 4] = [
    "tests/fixtures/rig.glb",
    "tests/fixtures/toy.glb",
    "tests/fixtures/parts.glb",
    "scenes/solids/assets/crate.glb",
];

struct Rng(u64);

impl Rng {
    fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n.max(1) as u64) as usize
    }
}

fn root(path: &str) -> Vec<u8> {
    std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(path)).unwrap()
}

fn capped<T: Send + 'static>(what: String, work: impl FnOnce() -> T + Send + 'static) -> T {
    let (send, receive) = mpsc::channel();
    std::thread::spawn(move || send.send(work()));
    match receive.recv_timeout(Duration::from_secs(20)) {
        Ok(value) => value,
        Err(mpsc::RecvTimeoutError::Timeout) => panic!("hung on {what}"),
        Err(mpsc::RecvTimeoutError::Disconnected) => panic!("panicked on {what}"),
    }
}

fn damaged(rng: &mut Rng, data: &[u8]) -> Vec<u8> {
    let mut out = data.to_vec();
    let at = rng.below(out.len());
    match rng.below(4) {
        0 => out[at] ^= 1 << rng.below(8),
        1 => out[at] = [0, 0xff, 0x7f, 0x80][rng.below(4)],
        2 => out.truncate(at),
        _ => out[at..].iter_mut().take(4).for_each(|b| *b = 0xff),
    }
    out
}

fn numbers(value: &Value, at: String, out: &mut Vec<String>) {
    match value {
        Value::Number(_) | Value::String(_) => out.push(at),
        Value::Object(map) => map.iter().for_each(|(k, v)| {
            numbers(v, format!("{at}/{k}"), out);
            out.push(format!("{at}/{k}"));
        }),
        Value::Array(items) => {
            items.iter().enumerate().for_each(|(i, v)| numbers(v, format!("{at}/{i}"), out))
        }
        _ => {}
    }
}

fn reshaped(rng: &mut Rng, glb: &[u8]) -> Vec<u8> {
    let word = |at: usize| u32::from_le_bytes(glb[at..at + 4].try_into().unwrap()) as usize;
    let mut json: Value = serde_json::from_slice(&glb[20..20 + word(12)]).unwrap();
    let mut slots = Vec::new();
    numbers(&json, String::new(), &mut slots);
    for _ in 0..1 + rng.below(3) {
        let slot = &slots[rng.below(slots.len())];
        let Some(value) = json.pointer_mut(slot) else { continue };
        *value = match value {
            Value::String(_) => WORDS[rng.below(WORDS.len())].into(),
            Value::Number(_) => NUMBERS[rng.below(NUMBERS.len())].into(),
            _ if rng.below(2) == 0 => Value::Null,
            _ => serde_json::json!([]),
        };
    }
    rebuilt(glb, &json)
}

fn rebuilt(glb: &[u8], json: &Value) -> Vec<u8> {
    let word = |at: usize| u32::from_le_bytes(glb[at..at + 4].try_into().unwrap()) as usize;
    let mut text = json.to_string().into_bytes();
    text.resize(text.len().next_multiple_of(4), b' ');
    let rest = &glb[20 + word(12)..];
    let mut out = b"glTF".to_vec();
    out.extend(2u32.to_le_bytes());
    out.extend(((20 + text.len() + rest.len()) as u32).to_le_bytes());
    out.extend((text.len() as u32).to_le_bytes());
    out.extend(b"JSON");
    out.extend(text);
    out.extend(rest);
    out
}

fn opened(bytes: Vec<u8>) -> bool {
    let header = ModelFile::parse(&bytes).is_ok();
    let model = Model::decode(&bytes).map(|m| (m.vertex_count(), m.radius(), m.middle())).is_ok();
    let posed = Model::decode_posed(&bytes).is_ok();
    let rig = Model::skeleton(&bytes).is_ok();
    header && model && posed && rig
}

#[test]
fn a_damaged_model_is_refused_or_read() {
    let mut rng = Rng(0x6c62_272e_07bb_0142);
    for path in MODELS {
        let glb = root(path);
        let whole = capped(format!("{path} whole"), {
            let glb = glb.clone();
            move || ModelFile::parse(&glb).is_ok() && Model::decode(&glb).is_ok()
        });
        assert!(whole, "{path} does not read before it is damaged");
        for round in 0..300 {
            let broken = if round % 2 == 0 { reshaped(&mut rng, &glb) } else { damaged(&mut rng, &glb) };
            capped(format!("{path} round {round}"), move || opened(broken));
        }
    }
}

#[test]
fn a_model_with_fewer_texture_places_than_corners_is_refused() {
    let glb = root("scenes/solids/assets/crate.glb");
    let size = u32::from_le_bytes(glb[12..16].try_into().unwrap()) as usize;
    let mut json: Value = serde_json::from_slice(&glb[20..20 + size]).unwrap();
    let place = json["meshes"][0]["primitives"][0]["attributes"]["TEXCOORD_0"].as_u64().unwrap() as usize;
    json["accessors"][place]["count"] = 1.into();
    let refused = capped("few texture places".into(), move || Model::decode(&rebuilt(&glb, &json)).err());
    assert!(refused.is_some_and(|e| e.contains("count different vertices")));
}

fn wav() -> Vec<u8> {
    let mut out = b"RIFF\x24\x00\x01\x00WAVEfmt \x10\x00\x00\x00\x01\x00\x02\x00".to_vec();
    out.extend(44_100u32.to_le_bytes());
    out.extend((44_100u32 * 4).to_le_bytes());
    out.extend(b"\x04\x00\x10\x00data\x00\x00\x01\x00");
    out.extend(vec![0; 256]);
    out
}

fn mp3() -> Vec<u8> {
    let frame = [&[0xffu8, 0xfb, 0x90, 0x64][..], &[0; 413]].concat();
    [b"ID3\x04\x00\x00\x00\x00\x00\x00".to_vec(), frame.repeat(4)].concat()
}

#[test]
fn a_damaged_sound_is_refused_or_read() {
    let mut rng = Rng(0x3c6e_f372_fe94_f82b);
    let flac = [
        b"fLaC\x80\x00\x00\x22".to_vec(),
        vec![0x10; 10],
        vec![0x0a, 0xc4, 0x42, 0xf0, 0, 0, 0xac, 0x44],
        vec![0; 16],
    ]
    .concat();
    for (name, sound) in [
        ("water.ogg", root("scenes/pond/assets/water.ogg")),
        ("a.wav", wav()),
        ("a.flac", flac),
        ("a.mp3", mp3()),
    ] {
        for round in 0..400 {
            let broken = damaged(&mut rng, &sound);
            capped(format!("{name} round {round}"), move || drop(Clip::read(name, &broken)));
        }
    }
}

#[test]
fn a_damaged_colour_table_or_picture_header_is_refused_or_read() {
    let mut rng = Rng(0x510e_527f_ade6_82d1);
    let mut cube = "TITLE \"t\"\nLUT_3D_SIZE 2\nDOMAIN_MIN 0 0 0\nDOMAIN_MAX 1 1 1\n".to_string();
    (0..8).for_each(|i| cube += &format!("{} {} {}\n", i & 1, i >> 1 & 1, i >> 2));
    assert!(Lut::parse(cube.as_bytes()).is_ok());
    let pictures =
        ["scenes/pulse/assets/cover.png", "scenes/pond/preview.jpg", "tests/fixtures/blink-16x8.gif"];
    for round in 0..600 {
        let broken = damaged(&mut rng, cube.as_bytes());
        capped(format!("cube round {round}"), move || drop(Lut::parse(&broken)));
        let broken = damaged(&mut rng, &root(pictures[round % 3]));
        capped(format!("picture round {round}"), move || _ = picture_size(&broken));
    }
}
