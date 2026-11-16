const report = (name, value) => console.log(`probe ${name} ${JSON.stringify(value)}`);
const canvas = document.getElementById("c");
canvas.width = innerWidth;
canvas.height = innerHeight;
const draw = canvas.getContext("2d");
let frames = 0;
const tick = () => {
  frames += 1;
  draw.fillStyle = `hsl(${frames % 360}, 80%, 50%)`;
  draw.fillRect(0, 0, canvas.width, canvas.height);
  requestAnimationFrame(tick);
};
requestAnimationFrame(tick);
setInterval(() => { report("frames", frames); frames = 0; }, 1000);
report("early", window.early);
report("zone", Intl.DateTimeFormat().resolvedOptions().timeZone);
report("language", navigator.language);
const gl = document.createElement("canvas").getContext("webgl");
const info = gl && gl.getExtension("WEBGL_debug_renderer_info");
report("webgl", gl ? String(gl.getParameter(info ? info.UNMASKED_RENDERER_WEBGL : gl.RENDERER)) : null);
reiwand.onProperties((changed) => report("properties", changed));
reiwand.onSettings((settings) => report("settings", settings));
reiwand.onPause((paused) => report("paused", paused));
reiwand.onAudio((bands) => report("audio", bands.left.length));
reiwand.onMedia((m) => {
  report("media", m.enabled ? [m.state, m.title, m.colors && m.colors.primary.hex, Boolean(m.cover)] : false);
  if (m.cover) fetch(m.cover).then((r) => r.blob()).then((b) => report("cover", [m.cover.slice(0, 5), b.type, b.size > 0]));
});
addEventListener("pointermove", (e) => report("move", [Math.round(e.clientX), Math.round(e.clientY), e.isTrusted]));
addEventListener("click", (e) => report("click", [Math.round(e.clientX), Math.round(e.clientY), e.isTrusted]));
const reach = (name, url) => fetch(url).then((r) => r.text()).then((t) => report(name, `read ${t.length}`), () => report(name, "blocked"));
reach("own", "probe.js");
reach("manifest", "manifest.json");
reach("other", "wpak://org.example.other/index.html");
reach("file", "file:///etc/passwd");
reach("escape", "wpak://org.example.probe/../../etc/passwd");
reach("user", "wpak-user://photos/0000000000000000.png");
reach("local", `http://127.0.0.1:${reiwand.properties.port || 9}/`);
reiwand.files("photos").then((list) => report("files", list));
reiwand.randomFile("photos").then((one) => one && reach("picked", one));
reiwand.onFiles("photos", (change) => report("change", change));
try {
  localStorage.setItem("visits", String(Number(localStorage.getItem("visits") || 0) + 1));
  report("visits", localStorage.getItem("visits"));
} catch (e) {
  report("visits", String(e));
}
const local = `127.0.0.1:${reiwand.properties.port || 9}`;
reach("encoded", "wpak://org.example.probe/%2e%2e/%2e%2e/etc/passwd");
try {
  const socket = new WebSocket(`ws://${local}/`);
  socket.onopen = () => report("socket", "open");
  socket.onerror = () => report("socket", "blocked");
} catch (e) {
  report("socket", "blocked");
}
try {
  navigator.sendBeacon(`http://${local}/beacon`, "x");
} catch (e) {}
new Image().src = `http://${local}/image.png`;
report("window", window.open("probe.js") === null ? "blocked" : "opened");
const asked = (name, ask) => {
  try {
    Promise.resolve(ask()).then(() => report(name, "given"), () => report(name, "blocked"));
  } catch (e) {
    report(name, "blocked");
  }
};
asked("camera", () => navigator.mediaDevices.getUserMedia({ audio: true, video: true }));
asked("clipboard", () => navigator.clipboard.readText());
asked("place", () => new Promise((given, refused) => navigator.geolocation.getCurrentPosition(given, refused)));
