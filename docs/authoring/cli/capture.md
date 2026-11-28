# Capture

### `scene render`: an image or a clip, without a desktop

```
reiwand-ctl scene render my-scene -o view.png --size 640
```

```
view.png
```

It draws the scene the way the daemon would draw it, without a daemon or a screen, and writes the
file. It prints the files it wrote. It draws the same as the editor's preview: if you chose “show
only” for a layer, only that one; with the particle seed fixed (a scene without `clock.seed` comes
out the same in every file of the same run) and with the lights muted or soloed.

| Option | What it does |
|--------|--------------|
| `-o FILE` | `.png`, `.jpg`, `.webp`, `.avif` or `.gif` (animated). Required. |
| `--time S` | The scene second of the first frame. Particles have `prewarm` plus S seconds simulated (capped at 30 s in total); a layer with `instance.time_scale` simulates S times that scale, as at S seconds live. |
| `--simulate` | Instead of seeking to `--time`, plays the scene from second 0 up to it, frame after frame at the step of `--fps` (30 by default), and draws from there: particles go through every step they would take live (steps of 1/60 s, two per frame at 30 fps), with no 30 s cap, and so do animations, timelines and scripts. The picture is the frame a live run at that frame rate shows at that second, the same every time with a fixed `clock.seed` (a render steps particles on the processor, so the graphics card does not change their drawing order between runs). It plays at most 600 s and says on the error output how many frames it plays first; each costs a frame drawn without being written, so a long stretch takes a while. A `--time` that is no whole number of frames ends with one shorter step. |
| `--time-of-day TIME` | The wall-clock time that the scene and its scripts see: `HH:MM[:SS]` of today or `YYYY-MM-DD HH:MM[:SS]`. Without it, this computer's time; set it so the image always comes out the same. |
| `--media FILE` | A made-up player track (JSON: `title`, `artist`, `album`, `playing`, `position`, `duration`, `rate`, `cover`…; the cover is an image in the folder) that the scripts and the images that follow the cover see. Without it, there is no player. |
| `--audio TRACK` | A sound file (WAV, FLAC, Ogg, MP3 or the sound of a video: whatever GStreamer reads) that the scene hears as if it were playing at full volume from second 0 of its clock. The `audio` input (layers, effects, particles and scripts that read it) receives on every frame the same analysis that the daemon does of the computer's sound (the 64 bands per channel, `bass`, `mid`, `treble`, `level`, `beat`, with the same smoothing and the same code), computed with the frame's time (`--time`, `--fps`, `--frames`) and not with the clock: the same scene with the same track gives the same bytes. With `--time S` the analysis has already heard the minute of track before the first frame, but what the scene did with the sound before that frame (particles launched by an earlier hit) is not there: to see the whole response, draw from `--time 0`. After the end of the track, silence. **Without `--audio`, the scene hears silence** (the `audio` input stays at zero, as with no sound on the computer). In a scene that does not use `audio`, nothing changes. |
| `--quality q0`…`q3` | The highest quality level (`q3` by default). |
| `--size PX` | The longer side of the image (1280 by default). It never draws larger than the canvas. |
| `--screen WIDTHxHEIGHT` | Draws what the daemon would show on a screen of that size (16 to 8192 per side), for example `--screen 1920x1080`: the image is exactly that size, with the canvas `fit` (`fill` crops, `fit` leaves bars of the background color) and each layer drawn at the screen's resolution, also when it is larger than the canvas. It cannot be combined with `--size`. |
| `--frames N`, `--fps F` | With more than one frame, a clip: `.gif`, or a sequence `NAME-0001.EXT` (up to 600 frames). |
| `--pointer X,Y[:X,Y]`, `--click` | A made-up pointer at those fractions of the canvas, which moves to the second pair and clicks about halfway through the clip. |
| `--against q0`, `--without-bloom`, `--split F` | The right part is shown at another level or without glow, to compare. |

A web package (its folder or the built `.wpak`) is drawn the same way, by the web engine, without a
window. Its clocks stand still between frames: each frame runs the timers due by its time, then the
animation frames, and stops the page's CSS and Web animations there, so the same page, options and
`--time-of-day` give the same images. Video, sound, workers and network are not tied to that clock.
The page plays from second 0 (`--time` skips writing the frames before it), with the default values
of its settings and without network. It draws in software, WebGL included. `--screen` gives the size (else 16:9 at `--size`).
`--media`, `--audio`, `--battery`, `--calm`, `--against` and `--without-bloom` do not apply.

```
reiwand-ctl scene render my-page -o frames/f.png --screen 1920x1080 --fps 10 --frames 240 --pointer 0.5,0.5 --time-of-day "2026-01-01 12:00"
```
| `--battery`, `--calm` | As on battery, or with less motion. |
| `--language LANGUAGE` | The language that the per-language texts and the scripts see (`de`, `pt-BR`); if the package does not have it, the main language. |

**It runs the scripts.** For the frames it draws, it runs the scene's scripts with the wall-clock
time of `--time-of-day` (or this computer's) and writes on the error output what they print with
`console` (`script: log scripts/clock.js: ...`) and their errors, with file and line (`script:
error scripts/clock.js: TypeError: ... (scripts/clock.js line 4)`). Before the first frame it steps
the scripts a few times so they start up, so the `console` lines come out repeated (one per step
and per frame). It hears silence unless you pass `--audio TRACK`, and the player only if you pass
`--media`. The scripts never run in the `reiwand-ctl` process (nor in the app's): they go to a
separate sandboxed process, the same one the daemon uses and with the same time and memory limits.
If that process cannot start, it draws without scripts and says so in one line on the error output.

### `scene thumbnail`: the package image

```
reiwand-ctl scene thumbnail my-scene capture.png
reiwand-ctl scene thumbnail my-scene --capture --time 3
reiwand-ctl scene thumbnail my-scene --capture --animated --seconds 4 --fps 10
```

```
assets/preview.png
```

It puts a preview in `assets/` and writes it in the manifest's `previews.thumbnail`: the image you
give, reduced if it is over 1024 pixels or 1 MiB; or, with `--capture`, a frame drawn by the
graphics card (with the daemon). With `--animated`, a `.gif` or `.webp` of up to 640 pixels per
side, 15 s and 8 MiB (`assets/preview-animated.gif`, in `previews.animated`). `--time` is the
second that is saved, `--from` where the recording starts, `--seconds` (0.1 to 15; 4 by default)
how long it lasts and `--fps` (1 to 30; 10 by default) at how many frames per second. With
`--capture` the scene's scripts do run, in the usual isolated process and with this computer's
time, so a text that they fill in comes out with its value and not with the initial one. It does
not simulate the player. The frame comes out without sound or pointer: a scene that reacts to them
looks at rest (to see how it responds to a track, `scene render --audio TRACK`).
