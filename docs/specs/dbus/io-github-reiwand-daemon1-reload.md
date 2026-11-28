# `io.github.reiwand.Daemon1`: `Reload` to Signal `JobFinished`

On this page: `reload`, `restart`, `retryeffects`, `retrylayers`, `getstate →`, `getschema →`, `setpaused`, `setvolume`, `setmuted`, `setprivatemode`, `waitchanged →`, `listjobs →`, `getjob →`, `canceljob`, property `paused`, property `volume`, property `muted`, property `privatemode`, property `serial`, property `model`, signal `streamchanged`, signal `changed`, signal `jobprogress`, signal `jobfinished`.

## `Reload()`

Re-reads the configuration file and applies it. If the file is invalid, the current
configuration stays, `config_error` is set and the call returns `Failed`.

## `Restart()`

Starts the daemon again, to take a new `render.gpu` or `surfaces.layer_namespace`. Under systemd
(`INVOCATION_ID` set) it asks systemd to restart its own unit (`GetUnitByPID`, then `Restart` with
`replace`, only when the daemon is the main process of that unit), so the unit stays in charge; otherwise, or when systemd refuses, it replies, stops as on
`SIGTERM` (the RGB devices get their own mode back) and runs its own program again in the same
process with the same arguments and environment. Clients see the bus name go and come back.
`reiwand-ctl restart` waits for the new daemon and prints the card it renders with; the app shows
*Restart the daemon now* next to the notices of both settings. The renderer is made once and shared
by every output, video, stream and DMA-BUF export, and a lost device stops the process, so a new card
needs this restart rather than a switch in place.

## `RetryEffects()`

Draws again the package effects left out because they stopped the GPU or the program while they
were checked, built or drawn, took too long to build, were refused by the driver or took more GPU
time a frame than the device allows (see "Package effects" in `scene.md`): it forgets the strikes in
`stopped-effects` in the cache folder and builds again every scene that left an effect out, so those
effects are built and measured again.
Library packages whose `error` says their shader is not checked again, or could not be checked
because the helper that checks it crashed or ran out of memory or time, are read again, and
`Changed` names them under the `library` topic. `reiwand-ctl retry-effects` and the Try again button
next to a left-out effect in the app and in the Plasma tray applet call it.

## `RetryLayers(s path)`

Builds again the depth layers of the picture at `path` (a path or a `file://` URI) after a build or
a load failed or the person cancelled its `layers` job: it deletes the `.layers.failed` marks of
that picture in the cache, forgets the failures and the cancellation the daemon keeps in memory
and reconfigures the screens, so a screen that shows the picture with parallax on starts a new
`layers` job. A picture that is on no screen only loses its marks. `reiwand-ctl retry-layers PATH`
and Try again in the menu of the picture's cover in the app call it.

## `GetState() → (s json)`

A `State` document (see "JSON model"). It never waits for the library: the library item, preset,
playlist and credit of each output are kept up to date when they change. `compute` says whether
the graphics card runs compute shaders in hardware, and each output's `accent` is the accent color
(`#rrggbb`) its wallpaper suggests: `display.accent` of its package (with the current value of a
color property it names), else the dominant color of the preview or picture, or `null`; the Plasma
wallpaper plugin gives it to Plasma as the accent color of the wallpaper. `filter` is the filter of
the output (`SetFilter`), or `null` when it shows its pictures as they are. `tones` are the tones
of the interface that its wallpaper gives (`night`, `night_high`, `night_line`, `mist`,
`mist_low`, `moon`, `moon_deep` as `#rrggbb`, and `veil` from 0.62 to 0.96), derived once with the
palette module from the picture itself or from the cached 256 px preview of the item, when the
wallpaper of the output changes and when that preview becomes ready; they are `null` while there is
no picture to derive from, and then a surface outside the app (the tray) uses the fixed tones.
`reiwand-ctl state --json` shows them.

## `GetSchema() → (s json)`

A `Schema` document describing every value a client would otherwise hard-code: the keys of
`SetOption`, `Playlists1.SetRule`, `Profiles1.SetAuto` and `Profiles1.SavePreset`, the provider
kinds and their options, the search keys, the thumbnail sizes and the fits. Each field carries
`label`, always written: `null` for the daemon's own keys, which clients translate with their own
strings and fall back to the key, and the label the plugin declares for a setting of a `plugin:<id>`
provider kind, which clients show as it is. An `int` field lists in `values` the words it also
takes, such as `display` for `playback.max_fps`. Its provider kinds change when a provider plugin is
installed or removed, announced with the topic `providers`; nothing else changes while the daemon
runs.

## `SetPaused(b paused)`

Pauses or resumes every animated wallpaper.

## `SetVolume(d volume)`

Sets the volume of video wallpapers and scenes with sound, `0.0` to `1.0`. Their sound goes to
PipeWire through its PulseAudio interface (`pulsesink`, stream `reiwand` with `media.role = Movie`,
`latency-time` from the 20 ms cycle) when a PulseAudio server is reachable (`PULSE_SERVER` or
`$XDG_RUNTIME_DIR/pulse/native`; the PipeWire plugin can deadlock its own stop against its loop
thread), otherwise directly (`pipewiresink`, same stream) when the
GStreamer PipeWire plugin 1.2 or newer is installed; without it, with an older one (whose buffer
pool can deadlock), or when PipeWire cannot be reached outside the Flatpak, it goes to the default
audio output of GStreamer. While PipeWire has no audio output (and in the Flatpak while PipeWire
cannot be reached) the video plays without sound and its sound goes back to PipeWire, at the same
position, once an audio output appears. Videos with sound loop by seeking back at their end.

## `SetMuted(b muted)`

Mutes or unmutes video wallpapers and scenes with sound (see "Scene sound"). While the screen is
locked they stay silent whatever this says.

## `SetPrivateMode(b on)`

Turns private mode on or off; the same as `SetOption("network.private", …)`. While it is on the
program sends nothing to any network and looks no name up: every network client of the daemon
is built by one gate (`reiwand_core::net`) that refuses before it resolves a name or connects, with
the error `private mode is on: the network is cut …`. It cuts the sources, the repositories and
their downloads, covers from the internet, the accounts and publishing to servers, and OpenRGB
(also on this computer, as it speaks over TCP). Web wallpapers are reloaded without the network
they were given, and their helper runs in a namespace with no network. Everything on the computer
keeps working. It is off by default, is kept in the configuration and is read when the daemon
starts, before any network task. `reiwand-ctl`, the app and the editor read the same setting
before their own requests (publishing, signing against a server, model downloads) and refuse with
the same sentence. `reiwand-ctl private on|off|status`.

## `WaitChanged(t serial, u timeout_ms) → (t serial, as topics)`

Long poll for scripts; see "Change notification". `timeout_ms` is clamped to `0..=20000`.

## `ListJobs() → (s json)`

A `JobsReply`: queued and running jobs plus the jobs that finished in the last 10 minutes (at
most 64 of them).

## `GetJob(t job) → (s json)`

A `JobReply`. `NotFound` for a job that is unknown or finished too long ago.

## `CancelJob(t job)`

Cancels a job; see "Jobs". `NotFound` for an id this daemon run never issued.

## Property `Paused` (`b`)

Paused by the user. `PropertiesChanged` is emitted whenever it changes.

## Property `Volume` (`d`)

Volume of video wallpapers and scenes. `PropertiesChanged` is emitted whenever it changes, whichever
method changed it (including `SetOption` and `Reload`).

## Property `Muted` (`b`)

Video wallpapers and scenes are muted. `PropertiesChanged` is emitted whenever it changes. It
does not follow the screen lock, which silences them too.

## Property `PrivateMode` (`b`)

Private mode is on (`SetPrivateMode`). `PropertiesChanged` is emitted whenever it changes,
whichever method changed it (including `SetOption` and `Reload`).

## Property `Serial` (`t`)

Change counter; see "Change notification". No `PropertiesChanged` is emitted for it.

## Property `Model` (`u`)

Version of the JSON data model of the replies; constant.

## Signal `StreamChanged(s output, u node_id, t serial)`

Emitted for every output with a stream after the daemon reconnects to a restarted PipeWire, as
soon as the stream of `output` exists again under a new node. The claims stay valid, but
consumers lose their own PipeWire connection and must connect again to `serial` (or `node_id`
when `serial` is `0`).

## Signal `Changed(s topic, as ids, t serial)`

Something of `topic` changed; `serial` is the new value of `Serial`. See "Change notification".

## Signal `JobProgress(t job, t done, t total)`

Progress of a running job, at most 4 times per second per job; `total = 0` means unknown.

## Signal `JobFinished(t job, s state, s error_name, s message)`

A job ended. `state` is `done`, `failed` or `cancelled`; `error_name` and `message` are empty
unless the job failed. A cancelled job has `message = "cancelled"`. A failed job never has an
empty `message` (see "Errors"); the app shows that reason, and an empty message from an older
daemon, in the user's language, and `reiwand-ctl` prints `failed without giving a reason`.
