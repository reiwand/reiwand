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
        const doc = document.createElement('iframe');\n\
        doc.srcdoc = '<p>x</p>';\n\
        doc.onload = () => {{\n\
          const reopened = doc.contentWindow.document.open();\n\
          say('tried', [attempt(blank.contentWindow), attempt(doc.contentWindow), attempt(window)]);\n\
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

#[test]
fn the_hidden_language_reaches_intl_in_the_page_and_in_its_frames() {
    if !available() {
        return eprintln!("skipped: no Qt WebEngine 6.8 or newer");
    }
    let dir = tempfile::tempdir().unwrap();
    let script = format!(
        "{REPORT}\
        const look = (w) => ({{ locale: new w.Intl.DateTimeFormat().resolvedOptions().locale, \
        number: (1234.5).toLocaleString(), language: w.navigator.language }});\n\
        const blank = document.createElement('iframe');\n\
        document.body.append(blank);\n\
        say('intl', {{ main: look(window), blank: look(blank.contentWindow) }});"
    );
    let mut start = with(dir.path(), 0, &[("probe.js", script.as_bytes())]);
    start.languages = vec!["fr-FR".into()];
    let mut page = Page::open(start, ISOLATION);
    let seen = page.wait("intl", 30);
    for name in ["main", "blank"] {
        assert_eq!(seen[name]["locale"], json!("fr-FR"), "{name}: {seen}");
        assert_eq!(seen[name]["language"], json!("fr-FR"), "{name}: {seen}");
        assert!(seen[name]["number"].as_str().unwrap().contains(','), "{name}: {seen}");
    }
}
