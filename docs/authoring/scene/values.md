# Values that move

Almost every number, color, switch, text and image of a layer can be a fixed value or an object
with exactly **one** source. These are the sources:

| Source | Example | What for |
|--------|---------|----------|
| User property | `{ "prop": "theme", "map": { "day": "#ffe08a", "night": "#8fb4ff" } }` | What the user chooses. See [properties.md](../properties.md). |
| Keyframes | `{ "timeline": "jump", "keys": [[0, 0.4, "out_bounce"], [3, 0.4]] }` | An animation over time. |
| Sound | `{ "audio": "bass", "kind": "relative", "unit": "db" }`, `{ "audio": 3, "kind": "relative", "unit": "db", "channel": "left" }` | What plays on the computer. |
| Pointer | `{ "cursor": "x" }` | Where the mouse or the finger is. |
| Clock | `{ "clock": "HH:mm" }`, `{ "clock": "hour" }` | The local time and date. |
| Media | `{ "media": "title" }` | What plays in the media player. |
| Script | `{ "script": "scripts/spin.js" }` | See [scripts.md](../scripts.md). |
| Layer | `{ "layer": "other" }` | Only in `texture` and `mask` parameters of effects. |

The complete example is in [`examples/values`](../examples/values/scene.json).

### Properties

`{ "prop": "id" }` takes the value of a user property. With `map` a value is chosen according to
the option: `{ "prop": "theme", "map": { "day": "#ffe08a", "night": "#8fb4ff" } }`. In images and
videos, `map` switches the asset: `{ "prop": "style", "map": { "night": "sky" } }`.

### Keyframes

A `keys` list of `[time, value]` or `[time, value, easing]`. Time is in seconds of the timeline you
name with `timeline`, or of the scene clock if you name none. Without a timeline, `loop` (`once`,
`repeat`, `pingpong`) says what happens on reaching the end. Up to 256 keys per value.

The easing up to the next key can be `linear`, `step`, `hold` (jumps at once), or `in_`, `out_`,
`in_out_` followed by `sine`, `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic`
or `bounce`, or a Bézier curve `[x1, y1, x2, y2]`.

Colors are blended in the OKLab space, by hue, saturation and brightness, and the hue takes the
short way: from red to green it passes through orange and not through a dark olive.

Variants:

| Form | Example | What it does |
|------|---------|--------------|
| Handles | `[2, 1, { "in": [0.5, 0], "out": [0.5, 0.2] }]` | Custom Bézier curves around a key. `out: [dt, dv]` is the point (t + dt, v + dv) and `in: [dt, dv]` the point (t − dt, v − dv); equal handles give a smooth key, `dv: 0` a flat one. In positions, colors and lists, `dv` is the fraction of the way between the two keys. One side can be a list of handles, one per component. |
| Parts separately | `{ "x": { "keys": ... }, "y": { "cursor": "y", "scale": 100 } }` | Each part of a position, and `r`, `g`, `b`, `a` of a color, is a value of its own. The missing part takes the value of `base`, or 0 (opacity 1). |
| Relative | `{ "base": [960, 900], "keys": [[0, [-300, 0]], ...] }` | The keys are added to `base`. |

### Sound

`{ "audio": "bass", "kind": "relative", "unit": "db" }` goes from 0 to 1. There are 64 bands per
channel, from 40 Hz to 16 kHz. Each one is in decibels above -70 dB, relative to the strongest band
of the last few seconds, so quiet music and loud music fill the same range.

- A number is a band (`{ "audio": 3, "kind": "relative", "unit": "db" }`); a pair is a range (`{ "audio": [0, 3], "kind": "relative", "unit": "db", "of": 16,
  "reduce": "max" }`, with `of` 16, 32 or 64 and `reduce` `mean` or `max`).
- The names: `bass` (under 250 Hz), `mid` (up to 4 kHz), `treble` (above), `level` (all the
  sound) and `beat` (1 on each beat).
- `channel` is `average` (default), `left` or `right`.
- The value rises in 20 ms and falls in 200 ms, unless the scene changes `audio`, below.

A scene with `audio` needs the audio input. Everything that follows the sound is 0 while there is
no audio.

### Pointer

`{ "cursor": "x" }`, with `x`, `y` (from 0 to 1, relative to the screen), `inside` (1 if the
pointer is on the screen) and `speed`. The pointer follows the real one with a spring that settles
in about 0.15 s, and returns to the center when it leaves. On the phone it only counts when the
screen is touched.

### Clock and dates

- Text: `{ "clock": "HH:mm:ss a" }` with `HH`, `H`, `hh`, `h`, `mm`, `m`, `ss`, `s` and `a`.
- Number: `{ "clock": "day" }` goes from 0 to almost 1 during the day; also `hour` (0 to 23),
  `minute`, `second` (0 to 59), `weekday` (1 Monday to 7) and `month` (1 to 12). All in local time.
  `day` changes at most every minute, or every `clock.step` seconds.
- Dates, only in text: `{ "date": "EEEE d MMMM yyyy" }` with `yyyy`, `yy`, `MMMM`, `MMM`, `MM`,
  `M`, `dd`, `d`, `EEEE`, `EEE`, `D` (day of the year) and `w` (ISO week), plus the symbols of the
  time. The names come out in the language of the package (Spanish or English; English for the
  others). Text between single quotes stays as is, and `''` writes a quote.

### Media

`{ "media": "title" }` follows what the computer's media player plays: the texts `title`, `artist`,
`album`, `album_artist`, `subtitle`, `genres` and `kind`; the numbers `position`, `duration` (in
seconds), `progress` (0 to 1) and `playing`; and the colors `primary`, `secondary`, `tertiary` of
the cover, `text` (readable over `primary`) and `contrast` (white or black). With `placeholder` (a
text, or one per language, up to 512 characters) you say what to show while nothing is playing.
Numbers also work as text (`m:ss` for `position` and `duration`, a percentage for `progress`). It
needs the `media` input.

### Tweaks to a value

A value that is not fixed accepts, in this order: `bounds`, `exponent`, `invert`, `scale`,
`offset`, `clamp` and `smooth`.

| Field | What it does |
|-------|--------------|
| `bounds: [a, b]` | Takes the value to (x − a) / (b − a), clamped to 0 and 1. |
| `exponent` | From 0.1 to 10: raises the value, curves it. |
| `invert` | Flips it (1 − value). |
| `scale`, `offset` | Multiplies and adds. |
| `clamp: [min, max]` | Clamps it. |
| `smooth` | Seconds of smoothing, or `{ "up": 0.05, "down": 0.4 }` to rise and fall at different speeds. |

### Layer as texture

`{ "layer": "id" }` in a `texture` or `mask` parameter of an effect draws that layer separately,
even if it has `visible: false`, and uses it as an image. A layer cannot feed itself.
