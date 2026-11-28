# Newer forms, events and media

On this page: newer value forms, events, triggers and media values.

## Newer forms

Newer values add these forms, each with its feature:

| Form | Example | Feature |
|------|---------|---------|
| Keyframe handles | `[2, 1, { "in": [0.5, 0], "out": [0.5, 0.2] }]`: Bézier handles in time and value around a key of a number, the time of each within the neighbouring segment; `out: [dt, dv]` is the point (t + dt, v + dv) and `in: [dt, dv]` the point (t − dt, v − dv), so equal handles make a smooth key and `dv: 0` a flat one; a side without a handle points a third of the way to the next key. Keys of positions, sizes, colors and other lists of numbers take handles too: there `dv` is a share of the way between the two keys (0 at the first, 1 at the second), so the curve shapes how fast every part moves at once and the value between is the usual blend of the two keys at that share; when both keys of a segment of such a field are written as one number (a pair or triple taken from one number, such as a `scale` of `2`), `dv` is in value units as for a number, every part follows that one curve and the key is written back as the number it was; a side may also be a list of `[dt, dv]`, one for each part (`"out": [[0.5, 0], [0.2, 30]]` for the x and the y of a position, up to four for the red, green, blue and alpha of a color, which then mix channel by channel instead of in OKLab), each in the units of its own part like the handle of a number, so the tracks of a value are shaped independently and even a part that does not change between the two keys can bend; a part missing from the list points a third of the way, and when one side of a segment is a list and the other a single pair, that pair is read as the same share of the way for every part; texts and switches take no handles | `tangents` |
| Parts on their own | `{ "x": { "keys": [[0, 0], [2, 100]] }, "y": { "cursor": "y", "scale": 100 } }` for a position, `r`, `g`, `b`, `a` for a color; each part is a value of its own and a missing part is that of `base`, or 0 (alpha 1) | `tracks` |
| Relative | `{ "keys": [[0, 0], [1, 30]], "base": 120 }` adds the animation to `base` (numbers, positions and colors, part by part; a lone number in an effect or script parameter is added to both parts of a pair and to the red, green and blue of a color, and the alpha of a sum stays within 0 to 1) | `relative_values` |
| Time as a number | `{ "clock": "day" }` from 0 at midnight to almost 1 before the next; also `hour` (0 to 23), `minute` and `second` (0 to 59), `weekday` (1 for Monday to 7 for Sunday) and `month` (1 to 12), all in local time; a value that follows `day` changes at most once a minute, or every `clock.step` seconds (1 to 3600, always written; 60 is the usual value), and the others when they change | `clock_values` |
| Dates | `{ "date": "EEEE d 'de' MMMM yyyy" }` (text only) with `yyyy`, `yy`, `MMMM`, `MMM`, `MM`, `M`, `dd`, `d`, `EEEE`, `EEE`, `D` (day of the year) and `w` (ISO week) besides the tokens of a clock, names in the language of the package (English or Spanish, English for the others); text between single quotes stays as written and `''` writes a quote; a text without hours, minutes or seconds changes at midnight. A `clock` text keeps only the tokens of the time, and every other letter stays as written | `clock_dates` |
| Frequency ranges | `{ "audio": [0, 3], "kind": "relative", "unit": "db", "of": 16, "reduce": "max" }` follows bands 0 to 3 of 16 (`of` is 16, 32 or 64, always written with an indexed band or a range; 64 is the number the editor suggests) and takes their `mean` (prefilled) or `max` | `audio_ranges` |
| Shaping | `bounds: [a, b]` maps a value to (x − a) / (b − a) clamped to 0 to 1, `exponent` (0.1 to 10) raises it, both before `scale` and `offset`; `smooth: { "up", "down" }` rises and falls at different speeds | `shaping` |
| Layer as texture | `{ "layer": "<id>" }` for a `texture` or `mask` parameter of an effect: the layer is drawn on its own (even with `visible: false`) and fed to the effect aligned with the screen, so that each point of the effect reads the point of that layer above it, at the size the effect's layer has on the screen; it is drawn again only when that layer changes, and each one costs as much as a layer drawn on its own; layers may not feed themselves through other layers, masks or groups | `layer_textures` |

## Events and triggers

Timelines may name `events`, at most 64 `{ "t", "name" }` with `t` within the duration and names
of `[a-z0-9_]{1,32}`; each fires once per pass, also when a frame skips over it (on a `pingpong`
timeline once each way), in the order of their time. A `trigger` or a `play_on` of `event:<name>`
starts on it, as if it had started at the very moment of the event even when the frame came later,
so its own events up to that frame, those at 0 included, fire in the same frame, and scripts receive it (`timeline_events`, which every platform runs). A still scene wakes for the
next event only when a trigger, a sound or a script that the platform runs listens to its name. Clips of meshes and models
name events the same way.

Triggers may also be `click:<layer>`, `enter:<layer>` and `leave:<layer>` for a layer that is
`solid` or has `hit` bones, evaluated without scripts (`layer_events`; `click:` needs the `clicks`
input, the others `pointer`): a click is a press and a release of any button both on the layer,
which keeps the pointer from the press until the release (so dragging off it and letting go is no
click), and `enter`/`leave` follow the topmost solid layer under the pointer. They also start sounds
whose `play_on` names them. Pointer events never make extra frames: they are worked out when the
scene draws a frame for the pointer it already follows, and a still pointer makes none. And `media:play` (the media player starts playing), `media:pause` (it pauses or stops, or the
player goes away while it played), `media:track` (a new track, also the first one the scene sees)
or `media:cover` (a new cover) (`media_info`, input `media`), evaluated without scripts when the
reading of the player changes, never while the package lacks the grant or no player is there.

## Media values

`media` values follow what the media player of the computer plays, only while the scene is shown,
the package declares `media` and the user allows it: the texts `title`, `artist`, `album`,
`album_artist`, `subtitle`, `genres` and `kind`; the numbers `position`, `duration` (seconds) and
`progress` (0 to 1); `playing` as a flag or number; and the colors `primary`, `secondary` and
`tertiary` of the cover, `text` (readable over the primary color) and `contrast` (black or white).
Texts show their `placeholder` while nothing plays or the grant is missing, and are empty without
one: a text, or a map from language to text picked like the texts of the manifest by the language
of the screen, at most 512 characters each, only beside a `media` text value (`media_info`);
colors are those of the placeholder of the cover. The numbers also fit
texts, written `m:ss` (`h:mm:ss` from an hour) for `position` and `duration` and as a whole
percentage for `progress`. The position runs on from the last reading of the player at its rate,
and starts again from where the player says it jumped (`Seeked`), without asking the player while
it plays: a number that follows `position` or `progress` asks for at most 30 frames a second and a
text one frame a second, only while the track plays and the scene is shown; paused, stopped or at
the end of the track they wake nothing. An asset with `media: current` shows the
cover of the track and one with `media: previous` the cover of the track before it (both still
pictures, usable wherever a picture is: image layers, masks, effects, particles); `path` is the
placeholder shown until a cover arrives, while nothing plays, without the grant and when the
cover cannot be read. Covers are read once per track (the last eight fetched from the internet are
kept in memory by address while `media.remote_covers` stays on, so they are not fetched again), also when a new track keeps the address of
the one before (a cover with the same bytes stays the same cover), from `file://` addresses of at
most 16 MiB or, only with `media.remote_covers` on (`dbus.md`), from `http` and `https` addresses
of at most 4 MiB within 5 seconds, pictures of more than 4096 pixels a side are refused, and JPEG covers are decoded already scaled down by the decoder (at most 1/8 of each side, never below 256 pixels on the long side) instead of at full size; they
are reduced in memory to at most 256 pixels on the longer side in
sRGB, keeping their shape, and never written to disk. A layer that shows a cover keeps its `size`,
so a square `size` suits most covers. The colors are worked out once per cover: the three
most common colors of the cover reduced to 64 × 64 (k-means of three in OKLab, transparent pixels
left out), the most common first; `text` is the secondary or tertiary color with a WCAG contrast of
at least 4.5 against the primary one, the higher one first, and otherwise white or black;
`contrast` is white or black, whichever contrasts more with the primary color. Without a cover
they are the colors of the placeholder of the first `current` cover asset, or dark greys with white
text when there is none; `smooth` eases between them part by part.
