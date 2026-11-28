# Privacy and The page interface

On this page: privacy, the page interface.

## Privacy

A page sees, by default, the real time zone, the language of the user, what the graphics card can
do through WebGL, and keeps its own storage when it declares `web.storage_kib`. On the desktop WebGL
tells the page the model that the engine reports for the card (the debug renderer extension), as on
the phone. Each of them is
restricted per package with `Library1.Confirm` (`hide_zone`: UTC; `hide_language`: `en-US`, also in
`reiwand.locale` and the labels of combos, and the helper itself starts with that language (`--lang`, `LC_ALL`, `LANG`, `LANGUAGE`), so `Intl`, `toLocaleString` and `navigator.language` answer it in every frame and worker; `hide_gpu`: a script injected before the page's (`mask.js`) makes WebGL keep working with a
generic vendor and renderer (`WebKit`, `WebKit WebGL`) and no debug extension, and `navigator.gpu`
(WebGPU) is absent, in every frame, and workers are refused (`worker-src 'none'`) while it applies, because a worker cannot be given the mask; `hide_storage`: nothing is kept). `PackageInfo.exposes` lists what a page
sees now, and the app (details and *Apply to…*), the tray (each screen) and `reiwand-ctl` (`lib apply`
and `lib show`) show the same notice from it: "This web wallpaper uses your time zone, your
language, what your graphics card can do and data it keeps on this computer."

Inner frames. The scripts that mask the card, remove WebRTC and pace `requestAnimationFrame` run
at the start of every document, in every frame, but a new frame's window can be reached before its
own document runs them (`iframe.contentWindow` or `window[0]` right after `append`). So the same
scripts also cover every frame of the page's origin from the outside, synchronously: after every
DOM insertion (`appendChild`, `append`, `innerHTML`, `insertAdjacentHTML`, `document.write` and the
rest), whenever a frame element is asked for its `contentWindow` or `contentDocument`, and, for
frames the parser writes, before the next script of the document runs. A frame of another origin
(a sandboxed one) cannot be reached from the page and gets the scripts from its own document.

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
debug level, through the same bucket of 50 lines refilled at 10 a second (200 lines a load is the phone). The folder of a `folder` property is listed by the daemon (`max_files`, `recursive`, hidden
files and links left out, the same rules as scenes), checked again every 5 s while the page is
shown and never while hidden; the helper serves only the listed files, opened without following
links.
