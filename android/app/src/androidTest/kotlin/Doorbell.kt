package io.github.reiwand

import java.net.InetAddress
import java.net.ServerSocket
import java.util.concurrent.atomic.AtomicInteger

class Doorbell : AutoCloseable {
    private val socket = ServerSocket(0, 8, InetAddress.getByName("127.0.0.1")).apply { soTimeout = 100 }
    val seen = AtomicInteger()
    val base = "127.0.0.1:${socket.localPort}"

    init {
        Thread { while (!socket.isClosed) runCatching { socket.accept().close(); seen.incrementAndGet() } }.start()
    }

    override fun close() = socket.close()
}

const val PRIVATE_PAGE = """<!DOCTYPE html><html><head></head><body><script>
window.probe = { own: "waiting" };
fetch("own.txt").then(r => r.text()).then(t => { window.probe.own = t.trim(); }).catch(() => { window.probe.own = "blocked"; });
const hint = (rel, at) => { const link = document.createElement("link"); link.rel = rel; link.href = at; document.head.appendChild(link); };
const at = () => "https://ORIGIN/" + Date.now();
const ways = {
    fetch: () => fetch(at()).catch(() => {}),
    image: () => { new Image().src = at() + ".png"; },
    socket: () => new WebSocket("wss://ORIGIN/" + Date.now()),
    events: () => new EventSource(at() + ".events"),
    beacon: () => navigator.sendBeacon(at() + ".beacon", "x"),
    video: () => { const v = document.createElement("video"); v.src = at() + ".mp4"; v.load(); },
    preconnect: () => hint("preconnect", "https://ORIGIN"),
    prefetch: () => hint("prefetch", at() + ".next"),
};
window.ways = Object.keys(ways);
window.going = null;
window.go = (name) => { clearInterval(window.going); window.going = name ? setInterval(() => { try { ways[name](); } catch (e) {} }, 200) : null; };
</script></body></html>"""

fun privateManifest(origin: String) =
    sample("cut", "manifest").replace("org.example.cut", "org.example.cut${System.nanoTime()}").replace("ORIGIN", origin)
