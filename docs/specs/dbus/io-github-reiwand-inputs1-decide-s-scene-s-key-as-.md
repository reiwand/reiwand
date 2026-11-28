# `io.github.reiwand.Inputs1`: `Decide` to `Bands →`

On this page: `decide`, `revoke`, `getinputs →`, `setclicks`, `simulatemedia`, `simulatetouch`, `simulatetrigger`, `bands →`.

## `Decide(s scene, s key, as allow, as deny)`

The answer to the one question a scene asks: every input in `allow` is granted and every input in
`deny` is declined, as `GrantPackage` and `Revoke` do for the package with id `scene` bound to `key`
(`""` while one package has that id). From the app, the tray and `reiwand-ctl` both lists apply at
once. From any other caller, and from every caller in the Flatpak, where the daemon cannot tell
who calls, `deny` still applies at once, and the inputs of `allow` go to the user in one desktop
notification for all of them, as `GrantPackage` does for such a caller: the call returns
`Unconfirmed` and the answer given in the notification is applied when it comes. `network`, `rgb`,
`media_network` and `audio_network` in `allow` stay refused for such a caller (`AccessDenied`, `only
the app, the tray and reiwand-ctl give wallpapers the network or lights`), the other inputs of `allow`
are still asked; without a notification service the call returns `AccessDenied` (`... nothing can
ask the user`). The app closes its question on `Unconfirmed` and says to confirm it in the desktop
notification. `InvalidArgs` for an unknown input, an input in both lists or a malformed
id; nothing is changed then. Inputs left out of both lists keep their state. The inputs in `deny`
stay off, the scene runs without them and `shown` lists them in `without`; the ones the user has not
answered yet are in `undecided`. A scene that asks for several inputs at once gets one notification
for all of them, with *Allow* and *Not now* for each input, and its answer is applied as `Decide`
does; the scene starts meanwhile without what is not allowed.

## `Revoke(s scene, s input)`

Takes `input` back from `scene` and stops it at once if that scene was using it. It also records
that the user declined `input` for `scene`, in `inputs-declined.json` next to `inputs.json` (mode
`0600`), so that clients stop asking; `Grant` and `GrantPackage` clear that record.

When `scene` is the full path of a picture listed in `grants` and `input` is `pointer`, it turns
the parallax of that picture off in its edit, keeping its values and recording the step in its
history, as the switch of the app does; turning that switch on again asks nothing.

Clients ask once per wallpaper, scene or web page, for all its inputs together (`Decide`), and every text speaks of the wallpaper: when `shown` lists an input that is neither in its `granted` nor
declined for the scene, whether the scene was applied from a client, a playlist or `reiwand-ctl`, the app and
the tray explain what is captured and that nothing is stored or sent, and offer *Allow*
(`GrantPackage` with the `key` of that output) or *Not now* (`Revoke`); while the notification
asks (`asking`), they say so instead of showing an error, and while the question rests (`quiet`)
they say that the notification was closed and keep *Allow* off until it can be asked again. The
scene settings show the same hint with these actions until the user decides, and their switch for
each input says the same while the notification asks or the question rests. Only screens ask: a `preview:` output (a library card, the preview window, the editor) plays without what was not granted, and neither the app nor the tray asks for it, marks itself as needing attention or opens for it. `reiwand-ctl lib apply`, `playlist start`, `next` and `prev` follow
the same rule: once the daemon lists the inputs of the scene they showed in `shown`, they print the
`reiwand-ctl input grant` command for each input that is neither granted nor declined, with `-o` and
the output when another output shows another package with the same id.

## `GetInputs() → (s json)`

An `InputsReply`: the grants by package id (`grants`, for released clients: the inputs granted to
the packages with that id that the outputs show, or, while none is shown, every input granted to
some package with that id), the declined inputs (`declined`), the questions open in a desktop
notification (`asking`) and the ones that rest after the user answered *Not now*, closed the
notification or left it unanswered (`quiet`), both by package id, the scenes that declare inputs on each output with their
`title`, the package they are bound to (`key`) and the inputs granted to that very package
(`granted`, what clients check before asking), the outputs whose
parallax image follows the pointer (`parallax`), whether audio is being captured (`capturing`)
and whether the pointer is being followed (`tracking`), the outputs that take clicks (`clicks`, the
setting of `SetClicks`), the ones whose scene takes them now (`clicking`) and the ones that clicks
cannot reach here (`unreached`: in the Flatpak, the outputs that a client such as the Plasma
wallpaper plugin draws, since only a Plasma shell outside the Flatpak may call `Touch`), where the
app warns on the settings of a scene that uses clicks. `media` says whether the media players are
read now (`listening`), for which outputs (`using`), how many players are open (`players`), whether
the chosen one has a track (`track`), plays it (`playing`) and has a cover shown (`cover`), and the
value of `media.remote_covers` (`remote_covers`); it never carries what plays.
`grants` also lists, under its full path and with `pointer`, every picture shown now on a
screen whose parallax follows the pointer, so that clients show and revoke it next to the scenes.
`gates` says, for each real screen, whether its wallpaper gets its inputs now: `on` (in view),
`covered` (the desktop reported a window covering it with `SetOutputCovered`), `unseen` (its stream
is not playing, so nothing on it is shown) or `still` (the wallpaper is paused); previews are not
listed. The service logs the same, once per change and per screen (`screen inputs`).

## `SetClicks(s output, b on)`

Lets clicks and taps on `output` reach its wallpaper, or stops them; an empty `output` sets every
output and drops the setting of each one. The setting is kept in `config.toml` (`clicks` of
`[outputs.<name>]` or `[default]`) and is off until the user turns it on. Only the app, the tray and
`reiwand-ctl` (identified as for `Grant`) may turn it on; other callers get `AccessDenied`, and anyone
may turn it off. A scene still needs the `clicks` grant of its package.

## `SimulateMedia(s output, s track, ay cover)`

Shows a made-up track on the wallpaper of `output` instead of what a media player plays, so that
`media` values, covers, `media:` triggers and script events can be tried without music playing, as
the `media` of a preview does (`editor.md`): `track` is a JSON object of at most 64 KiB with
`title`, `artist`, `album`, `album_artist`, `subtitle`, `genres`, `playing`, `position`, `duration`
and `rate`, checked with the same limits, and `cover` the bytes of a picture (at most 16 MiB, empty
for none), reduced and coloured like a real cover. The track and its cover keep their identity
while only `playing`, `position`, `duration` or `rate` change; another title, artist or album makes
a new track and another picture a new cover, the one before becoming the previous cover. An empty
`track` ends it. It needs no `media` grant and never reads the players, and while it lasts the
listener does not read them for that output either. Only the app, the tray and `reiwand-ctl`
(identified as for `Grant`) may call it; other callers get `AccessDenied`. `reiwand-ctl media simulate
OUTPUT FILE.json` (the `cover` of the file is a picture beside it) and `--stop` call it.

## `SimulateTouch(s output, d x, d y, u buttons)`

Shows a made-up pointer on the wallpaper of `output` instead of the real one, the same `x`, `y`
(fractions of the output, outside `0..=1` for a pointer that left) and `buttons` as `Touch`, so that
`input.cursor`, `click:`, `enter:` and `leave:` can be tried without a mouse there and without the
checks `Touch` makes for a real screen (`clicking`, the Plasma shell, the pointer truly being on the
output): it works on any output, including a `preview:` stream. Only the app, the tray and
`reiwand-ctl` (identified as for `Grant`) may call it; other callers get `AccessDenied`. `reiwand-ctl
input touch OUTPUT X,Y [--buttons N]` and `--stop` call it; the app calls it from the canvas while
Test the pointer and clicks is on.

## `SimulateTrigger(s output, s trigger)`

Fires `trigger` on the scene shown on `output`, a `preview:` stream included, as if it happened:
every timeline whose `trigger` and every sound layer whose `play_on` is exactly `trigger` (for
example `event:boom`, `click:button`, `media:play` or `prop:flash`) starts from its beginning, so an
author can try them without the event, the pointer or the player that would fire them. Nothing else
of the scene changes and nothing is stored. A trigger of no bytes or over 256 returns `InvalidArgs`;
only the app, the tray and `reiwand-ctl` (identified as for `Grant`) may call it, other callers get
`AccessDenied`. `reiwand-ctl input trigger OUTPUT TRIGGER` calls it, and the timeline panel of the
scene editor calls it on its preview with *Fire it in the preview*.
## `Bands(s output) → (s json)`

The levels of the sound that the scene shown on `output` follows, for the meter of the editor
(`editor.md`). The reply is `{"granted", "listening", "sound", "bands": 64}`. `listening` is `true`
while the daemon has the sound capture open and the scene holds the audio grant (it stays `true`
in silence); `sound` is `true` only while levels of sound are arriving now. While `sound` is `true`
the reply also carries `raw` and `loud` (the levels of the 64 bands of the left and the right channel, from 0 to
1, relative to the loudest band of the last seconds and measured from full scale), `loudness`,
`beat` and `silent`, exactly what the analysis hands the scene before the attack and release of
the scene smooth it. `granted` is `false` while the scene on `output` lacks the audio grant, and
then no level is given and `listening` is `false`. Asking is what turns the meter on: the daemon keeps the last reading of an
output only for 3 s after the latest call, drops it when nobody asks and refuses a reading older
than 1 s, so nothing is stored beyond the latest frame of levels, no sound is kept or copied and
no capture starts for it: the levels exist only while the scene follows the sound on its own
account (`uses.audio`, the grant, the screen shown and watched). Only the app, the tray and
`reiwand-ctl` (identified as for `Grant`) may call it; other callers get `AccessDenied`. `reiwand-ctl
input bands OUTPUT` calls it and prints the reply.
