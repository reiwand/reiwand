# Engine, Frames, Lifecycle and Session and the page's view

On this page: engine, frames, lifecycle, session and the page's view.

## Engine

`reiwand-web` is built against Qt Quick and Qt WebEngine 6.8 or newer (the version that lets a
content security policy go with each URL scheme) and runs the page in an off-screen `WebEngineView`
rendered with `QQuickRenderControl`, so no window ever shows. Qt WebEngine comes with the distribution
(or, in the Flatpak, with `io.qt.qtwebengine.BaseApp`): Reiwand bundles no engine. A build whose
computer has no Qt WebEngine 6.8 or newer (`no_engine`) still produces the helper, and the daemon
refuses web wallpapers with a translated reason, in `Library1.Apply`, `reiwand-ctl lib apply` and the
error of the screen, instead of drawing nothing:

| Where | What it has | Web pages |
|-------|-------------|-----------|
| Arch Linux, Fedora | `qt6-webengine` / `qt6-qtwebengine` 6.8 or newer | shown |
| Flatpak | `io.qt.qtwebengine.BaseApp` 6.11, which the manifest builds on, and `QtWebEngineProcess` in `/app/bin` | shown |
| AppImage and RPM, on any system (Ubuntu 24.04 included) | Qt WebEngine of Qt 6.12, carried next to the helper | shown |
| Ubuntu 24.04, a local build | Qt 6.4, which is older than 6.8 and has no per-scheme content security policy | not shown: the reason names Qt WebEngine 6.8 or newer |
| any, without the helper | — | refused: `the web helper reiwand-web is not installed` |
| Android (the app) | the system WebView | shown with it, see "On the phone" |

The *Tests* workflow (started by hand) proves each row. Its `web` job runs the page tests, the daemon and the video sound in an Arch
Linux container with `qt6-webengine` (`scripts/web-engine-test.sh`; it says whether the runner has a
render node, and without one the pages run on the software path); `build-qt` builds the helper of the
release binaries on Ubuntu 24.04 against the Qt that the AppImage carries (`scripts/qt-prefix.sh`),
while the other Ubuntu 24.04 jobs compile it as `no_engine` (they still check, lint and test the
Rust side and the refusal, `scripts/integration-web.sh`); the Flatpak
job builds on the BaseApp and `packaging/flatpak/web-check.sh` runs `reiwand-web --check` inside the
sandbox on a headless Wayland compositor.

The daemon finds the helper in `REIWAND_WEB`, next to `reiwandd` or in `PATH`, and runs
`reiwand-web --check` once to learn whether it can start (it prints the Qt version, or why not).

## Frames

The helper grabs the frame of the off-screen view itself, so it decides when a frame is done: it
reports a frame as presented only at the frame rate the daemon asks for (the output's cap, battery
and power saving included), so the cap limits every frame that reaches the screen. Chromium itself
runs `requestAnimationFrame` at the display's rate, so a script injected before the page's own
paces it at the cap in the page and in every frame: a frame hands its callbacks to its parent's
capped function (from the moment the page can reach it, see Privacy), and a frame that cannot reach
its parent (a sandboxed one) is paced on its own. A dedicated worker cannot be given a script, so a
worker's own `requestAnimationFrame` (of an `OffscreenCanvas`) is not paced: what it draws still
reaches the screen only at the cap, but its script runs at the display's rate. A page that does not change draws nothing. An idle helper (no view visible for 5 minutes) sends `Idle` and the daemon stops it; it starts again when a view is visible. `GetState().outputs[x].page_frames`
counts the frames the page drew for an output. A fade to a page waits for its first frame (or for
the page to fail), so the wallpaper before it never fades into an empty output; a fade from a page
starts from its last frame.

When the daemon's GPU can import them, the helper exports the frame of its OpenGL ES texture as a DMA-BUF in the formats and
modifiers that GPU samples in place (`XRGB8888`/`ARGB8888` with one plane, on the render node of the
daemon) and sends each buffer once (`decode` reads `zero-copy`); the daemon gives a
buffer back once a newer frame replaced it. Otherwise frames travel through a sealed `memfd` with two
slots (`decode` reads `copy`). A page gets a GPU only when the daemon renders on a hardware GPU
whose render node can allocate a GBM buffer (`reiwand-web --gpu <node>`, checked once per node);
otherwise (`vgem`, virtual GPUs without 3D, software rendering) it starts with `LIBGL_ALWAYS_SOFTWARE=1`
and draws in memory, as it also does when it draws nothing on the GPU within 10 s; when it goes back to software the helper starts in its own mount namespace with an empty tmpfs over `/dev/dri`, so the engine cannot find a GPU. Pages are
always opaque.

## Lifecycle

- A page is visible while its screen is active and seen; a stream that loses its last consumer
  hides it after 2 s, so readers that come and go do not restart it. Hidden pages get
  `document.visibilityState = "hidden"`, stop their animation frames and draw nothing; after 5
  minutes hidden the web process is stopped and the last frame stays on screen until it is visible
  again, when the page loads again.
- The helper asks each page for an answer every 5 s while its web process runs; a page that does
  not answer for 10 s (a script stuck in a loop) is stopped and loads again like a crashed one. The
  helper reloads a page whose web process crashed, ran out of memory or stopped answering after 1, 4
  and 9 s; a fourth time in a row turns the page off, the screen shows its background color and its
  error says why. A page that runs for a minute without crashing starts counting again. The daemon
  restarts a helper that stopped or failed after 1, 5 and 30 s; after the fourth failure in a row (three restarts) the
  screen shows its background color and its error says why. A page that draws nothing within 10 s,
  also in memory, stops and restarts the same way, and its error is "drew nothing for 10 seconds and is off".
  A helper whose systemd scope fails before it is ready restarts once without the scope.
- The helper and the processes of Qt WebEngine run at nice 10. When a systemd user manager runs, the helper
  starts in its own transient scope with `CPUQuota=100%`, `CPUWeight=20`, `MemoryHigh=768M`, `MemoryMax=1536M`,
  `MemorySwapMax=0` and `TasksMax=512`, so a page takes at most one processor, is slowed down past
  768 MB and is killed past 1.5 GB in all (the scope is the only memory limit: Chromium has no
  per-page cap of its own here);
  where the scope cannot be made the helper starts without it.
- A page whose kept data (what its origin stores in `data` below: local and session storage, IndexedDB, service workers, file system, databases and cookies; the engine's own caches do not count) grows past twice its `web.storage_kib` plus 1 MiB starts
  without keeping anything, and its error says so until its data is cleared. On the desktop the
  limit is checked only when the page starts, because the helper has no quota per origin to set in
  its profile: a running page can keep more until its next start. On the phone the WebView
  has no quota per origin either, so each start of a page measures what its origin keeps (the
  WebView's own count for IndexedDB, Cache Storage and the like, plus `localStorage`, which a script
  that runs before the page counts) against the same limit. While it runs, a `localStorage` write
  past the limit, with `setItem` or as a property, in the page or its frames, throws
  `QuotaExceededError`; a write to IndexedDB or Cache Storage has the origin measured again two
  seconds later, and so does hiding the page; a page that starts a worker has the origin measured
  every ten seconds as well, since a worker's writes cannot be observed. A `localStorage` write in
  one frame marks the totals of the other frames of its origin as stale (through the `storage`
  event), so each frame measures again before its next write and the frames together stay under the
  limit. A page that grows past the limit while it runs starts again at once on the passing origin. A
  page over the limit runs on a passing origin that keeps nothing, and its page in the app says so
  until *Clear data*, which also starts a new origin.
- Changing a property sends only the changed values to the running page; changing what the page may
  see (below), clearing its data or granting the network starts it again.

## Session and the page's view

Qt needs an OpenGL ES context to render, so the helper connects to the Wayland or X session of the
daemon (`WAYLAND_DISPLAY` or `DISPLAY`) only to create that context and a surface that is never
mapped: it opens no window and takes no input from the session. The page sees none of it: the size
of its screen, the pointer and the clicks come from the daemon as messages, and the engine runs
with the platform of that session only for the GL context. `reiwand-web --check` creates the same
context, so it needs a session with working GL (a headless compositor serves; the offscreen Qt
platform does not). Without a session the helper fails to start and the daemon shows the error of
the screen.
