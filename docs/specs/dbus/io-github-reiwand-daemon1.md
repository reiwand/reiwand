# `io.github.reiwand.Daemon1`

## `RequestStream(s output, u width, u height) → (u node_id, t serial)`

Adds a claim of the caller on `output` and creates or reuses the PipeWire video stream at
`width` × `height` physical pixels. Returns the node id and `object.serial` of the stream node.
Consumers should connect by serial; `serial` is `0` only if PipeWire did not publish one within
2 s, in which case the node id can be used.

- `output`: connector name such as `DP-1`.
- A request with the same size returns the existing stream.
- A request with a different size replaces the stream; the old serial becomes invalid.
- Each edge must be in `1..=min(16384, GPU limit)` and one frame, with rows padded to 256
  bytes, must fit in 256 MiB and in the GPU buffer limit (for example 8192 × 8192); otherwise
  `InvalidArgs`.
- A stream is never larger than the screen: when the daemon knows the pixels of the output (the
  current mode of the `wl_output` with that name, turned with the output, while the daemon is
  connected to the Wayland compositor, which it reconnects to when the compositor restarts;
  otherwise `width` × `scale` from `SetOutputInfo`, which only reduces a request that was not
  built from that same scale), a larger request is reduced to them keeping its aspect ratio, and
  `GetState` shows the size used. When the pixels of the screen change, the stream follows them
  again, up to the size requested. A later request of the same size returns that stream.
- Call `ReleaseStream` once for every successful `RequestStream`.

Stream format: raw video, `BGRx` (or `BGRA`), fixed size, variable frame rate, buffers in GPU
memory (DMA-BUF) or one memfd per buffer mapped from offset 0 (see "Stream buffers"). Frames are
produced when the picture changes (new wallpaper, transition, video frame, a scene that animates or
whose inputs changed), when a consumer starts streaming or another consumer links to the stream,
again 50 ms, 250 ms, 1.25 s and 6.25 s later (so a consumer that was not ready for the first frame,
or a link that was still being set up, gets the last picture), and every 10 s as a keep-alive.
While no consumer is streaming an output, its video is paused and the daemon renders only the final
picture after each change, which a consumer receives as soon as it connects.

## `HoldStream(s output, u width, u height) → (u node_id, t serial)`

Same as `RequestStream`, but with the held claim. Used by `reiwand-ctl stream` for tests and
tools that exit right after the call.

## `ReleaseStream(s output)`

Removes one claim of the caller on `output`, or the held claim if the caller has none. When the
last claim of a `preview:*` output goes, the settings applied to it (such as a wallpaper shown there
with `Library1.Apply`) are forgotten too. Settings of `preview:*` outputs live only in memory while
they are claimed: they never reach the saved configuration file.

## `SetOutputCovered(s output, b covered)`

Reported by the desktop integration when a maximized or fullscreen window covers `output`.
See "Streams and claims".

## `SetOutputInfo(s output, i x, i y, u width, u height, d scale, d px_per_mm, s identity)`

Tells the daemon where the screen of `output` sits, for the `span` fit and the per-screen crops of
edits (see "Edits and spanning"). `x`, `y`, `width` and `height` are its logical position and
size in the desktop layout, `scale` its device pixel ratio (`0.1` to `16`), `px_per_mm` its
density in physical pixels per millimetre (`0` when unknown; a density that is not a number or is
outside `0` to `1000` counts as unknown and the rest is kept) and `identity` the maker, model and
serial number of the monitor (at most 256 bytes, no control characters). The desktop
integration calls it after every successful `RequestStream`; layer-shell outputs take the same
values from `wl_output` and `xdg_output`, and X11 outputs from RandR. `NotFound` when `output`
has no stream. When the screen has fewer pixels than the stream, or more pixels than a stream
that was reduced, the daemon creates the stream again at the size of the screen (never above the
size requested) and emits `StreamChanged`.

## `PreviewEdit(s path, s edit_json)`

Shows the image `path` with `edit_json`, an edit as described in "Edits and spanning", instead
of its saved edit wherever `path` is shown, without saving anything. Calls replace the preview
and are rendered at most once per frame; an empty `edit_json` ends it, and so do 60 s without a
call. `path` is absolute; a relative path, an invalid edit (the message names the field) or more
than 2 MiB of JSON return `InvalidArgs`, a missing file `NotFound`. While a preview lasts the
daemon keeps the decoded image, so a new call only renders the edit again; at most 4 images are
previewed at once and a fifth one ends the preview that was called the longest ago.

## `ShowCalibration(b on)`

Shows on every output that has its place from `SetOutputInfo` (and is not a `preview:` output)
a pattern drawn over the canvas in millimetres of "Edits and spanning", instead of its wallpaper:
lines at 45° and circles every 100 mm around the centre of the canvas, a horizontal and a
vertical line through it and an orange frame along the edges of each screen. Where screens are
placed right, the lines continue straight from one screen to the next behind their frames; a
screen is moved with `SetOption("layout.monitors.<identity or output>", "x,y,w,h")` and the
frames with `layout.bezel_mm`, and the pattern follows at once. `false` shows the wallpapers
again, and so do 10 minutes after the last call with `true`. `GetState().calibrating` tells
whether it is shown; `reiwand-ctl calibrate on|off` and the *Screens* section of the image editor
call it.

## `PreviewScene(s output, s dir, s document, d time)`

Shows a scene that is being edited on `output`, or on every output when `output` is empty,
instead of what the output shows, without saving or applying anything (see `editor.md`).
`dir` is the absolute path of the scene folder, whose files provide the assets, and `document`
is `{"manifest": …, "scene": …}`, the unsaved manifest and `scene.json`. For a web page, whose
manifest `kind` is `web`, `document` is `{"manifest": …, "preview": …}` and `dir` is the folder of
the page: the daemon packs it into a temporary package with that manifest (without changing the
folder) and shows it like a library page; `preview.quality` (`q0` to `q2`) runs it at that level,
`preview.fps` caps its frames, a new `preview.restart` builds it again, and `preview.media` is a
made-up track for `window.reiwand.onMedia`. For a scene or a page, `preview.values` is a map of user
property values (property id to JSON value) that the preview uses instead of the values saved for the
package: properties it leaves out take their defaults, and an unknown or invalid value is treated as
in the saved settings. Nothing of it is saved, and a call with another map shows the package again
without building it again. The lines the page writes to `console` and its errors are
in `GetState().previews[x].console` of a `preview:` output. A `time` of 0 or more
made-up track for `window.reiwand.onMedia`. For a scene, `preview.quality` caps its level the same way and `preview.shift` set to
`true` draws the displacement of refracting particles instead of bending what is behind them (see
`editor.md`). The lines the page writes to `console` and its errors are
holds the scene at that second (up to 86400, a day), as a still frame that only renders again when the call changes;
a negative `time` plays it. The scene is checked for what it needs to render: the id, the user
properties, the presets and the scene itself, while texts, rating, origin and credits may still be
incomplete; an invalid document, a path outside `dir` or an asset over the limits return
`InvalidArgs`. Calls replace the preview of the
output and reuse the decoded assets of the previous one, without a crossfade; a playing preview
of the same folder keeps its clock, so an edit does not restart it. A preview document may carry
`preview: { quality, restart, media, battery, calm, time_of_day }` (`editor.md`); `battery` and `calm` make the output run as on
battery and with reduced motion, and `time_of_day` (seconds from midnight, 0 to 86 399) starts the scene's wall clock at
that hour of today and runs it on from there, for that preview only. An empty `document` ends it,
and so do 60 s without a call or the output going away. Outputs whose name starts with
`preview:` are virtual: a client creates one with `RequestStream` to show the preview inside its
own window, and it is left out of `GetState` (but for the time its console has lines, which is how a client reads them), of the conditions and holds of profiles, of
playlists and of the record of what was shown, so it never counts as a screen.
A `preview:` output whose name starts with `preview:card-` is a library card: whatever is shown
there plays without sound (no audio output is opened for its sound layers or videos), while the
same wallpaper on a screen keeps its own volume.

## `SetWallpaper(s path, s output, s fit)`

Sets the wallpaper of `output`, or of every output when `output` is empty (this also clears
per-output wallpapers). `path` is an image or a video (`mp4`, `m4v`, `webm`, `mkv`, `mov`,
`avi`, `ogv`, `mpg`, `mpeg`); videos loop and are decoded with GStreamer. `fit` is `fill`,
`fit`, `stretch`, `center`, `tile`, `span`, or empty to keep the current mode. Only local files are
accepted: an absolute path or a `file://` URI with an empty or `localhost` host, which the
daemon percent-decodes; any other URI returns `InvalidArgs`. Clients should still convert URIs
to paths themselves, as the other methods reject URIs. A `.wpak` package returns `InvalidArgs`
(`import the package and use Library1.Apply`). Images are decoded before the configuration is
saved. A new wallpaper replaces the old one with the configured transition.

A non-empty `path` also stops the playlist of `output` (of every output when `output` is
empty) and clears the `playlist` and `preset` of the affected configuration sections. If `path`
is a library item, `GetState().outputs[x].item` reports its uid. No history is recorded.

With an empty `path`, only the fit changes and the current wallpapers stay: the fit of
`output`, or of every output when `output` is empty. A fit for every output also clears
per-output fits, with or without a path. An empty `path` and an empty `fit` return
`InvalidArgs`.
