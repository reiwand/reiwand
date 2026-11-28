# `io.github.reiwand.Inputs1`

Scenes may follow the sound that plays and the pointer, and take clicks and taps. Every input is off
for every scene until the user grants it to that scene; grants are kept in `~/.local/share/reiwand/inputs.json`
(mode `0600`) per scene package id and per package: the signing key of a package installed from a
repository (`key:<fingerprint>`, so its updates keep the grant), the SHA-256 of the file of any other
package (`sha256:<hex>`) or, for a scene previewed from a folder, the SHA-256 of the folder path
(`folder:<hex>`) when the app or `reiwand-ctl` (identified as for `Grant`) asked for the preview, so
that edits keep the grant, and the SHA-256 of the folder path, the document and the script sources
(`draft:<hex>`) when another caller did. Another package with the same id never inherits a grant. Grants can be revoked at any time. A scene
never follows the sound it plays itself: sound layers cannot take their values from the analysis. A
still image whose depth parallax has `source` `pointer` (see "Edits and spanning") also uses the
pointer: choosing that source for that image is the user's consent, so it needs no grant, and it
follows the same rules as a scene, without clicks.

An input is on only while an output shows a scene that declares it and holds the grant (or a
pointer parallax image that moves: not under reduced motion, nor with software rendering unless
`playback.software_parallax` is set), the screen is unlocked (whatever `pause.when_locked`
says) and that output is active (not paused, and not covered while `pause.when_covered` holds);
both inputs also stop while the output is covered, whatever `pause.when_covered` says, and while
a streamed output has no consumer (in Plasma, while the desktop is not shown). Revoking,
locking, pausing, covering or hiding the scene or image stops it at once, and showing it again
starts it again. `playback.scene_capture = false` turns audio off for every scene, granted or not.

Web pages (`web.md`) take the same `audio`, `pointer` and `clicks` grants; `network` is a fourth
input that only web pages that list origins in `permissions.network.allow` declare. Only the app,
the tray and `reiwand-ctl` grant it (`AccessDenied` for other callers, which cannot ask the user for
it), and granting or revoking it starts the shown page again with or without those origins.
`rgb` is a fifth input, declared by packages that set `permissions.rgb` (`package.md`): a scene with
an `adjustment` layer that sets `rgb: true` or a web page that calls `window.reiwand.rgb.setImage`.
Only the app, the tray and `reiwand-ctl` grant it, as `network`; the colors reach the lights only while
`rgb.enabled` is on (see `Rgb1`).

- Audio: the daemon captures the monitor of the default output device through a PipeWire
  stream named "Reiwand audio analysis" (desktop microphone indicators may show it) and
  turns it into at most 64 spectrum bands per channel, levels and beats on the render thread.
  Samples stay in memory only for that analysis: they are never stored, logged or sent anywhere,
  and scenes and scripts only get the bands and levels. A scene without the grant gets silence
  even while another output's scene is listening. The stream is destroyed when no scene needs it.
  When the PipeWire connection breaks (PipeWire restarts), the capture connects again by itself
  after 0.5 s, then twice as long each time up to 30 s; nothing wakes it while it is connected.
- Pointer: on layer-shell outputs only the background of an output whose scene or parallax image
  uses the pointer, or whose scene takes clicks, accepts pointer and touch events, showing the
  default cursor, so other backgrounds never take desktop clicks; the render thread applies that
  input region with its next commit, so it never commits the surface in the middle of a frame. On
  X11 the desktop window of such an output takes the pointer the same way (its input shape and
  its pointer events are on only then), and events that another program sent (`SendEvent`) are
  ignored. In Plasma the daemon loads a KWin script that reports `workspace.cursorPos` with
  `SetPointer`, at most 60 times per second and only while the pointer moves, and unloads it when
  no scene or parallax image needs the pointer. Keys are never read, and positions are neither
  kept nor logged: each output only has the latest one.
- Media: what a media player plays (`media` values, covers, `media:` triggers and scripts that
  declare it). The daemon follows the players on the session bus (`org.mpris.MediaPlayer2.*`
  names, their `PropertiesChanged` and `Seeked` signals, never polling) only while an output that
  is active, shown and not covered shows a package that declares `media` and holds the grant; it
  stops listening as soon as none does. It chooses the player that plays, the one that started
  playing last when several do, otherwise the one that changed last. Titles, artists, albums,
  genres, the length and the cover address are read only to be shown: they are never kept, logged
  or sent anywhere. A package without the grant receives empty texts and its placeholders.
- Clicks: presses and releases of the left, right, middle, back and forward buttons and taps on a
  touch screen (a tap is a press of the left button). They reach a scene only on an output that
  takes clicks (`SetClicks`, off for every output by default), when the scene declares them
  (`clicks:` triggers, pointer events of scripts on `solid` layers, effects that use the buttons)
  and its package holds the `clicks` grant. They come from the compositor on layer-shell and X11
  and on Plasma from the wallpaper plugin through `Touch`, which only the Plasma shell may call, so
  other programs cannot send clicks; code that runs inside the Plasma shell could, and the daemon
  bounds it as `Touch` says. At most 16 presses wait for the next frame of each output and they are
  dropped when the output stops taking clicks. Opening what the user chose for a `shortcut`
  property only ever happens inside one of these clicks.

## `Grant(s scene, s input)`

Lets the scene or web package with id `scene` that an output shows now use `input`, `audio`, `pointer`, `clicks`, `network`, `rgb`, `media`, `media_network` or `audio_network`,
whenever it is shown. `media_network` is the second confirmation that a web package which declares
network origins needs besides `media`: without both, `onMedia` gets `{enabled: false}` and the media
players are not read for it, since the title, artist and cover could leave the computer through
those origins; like `network` and `rgb`, only the app, the tray and `reiwand-ctl` give it, and the app
and the tray ask for it only after `media` was allowed. `audio_network` is the same second
confirmation for `audio`: a web package that declares network origins and `audio` hears nothing
(`onAudio` never fires and the sound is not analysed for it) until both are granted, because what
it hears, the sound that plays or the microphone, could leave the computer through those origins;
only the app, the tray and `reiwand-ctl` give it, after `audio` was allowed. A package that already has the input gets it again at once, from any caller
and without asking. `InvalidArgs` for another input or a malformed id (1 to 255 characters of
`a-z`, `0-9`, `.`, `_`, `-`) and while the outputs show more than one package with that id (other
versions, signers or files), which `GrantPackage` tells apart; `NotFound` while no output shows a
scene with that id. Granting twice changes nothing.

Only the app, the tray and `reiwand-ctl` grant at once: the daemon reads the executable of the calling
process (from the process id that the bus gives) and accepts the `reiwand-ctl` and `reiwand-app` next to
its own executable and the Plasma shell that runs the tray, a system file found on `PATH`. For any
other caller, and for every caller of the Flatpak, where the daemon cannot see the processes outside
its sandbox, it asks the user in a desktop notification (`org.freedesktop.Notifications`) with
*Allow* and *Not now*, returns `Unconfirmed` (`the user is asked in a notification; the input is
granted once allowed`) and applies the answer when it comes to the package the notification named:
*Allow* grants it while an output still shows it, *Not now* records the decline as `Revoke` does
but takes the input back only from the package the notification named, never from another package
with the same id, and a notification closed or left unanswered for two minutes changes nothing. The
notification carries the icon and desktop entry `io.github.reiwand` and asks whether to let
the scene, named by its title, use the input, whoever asked. `Changed("inputs")` follows when a
question opens, when it ends (answered, closed or unanswered) and when its quiet time ends, and
`GetInputs` lists the open questions in `asking` and the resting ones in `quiet`. Only the notification
service can answer, and one question per scene and input is open at a time; after *Not now*, a
closed notification or no answer, the same question is not asked again for ten minutes, and callers
get `AccessDenied` meanwhile. An input the user declined is asked for again, since only an explicit
`Grant` asks, and *Allow* clears the decline. A caller gets `AccessDenied` when no notification
service can ask.

## `GrantPackage(s scene, s key, s input)`

`Grant` for the package with id `scene` bound to `key`, the `key` of an entry of `shown` in
`GetInputs`: another package with the same id, shown on another output or applied after the client
read `shown`, never gets the grant. Callers are treated as for `Grant`. `NotFound` while no output
shows that package.
