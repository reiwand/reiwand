# Sources, modifiers and timelines

On this page: where a value comes from, its modifiers, audio bindings and timelines.

## Sources of a value

Every number, color, flag, text and image of a layer may be a literal or an object with exactly
one source:

| Source | Example |
|--------|---------|
| User property | `{ "prop": "style", "map": { "night": "sky" } }` |
| Keyframes | `{ "keys": [[0, 0], [2, 1, "in_out_sine"]], "timeline": "intro", "loop": "repeat" }`; keys without a `timeline` write `loop` (`once`, `repeat` or `pingpong`), with a `timeline` it is ignored |
| Audio | `{ "audio": "bass", "kind": "relative", "unit": "db" }`, `{ "audio": 3, "kind": "relative", "unit": "db", "channel": "left" }`, `{ "audio": "bass", "kind": "surge", "unit": "amplitude", "window": 1, "reference": "band", "hold_silence": false }` |
| Pointer | `{ "cursor": "x" }` (`x`, `y`, `inside`, `speed`) |
| Clock | `{ "clock": "HH:mm:ss a" }` (text only: `HH`, `H`, `hh`, `h`, `mm`, `m`, `ss`, `s`, `a`) |
| Script | `{ "script": "scripts/spin.js" }`; with `"fn": "pulse"` and `"params": { "speed": 2 }` a module serves many values (`scripts2`, see "Scripts") |
| Media player | `{ "media": "title" }` (`media_info`, see below) |

## Modifiers and audio bindings

Non-literal values accept `scale`, `offset`, `clamp: [min, max]`, `invert` and `smooth` (seconds).
Every audio binding states, with no default, a `kind` and a `unit` (and the fields of `surge`):
`kind: "relative"` is the level of each of the 64 bands per channel (40 Hz to 16 kHz, spaced evenly in
pitch) against the loudest band of the last seconds, so that quiet and loud music both fill the range;
`"absolute"` is the level from full scale (what a quiet passage really is, whatever the music did
before); `"surge"` is how much louder than usual the band is now, `1` meaning as usual, with no
ceiling (it is the only kind that leaves 0 to 1: whatever consumes it should bound it with
`bounds` or `clamp`); the usual is never taken below −60 dB of full scale, the level at which the
analysis calls the sound quiet, so a band without sound (a silence, or a band the music never
reaches) is `0`, a band that stays quieter than that is below `1`, and the first sound after a
silence is far above `1` and decays back to within 5 % of `1` as the average catches up (in about 2.4 times
`window`). `unit: "db"` shows relative and absolute in decibels above −70 dB (0 to 1), so that a
45 % dip in amplitude is only about 0.07; `"amplitude"` shows them linearly (0 to 1), so dips after hits are
visible. For `surge` `amplitude` is the ratio of amplitudes (`sqrt` of the power ratio) and `db` is
`1 + dB / 20` (each 20 dB above usual adds 1; never below 0). `surge` also requires `window` (0.05 to 60 s):
the usual is the running average of the power over that time (short: hits stand out; long: the music coming
back after a silence), `reference` (`"band"`: each band against its own average; `"all"`: against the
average of the summed power of all bands, shared by every band, so a band drops when a louder one enters and
a quiet band returning does not spike) and `hold_silence` (`true`: the average does not change while the
analysis reports silence, so a pause does not recharge the next burst); these three are refused on the
other kinds. The `level` of the whole sound has no band reference: relative and absolute coincide in `db`
and in `amplitude` it is the amplitude of the whole sound. `beat` ignores the kind and is 1 on a beat.
`bass`, `mid` and `treble` average the bands below 250 Hz, up to 4 kHz and above, `level` is the
loudness of the whole sound. (`unit` is not called `scale` because `scale` already multiplies the value.)
Particle systems (`init.turbulent`, `init.remap`, `remap` forces) carry the five fields of the binding
next to their `audio` or `audio:<band>` input, all required as for a binding (`kind` and `unit` always, `window`,
`reference` and `hold_silence` only with `surge`). Shaders and scripts declare what they read as
`Listen` entries, see "Package effects" and the scripts API; the named levels `bass`, `mid`, `treble`, `level`
and `band()` stay relative in decibels. Values rise within 20 ms and fall within
200 ms unless the scene sets `audio`. The order is `bounds`, `exponent`, `invert`, `scale`,
`offset`, `clamp` and then `smooth`; `bounds` and `exponent` also shape properties, the pointer and
scripts, and an `exponent` keeps the sign of negative values.
Easings are `linear`, `step`, `hold`, `in_`, `out_` or `in_out_` followed by `sine`, `quad`,
`cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic` or `bounce`, or a cubic Bézier
`[x1, y1, x2, y2]`. Colors interpolate in the OKLab space, lightness, chroma and hue apart, the hue
along the shorter way round, so that red to green passes through orange and not through a dark
olive, and a grey takes the hue of the other end; a color field mixes this way however its keys are written, as `"#rrggbb"` or `"#rrggbbaa"`.

## Timelines

`timelines` (at most 64) have `id`, `duration`, `loop` (`once`, `repeat`, `pingpong`), `speed`
(−100 to 100; below zero the timeline runs backwards from its start without `delay`, so a loop
plays from its end and one that plays `once` ends at once), `delay`, `autoplay` and `trigger` (`start`, `beat` or `prop:<id>`, which also fires when the
`button` setting `id` is pressed). A timeline without a `trigger` plays from the start of the scene when
`autoplay` is on; `trigger: "start"` fires when the scene starts, so it plays from the start
whatever `autoplay` says, and any other trigger waits for what it names. A timeline counts either seconds, in
`duration`, or `frames` at `fps` (1 to 240) frames a second, both together; as every field is
written (see [explicit values](../../explicit-values.md)), one in seconds writes `frames` and `fps` as
`null` and one in frames writes `duration` as `0`, and any other mix is refused; keys stay in
seconds (`timeline_frames`, which every platform runs). `wrap: true`, only on a timeline that loops
with `repeat`, blends the values from their last key back to their first one before the loop
starts again: the stretch from the last key to the first key of the next pass is one more segment
with the easing of the last key, so a value with keys at 1 s and 3 s on a 4 s timeline goes from
the last value back to the first between 3 s and 5 s (`timeline_wrap`, which every platform runs). The scene
clock only runs while the output is shown and active, keeps real time while a still scene waits
(up to ten minutes between frames, in steps of at most 0.1 s), and never catches up after a pause.
