# Example scenes and sound

Reiwand ships six example scenes, and scenes can have sound of their own. This page lists the
examples, how they are installed, and how scene sound works.

## The example scenes

The `scenes/` folder holds six public-domain (CC0) example scenes:

| Scene | What it shows |
|-------|---------------|
| `dusk` | Layers, blend modes, masks, timeline and depth that follows the pointer or a slow orbit with a map computed from its shapes |
| `snowfall` | Particles, effects, the sound of wind and the images of a folder you choose behind the snow |
| `pulse` | Sound analysis with the cover, title and artist of what plays in the center |
| `pond` | Pointer and the sound of water |
| `orbits` | A script with an image or a video of your choice as background |
| `solids` | A 3D model of its own under 64 KiB, a crate with its knob, which turns under a light with a shadow |

The `solids` model is in `scenes/solids/assets/crate.glb`. That way whoever edits can try 3D by
copying that scene, without looking for a file.

`scripts/ambient-sounds.sh` synthesizes those two sounds (filtered noise and drops, also CC0) and
saves them as Ogg Vorbis.

## How the examples are installed

| Package | Examples |
|---------|----------|
| AppImage | Installs them in `~/.local/share/reiwand/examples` (`--no-examples` omits them) |
| RPM | In the optional `reiwand-examples` package |
| Flatpak | Includes them |

The daemon offers them as an already added local repository, "Example wallpapers" (`remote ls`
marks it `bundled`).

- It is signed on first run with keys of its own for each installation; none goes in the code or
  the packages.
- It is updated at startup and once a day.
- They are installed from there like any package.

If it is removed with `remote rm`, it is not added again by itself. `reiwand-ctl remote examples`
adds it back.

## Scene sound

Scenes can have sound of their own: `sound` layers with sounds from the package.

| Aspect | Details |
|--------|---------|
| Formats | Ogg Vorbis, Opus, FLAC or WAV |
| Length | 60 s at most each |
| Playback | Looped or one-shot |
| Control | Start and stop with a timeline or a property |
| Mixing | Volume, panning and fades, mixed into a single stream |

## When scenes sound

Scene sounds start silent, like videos. They sound with `reiwand-ctl unmute` or the sound button
of the app, the tray or each screen that shows a scene with sound.

- They follow the master volume and the *Volume* property that appears in the settings of each
  scene with sound.
- They only sound while the scene is seen on an active, uncovered screen.
- They pause and continue with it when covered (even if the scene does not pause), when pausing or
  on battery, and stop when it is removed.
- They never sound with the screen locked.
- A scene that is seen on several screens sounds on only one.

## Turning scene sound off

`playback.scene_sound = false` silences the sound of all scenes without touching that of videos.
It is also *Let scenes play their sounds* in the app settings and the tray, or:

```sh
reiwand-ctl config playback.scene_sound false
```

> **Note:** The phone does not play the sound of scenes.

## Timing

Each layer sounds at the point marked by the scene's time. A layer that starts with a timeline:

- begins at the position of that timeline (not from zero),
- starts again with each loop of the timeline,
- keeps sounding if the timeline stops,
- jumps to where the timeline is when it resumes.

The daemon realigns the sound with the scene every second, counting from the position that the
audio output plays, plus a PipeWire cycle of 20 ms when the sound goes to PipeWire.

## Without an audio output

If the audio output fails or there is none (also with the old PipeWire plugin), the scene and the
videos continue without sound. They recover it by themselves when an output appears.
