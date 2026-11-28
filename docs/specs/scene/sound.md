# Sound

```json
{ "blend_space": "canvas", "id": "gusts", "type": "sound", "sound": "wind", "loop": true, "fade_in": 3, "fade_out": 3,
  "play": { "prop": "wind", "map": { "calm": false, "breeze": true } },
  "volume": { "timeline": "storm", "keys": [[0, 0.4], [20, 1, "in_out_sine"], [40, 0.4]] }, "pan": -0.3 }
```

A `sound` layer plays a sound asset: `.ogg` (Vorbis or Opus), `.opus`, `.flac` or `.wav`, mono or
stereo, sampled at 8 to 192 kHz, at most 8 MiB and 60 s long, and at most 180 s for all the sounds
of a scene together; the package reader checks the headers and the daemon stops decoding a sound
that goes on longer than its header says (an MP3 without a length in its header may last up to the
60 s). A scene has at most 8 sound layers, which mix together.

Sound follows the time of the scene. Each layer knows how long ago the scene started it: since the
scene started for a `play` that is always on, since the key that turned it on for `play` keyframes
(on a looping timeline, since the start of the pass), since the trigger for `play_on`, and since it
turned on for the rest. The sound plays at that point of its clip or of its list, so a loop that
drifts more than 60 ms from the scene jumps back in step with a 5 ms fade, a scene that resumes
plays from where its timelines are, and a sound started again starts over. A layer keyed on a
timeline that is paused or has ended keeps playing on, and jumps back to the timeline when it plays
again; a timeline started late or sought starts its sounds at its position, not from zero. The
engine re-aligns playing sounds with the scene once a second. About 50 ms pass between the scene and
the speaker (the soundtrack asks PipeWire for 20 ms periods and keeps about 20 ms queued); positions
are counted from the playing position of the audio output, plus one 20 ms PipeWire period when the
sound goes to PipeWire, so the queue does not shift sounds.

| Field | Meaning |
|-------|---------|
| `loop` | `true` (prefilled) repeats the sound without a gap; `false` plays it once each time `play` turns on |
| `play` | starts the sound when it turns on and stops it when it turns off (prefilled `true`); the sound plays from the point the scene says, see above |
| `volume` | `0` to `1` (prefilled `1`); changes glide over 20 ms, so keyframed curves sound smooth |
| `pan` | `-1` (left) to `1` (right), prefilled `0`, gliding like `volume` |
| `fade_in`, `fade_out` | seconds (`0` to `60`) the sound takes to rise when it starts and to fall when it stops |

Newer sound layers may also use (`sound2`):

| Field | Meaning |
|-------|---------|
| `sound` as MP3 | `.mp3` files (MPEG Layer III) whose first frame follows the ID3 tag, checked like the others; only a file named `.mp3` is read as MP3 |
| `sounds` | instead of `sound`, a list of 1 to 16 sound assets |
| `mode` | with `sounds`: `sequence` (prefilled, in order) or `random` (picked with the scene seed) |
| `pause` | `[min, max]` seconds (0 to 600) of silence between clips, picked with the scene seed |
| `stream` | `true` plays the files as they are read instead of from memory, for long sounds: at most 192 MiB and 3 hours each and 384 MiB for all the streamed sounds of a scene, outside the 180 s of the others |
| `play_on` | a trigger (`start`, `beat`, `prop:<id>`, `event:<name>`, `click:<layer>`, `enter:<layer>`, `leave:<layer>` or `media:<what>`, as for timelines) that starts the sound, always under the user's volume and mute |

A layer has either `sound` or `sounds`. With `loop: false`, `sequence` plays the list once and
`random` one clip each time it starts. `random` never plays the same clip twice in a row when the
list has more than one, and the clips and pauses it picks follow the scene seed, so the same scene
with a fixed `clock.seed` plays the same order. A streamed sound is decoded a little ahead of where
it plays, and the next streamed clip of a list starts decoding 1.5 s before its turn, so lists and
loops of long sounds play without gaps; nothing of a stream is kept once it has played. `play_on`
starts the sound again each time its trigger fires; `play` still mutes it while it is off. The
desktop and the phone play these sounds (the phone decodes them with the codecs of the system when they
first sound, reads a `stream` sound as it plays and keeps about 0.2 s between the scene and the speaker).

`play`, `volume` and `pan` may be literals or follow a user property or keyframes on the scene
clock, which a timeline triggered by a property or a beat can drive; they never follow the audio
analysis, the pointer, a clock text or a script, so a scene cannot hear its own sound. Values that
change over time are read at the frame rate of the output without rendering a frame. A sound layer
draws nothing, takes none of the visual fields (`transform`, `opacity`, `effects` and so on) and
cannot be a mask.

Every scene with sound has a `volume` user property, a slider from 0 to 100 % (prefilled 100)
added automatically; a manifest may declare it to change its label, prefilled or order, as long as
it stays a slider within 0 to 100. Scenes start muted like videos and follow `playback.muted` and
`playback.volume`; only one output plays the sound of a scene shown on several outputs, and nothing
plays on the lock screen (see "Scene sound" in `dbus.md`). On the phone the sound plays only while the
wallpaper is shown on the home screen (not on the lock screen or in a preview), pauses with the scene
when battery saver, a low battery or heat stop it, follows `playback.duck` and releases its audio output
5 s after every layer has stopped; a scene that has sound layers gets its `volume` setting there too. The
settings of the app set `playback.volume` with a slider in steps of 5 % under the sound switch; the
output asks for the power-saving mode of AAudio and the timing of the sound subtracts the frames it
still holds.
