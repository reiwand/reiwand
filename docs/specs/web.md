# Web pages

A `web` package (`package.md`, "Web pages") is shown by `reiwand-web`, a helper process that the
daemon starts for each web package shown and stops as soon as no screen shows it. The helper runs
the page in Qt WebEngine (Chromium), hands every frame to the daemon, and the daemon draws it like any other
wallpaper: with the same transitions, fit, frame rate cap, pause rules and streams.

## Engine

`reiwand-web` is built against Qt Quick and Qt WebEngine 6.5 or newer (the version that lets a
content security policy go with each URL scheme) and runs the page in an off-screen `WebEngineView`
rendered with `QQuickRenderControl`, so no window ever shows. Qt WebEngine comes with the distribution
(or, in the Flatpak, with `io.qt.qtwebengine.BaseApp`): Reiwand bundles no engine. A build whose
computer has no Qt WebEngine 6.5 or newer (`no_engine`) still produces the helper, and the daemon
refuses web wallpapers with a translated reason, in `Library1.Apply`, `reiwand-ctl lib apply` and the
error of the screen, instead of drawing nothing:

| Where | What it has | Web pages |
|-------|-------------|-----------|
| Arch Linux, Fedora | `qt6-webengine` / `qt6-qtwebengine` 6.5 or newer | shown |
| Flatpak | `io.qt.qtwebengine.BaseApp` 6.11, which the manifest builds on, and `QtWebEngineProcess` in `/app/bin` | shown |
| Ubuntu 24.04 (the CI runner) | Qt 6.4, which has no per-scheme content security policy | not shown: its tarball carries the helper built on Fedora (below), which needs Qt WebEngine on the computer; the reason names Qt WebEngine 6.5 or newer |
| any, without the helper | — | refused: `the web helper reiwand-web is not installed` |
| Android (the app) | the system WebView | shown with it, see "On the phone" |

The CI proves each row. The `web` job runs the page tests, the daemon and the video sound in an Arch
Linux container with `qt6-webengine` (`scripts/web-engine-test.sh`; it says whether the runner has a
render node, and without one the pages run on the software path); `build-web` builds the helper of the
release binaries in a Fedora container, because the Ubuntu 24.04 jobs compile it as `no_engine` (they
still check, lint and test the Rust side and the refusal, `scripts/integration-web.sh`); the Flatpak
job builds on the BaseApp and `packaging/flatpak/web-check.sh` runs `reiwand-web --check` inside the
sandbox on a headless Wayland compositor.

The daemon finds the helper in `REIWAND_WEB`, next to `reiwandd` or in `PATH`, and runs
`reiwand-web --check` once to learn whether it can start (it prints the Qt version, or why not).

## Frames

The helper grabs the frame of the off-screen view itself, so it decides when a frame is done: it
reports a frame as presented only at the frame rate the daemon asks for (the output's cap, battery
and power saving included), and Chromium paces animation frames, CSS animations and video by that, so the cap limits every frame the page draws, not only
`requestAnimationFrame`. A page that does not change draws nothing. An idle helper (no view visible for 5 minutes) sends `Idle` and the daemon stops it; it starts again when a view is visible. `GetState().outputs[x].page_frames`
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
and draws in memory, as it also does when it draws nothing on the GPU within 10 s. Pages are
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
  restarts a helper that stopped or failed after 1, 5 and 30 s; after the third failure in a row the
  screen shows its background color and its error says why.
- The helper and the processes of Qt WebEngine run at nice 10. When a systemd user manager runs, the helper
  starts in its own transient scope with `CPUQuota=100%`, `CPUWeight=20`, `MemoryHigh=768M`, `MemoryMax=1536M`,
  `MemorySwapMax=0` and `TasksMax=512`, so a page takes at most one processor, is slowed down past
  768 MB and is killed past 1.5 GB in all (the scope is the only memory limit: Chromium has no
  per-page cap of its own here);
  where the scope cannot be made the helper starts without it.
- A page whose kept data (`data` below) grows past twice its `web.storage_kib` plus 1 MiB starts
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

## Protocol

The helper gets one end of a `SOCK_SEQPACKET` socket pair as descriptor 3. Each packet is one JSON
message (`ToWeb` from the daemon, `FromWeb` from the helper, in `apps/reiwand-web/src/lib.rs`), with
buffer descriptors passed as `SCM_RIGHTS`; a message over 32 KiB (long folder listings) travels in a
sealed `memfd` instead, up to 64 MiB. The daemon maps frames only from `memfd`s sealed against
shrinking and as large as the frames they hold. The helper dies with the daemon
(`PR_SET_PDEATHSIG`).

Each helper leads its own process group, which the processes of Qt WebEngine join. To stop a page the daemon
sends `Quit` (closing the socket counts the same): the helper closes its views, lets go of its
profile, waits up to one second for those processes to end, kills
those left and exits. The daemon never waits for this on its render thread: a thread of its own
gives the helper 1.5 s, then kills it, and kills whatever stays in its group 3 s later. A page that
keeps data starts again only once its earlier helper and group are gone, and `ClearData` deletes
the data only after that, so two engines never share it. When the daemon stops it waits for every
helper to end.

## Console of a preview

A page shown for the editor on a `preview:` output (`PreviewScene`, `dbus.md`) keeps its last 50
console lines and errors per screen, which the helper sends as `Console` with the view that wrote
them, and `GetState` shows them in `console` of that output; other screens keep nothing, so a page
that writes often never wakes the bus. The helper limits what it forwards with a bucket of 50 lines that refills at 10 a second, so a
flood is cut but a preview that runs for hours keeps showing its latest lines.

## Sandbox

Every layer applies to every page:

1. The page only reads its own package through `wpak://<package id>/<path>`: only files listed in
   the package (never `manifest.json` or its signature), percent-decoded, without `..`. A file the
   user chose reaches it only through an opaque `wpak-user://<property>/<token>` (below). Nothing
   else resolves: `file:`, other packages, other schemes.
2. A content security policy that the page cannot remove (the scheme handler adds it to every
   response of `wpak:` and `wpak-user:`, as a header) limits every load to `wpak:`, `wpak-user:`,
   `data:` and `blob:`, plus the declared origins once the network is allowed; no objects, forms or
   `<base>`.
3. A request interceptor of the profile cuts every request but those schemes and origins, including
   `fetch` and WebSockets (a request refused by the interceptor fails as a blocked request).
4. Navigation outside the package, new windows, downloads, dialogs, the file chooser,
   notifications and full screen are refused; every permission request (camera, microphone,
   location, clipboard and the rest) is denied; WebRTC and media capture are removed by a script
   injected before the page's own (`RTCPeerConnection` and its relatives are deleted, and
   `getUserMedia`, `getDisplayMedia` and `enumerateDevices` reject); keys never reach the page.
5. Unless the package declares origins and the user granted `network`, the helper starts in its own
   user and network namespaces with no network at all, so not even the network service of
   Chromium can reach anything.
6. Chromium runs the page in its renderer process inside its own user namespace and seccomp
   sandbox; the memory of everything is limited by the scope of the helper (see Lifecycle).

Limit: a `fetch` of a file that is not in the package answers with a network error because the helper redirects it to a blocked address; Qt WebEngine never settles a `fetch` when the request job is failed with `fail`, so the failure cannot be reported with a proper error code.

`OutputState.sandbox` is `"on"` when every layer is in place and `"reduced"` when the helper
could not make the namespaces and started Qt WebEngine with its sandbox off
(`QTWEBENGINE_DISABLE_SANDBOX`; some containers forbid them); clients show
it next to the privacy notice. `reiwand-ctl` prints the same notice, on standard error, after `lib apply`,
`playlist start`, `next` and `prev` (for the screens they touch) and in `state` when run in a terminal.

`reiwand-ctl scene thumbnail DIR --capture` takes the thumbnail or the animated preview of a page from the page the daemon shows: `CapturePage` (`dbus.md`) sends `Shot` to the helper, which copies the last frame it drew, even while the view is hidden, and sends it as pixels. No second engine starts, so the sandbox and the network of the capture are those of the wallpaper.

## Privacy

A page sees, by default, the real time zone, the language of the user, what the graphics card can
do through WebGL, and keeps its own storage when it declares `web.storage_kib`. On the desktop WebGL
tells the page the model that the engine reports for the card (the debug renderer extension), as on
the phone. Each of them is
restricted per package with `Library1.Confirm` (`hide_zone`: UTC; `hide_language`: `en-US`, also in
`reiwand.locale` and the labels of combos; `hide_gpu`: a script injected before the page's (`mask.js`) makes WebGL keep working with a
generic vendor and renderer (`WebKit`, `WebKit WebGL`) and no debug extension, and `navigator.gpu`
(WebGPU) is absent; `hide_storage`: nothing is kept). `PackageInfo.exposes` lists what a page
sees now, and the app (details and *Apply to…*), the tray (each screen) and `reiwand-ctl` (`lib apply`
and `lib show`) show the same notice from it: "This web wallpaper uses your time zone, your
language, what your graphics card can do and data it keeps on this computer."

Storage lives in `~/.local/share/reiwand/web/<package id>/` (mode `0700`), is never synced,
and goes away with `Library1.ClearData` (`reiwand-ctl lib clear-data`, *Clear data* in the app) and
when the last item of the package is removed. The network is the `network` input of `Inputs1`,
granted only from the app, the tray or `reiwand-ctl` (`reiwand-ctl input grant <id> network`); a page
reaches only the origins it declares.

## The page interface

The helper injects `window.reiwand` before the page's own scripts (and before `web.preload`):

| Member | What it does |
|--------|--------------|
| `properties`, `onProperties(cb)` | all values when the callback is added, then only the changed ones; colors as `{hex, r, g, b, a}` (0 to 1), combos as `{value, label}`, files as `wpak-user://` tokens, folders as `wpak-user://<id>/` |
| `fps`, `quality`, `locale`, `volume`, `onSettings(cb)` | the frame rate it is drawn at (never above `playback.max_fps`), the quality level (0 to 3, from `playback.quality` or the own level of the output that shows the page, lower on battery, in power saving or with heat; properties with `quality` values change with it; outputs with different own levels run the page apart), the language picked among the package's as a valid language tag such as `es-AR` (`en-US` when the page may not see the language or none is known), and the wallpaper volume (0 to 1, the general volume times the page's own `volume` setting) for its own sound: the page may play sound without a gesture, its `<audio>` and `<video>` play at their own volume times the wallpaper's (`volume` reads what the page set) and an `AudioContext` sends its `destination` through a gain at the wallpaper's volume; the page is muted at 0 |
| `paused`, `onPause(cb)` | `true` while hidden |
| `onAudio(cb)` | `{left, right}`, 64 bands each from bass to treble, at most 30 times a second, once the user allows `audio` for the package (and `audio_network` as well when it declares network origins); the first callback subscribes |
| pointer and clicks | real DOM events: `pointermove` while `pointer` is allowed, and `click`s only from real clicks on screens where the user turned clicks on and the package holds `clicks` |
| `files(prop)`, `randomFile(prop)`, `onFiles(prop, cb)` | the tokens of a chosen folder: `files` (and the `{prop, added, removed, changed}` notices, `changed` as `[old, new]` pairs for files rewritten in place) only with `mode: all`; `randomFile` in both modes; a token changes with the size and time of its file, so a rewritten file loads anew |
| `onAction(cb)` | the id of a pressed `button` property |
| `openShortcut(prop)` | asks to open what the user chose for a `shortcut` property; it returns `false` outside a user activation, and the daemon opens it only within 2 s of a real click that reached the page, at most once a second; the page never learns the target |
| `rgb.setImage(bytes, w, h)` | RGBA bytes shrunk on the page to at most 64 × 16 and checked again by the helper (at most 64 × 16, 30 times a second); the daemon tells each view whether it may light (`permissions.rgb`, the `rgb` grant of its output and RGB lighting on, or a preview) and the call returns `false` and posts nothing otherwise; the helper drops strips of a view that may not light, and the daemon sends them only under the same conditions (`package.md`) |
| `onMedia(cb)` | what the media player plays, once the user allows `media` for a package that declares it (and `media_network` as well when it declares network origins): `{enabled, state, title, artist, album, cover, colors, position, duration, rate}`, with `state` `playing`, `paused` or `stopped`, `cover` a `blob:` address of a PNG of at most 256 pixels a side made on the page (or `null`), `colors` the `primary`, `secondary`, `tertiary`, `text` and `contrast` colors of the cover as `{hex, r, g, b, a}`, and `position` and `duration` in seconds at the moment of the call (the page runs the position on at `rate`); `{enabled: false}` otherwise. The first callback subscribes and gets the last state at once; then every callback gets each change of the track, the cover, the playback or a jump. Never a path, an address of the cover or the name of the player |

Audio and video files of the package and of the user's folder play straight from their `wpak:` or
`wpak-user:` address, which Chromium reads with range requests. The helper serves the files of the
package from sealed `memfd`s and keeps at most 64 MiB of them in a cache of its own (the oldest leave
first; a file larger than 64 MiB is not kept and is read from the package again on each request,
so a large video costs one read per seek). Files the user chose are opened on each request and never
cached. `console` output and uncaught errors reach the daemon's log at
debug level, at most 200 lines a run. The folder of a `folder` property is listed by the daemon (`max_files`, `recursive`, hidden
files and links left out, the same rules as scenes), checked again every 5 s while the page is
shown and never while hidden; the helper serves only the listed files, opened without following
links.

## On the phone

The Android app shows the same packages with the system WebView, the engine Android keeps up to
date for every app, instead of `reiwand-web`: packages never bring an engine, and the app starts no
process of its own. The package, `window.reiwand`, the privacy settings and the notice are the same;
these are the differences:

- **Drawing.** The page is laid out at the size of the wallpaper on a private virtual display whose
  output is the wallpaper's own surface, so its frames reach the screen without copies. The change
  to or from a page has no fade.
- **Addresses.** The page lives at `https://<16 hex>.wpak.invalid/`, a host made from the package id
  that never resolves, and every request goes through the app: it serves only the files the package
  lists (never `manifest.json`), the interface at `/:reiwand/api.js` (added to every HTML file, after
  its doctype) and the files the user chose as `https://<host>/:user/<property>/<token>` instead of
  `wpak-user://`. Both are opaque to the page. The content security policy is the one above with
  `'self'` in place of the schemes of the desktop.
- **Frame rate.** `requestAnimationFrame` is paced at the cap by the injected script; CSS animations
  and videos run at the rate of the display.
- **Lifecycle.** A page is visible while its screen is shown and the phone neither saves power, nor
  runs low on battery, nor is very hot; otherwise the WebView is paused (`visibilityState` is
  `"hidden"` and `onPause(true)` fires). After 5 minutes hidden it is closed and loads again when
  shown. A renderer that crashes loads the page again, up to three times. Touches reach the page
  only after its first paint.
- **Media.** `onMedia` gets the same as on the desktop from the media sessions of the phone, once
  the user allows `media` (and `media_network` for a page with network origins) and gives the app
  notification access; the cover arrives as a PNG the page turns into a `blob:` address. The app
  starts reading the players again by itself when notification access comes back, and tries again
  six times (1 s, then each wait doubled) when the system refuses to list them; taking access away
  stops the reading at once and the settings screen follows both changes.
- **Sandbox.** Layers 1 to 4 are the same, enforced by intercepting every request. Network loads
  are blocked in the WebView unless the package declares origins and the user allowed `network`,
  and only those origins pass; cookies, service workers, safe browsing and the WebView's metrics
  are off, and peer connections (WebRTC) never work, with or without the network: every response
  carries `webrtc 'block'` in its content security policy, which every frame inherits, including
  `about:blank` and `srcdoc` ones, on engines that honour it (WebView 113 does not: it reports the
  directive as unknown and ignores it), and the guard below removes them from the page, its frames,
  the frames inside shadow trees, `window.open` windows and re-opened documents, on every WebView
  the app can run on (Android 12 and later, whose WebView has `addDocumentStartJavaScript`; the
  guard also runs from the page's interface script where it does not). Nothing in the privacy
  notice depends on the engine's version. The renderer runs in the WebView's isolated process.
- **Privacy.** `hide_zone` and `hide_language` are applied by a guard script that runs before the
  page's: `Date`, `Temporal`, `Intl`, the `toLocale…` methods, `localeCompare` and
  `navigator.language` see UTC and `en-US`, also through `constructor`, when an option or the list
  of languages is left empty (`timeZone: undefined`, `[]`) or names no language the engine knows,
  in dates written without an offset (`new Date("2020-01-01T00:00")`, `Date.parse`, read as UTC),
  in `getYear` and `setYear` and in `document.lastModified`. The WebView runs the guard
  at the start of every frame of the page's origin (`addDocumentStartJavaScript`), and the guard
  also applies itself to the frames the page creates, reads through `contentWindow` or reaches
  through `window[n]`, to its workers (each starts from `/:reiwand/worker.js`, which runs the guard
  and then the page's script) and to its worklets. `hide_gpu` uses the same mask as the desktop,
  where the phone's WebView would report the model of the graphics chip. The disguise is by script
  only: what the engine draws itself (date and time pickers, the fonts it picks for a language) and
  the `Accept-Language` of requests keep following the phone. Kept storage is the
  WebView's storage of the page's origin; *Clear data* and removing the last item of the package
  move the page to a new origin and delete the old one, and a page that keeps nothing gets a new
  origin each time it loads, deleted when it closes. The notice is shown on the page of the
  package, above *Set as wallpaper*, in its *Privacy* section with one switch per item and *Clear
  data*, and as a message after importing it.
- **Inputs.** *React to touch* is the pointer (`pointermove` from the finger, as a mouse hover) and
  *React to taps* the clicks: a tap that does not move becomes one real click. With both the page
  gets the touches as they are. The lock screen gives none.
- **Sound.** The page's own sound follows the phone's sound setting for videos: `reiwand.volume` is 0,
  and the page muted, unless video sound is on; then it is the page's *Volume* setting (from the
  author's `playback.volume`, or 0 with `muted`, unless the page has a `volume` property of its own)
  times the fade of `playback.duck`. For that fade a page counts as the wallpaper's own sound only
  while one of its `audio` or `video` elements plays unmuted with a volume above 0 or one of its
  `AudioContext` runs: the page tells the app when that changes, so a silent page no longer hides
  another app playing; a player of the wallpaper that stops still counts as its own for 1 s while
  its stream goes away. `playback.max_fps` caps `settings.fps` as on the desktop.
  `reiwand.openShortcut(prop)` opens the app or `https://` address the user chose, within 2 s of a tap
  on the home screen and with the taps grant, at most once a second. `onAudio` reads the Visualizer of Android, as scenes
  do, at most 30 times a second.
- **Console.** `console` output and errors reach logcat (tag `reiwand`, `page:`), at most 200 lines a
  load.
