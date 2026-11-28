# The rules of the model

The Rust types are in `crates/reiwand-core/src/model`: none has `serde(default)` or
`skip_serializing_if`, and every `Option` is read through `explicit::null` (a key that is present
holds a value or `null`; absent is an error). `crates/reiwand-core/prefill/model.tsv` lists each
optional field (`Type.field`) with its class: `block` (an optional block, a nested object or `null`: in a stored file it may be absent, in a reply
the daemon writes it) or `value` (a single
value that is always written, `null` for none). The test `tests/model_explicit.rs` fails when a field is
missing from the table or a default comes back. The messages that clients send are not part of this
model: they are the typed arguments of each method and the JSON of `SetSceneProperty`, `PreviewEdit`
and `scene act`, which change only what they name ("change only this"), by design. Adding a field, an enum value or a method keeps the model version;
removing, renaming or retyping something increases it. Times are Unix seconds; `Fit` is
`"fill"`, `"fit"`, `"stretch"`, `"center"`, `"tile"` or `"span"`; enums are written in snake case.
`edited` tells whether an edit applies to the shown image and `screen` repeats `SetOutputInfo`;
`mm` is `[x, y, w, h]`, where the screen sits on the canvas in millimetres (see "Edits and
spanning"), or `null` before `SetOutputInfo`. `calibrating` tells whether `ShowCalibration` shows
its pattern.
`parallax` tells whether the image moves in depth; while it does not and its edit asks for it, `still`
says why (`no_model`, `no_map`, `edited`, `building`, `building_simple`, `failed`, `cancelled` or
`flat`, see "Depth layers"), else `null`; `cancelled` follows a `CancelJob` of its `layers` job; `building_simple` is a build that uses the simple fill
because LaMa is not installed. A picture whose layers are built but that the computer draws with its
processor reports `software` until `playback.software_parallax` is set. `page_errors` lists the
errors a web page wrote since it started (its console lines that are not plain logging, the latest
50 at most); the library marks the item of that page as broken with the latest one, and `reiwand-ctl
state` prints them. `frames` counts the frames rendered for the
output since it appeared; `page_frames` counts the frames the web page of the output drew (`0` for
other wallpapers), which the frame rate cap limits. `refresh` is the refresh rate of its screen in Hz (`0` while unknown)
and `fps` the most frames per second it renders now (see "Frame rate"). `buffers` is `"dmabuf"`
or `"memfd"` while a consumer streams the output, and `""` otherwise (see "Stream buffers").
`sound` is `""`, `"off"`, `"playing"`, `"paused"` or `"elsewhere"` (see "Scene sound").
`decode` is `"zero-copy"` when the video or web page of that output reaches the GPU as a DMA-BUF
that it reads in place, `"copy"` when its frames go through system memory, and `""` for other
content (see "Video decoding" and `web.md`). `sandbox` is `""` for other content, `"on"` while a
web page runs with every layer of its sandbox, and `"reduced"` when the web engine could not start
its own sandbox or the page could not get its own empty network, so clients can warn about it.
`helper` is the process id of the web helper that draws the page of that output, and `null` for
other content or while no helper runs.
`flashes` is `""`, or `"slowed"` or `"paused"` once the guard of `playback.allow_flashing` (off) slowed or stopped the
scene of that output. `left_out` lists the package effects the scene of that output plays without,
as `the effect <id> was left out: <reason>`, and the uses the hard budget drew with their lighter
variant as `the effect <id> on <layer> was drawn with its lighter variant: <reason>`; it is a note,
not a failure. `particles` is `"gpu"`
or `"cpu"` for a scene with particles, and `""` otherwise. `quality` is the quality level the scene
of that output draws at (`"q0"` to `"q3"`, `""` for other content) and `quality_reason` why:
`"full"` (the highest), `"device"` (the most its graphics card class allows), `"cost"` (the scene is
too heavy for the card at a higher level), `"battery"`, `"saver"` (power saving), `"heat"`,
`"slow"` (its frames took too long on the GPU) or `"user"` (`playback.quality` fixed the level or
kept it above a floor); see "Cost and frame rate" in `scene.md`. `quality_choice` is the own value
of `playback.quality.<output>` or `null` when the output follows `playback.quality`;
`quality_cost` and `quality_budget` are the units a frame the scene costs at its level and the
units the GPU class holds, both `0` unless the user's choice (a fixed level or a floor) put the level
where it costs more than the class holds, in which case clients warn that frames may drop and
battery use rise. `quality_auto` says what the automatic choice does or would do with the scene:
`"still"` (the scene is too heavy for the class and stays at its `poster_time` because the
automatic choice is in charge), `"stop"` (a fixed level or a floor plays a scene that `auto` would
keep still, also when only its model triangles exceed the class and its cost fits, in which case
clients warn that it draws more model triangles than the device holds, whatever the reason of the
level), `"lower"` (`auto` would only lower the level: with the user's choice, the frames measured at that
level take more than 85 % of their interval, or, before 12 frames are measured, the level costs
more than the class holds) or `""`, also when the estimate is over budget but the measured frames
fit. `quality_frame_ms` is the average GPU time of the last frames at that level (`null` until 12
are measured, and again for a moment after each change), `quality_period_ms` the interval it is
judged against (that of the pace in use), `quality_step` the last decision of the automatic choice
(`model`, `steady`, `climbed`, `dropped`, `top`, `held`, `still`, `halved` or `doubled`, see
"Cost and frame rate" in `scene.md`; `""` for other content) and `quality_pace` its pace divisor
(`1`, or `2` while it plays at half the rate, `0` for other content); `reiwand-ctl quality` shows them
in its `MEASURED` column and the app and the tray under the level of each screen. With a measure,
clients say how long the frames take against the interval, and the rate it allows, instead of the
estimate.
Clients tell a still scene from a playing one by this value, never by `quality_reason`, which keeps
the automatic reason (or `"slow"`) when a floor does not raise the level. `tier` is
the class of the graphics card, `"t0"` to `"t3"`, and `tier_measured` whether a short measured
render set it rather than the model of the card alone. `adapters` lists the graphics cards the
daemon finds, at start and again a second after a card is plugged in or taken out, each with its
`name`, `kind` (`"integrated"`, `"dedicated"`, `"virtual"`, `"software"` or `"other"`), `backend`
(`"vulkan"` or `"gl"`), `used`, true for the one it renders with, and `bus`, its PCI bus id in lower
case (empty when the driver gives none) and `compositor`, true for the card the compositor renders
with (none while the daemon does not know it). `gpu_note` says, in the language of the daemon, that the
compositor renders with another card than the one in use, so every frame is copied through the
processor (a card forced by `render.gpu`, or the fallback), or is `null`; the app shows it under the
card picker and `reiwand-ctl gpu` prints it with a `DESKTOP` column. `gpu_warning` says, in the language of the daemon, that no card matched
`render.gpu`, or that the matching card could not be opened, and the automatic choice was taken, or
is `null`; `gpu_restart` is true while `render.gpu` differs from the value the daemon started with,
without regard to case. `qualities` maps every output that has its own `playback.quality.<output>`
in the configuration, connected or not, to that value, and `hdrs` does the same for
`playback.hdr.<output>`. `hdr` of an output tells its HDR output: `choice` (the value in use),
`own` (its own `playback.hdr.<output>` or `null`), `active` (it presents brighter than white now),
`offered` (its surface can), `display` (the desktop says the screen shows HDR), `peak_nits` and
`reference_nits` (from the desktop, or `null`), `encoding` (`"scrgb"`, `"pq"`, `"pq-own"` for the
description of its own, `"sdr"`, or empty on streams), `max_cll` and `max_fall` (the metadata of
that description, `null` otherwise) and `reason`: `"auto"`, `"on"`, `"off"`, `"display"`
(the screen, the desktop or the driver offers no HDR, with `auto` also when the desktop does not say the screen shows HDR, and always on streams), `"battery"` or
`"quality"` (no scene with `canvas.hdr` and a tone map is shown, or it draws below `q3`). `previews` holds, only for the
`preview:` outputs of the scene editor (which `outputs` leaves out) that have any, their `error`, the
`console`: the last 50 lines their scripts wrote, as `<level> <module>: <text>`, and `left_out`: the
effects the preview leaves out, as `outputs[x].left_out` gives them; other outputs have no console so
that scripts that write often never wake the bus.
`transitions` maps every output that has its own `transition.kind.<output>`, connected or not, to that value. `transition` of an output tells: `choice` (the value in use there), `own` (its own `transition.kind.<output>` or `null`), `plays` (what played at the last change of wallpaper on it: `own`, `fade` or `none`) and `why`: `"own"` (the wallpaper's transition), `"chosen"` (the plain fade or the cut that was chosen, also a playlist with `transition_ms` `0`), `"reduced"` (less motion turned it into the plain fade), `"fallen"` (the wallpaper's transition could not be built or drawn, or was too costly for the quality, and the plain fade played), `"plain"` (neither wallpaper brings a transition) or `"cut"` (no change yet, or the screen could not fade). `reiwand-ctl config transition.kind` prints one line per output with its choice and why.
