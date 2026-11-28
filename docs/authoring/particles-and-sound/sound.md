# Sound

On this page: a sound layer and a scene that reacts to sound.

## A sound layer

```json
    {
      "blend_space": "canvas", "id": "ambient", "type": "sound", "sound": "hum", "loop": true,
      "fade_in": 2, "fade_out": 2, "play": { "prop": "sound_on", "invert": false },
      "volume": {
        "timeline": "tide", "keys": [[0, 0.3], [10, 0.8, "in_out_sine"], [20, 0.3, "in_out_sine"]],
        "invert": false
      },
      "pan": -0.2, "blend": "normal", "cache": false, "depth_test": false, "effects": [],
      "mode": "sequence", "opacity": 1.0, "parallax": [0, 0], "solid": false, "sounds": [],
      "stream": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

A `sound` layer draws nothing and does not admit visual fields (`transform`, `opacity`,
`effects`...); nor can it be a mask. A scene has up to 8, which are mixed.

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `sound` | | | The id of a sound asset. |
| `sounds` | | 1 to 16 | A list of assets, instead of `sound`. Set one of the two. |
| `loop` | `true` | | Repeats without cuts; with `false` it plays once each time `play` turns on. |
| `play` | `true` | | Starts when turned on and stops when turned off. It can follow a property or keys. |
| `volume` | 1 | 0 to 1 | Changes with 20 ms of smoothing. |
| `pan` | 0 | -1 to 1 | Left to right. |
| `fade_in`, `fade_out` | 0 | 0 to 60 s | How long it takes to rise on starting and to fall on stopping. |
| `mode` | `sequence` | `sequence`, `random` | With `sounds`, in order or at random (without immediate repeats; with the seed of the scene). |
| `pause` | | `[min, max]` from 0 to 600 s | Silence between sounds, chosen with the seed. |
| `stream` | `false` | | Reads the files while they play, for long ones. |
| `play_on` | | | A trigger that starts it: `start`, `beat`, `prop:<id>`, `event:<name>`, `click:<layer>`, `enter:<layer>`, `leave:<layer>` or `media:<what>`. It plays again each time it fires. |

`play`, `volume` and `pan` can be a fixed value or follow a property or keyframes of the scene
clock, but never the sound analysis, the pointer or a script: the scene cannot hear its own sound.

Sound assets are OGG (Vorbis or Opus), OPUS, FLAC, WAV or MP3 (only if named `.mp3`), mono or
stereo, from 8 to 192 kHz. Each weighs up to 8 MiB and lasts up to 60 s; all together, up to 180 s.
With `stream`, up to 192 MiB and 3 hours each and 384 MiB in total, outside the 180 s.

Sound follows the scene clock: it plays from the point where the scene says it should be, and if it
drifts by more than 60 ms it is put back in place. A sound with `play_on` plays where the trigger
fires.

Every scene with sound has a `volume` property (a slider from 0 to 100 %, 100 by default) that is
added by itself; the manifest can declare it to change its label, its value or its order, as long
as it stays a slider from 0 to 100. Scenes start silent, like videos, and follow the
`playback.muted` and `playback.volume` settings that the user sets. If the scene is shown on
several screens, only one plays its sound, and on the lock screen nothing sounds. On the phone it
only sounds on the home screen, and it stops with battery saving, with low battery or with heat.

The example [`animation-and-interaction`](../examples/animation-and-interaction/scene.json) plays a
sound when a button is clicked: its timeline fires an event and the sound layer uses it with
`play_on`:

```json
    {
      "blend_space": "canvas", "id": "sound", "type": "sound", "sound": "tap", "loop": false,
      "play_on": "event:hit", "volume": 0.6, "blend": "normal", "cache": false,
      "depth_test": false, "effects": [], "fade_in": 0.0, "fade_out": 0.0, "mode": "sequence",
      "opacity": 1.0, "pan": 0.0, "parallax": [0, 0], "play": true, "solid": false, "sounds": [],
      "stream": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

## Make the scene react to sound

A scene can move with the sound that plays on the computer (music, videos...). It needs the `audio`
input in the manifest (see [scene.md](../scene.md)); the user grants it once.

There are three ways:

1. **Values**: any number, color or switch can follow the sound with `{ "audio": "bass", "kind":
   "relative", "unit": "db" }`, a band, or a range of bands, as explained in
   [scene.md](../scene.md). With `scale`, `offset`, `clamp` and `smooth` you adjust how much and how
   fast.
2. **Spectrum layers**: a `spectrum` layer draws the bars, the circle or the wave of the sound (see
   [layers.md](../layers.md)).
3. **Things that already look at the sound**: the `pulse` effect (with `audio_band`), the `rate`
   and the `burst`s of particles with `on: "beat"`, `audio_gate`, and own shaders with
   `frame.bass`, `frame.beat` and the others.

```json
    {
      "blend_space": "canvas", "id": "beat", "type": "shape",
      "shape": { "ellipse": { "size": [220, 220] } }, "fill": "#ffffff",
      "opacity": {
        "audio": "level", "kind": "relative", "unit": "db", "scale": 1.2, "offset": 0.2,
        "clamp": [0.2, 1], "smooth": 0.1, "invert": false
      },
      "transform": {
        "scale": {
          "audio": "bass", "kind": "relative", "unit": "db", "scale": 0.5, "offset": 1,
          "smooth": { "up": 0.03, "down": 0.3 }, "invert": false
        },
        "origin": "anchor", "rotation": 0.0, "skew": [0.0, 0.0]
      },
      "blend": "normal", "cache": false, "depth_test": false, "effects": [], "parallax": [0, 0],
      "solid": false, "visible": true
    }
```

```json
    {
      "blend_space": "canvas", "id": "ring", "type": "spectrum", "size": [500, 500],
      "style": "circle", "bands": 64, "channel": "left", "color": "#f472b6",
      "visible": { "prop": "show_ring", "invert": false }, "blend": "normal", "cache": false,
      "depth_test": false, "effects": [], "gap": 0.2, "opacity": 1.0, "parallax": [0, 0],
      "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] }
    }
```

Sound values go from 0 to 1. `bass`, `mid` and `treble` average the bands below 250 Hz, up to 4 kHz
and above; `level` is the strength of the whole sound; `beat` is 1 on a beat. The analysis has 64
bands per channel (from 40 Hz to 16 kHz), and each one is in decibels above -70 dB, relative to the
strongest band of the last few seconds; this way quiet music and loud music fill the same range.
That is what a particle input, a script or a shader reads. A scene link (`audio` in a value) also
chooses how it is measured, with no default value:

- `kind`: `relative` (against the strongest recent band), `absolute` (from the digital maximum: how
  much it really sounds, whatever came before) or `surge` (how much louder than usual it sounds
  now; 1 is usual and it has no ceiling, so whoever consumes it must bound it with `bounds` or
  `clamp`). A band without sound (a silence, or a band the music does not reach) is 0, and the
  first sound after a silence comes out far above 1 and falls to usual in about 2.4 times
  `window`. This way, with `bounds: [0.5, 1.5]`, an emitter starts full after a silence, drops to
  half while the band sounds steadily and emits nothing in silence.
- `unit`: `db` (decibels, 0 to 1) or `amplitude` (linear, 0 to 1: the drops after a beat can be
  seen).
- Only with `surge`: `window` (0.05 to 60 s, the time over which the usual level is averaged),
  `reference` (`band`: each band against its own average; `all`: against the average of all
  together, so a band goes down when a stronger one comes in and a band that returns does not
  spike) and `hold_silence` (`true`: the average freezes during silence, so that a pause does not
  recharge the next beat).

The band selector of the editor shows these fields with the live meter, already filled with
`relative`, `db`, a 1 s window, `band` and `false`. A scene with the old `gain` is rejected naming
the field.

An `audio` block at the root of `scene.json` changes how fast those values rise and fall (`attack`
and `release`) and how long the relative reference takes to come back after a beat
(`gain_release`, required): see [scene.md](../scene.md).

A scene is only redrawn when what it hears changes. Without audio on the computer, or without the
user allowing it, everything that follows the sound is 0.
