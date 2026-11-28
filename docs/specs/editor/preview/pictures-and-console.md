# Pictures, clips and the console

On this page: saving pictures and clips, console lines, errors and inputs.

## Saving pictures and clips

*Save a picture…* and *Save a clip…* render the scene to a file without the desktop, from the
unsaved document, with the same code as `reiwand-ctl scene render` (`Renderer::rehearse` of
`reiwand-render`): the scene is drawn at the level, with the render scale of that level, battery and
reduced motion that the panel shows, from the second the playhead is at. A picture is one
frame (.png, .jpg, .webp or .avif); a clip is 90 frames at 30 a second, written as an animated
.gif or, for any other extension, as an image sequence named `NAME-0001.EXT`. *Move a pointer
across and click* adds a made-up pointer that crosses the canvas and presses over the middle of
the clip, so that what follows the pointer moves in the file too. The app tells where the files
went with the `rendered` signal. The file shows what the preview shows (`reiwand_editor::reach::shown`: the layer shown alone, the lights muted or alone, the particle seed pinned), runs the scripts in the sandboxed script host, and gives them the clock of *Set the time the clock shows* (the wall clock of the computer when it is off) and, when the simulated player of the preview is on, its track and cover (`Rehearsal::media`, `media::dress`). The same scene document is drawn by `scene thumbnail --capture` and by the poster of the screen layout preview (`--layout`).

`reiwand-ctl scene render DIR --out FILE [--media TRACK.json] [--audio SOUND] [--time-of-day T] [--time S] [--simulate] [--quality Q] [--battery] [--calm] [--frames N
--fps F] [--size PX] [--pointer X,Y[:X,Y] [--click]] [--against Q] [--without-bloom] [--split
F]` does the same from the saved folder: a picture for one frame, an animated .gif or an image
sequence for more (up to 600 frames), a made-up pointer at fractions of the canvas that moves to
the second pair over the clip and presses over its middle with `--click`, and with `--against`
or `--without-bloom` the right of the picture from that level or without the glow, joined at
`--split` (0 to 1) of the width. With `--simulate` it does not seek: `Rehearsal::simulate` plays
the scene from second 0 up to `--time` (at most `Rehearsal::MAX_SIMULATE`, 600 s; a longer one is refused),
frame after frame at the step of `--fps`, drawing each of those frames and waiting for the device
without reading it back, then a last shorter step when `--time` is no whole number of frames,
and draws the frames asked for from there: particles take every 1/60 s step a live run at that
frame rate takes, with no 30 s cap, and scripts, timelines, the sound and the made-up pointer (held
at its start) run through those frames too, so the first frame equals frame `--time · --fps` of a
render from 0 and the same seed gives the same picture. It says on the error output how many
frames it plays first. The made-up pointer goes through the same hit test as the
daemon, so `click:`, `enter:` and `leave:` triggers fire and scripts get its touches. Videos are
decoded with GStreamer at the second the clock of each one is at (`reiwand_render::Films`,
`reach::films`), as `scene thumbnail --capture` does. It prints the files it wrote and, on the
error output, the effects the renderer left out and each video layer it could not decode, which
it draws empty. With `--audio SOUND` (any sound GStreamer decodes, resampled to 48 kHz stereo; a mono file is heard on both sides) the `audio` input of every frame is what the daemon would hear if that file played at full volume from second 0 of the scene clock: `Rehearsal::audio` holds the decoded `reiwand_audio::Track` and `reiwand_audio::Played` feeds the same `Analyzer` the capture of the daemon uses, one update per frame, with the samples up to the time of the frame and the frame step as `dt`, so the result depends only on the scene, the file and `--time`, `--fps` and `--frames`, never on the wall clock. With `--time S` the analysis first hears the minute of the track before S, one update per frame step; the scene itself is fast-forwarded to S without sound as before, so what it did with earlier sounds is not there. Only scenes that use `audio` get the frames, as in the daemon. Without `--audio` the input stays at rest (`AudioFrame::default()`, silent), as before. A web package drawn by `scene render` runs on its own frame clock (`web.md`, *Frame clock*) and refuses `--audio`. `reiwand-ctl scene bench PACKAGE --quality Q --shot FILE` gives the
frame of a built package at a level together with its timings.

## Console, errors and inputs

The lines its scripts pass to `console.log` and the kind, and the error of a script or layer that
stopped, are part of `GetState().outputs[OUTPUT]` (`console`, `error`); the Console panel shows
them while the preview is on (a web package has it too), and `reiwand-ctl scene preview DIR --console`
prints them as they change, along with the error, until stopped (Ctrl-C, or another `scene
preview` on the same output).

When a script `uses` an input such as `pointer` or `audio` that the preview output was not
granted, the Problems panel warns of it, and `reiwand-ctl state` on a
terminal prints the same warning next to the output, from `Inputs1.GetInputs().shown[OUTPUT]`
(its booleans against `granted`); it prints the stopped-module error there too.
