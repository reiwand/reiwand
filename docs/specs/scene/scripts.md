# Scripts

`scripts` lists at most 64 JavaScript modules of the package (`{ "path": "scripts/spin.js" }`; more
than 16 name `scripts2`), each path once, at most 1 MiB together: `pack build`, every reader and the
engine refuse more. The syntax check of `pack check`, `scene validate` and the problem list of the
editor compiles the modules without running them, inside the program that checks, with the same 32 MiB and stack
limits, only when they are at most 1 MiB together, and once per change of their text; a module made
to be slow to compile (tens of thousands of declarations) can still take a few seconds then. On the computer all the modules of a scene share one process, also when several
screens show the scene with the same settings, and on phones one thread: it has no files, network or other processes, 32 MiB of memory, a
256 KiB stack, 100 ms to load and to run the first frame, and 4 ms of CPU per frame for all the
modules together. Results reach the next frame, so a slow script never holds up rendering, and what
a frame sends to the scripts is written in place: numbers and the 64 bands of each channel go as
they are, the user settings only when they change. What the scripts send back is not compared as
text: each layer field, effect setting, particle or sound control a script writes is compared with
the value it holds, and only the parts that changed (single layers, or the values the modules
returned, compared in depth) go back, so a frame in which nothing changed sends nothing; after a
frame that failed everything is sent again. Every module gets that frame object itself.

The problem list of the editor, `reiwand-ctl pack check` compile every module
without running it (`reiwand_editor::scriptcheck`, with the engine the scenes use): a syntax error is a problem
with the file and line (`scripts/spin.js:4: …`), and so is a `scripts` entry whose file is not in the folder.

## Lifecycle

A module may export these functions; the engine calls each one inside the time of a frame:

| Export | When |
|--------|------|
| `init(ctx)` | once, at the first frame, with `ctx.params` (the defaults of the module's `params`) and `ctx.engine` |
| `update(frame)` | every frame while the screen is shown; what it returns is the value of `{ "script": "<its path>" }` sources. `frame` has `time`, `dt`, `props`, `audio`, `cursor`, `actions`, `wall`, `zone`, `timelines` and `events` |
| `destroy()` | when the scene stops |
| `resize({ screen, canvas })` | when the size or the turn of the screen changes |
| `properties(changed)` | at the first frame with every user setting, then with the ones that changed |
| `settings({ language, fps, quality })` | at the first frame, then when the frame rate or the quality changes |
| `timelineEvent(e)` | for each `{ timeline, name, t }` of timeline and clip events |
| `mediaStatus({ enabled })` | at the first frame, then when the user grants or takes back `media` for the package (modules that declare `media`) |
| `mediaPlayback({ state })` | when the player starts (`playing`), pauses (`paused`) or stops or goes away (`stopped`) |
| `mediaProperties({ title, artist, album, album_artist, subtitle, genres, kind })` | when a new track arrives or its texts change, also to empty texts when it ends |
| `mediaThumbnail({ has_cover, colors })` | when a new cover arrives or goes away; `colors` holds `primary`, `secondary`, `tertiary`, `text` and `contrast` as `{ r, g, b, a }` (those of the placeholder while there is no cover, `null` without them) and never the picture itself |
| `mediaTimeline({ position, duration, rate })` | when a track starts, its length or rate changes, or the player jumps (the position is not where it would have run on to); seconds |

A scene whose modules export no `update` and bind no value with `fn` to an `update` only runs its
scripts for events: timers, settings, properties, timeline events and pointer events, and draws no
frame for them otherwise. Once the results of busy scripts stay the same for 30 frames and nothing
else moves, the scene runs its scripts only twice a second, or at its next timer, until a result
changes again.

A value `{ "script", "fn", "params" }` binds one value to one function of a module, so a module serves
many values. `fn` names an export other than the functions above: a function called every frame as
`fn(value, ctx)`, or an object with `init(value, ctx)` (first frame) and `update(value, ctx)` (every
frame). `value` is what it returned last time (`null` at first), and `ctx` has `path` (the JSON
Pointer of the value in `scene.json`), `layer` (the id of the layer that holds it), `key` and
`params` (the module's `params` with the binding's own on top). What it returns is the value, added to
`base` and shaped like any source. `params` without `fn` is refused.

```js
import { scene, audio, setInterval, console } from "reiwand";
import { Vec2, smoothstep } from "reiwand/math";

const bands = audio.buffers(32);
const bars = [];
export function init({ params }) {
    for (let i = 0; i < 32; i++) bars.push(scene.spawn("bar", { position: [40 + i * 60, 900] }));
    setInterval(() => console.log("loudest band", bands.average.indexOf(Math.max(...bands.average))), 5000);
}
export function update() {
    bars.forEach((bar, i) => (bar.scale = [1, 0.1 + smoothstep(0, 1, bands.average[i]) * 4]));
}
export const sway = (value, { params }) => Math.sin(scene.layer("tree").rotation + params.speed);
```
