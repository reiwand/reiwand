use super::helper::{Page, available, with};
use reiwand_web::{Isolation, Start};
use serde_json::{Value, json};

const ISOLATION: Isolation = Isolation { network: false, devices: false, limits: false };

/// What a window shows of the things the page must not have: `true` where it is still the engine's own.
const LOOK: &str = "const native = (f) => typeof f === 'function' && String(f).includes('[native code]');\n\
    const look = (w) => ({ rtc: typeof w.RTCPeerConnection !== 'undefined', \
    devices: native(w.navigator.mediaDevices && w.navigator.mediaDevices.enumerateDevices), \
    card: native(w.WebGLRenderingContext && w.WebGLRenderingContext.prototype.getParameter), \
    gpu: typeof w.navigator.gpu === 'object', raf: native(w.requestAnimationFrame) });\n";

fn report() -> String {
    format!("const say = (n, v) => console.log(`probe ${{n}} ${{JSON.stringify(v)}}`);\n{LOOK}")
}

fn untouched(name: &str, seen: &Value) {
    for (what, native) in seen.as_object().unwrap_or_else(|| panic!("{name}: {seen}")) {
        assert_eq!(native, &json!(false), "{name}: {what} is the engine's own ({seen})");
    }
}

#[test]
fn a_frame_is_protected_from_the_moment_the_page_can_reach_it() {
    if !available() {
        return eprintln!("skipped: no Qt WebEngine 6.8 or newer");
    }
    let dir = tempfile::tempdir().unwrap();
    let script = format!(
        "{}\
        window.addEventListener('message', (e) => say(e.data[0], e.data[1]));\n\
        const blank = document.createElement('iframe');\n\
        document.body.append(blank);\n\
        say('appended', {{ property: look(blank.contentWindow), indexed: look(window[0]) }});\n\
        const holder = document.createElement('div');\n\
        document.body.append(holder);\n\
        holder.innerHTML = '<iframe></iframe>';\n\
        say('written', look(window[1]));\n\
        const parsed = document.createElement('iframe');\n\
        parsed.src = 'wpak://org.example.probe/parsed.html';\n\
        document.body.append(parsed);",
        report()
    );
    let parsed = format!(
        "<!doctype html><body><iframe></iframe><script>{LOOK}parent.postMessage(['parsed', look(window[0])], '*');\
        </script></body>"
    );
    let files: &[(&str, &[u8])] = &[("probe.js", script.as_bytes()), ("parsed.html", parsed.as_bytes())];
    let start = with(dir.path(), 0, files);
    let mut page = Page::open(Start { mask_gpu: true, ..start }, ISOLATION);
    let appended = page.wait("appended", 30);
    untouched("contentWindow right after append", &appended["property"]);
    untouched("window[0] right after append", &appended["indexed"]);
    untouched("a frame written with innerHTML", &page.wait("written", 10));
    untouched("a frame the parser inserted", &page.wait("parsed", 20));
}

#[test]
fn every_frame_draws_at_the_pace_of_the_page() {
    if !available() {
        return eprintln!("skipped: no Qt WebEngine 6.8 or newer");
    }
    let dir = tempfile::tempdir().unwrap();
    let count = "let n = 0; const tick = () => { n += 1; requestAnimationFrame(tick); }; requestAnimationFrame(tick);";
    let script = format!(
        "{}\
        window.addEventListener('message', (e) => say(e.data[0], e.data[1]));\n\
        const counted = (raf, name) => {{ let n = 0; const tick = () => {{ n += 1; raf(tick); }}; raf(tick); \
        setTimeout(() => say(name, n), 3000); }};\n\
        counted(requestAnimationFrame, 'page');\n\
        const blank = document.createElement('iframe');\n\
        document.body.append(blank);\n\
        counted(blank.contentWindow.requestAnimationFrame, 'early');\n\
        const shut = document.createElement('iframe');\n\
        shut.sandbox = 'allow-scripts';\n\
        shut.style = 'position: fixed; left: 0; top: 0';\n\
        shut.srcdoc = '<script>{count} setTimeout(() => parent.postMessage([`sandboxed`, n], `*`), 3000);</' + 'script>';\n\
        document.body.append(shut);",
        report()
    );
    let start = with(dir.path(), 0, &[("probe.js", script.as_bytes())]);
    let capped = f64::from(start.settings.fps) * 1.5;
    let mut page = Page::open(start, ISOLATION);
    let rate = |n: &Value| n.as_u64().unwrap_or_else(|| panic!("{n}")) as f64 / 3.0;
    let paced = rate(&page.wait("page", 30));
    assert!(paced <= capped, "the page drew {paced} frames a second");
    let early = rate(&page.wait("early", 10));
    assert!(early <= capped, "a frame reached at once drew {early} frames a second");
    let sandboxed = rate(&page.wait("sandboxed", 10));
    assert!(sandboxed > 1.0 && sandboxed <= capped, "a sandboxed frame drew {sandboxed} frames a second");
}
