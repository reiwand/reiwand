use super::helper::{Page, available, with};
use reiwand_web::{FromWeb, Isolation, Start, ToWeb};
use std::os::fd::OwnedFd;
use std::os::unix::fs::FileExt;
use std::time::{Duration, Instant};

const SCRIPT: &str = r#"
const canvas = document.getElementById("c");
canvas.width = innerWidth;
canvas.height = innerHeight;
const draw = canvas.getContext("2d");
let green = 0;
setTimeout(() => { green = 200; }, 1500);
const tick = (t) => {
  draw.fillStyle = `rgb(${Math.round(t / 100) * 10}, ${green}, ${Math.round(performance.now() / 100) * 10})`;
  draw.fillRect(0, 0, canvas.width, canvas.height);
  requestAnimationFrame(tick);
};
requestAnimationFrame(tick);
console.log(`probe wall ${JSON.stringify([Date.now(), new Date().getTime(), performance.now()])}`);
"#;

/// Waits for the next frame of the page (`None` when none came by `until`), and for `Stepped` when asked.
fn next(page: &mut Page, ring: &mut Option<std::fs::File>, until: Instant, stepped: bool) -> Option<Vec<u8>> {
    while Instant::now() < until {
        match page.link.recv::<FromWeb>(false) {
            Ok(Some((FromWeb::Pixels { view, slot, size, stride, .. }, fds))) => {
                let fds: Vec<OwnedFd> = fds;
                fds.into_iter().next().into_iter().for_each(|fd| *ring = Some(std::fs::File::from(fd)));
                let mut pixel = vec![0u8; 4];
                let at = u64::from(slot) * u64::from(stride) * u64::from(size.1);
                ring.as_ref().expect("no frame buffer").read_exact_at(&mut pixel, at).unwrap();
                page.link.send(&ToWeb::Freed { view, slot }, &[]).unwrap();
                if !stepped {
                    return Some(pixel);
                }
            }
            Ok(Some((FromWeb::Stepped { .. }, _))) if stepped => return Some(Vec::new()),
            Ok(Some((FromWeb::Console { line, .. } | FromWeb::Said(line), _))) => {
                if let Some(rest) = line.strip_prefix("log: probe wall ") {
                    page.said.entry("wall".into()).or_default().push(serde_json::from_str(rest).unwrap());
                }
            }
            Ok(Some(_)) => {}
            Ok(None) => panic!("the helper stopped"),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(5))
            }
            Err(e) => panic!("{e}"),
        }
    }
    None
}

const WEBGL: &str = r#"
const canvas = document.getElementById("c");
canvas.width = innerWidth;
canvas.height = innerHeight;
const gl = canvas.getContext("webgl");
const tick = (t) => {
  gl.clearColor(Math.round(t / 100) * 10 / 255, 0.5, 0, 1);
  gl.clear(gl.COLOR_BUFFER_BIT);
  requestAnimationFrame(tick);
};
requestAnimationFrame(tick);
console.log(`probe wall ${JSON.stringify([Boolean(gl)])}`);
"#;

/// Steps a page drawn without a render node and checks the colour it shows at each time.
fn steps(script: &str, wanted: &[(f64, [i32; 3])]) -> Page {
    let dir = tempfile::tempdir().unwrap();
    let start = with(dir.path(), 9, &[("probe.js", script.as_bytes())]);
    let start = Start { device: None, storage: None, preload: None, clock: Some(1_700_000_000.0), ..start };
    let mut page = Page::open(start, Isolation { network: true, devices: true, limits: false });
    let mut ring = None;
    let begun = Instant::now() + Duration::from_secs(20);
    while !page.said.contains_key("wall") && Instant::now() < begun {
        next(&mut page, &mut ring, Instant::now() + Duration::from_millis(50), false);
    }
    page.link.send(&ToWeb::Fps { view: 1, fps: 240 }, &[]).unwrap();
    std::thread::sleep(Duration::from_millis(700));
    for &(at, want) in wanted {
        page.link.send(&ToWeb::Step { view: 1, at }, &[]).unwrap();
        let stepped = next(&mut page, &mut ring, Instant::now() + Duration::from_secs(10), true);
        assert!(stepped.is_some(), "no answer to the step to {at}");
        while next(&mut page, &mut ring, Instant::now() + Duration::from_millis(150), false).is_some() {}
        page.link.send(&ToWeb::Shot { view: 1 }, &[]).unwrap();
        let pixel =
            next(&mut page, &mut ring, Instant::now() + Duration::from_secs(5), false).expect("no shot");
        let rgb = [pixel[2], pixel[1], pixel[0]];
        let near = rgb.iter().zip(want).all(|(got, want)| (i32::from(*got) - want).abs() <= 2);
        assert!(near, "at {at} s the page shows {rgb:?}, not {want:?}");
    }
    page
}

#[test]
fn a_page_on_a_frame_clock_draws_webgl_in_software() {
    if !available() {
        return eprintln!("skipped: no Qt WebEngine 6.8 or newer");
    }
    let page = steps(WEBGL, &[(0.0, [0, 128, 0]), (0.5, [50, 128, 0]), (1.0, [100, 128, 0])]);
    assert_eq!(page.said["wall"][0], serde_json::json!([true]), "no WebGL in software");
}

#[test]
fn a_page_on_a_frame_clock_draws_each_step_at_its_time_whatever_the_real_time() {
    if !available() {
        return eprintln!("skipped: no Qt WebEngine 6.8 or newer");
    }
    let wanted = [(0.0, [0, 0, 0]), (0.5, [50, 0, 50]), (1.0, [100, 0, 100]), (2.0, [200, 200, 200])];
    let page = steps(SCRIPT, &wanted);
    assert_eq!(page.said["wall"][0], serde_json::json!([1_700_000_000_000u64, 1_700_000_000_000u64, 0]));
}
