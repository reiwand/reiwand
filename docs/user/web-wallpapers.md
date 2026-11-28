# Web wallpapers

A `web` package is a page (HTML, CSS, JavaScript, WebAssembly, WebGL, images, fonts, sound and
video inside the package itself) that is shown as wallpaper with the same transitions, fit, FPS
limit and pauses as the others. It is drawn by `reiwand-web`, a separate process that the service
starts for each page that is seen and stops as soon as no screen shows it. It needs Qt WebEngine
(Qt 6.8 or later) and a graphical session, which the service already has. The AppImage and the
RPM carry their own Qt WebEngine (Qt 6.12) with the helper, so nothing has to be installed and web
wallpapers also work on Ubuntu 24.04; in the Flatpak it comes in the base
`io.qt.qtwebengine.BaseApp`, which is already included. A local build uses the Qt WebEngine of the
system it is built with (`qt6-qtwebengine` on Fedora, `qt6-webengine` on Arch); where the engine
is missing or too old the wallpaper is rejected with the reason instead of staying blank, in
`reiwand-ctl` and in the app, with the text in the interface language, and the library warns
before applying it: the card of the page marks it with the reason, its details show it and
`reiwand-ctl lib show` gives the line `not shown here`.

In the Flatpak the engine adds about 600 MB on install and the page is isolated by the Flatpak; if
it does not allow creating namespaces, Chromium's sandbox is turned off and `reiwand-ctl state`
shows `reduced` (see `docs/user/flatpak.md`).

The engine weighs more than the phone's WebView: a page at 1080p uses on the order of 400 MiB, and
while a page does not change it draws nothing. The Qt WebEngine carried by the AppImage and the
RPM is the one of conda-forge, expected to be built without the proprietary codecs: VP8,
VP9, Opus and Vorbis are expected to work, H.264 and AAC are not (untested with a real video).
`docs/specs/web.md` explains the engine, the isolation and the interface of the page, and `docs/authoring/web-wallpapers.md` is the guide for those who make
them. The fade into a page starts when the page draws its first frame (or fails), so the previous
wallpaper does not fade into an empty screen, and when leaving a page it fades from its last
frame. The FPS limit slows all the frames of the page, not only `requestAnimationFrame`: the web
engine does not draw the next one until the period passes, also for CSS animations, videos and
canvases; `reiwand-ctl state` counts those drawn by each screen (`page_frames`). On the phone the
page is drawn directly on the wallpaper surface, without a fade, and the limit only slows
`requestAnimationFrame`.

The page only reads the files of its package, does not reach the network unless it declares it and
you allow it (`reiwand-ctl input grant <id> network`, or the switch in its details in the app),
does not see paths, devices or the battery, and the web engine runs it in its own isolated
environment. By default it sees your time zone, your language, what your graphics card can do
(WebGL) and stores its data in `~/.local/share/reiwand/web/<id>/` if it declares so. When applying
it and in its settings, the app, the tray and `reiwand-ctl` announce what it uses, and you can
hide each thing per wallpaper: in the package details in the app (*Privacy*) or with
`reiwand-ctl lib confirm hide_zone|hide_language|hide_gpu|hide_storage ITEM` (`lib unconfirm`
undoes it). *Clear data* in the app or `reiwand-ctl lib clear-data ITEM` deletes what it stored,
and it is deleted by itself when the package is removed. If what it stores exceeds twice what it
declares plus 1 MiB, the page starts without storing anything and its error says so until its data
is deleted; on the desktop this is only checked when the page starts, because the web engine does
not allow setting a limit per origin. The sound, pointer and click permissions are the same as
those of scenes: clicks only arrive from real clicks on the screens where you turn them on. A
hidden page stops drawing at once and, after 5 minutes hidden, its process is stopped and its last
frame remains. The *Lights* example (`scenes/lights`) is a web page.

On the phone the same packages are shown with the system WebView, the engine that Android keeps up
to date for all apps, drawn directly over the wallpaper without copies. The page sees the same
interface and has the same restrictions: it only reads its files and those you choose (the app
serves each request), does not reach the network unless it declares it and you allow it, has no
cookies, windows, dialogs, downloads or permissions, and cannot open peer-to-peer connections
(WebRTC) even in its frames: a script that runs before its own removes them from the page, its
frames (also `about:blank`, `srcdoc` and those of shadow trees), windows and reopened documents,
and the content security policy carries `webrtc 'block'` for the engines that honor it (WebView
113 does not). Its page in the app warns, above *Set as wallpaper*, of what it uses (time zone,
language, model of the graphics chip and data it stores on the phone), and its *Privacy* section
has a switch for each thing, another for the network if it declares it and *Clear data*; the same
notice appears on importing it. The limit on what it stores is the desktop one: the phone measures
it every time the page starts (what the WebView reports for its origin plus its `localStorage`)
and, while it runs, rejects with `QuotaExceededError` what `localStorage` tries to store beyond
that (with `setItem` or as a property, in the page and in its frames); after writing to IndexedDB
or Cache Storage, and when it stops being seen, it measures the origin again, and if it exceeds
the limit the page starts again without storing anything. If on starting it already exceeds the
limit, the page runs without storing anything and its *Privacy* section says so until *Clear
data*. Hiding the time zone and the language is done by a script that runs before those of the
page (in it, its frames and its workers); it does not cover what the engine itself draws (date
and time pickers, typefaces according to the language) or the `Accept-Language` of requests,
which follow the phone. *React to touch* gives it the pointer and *React to taps* the clicks, only
from real touches and only on the home screen; its sound follows the sound setting of videos. It
pauses when it stops being seen, with power saving, with low battery or with the phone very hot,
is closed after 5 minutes hidden and `requestAnimationFrame` runs at the phone's frames per second
at most; the change to a page or from it has no fade. On an emulator, *Lights* draws 30 frames per
second with the limit at 30, a still page uses nothing after its first frame and a hidden page
neither draws nor uses the processor.
