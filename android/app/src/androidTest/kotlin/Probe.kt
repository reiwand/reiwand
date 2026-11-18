package io.github.reiwand

val PROBE_MANIFEST: String get() = sample("probe", "manifest")

const val PROBE_PAGE = """<!doctype html>
<html><body style="margin:0;height:100vh"><script src="probe.js"></script></body></html>"""

const val PEERS_PAGE = """<!doctype html>
<html><body><script>
window.peers = {};
window.trial = async (g) => {
  if (!g) return "missing";
  try {
    const c = new g.RTCPeerConnection();
    c.createDataChannel("x");
    await c.setLocalDescription(await c.createOffer());
    return "open";
  } catch (e) {
    return "blocked";
  }
};
const report = (name, g) => trial(g).then((r) => { peers[name] = r; });
const make = () => document.createElement("iframe");
const born = (add) => {
  const at = window.length;
  add();
  return window[at];
};
report("top", window);
report("plain", born(() => document.body.append(make())));
const call = "parent.trial(window).then((r) => parent.peers.shadowdoc = r)";
document.body.appendChild(document.createElement("div")).attachShadow({ mode: "open" }).innerHTML = "<iframe srcdoc='<scr" + "ipt>" + call + "</scr" + "ipt>'></iframe>";
report("fragment", born(() => {
  const part = document.createDocumentFragment();
  part.append(make());
  document.body.append(part);
}));
report("reopen", born(() => {
  const frame = document.body.appendChild(make());
  frame.contentDocument.open();
  frame.contentDocument.write("<p>");
  frame.contentDocument.close();
}));
const moved = make();
document.body.append(moved);
const at = window.length - 1;
moved.addEventListener("load", () => report("moved", window[at]), { once: true });
setTimeout(() => { moved.src = "about:blank"; }, 50);
document.body.append(Object.assign(make(), {
  srcdoc: "<scr" + "ipt>parent.trial(window).then((r) => parent.peers.srcdoc = r)</scr" + "ipt>",
}));
report("nested", born(() => {
  const outer = document.body.appendChild(make()).contentWindow;
  outer.document.body.append(outer.document.createElement("iframe"));
}));
</script>
<div id="declared"><template shadowrootmode="open"><iframe srcdoc="<script>fetch('http://127.0.0.1:PORT/fetch').catch(() => 0); parent.trial(window).then((r) => parent.peers.declared = r)</script>"></iframe><iframe></iframe><iframe hidden src="http://127.0.0.1:PORT/frame"></iframe></template></div>
<script>report("declaredgrab", document.getElementById("declared").shadowRoot.querySelectorAll("iframe")[1].contentWindow);</script>
</body></html>"""

const val PROBE_SCRIPT = """
const probe = (window.probe = { frames: 0, rates: [], paused: [], clicks: [], moves: 0, props: {} });
probe.early = window.early;
probe.state = document.visibilityState;
document.addEventListener("visibilitychange", () => { probe.state = document.visibilityState; });
probe.zone = Intl.DateTimeFormat().resolvedOptions().timeZone;
probe.hour = new Date(Date.UTC(2024, 0, 1, 12)).getHours();
probe.language = navigator.language;
probe.peer = typeof RTCPeerConnection;
const gl = document.createElement("canvas").getContext("webgl");
const info = gl && gl.getExtension("WEBGL_debug_renderer_info");
probe.webgl = gl ? String(gl.getParameter(info ? info.UNMASKED_RENDERER_WEBGL : gl.RENDERER)) : null;
const zoneOf = (g) => new g.Intl.DateTimeFormat().resolvedOptions().timeZone;
document.body.append(Object.assign(document.createElement("iframe"), { hidden: true }));
const inner = window[0];
probe.routes = {
  frame: zoneOf(inner), frameHour: new inner.Date(Date.UTC(2024, 0, 1, 12)).getHours(), frameLanguage: inner.navigator.language,
  framePeer: typeof inner.RTCPeerConnection, made: new Intl.DateTimeFormat.prototype.constructor().resolvedOptions().timeZone,
  madeHour: new (Object.getPrototypeOf(new Date()).constructor)(Date.UTC(2024, 0, 1, 12)).getHours(),
};
const leaks = (g) => [new g.Intl.DateTimeFormat(undefined, { timeZone: undefined }).resolvedOptions().timeZone,
  new g.Date("2024-01-01T12:00") - Date.UTC(2024, 0, 1, 12), new g.Intl.NumberFormat([]).resolvedOptions().locale,
  new g.Intl.DateTimeFormat(["zz"]).resolvedOptions().locale, Math.round((g.Date.parse(g.document.lastModified) - Date.now()) / 36e5)];
probe.leaks = [leaks(window), leaks(inner)];
const code = "postMessage([new Intl.DateTimeFormat().resolvedOptions().timeZone, new Date(Date.UTC(2024, 0, 1, 12)).getHours(), navigator.language, " +
  "new Intl.DateTimeFormat(undefined, { timeZone: undefined }).resolvedOptions().timeZone, new Date('2024-01-01T12:00') - Date.UTC(2024, 0, 1, 12)])";
const worker = new Worker(URL.createObjectURL(new Blob([code], { type: "text/javascript" })));
worker.onmessage = (event) => { probe.worker = event.data; };
worker.onerror = (event) => { probe.worker = ["error", String(event.message)]; };
reiwand.onProperties((changed) => {
  Object.assign(probe.props, changed);
  if (changed.tint) document.body.style.background = changed.tint.hex;
});
reiwand.onSettings((settings) => { probe.settings = settings; });
reiwand.onPause((paused) => probe.paused.push(paused));
addEventListener("click", (event) => probe.clicks.push(event.isTrusted));
addEventListener("pointermove", () => { probe.moves += 1; });
addEventListener("pointerdown", (event) => { probe.downs = (probe.downs || 0) + 1; });
const reach = (name, url) => fetch(url).then((r) => (r.ok ? r.text() : Promise.reject(r.status)))
  .then((text) => { probe[name] = "read " + text.length; }, () => { probe[name] = "blocked"; });
reach("own", "probe.js");
reach("manifest", "manifest.json");
reach("escape", "../manifest.json");
reach("other", "https://0000000000000000.wpak.invalid/index.html");
reach("file", "file:///system/etc/hosts");
reach("wpak", "wpak://org.example.probe/index.html");
reach("local", "http://127.0.0.1:" + reiwand.properties.port + "/");
if (reiwand.properties.photo) reach("photo", reiwand.properties.photo);
const count = () => { probe.frames += 1; requestAnimationFrame(count); };
requestAnimationFrame(count);
setInterval(() => { probe.rates.push(probe.frames); probe.frames = 0; }, 1000);
try {
  const visits = Number(localStorage.getItem("visits") || 0) + 1;
  localStorage.setItem("visits", String(visits));
  probe.visits = visits;
} catch (e) {
  probe.visits = String(e);
}
"""
