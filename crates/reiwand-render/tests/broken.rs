use reiwand_render::{decode_image, decode_reduced, image_size};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

const PICTURES: [&str; 6] = [
    "scenes/pulse/assets/cover.png",
    "scenes/pond/preview.jpg",
    "scenes/dusk/depth.png",
    "tests/fixtures/blink-16x8.gif",
    "tests/fixtures/green-64x32.png",
    "scenes/lights/preview.png",
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

fn damaged(rng: &mut Rng, data: &[u8]) -> Vec<u8> {
    let mut out = data.to_vec();
    for _ in 0..1 + rng.below(4) {
        let head = if rng.below(2) == 0 { 64 } else { usize::MAX };
        let at = rng.below(out.len().min(head));
        match rng.below(4) {
            0 => out[at] ^= 1 << rng.below(8),
            1 => out[at] = [0, 0xff, 0x7f, 0x80][rng.below(4)],
            2 => out.truncate(at.max(1)),
            _ => out[at..].iter_mut().take(4).for_each(|b| *b = 0xff),
        }
    }
    out
}

#[test]
fn a_damaged_picture_is_refused_or_shown() {
    let dir = tempfile::tempdir().unwrap();
    let mut rng = Rng(0x1f83_d9ab_fb41_bd6b);
    for (index, name) in PICTURES.iter().enumerate() {
        let whole = std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(name)).unwrap();
        for round in 0..150 {
            let path = dir.path().join(format!("{index}-{round}{}", &name[name.rfind('.').unwrap()..]));
            std::fs::write(&path, damaged(&mut rng, &whole)).unwrap();
            let (send, receive) = mpsc::channel();
            std::thread::spawn(move || {
                let size = image_size(&path).ok();
                let shown = decode_image(&path, 256).ok().map(|i| i.dimensions());
                let reduced = decode_reduced(&path, 64).map(|(_, s)| s);
                send.send((size, shown, reduced))
            });
            match receive.recv_timeout(Duration::from_secs(20)) {
                Ok(_) => {}
                Err(mpsc::RecvTimeoutError::Timeout) => panic!("hung on {name} round {round}"),
                Err(mpsc::RecvTimeoutError::Disconnected) => panic!("panicked on {name} round {round}"),
            }
        }
    }
}
