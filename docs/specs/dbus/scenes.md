# Scenes, Scene sound and Change notification

On this page: scenes, scene sound, change notification.

## Scenes

A scene package (`kind: scene`) holds a `scene.json` (see `scene.md`) and the assets it names.
The daemon renders it on the GPU for every output that shows it, with the same streams, fits of
the scene canvas, transitions and claims as other wallpapers.

- A frame is rendered only when something visible changes: a running animation or timeline, a
  clock text when its minute (or second) changes, a video frame, a changed user property, or an
  input the scene uses (audio levels, pointer, script outputs). A still scene renders once and
  then only the 10 s keep-alive of the stream runs.
- The frame rate is the lowest of `playback.max_fps`, the scene's `clock.max_fps`, the quality
  level and the GPU class, as described in "Frame rate"; a scene with `clock.max_fps = "display"`
  follows the refresh rate of each output.
- The quality level follows the GPU class (measured once per graphics card and driver), the power
  source and the GPU time of the frames: on battery, on a software renderer or when frames take too
  long, heavy effects run at a lower resolution or not at all when the scene marks them `optional`,
  and the scene itself renders at 0.75 or 0.5 of the output size. `GetState().outputs[x].quality`
  and `quality_reason` tell the level and why.
- An output that is not active (paused, covered, locked or paused on battery, see
  `SetOption`) or whose stream has no consumer renders nothing and its scene clock stops; it
  continues where it stopped, without catching up.
- `GetState().outputs[x].kind` is `scene` and `playing` tells whether the scene is animating.
- `GetState().outputs[x].sound` tells what the sound of the scene does there (see "Scene sound").
- Example scenes are packages in `reiwand/examples/` under `$XDG_DATA_HOME` or a folder of
  `$XDG_DATA_DIRS`. The daemon adds each one to the library once, when it starts and on a
  `Rescan` of every source; an example removed from the library is not added again, and the
  item of an example whose file was uninstalled is removed. `kind:scene` finds scene packages and `kind:web` web packages.

## Scene sound

The sound layers of a scene (see `scene.md`) play through the same audio path as videos (see
`SetVolume`), with the same handling of a missing audio output. The daemon mixes all the layers of
a scene into one stream, so a scene has one audio pipeline at most. When that pipeline is first
created, one worker thread decodes into memory the sounds of the layers that can play (not those
whose `play` is a fixed `false`), the playing ones first, once for each version of the package:
later pipelines of the same package reuse them, and they are freed when no output shows it. Each
sound is decoded up to the length that its header gives plus 0.5 s, and all the sounds of a
package up to 180.5 s; a sound that goes on longer is not played.

- Scenes start muted like videos. Their sound plays only while `playback.scene_sound` is `true`,
  `playback.muted` is `false` and the
  stream volume, `playback.volume × volume / 100`, is above 0, where `volume` is the user property
  that every scene with sound has (a slider from 0 to 100 %, 100 by default, added to
  `GetSceneProperties` unless the manifest declares it as such a slider itself).
- The pipeline is created only while the scene is shown on an active output (not paused, covered
  or paused on battery, and with a consumer for a stream) and the screen is unlocked. It pauses
  exactly when the scene pauses, together with the scene clock, and goes on from the same place;
  it is destroyed when the scene leaves the output and when the sound is muted or turned down to
  0. Once every sound layer has stopped and faded out it pauses, and 5 s later it is destroyed; a
  layer that starts again within these 5 s resumes it, and one that starts later creates it again.
  Nothing plays while the screen is locked or the output is covered, whatever `pause.when_locked`
  and `pause.when_covered` say; videos are muted while the screen is locked too.
- When several outputs show the same scene package, only one plays its sound: the first active
  one by output name whose scene volume is above 0, which hands the sound over to the next one
  when it pauses or is covered.
- Sound layers start, stop and change their volume and pan as their values change on the scene
  clock. Values that change over time are read at the frame rate of the output, without rendering
  a frame while nothing visible changes; fades and volume changes are applied to every sample, and
  values that stay the same cost nothing.
- A layer whose `play` follows a timeline plays from the place of that timeline: it starts at the
  offset of the timeline position since the key that turned it on, starts again with every turn of
  a repeating timeline, keeps running when the timeline is paused or ends, and jumps back to the
  timeline when it plays again. The daemon re-aligns every playing layer with the scene clock once a
  second, and a layer more than 60 ms away ducks for 5 ms and jumps. Positions count from the
  playing position of the audio output, whose clock leaves out what its sink still holds, plus one
  20 ms PipeWire cycle (the period the soundtrack asks for) when the sound goes to `pulsesink` or `pipewiresink`.
- When the audio output fails (no device, a sound server that went away, also with a PipeWire
  GStreamer plugin older than 1.2), the scene and the videos go on silent, and the output is tried
  again once a PipeWire or PulseAudio device provider lists an audio sink, after 1 s and then twice
  as long each time up to 60 s.
- `GetState().outputs[x].sound` is `""` for content without sound layers, `"off"` while the sound
  is muted, has not started or every layer has stopped, `"playing"`, `"paused"`, or `"elsewhere"`
  when another output plays it.

## Change notification

`Serial` starts at the daemon's start time in microseconds since the Unix epoch and grows by one
for every `Changed` signal. Topics and their `ids`:

| Topic | Emitted when | `ids` |
|-------|--------------|-------|
| `state` | Anything in `GetState` changes | output names, or `[]` for global changes |
| `lock` | A lock choice changes (`GetLock`) | `[]` |
| `library` | Items, sources or tags change | item uids, or `[]` for many items, sources or tags |
| `playlists` | Playlists change, start or stop | playlist uids |
| `profiles` | Profiles or presets change or are activated | profile or preset uids |
| `providers` | Providers change | provider ids |
| `repos` | Repositories or installed packages change | repository ids, or `[]` |
| `jobs` | A job is created or finishes | job ids as decimal strings |
| `inputs` | A grant or a decline changes, a question in a desktop notification opens, ends or stops resting, an output starts or stops showing a scene that declares inputs, or audio capture or the pointer feed starts or stops | `[]` |
| `accounts` | An account is added, removed, signed in again or linked to another repository | `[]` |

Per topic, the first change emits `Changed` at once and opens a 250 ms window; changes inside the
window are merged and emitted once when it closes. Merged `ids` are the union, or `[]` when one
change had no specific ids or the union exceeds 256 ids, meaning "re-read everything of this
topic".

Every query reply carries the `serial` read before the query ran. A client that has seen a
`Changed` of that reply's topic with a larger `serial` fetches again; otherwise the reply is
current.

`WaitChanged(serial, timeout_ms)` returns at once with `(Serial, topics changed after serial)`
when `serial` is older than `Serial` (every topic when it is older than the last 256
changes), and with `(Serial, all topics)` for a `serial` of a previous daemon run or one larger
than `Serial`. When `serial` equals `Serial` it waits until a change or the timeout, then
returns `(Serial, [])` on timeout.
