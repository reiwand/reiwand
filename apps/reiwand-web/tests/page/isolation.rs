use super::helper::{Page, available, with};
use reiwand_web::{Isolation, Start};
use serde_json::json;
use std::net::{TcpListener, UdpSocket};
use std::time::{Duration, Instant};

const ISOLATION: Isolation = Isolation { network: false, devices: false, limits: false };

const REPORT: &str = "const say = (n, v) => console.log(`probe ${n} ${JSON.stringify(v)}`);\n";

#[test]
fn hidden_graphics_card_stays_hidden_in_inner_frames_and_workers() {
    if !available() {
        return eprintln!("skipped: no Qt WebEngine 6.8 or newer");
    }
    let dir = tempfile::tempdir().unwrap();
    let script = format!(
        "{REPORT}\
        const canvas = document.createElement('canvas');\n\
        const gl = canvas.getContext('webgl');\n\
        const look = (w) => ({{ param: gl ? w.WebGLRenderingContext.prototype.getParameter.call(gl, 0x9245) : null, \
        ext: gl ? w.WebGLRenderingContext.prototype.getExtension.call(gl, 'WEBGL_debug_renderer_info') : null, \
        gpu: typeof w.navigator.gpu }});\n\
        const blank = document.createElement('iframe');\n\
        document.body.append(blank);\n\
        const found = {{ blank: look(blank.contentWindow) }};\n\
        const doc = document.createElement('iframe');\n\
        doc.srcdoc = '<p>x</p>';\n\
        doc.onload = () => {{\n\
          found.srcdoc = look(doc.contentWindow);\n\
          const own = document.createElement('iframe');\n\
          own.src = 'wpak://org.example.probe/frame.html';\n\
          own.onload = () => {{ found.own = look(own.contentWindow); say('frames', {{ gl: !!gl, found }}); }};\n\
          document.body.append(own);\n\
        }};\n\
        document.body.append(doc);\n\
        try {{\n\
          const w = new Worker(URL.createObjectURL(new Blob(['postMessage(typeof OffscreenCanvas)'])));\n\
          w.onmessage = () => say('worker', 'ran');\n\
          w.onerror = () => say('worker', 'blocked');\n\
          setTimeout(() => say('worker', 'silent'), 4000);\n\
        }} catch (e) {{ say('worker', 'blocked'); }}"
    );
    let start = with(dir.path(), 0, &[("probe.js", script.as_bytes()), ("frame.html", b"<p>f</p>")]);
    let mut page = Page::open(Start { mask_gpu: true, ..start }, ISOLATION);
    let frames = page.wait("frames", 30);
    for (name, seen) in frames["found"].as_object().unwrap() {
        assert_ne!(seen["gpu"], json!("object"), "{name}: navigator.gpu is visible");
        assert_eq!(seen["ext"], json!(null), "{name}: the debug renderer extension is visible");
        if frames["gl"] == json!(true) {
            assert_eq!(seen["param"], json!("WebKit"), "{name}: the card model is visible");
        }
    }
    assert_ne!(page.wait("worker", 10), json!("ran"), "a worker ran with the card hidden");
}

#[test]
fn peer_connections_send_nothing_even_from_an_inner_frame_with_network_granted() {
    if !available() {
        return eprintln!("skipped: no Qt WebEngine 6.8 or newer");
    }
    let dir = tempfile::tempdir().unwrap();
    let udp = UdpSocket::bind("127.0.0.1:0").unwrap();
    udp.set_nonblocking(true).unwrap();
    let stun = udp.local_addr().unwrap();
    let granted = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin = format!("http://127.0.0.1:{}", granted.local_addr().unwrap().port());
    let script = format!(
        "{REPORT}\
        const attempt = (w) => {{\n\
          try {{\n\
            const pc = new w.RTCPeerConnection({{ iceServers: [{{ urls: 'stun:{stun}' }}] }});\n\
            pc.createDataChannel('x');\n\
            pc.createOffer().then((o) => pc.setLocalDescription(o)).catch(() => {{}});\n\
            return true;\n\
          }} catch (e) {{ return false; }}\n\
        }};\n\
        const blank = document.createElement('iframe');\n\
        document.body.append(blank);\n\
        const early = {{ RTCPeerConnection: blank.contentWindow.RTCPeerConnection || window[0].RTCPeerConnection }};\n\
        const doc = document.createElement('iframe');\n\
        doc.srcdoc = '<p>x</p>';\n\
        doc.onload = () => {{\n\
          const reopened = doc.contentWindow.document.open();\n\
          say('tried', [attempt(early), attempt(blank.contentWindow), attempt(doc.contentWindow), attempt(window)]);\n\
        }};\n\
        document.body.append(doc);"
    );
    let start = with(dir.path(), 0, &[("probe.js", script.as_bytes())]);
    let mut page = Page::open(Start { network: vec![origin], ..start }, ISOLATION);
    page.wait("tried", 30);
    let end = Instant::now() + Duration::from_secs(5);
    let mut buffer = [0u8; 2048];
    while Instant::now() < end {
        page.pump(Instant::now() + Duration::from_millis(100));
        assert!(udp.recv_from(&mut buffer).is_err(), "a peer connection sent a datagram");
    }
}

/// The language that the page, an inner frame and a worker see through `Intl`, `toLocaleString` and `navigator`.
fn languages_seen(languages: Vec<String>) -> Option<serde_json::Value> {
    if !available() {
        eprintln!("skipped: no Qt WebEngine 6.8 or newer");
        return None;
    }
    let dir = tempfile::tempdir().unwrap();
    let look = "locale: new w.Intl.DateTimeFormat().resolvedOptions().locale, number: new w.Intl.NumberFormat().format(1234.5), \
        language: w.navigator.language, languages: w.navigator.languages";
    let script = format!(
        "{REPORT}\
        const look = (w) => ({{ {look} }});\n\
        const blank = document.createElement('iframe');\n\
        document.body.append(blank);\n\
        const code = 'const w = self; postMessage({{ {look} }})';\n\
        const worker = new Worker(URL.createObjectURL(new Blob([code])));\n\
        worker.onmessage = (e) => say('intl', {{ main: look(window), blank: look(blank.contentWindow), worker: e.data }});"
    );
    let start = Start { languages, ..with(dir.path(), 0, &[("probe.js", script.as_bytes())]) };
    Some(Page::open(start, ISOLATION).wait("intl", 30))
}

#[test]
fn the_language_given_reaches_intl_in_the_page_its_frames_and_its_workers() {
    let Some(seen) = languages_seen(vec!["fr-FR".into()]) else { return };
    for name in ["main", "blank", "worker"] {
        assert_eq!(seen[name]["locale"], json!("fr-FR"), "{name}: {seen}");
        assert_eq!(seen[name]["language"], json!("fr-FR"), "{name}: {seen}");
        assert_eq!(seen[name]["languages"], json!(["fr-FR"]), "{name}: {seen}");
        assert!(seen[name]["number"].as_str().unwrap().contains(','), "{name}: {seen}");
    }
}

#[test]
fn a_hidden_language_is_hidden_from_the_page_its_frames_and_its_workers() {
    let Some(seen) = languages_seen(reiwand_web::languages(&["fr_FR.UTF-8".into()], true)) else { return };
    for name in ["main", "blank", "worker"] {
        assert_eq!(seen[name]["locale"], json!("en-US"), "{name}: {seen}");
        assert_eq!(seen[name]["languages"], json!(["en-US"]), "{name}: {seen}");
        assert_eq!(seen[name]["number"], json!("1,234.5"), "{name}: {seen}");
    }
}
