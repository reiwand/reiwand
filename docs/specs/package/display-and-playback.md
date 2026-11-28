# Display and playback and Rating and photosensitivity

On this page: display and playback, rating and photosensitivity.

## Display and playback

`display` takes `fit` (`fill`, `fit`, `stretch`, `center`, `tile` or `span`: the placement used when
a picture or video is applied without one), `background` (`#rrggbb`, the color around a picture or
video that does not cover the screen, used when it is applied without one), `span` (`per-output`,
or `all-outputs` to spread over every screen: it is then applied with `span` unless the user chooses
another placement) and `flip` (`none`, `horizontal` or `vertical`: the picture or video is shown
mirrored, on the computer and on the phone, at no cost, since the screen pass reads it mirrored; on
the phone a mirrored video goes through the graphics card instead of playing straight on the
screen) for `image` and `video` packages, and `zoom` (1 to 8) and `position` (two numbers of -1 to 1) for videos. Scenes may also set `span`,
only to `all-outputs`, since drawing on each screen needs no key: it asks to draw one canvas across every screen, each output showing its part with the
transform of its monitor. The canvas covers the physical layout of the screens that use `span`
(`layout` in the daemon settings, frames between screens included, so a line crossing two screens
stays straight behind the frames); each screen renders only its own part at its own quality level,
and all of them follow one shared clock, so animations and timelines stay in step across screens.
It is applied with `span` unless the user chooses another placement, which shows the whole scene on
each screen as before. Older engines refuse such a scene with `spanned_scenes`, and the phone,
which has one screen, shows it there. `accent` suggests an accent
color for the desktop, `#rrggbb` or `{"prop": "<id>"}` of a color property (its current value), in
any kind but `assets`. Without it the accent is the dominant color of the preview. The daemon
reports it per output (`accent` in `GetState`) and changes it only when the value changes; the Plasma
wallpaper plugin gives it to Plasma as the accent color of the wallpaper, which Plasma uses when the
user chooses the accent color from the wallpaper, the phone gives it to Android as the primary
color of the wallpaper, and other desktops ignore it.

`playback` takes `speed` (0.1 to 4), `start` and `end` (0 to 3600 s, `end` after `start`) and
`loop` (`false` plays it once and keeps the last frame, which is not drawn again until something
changes) for videos, and `max_fps` (1 to 240, or
`display` to follow the screen), `volume` (0 to 1) and `muted` (it starts without sound) for videos,
web pages and scenes; a page decides itself how it repeats, so it ignores `loop`. A setting the user
makes always wins, and the frame rate never exceeds what the user, the author (`playback.max_fps`,
and `clock.max_fps` in a scene), the screen, the battery and the quality level allow, except for a
video the phone plays straight on the screen, where the author's cap only asks the screen for that
rate. Videos, web
pages and scenes with sound layers have a `volume` setting of their own (0 to 100 %, per screen or
shared like the other settings) that starts at the author's `volume`, or at 0 with `muted`, and
multiplies the general volume; a page reads the product as `reiwand.volume`, and one that declares a
`volume` property of its own keeps it instead. The general option `playback.duck` (off by default)
fades every wallpaper sound out while another application plays sound and back in when it stops:
the daemon counts the PipeWire playback streams of other processes that are running, without
reading their names, their media or their sound. The phone counts the active playbacks Android
reports (only how many there are, which names no app), listens only while a video or a page with
sound is shown, and fades out only when there are more of them than its own: one for each of its
videos that plays with sound, one for each page shown with sound and one for each scene that plays
its sound layers.

### Video settings

A video package offers the same settings everywhere, which the user changes per screen or for every
screen like the properties of a scene (`GetSceneProperties`, `reiwand-ctl prop`): `speed` (0.1 to 4
times, without changing the pitch of the sound), `start` and `end` (the part of the video that
loops, in seconds; an `end` of 0 plays to the end), `volume` (0 to 100 % of the general volume),
`zoom` (1 to 8) and `x` and `y` (-1 to 1, which part of the zoomed or cropped video shows). The
author writes all of them (`speed`, `start`, `end`, `loop`, `max_fps`, `volume`, `muted`; `null` where the value means «none», as in `end` and `max_fps`) when the block is present (fragment: the other fields of the manifest are written as shown above):

```json
{ "kind": "video", "entry": "rain.webm",
  "playback": { "speed": 0.75, "start": 2.5, "end": 18, "loop": true, "max_fps": null, "volume": 0.4, "muted": false },
  "display": { "fit": "fill", "background": null, "span": "per-output", "flip": "none", "zoom": 1.5,
               "position": [0, -0.5], "accent": null },
  "presets": [{ "id": "calm", "label": "Calm", "values": { "speed": 0.5, "volume": 20 } }],
  "requires": { "engine": null, "features": ["video_properties"], "packages": [], "gpu": null } }
```

Presets of a video package set these ids. The phone shows the same settings and presets on the page
of the package; a video with a zoom or a position goes through the graphics card there instead of
playing straight on the screen, a frame cap keeps it playing straight: the phone asks the screen for
that rate and also skips the frames above it before showing them, so a screen that cannot run at it
still gets no more than the cap (the decoder decodes every frame, only the showing is skipped), and the
volume applies while video sound is on.
Once the length of the video is known, `start` stays at least half a second before its end; a part
that plays nothing, such as a `start` past the end of the file, loops the whole video instead.
Speed and frame rate are independent: a faster video draws no more frames per second than the
screen, the battery or `playback.max_fps` allow (straight on the phone's screen it asks for
`playback.max_fps` and skips the frames above it), and changing the speed, the part or the volume
applies to the running video without starting it again.

## Rating and photosensitivity

The rating and the photosensitivity flag are declared by the author and never inferred. Clients
blur the thumbnails of packages rated `mature` until the user chooses to show them, from the
details of the item or with a click or a key on the blurred thumbnail of a screen in the app's
Settings page or the tray, and `library.hide_mature` leaves them out of searches and repository
listings. A photosensitive package carries the notice `flashes and is disabled` in every listing (app,
`reiwand-ctl` and the Android app) while the general setting `playback.allow_flashing` is off, which
is its default: the daemon refuses to apply it (`FlashingDisabled`), a profile leaves it out (the
screen reports why) and playlists and rotation skip it. With the setting on nothing is said. The
mature decision is kept once per package id (`Library1.Confirm`,
`reiwand-ctl lib confirm mature ITEM`). The self-hosted server refuses packages rated
`mature`.
