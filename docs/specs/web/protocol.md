# Protocol, Frame clock, Console of a preview and Sandbox

On this page: protocol, frame clock, console of a preview, sandbox.

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

## Frame clock (`scene render`)

`reiwand-ctl scene render` also takes a web package, built (`.wpak`) or as its folder (built into a
temporary file first). It starts the helper itself, with no daemon and no screen, and draws the page
at fixed times (`apps/reiwand-ctl/src/webrender.rs`). `Start::clock` (the wall clock at the first
frame, in seconds; the computer's clock or `--time-of-day`) turns on a frame clock in the page
(`apps/reiwand-web/src/clock.js`, injected at document creation before every other script of the
package): time stands still until the host sends `Step { view, at }`. A step runs, in order of their
due time, every timer due by `at` (`setTimeout`, `setInterval`, each followed by a turn of the real
event loop so promises settle), then every animation frame callback with `at` in milliseconds, then
pauses every animation of the document (CSS animations and transitions, Web Animations) at `at`
less the step at which it was first seen; after two real animation frames the page answers
`Stepped`, the host lets the drawn frame reach the view and asks for a `Shot`. `performance.now()`
is `at` in milliseconds, `Date.now()` and `new Date()` are the wall clock plus `at`. Frames of the
same origin follow the animation frames of the page (as they always do); their own timers and clocks
stay real.

Deterministic: everything the page computes from those clocks, timers, animation frames and
document animations; the same package, options and `--time-of-day` give the same bytes (240 frames
at 1920×1080 checked twice). Not deterministic: video and sound elements, workers, network and
file loads, event timestamps, and anything that measures real time some other way; `Math.random`
is the page's. The page loads on the real clock with its time at 0, and the first frame is drawn
once it has loaded (30 s at most). The page always plays from second 0: with `--time S` the frames
before S are stepped and not written (at most 600 s). It renders in software (with WebGL: only for this render the helper adds `--ignore-gpu-blocklist --disable-gpu-compositing`, since the engine refuses WebGL on software GL and a WebGL canvas composited on the GPU there comes out white; a wallpaper never gets them), without network, with
the default values of its properties at the level of `--quality`, `--language` or the computer's
languages, at `--screen` (or 16:9 at `--size` on its long side) and `--fps` as the page's frame rate.
`--pointer X,Y[:X,Y]` moves the pointer before each step (fractions of the page, from before the
first frame), `--click` presses over the middle of the clip. `--media`, `--audio`, `--battery`,
`--calm`, `--against` and `--without-bloom` are refused for a web package.

## Console of a preview

A page shown for the editor on a `preview:` output (`PreviewScene`, `dbus.md`) keeps its last 50
console lines and errors per screen, which the helper sends as `Console` with the view that wrote
them, and `GetState` shows them in `console` of that output; every screen keeps its last 50 lines, but `GetState` shows them only for previews; the other screens expose just their errors (`page_errors`), so a page
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
   `getUserMedia`, `getDisplayMedia` and `enumerateDevices` reject), in every frame including `about:blank` and `srcdoc` ones, from the moment the page can reach the frame (see Privacy); besides, every response carries `webrtc 'block'` and the engine runs with the peer-connection policy `disable_non_proxied_udp`, so no UDP or STUN leaves the helper even if a script got round the removal, with or without `network`; keys never reach the page.
5. Unless the package declares origins and the user granted `network`, the helper starts in its own
   user and network namespaces with no network at all, so not even the network service of
   Chromium can reach anything.
6. Chromium runs the page in its renderer process inside its own user namespace and seccomp
   sandbox; the memory of everything is limited by the scope of the helper (see Lifecycle).

Limit: a `fetch` of a file that is not in the package answers with a network error because the helper redirects it to a blocked address; Qt WebEngine never settles a `fetch` when the request job is failed with `fail`, so the failure cannot be reported with a proper error code.

`OutputState.sandbox` is `"on"` when every layer is in place and `"reduced"` when the helper
could not make the namespaces and started Qt WebEngine with its sandbox off
(`QTWEBENGINE_DISABLE_SANDBOX`; some containers and the Flatpak forbid them), or when the daemon could not make the empty network namespace; with the network allowed that namespace does not exist by design and `sandbox` stays `on`. Clients show
it next to the privacy notice. `reiwand-ctl` prints the same notice, on standard error, after `lib apply`,
`playlist start`, `next` and `prev` (for the screens they touch) and in `state` when run in a terminal.

`reiwand-ctl scene thumbnail DIR --capture` takes the thumbnail or the animated preview of a page from the page the daemon shows: `CapturePage` (`dbus.md`) sends `Shot` to the helper, which copies the last frame it drew, even while the view is hidden, and sends it as pixels. No second engine starts, so the sandbox and the network of the capture are those of the wallpaper.
