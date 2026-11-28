# Script basics

On this page: what a script can do, the language and adding one.

## What it can and cannot do

A script can read the time, the pointer, the sound, the properties the user chooses and what plays
in the media player. It can change the layers, the effects, the particles, the sound, the timelines
and the camera of the scene.

A script **cannot**:

- open files or know in which folder something is. It only sees the files of the package by name
  and, for what the user chooses, an identifier that is not a path;
- use the network or launch other programs;
- read the keyboard, the battery, the names of outputs or monitors, or the list of fonts;
- change the properties the user chooses. It reads them and nothing more;
- create layers out of nothing. It only copies the templates that the scene declares.

The system enforces this, not the goodwill of the script: the script runs in a separate process,
without files, without network and with a filter that only lets through the system calls needed to
compute. On the phone it runs in a separate thread with the same limits.

### Limits

| What | Limit |
|------|-------|
| Modules per scene | 64 in total; more than 16 make the package ask for a newer Reiwand (`scripts2`) |
| Size | 1 MiB across all modules |
| Memory | 32 MiB |
| Stack | 256 KiB |
| Time on load and on the first frame | 100 ms |
| Time per frame | 4 ms of CPU, all modules added together |
| Result of a frame | 256 KiB |
| `console` lines | 20 per second, 512 characters each |
| Timers | 64 waiting at once; none under 16 ms |
| `shared` | 256 values, 65,536 elements and 32 levels per value |
| `storage` | 64 KiB in each scope |
| Template copies | 256 at once, and 512 layers in total in the scene |
| Layers that report every frame | the first 64 that the script reads or asks for |
| Pointer events | 64 per frame |

Results arrive on the next frame, so a slow script never slows down drawing: at most, the scene is
seen one frame late. If a frame runs over its time or memory, that whole frame fails; three frames
in a row like that turn off the scripts of the scene (the scene keeps drawing without them). A
function that throws an error writes the error to the console and the frame goes on with the other
modules; a module whose functions fail on three frames in a row stops by itself, with its timers.

A script whose results do not change for 30 frames makes the scene run it only twice per second,
or at its next timer, until something changes. A scene without `update` only runs the scripts when
there is an event: a timer, a property change, a timeline event or a pointer event. Export `update`
only if you need it: without it, the scene does not draw frames for the script.

## The language

It is modern JavaScript with modules: `import`, `export`, arrow functions, classes,
destructuring, `Map`, `Set`, `Math`, `JSON`, `Date`. These things do **not** exist: `fetch`,
`XMLHttpRequest`, `require`, `window`, `document`, `process`, `localStorage`, `Intl`,
`TextEncoder`, `structuredClone` or `WebAssembly`. Promises and `async`/`await` do not run: do not
use them. Timers are those of the scene (below), not those of a browser.

Each file is a module. A module can only import three things:

```js
import { scene, engine, props } from "reiwand";
import { Vec2, mix, smoothstep } from "reiwand/math";
import { help } from "./lib/help.js";
```

`reiwand` is the interface of the scene, `reiwand/math` is the calculation functions and the third
form is another script of the same scene, with a relative path and listed in `scripts`.

`Math.random()` follows the seed of the scene clock: with `"clock": { "seed": 7 }` it always gives
the same series, and with `"clock": { "seed": null }` a different one on every start (`scene.md`).

`Date` uses the time zone of the computer, also when it changes. To write dates and times in the
language of whoever uses the wallpaper, use `time.format(...)` (below).

## How to add a script to the scene

1. Put the `.js` file in the scene folder, for example `scripts/clock.js`.
2. List it in `scene.json`, in `scripts`:

```json
"scripts": [{ "path": "scripts/follow.js", "events": ["update"], "uses": ["pointer"], "params": {} }]
```

There are two ways to do it without editing the JSON by hand:

```
reiwand-ctl scene act my-scene import_script '{"file":"my-scripts/follow.js"}'
reiwand-ctl scene act my-scene import_script '{"template":"timer"}'
```

The first copies the file to `scripts/` and writes the declaration (`events` and `uses`) from what
it exports and imports. For `follow.js` it prints:

```
scripts/follow.js was added
scripts/follow.js now uses pointer
```

`update` does not appear in `events` because all versions call it without it being declared. The
second starts from a template: `empty`, `media`, `touch`, `timer`, `audio`, `spawn`, `wave`,
`daytime`, `setting` or `launches`. In the editor, the *Scripts* panel has *Import a script…* and
does the same (`editor.md`).

### The declaration

Each entry of `scripts` can say what the module needs:

| Field | What it is |
|-------|------------|
| `path` | The path of the file inside the package. Each path only once. |
| `events` | The functions it exports and the events it handles: `init`, `update`, `destroy`, `resize`, `properties`, `settings`, `timelineEvent`, `mediaStatus`, `mediaPlayback`, `mediaProperties`, `mediaThumbnail`, `mediaTimeline`, and the pointer ones `click`, `down`, `up`, `enter`, `leave`, `move`. |
| `uses` | The parts of `reiwand` it uses: `audio`, `pointer`, `clicks`, `timers`, `layers`, `effects`, `particles`, `math`, `console`, `shared`, `media`, `storage`, `geometry`, `sounds`, `animations`, `camera`, `bloom`, `shortcuts`, `hierarchy` and `folders`. |
| `params` | Up to 32 settings of the module itself: `{ "type": ..., "default": ..., "min": ..., "max": ... }` with type `number`, `bool`, `text`, `color` or `vec2`. The ids are `[a-z0-9_]{1,32}`. |

The declaration has two effects. First: the scene knows which inputs the script uses without
reading its text (sound, pointer, clicks). Second: a version of Reiwand that does not know
something you declare rejects the package instead of running it wrong. `reiwand-ctl pack build`
puts the needed features in the manifest (`requires.features`, see `package.md`).

There are two modes, depending on what you declare:

- **Without declaration** (only `path`): the module receives a reduced interface, the same in all
  engines, and only `update` and `timelineEvent` are called. It has `engine` (`time`, `dt`,
  `locale`, `daytime`), `time`, `props` (`get`, `label`, `pressed`, `color`), `audio` (`bands` and
  the levels), `input.cursor` (`x`, `y`, `inside`, `buttons`), `scene.layer(id)` with `position`,
  `scale`, `rotation`, `opacity`, `visible`, `color` and `text`, `timeline` and `particles`. What
  it writes to a layer only holds for that frame: it has to be written on every frame. It cannot
  import `reiwand/math` or another module, or use `console` or timers. This is the mode of the
  clock example.
- **With declaration** (`events`, `uses` or `params`): it has the whole interface that this page
  describes. To use `timers`, `layers` or `effects` it has to declare them; without declaring them,
  the call fails. What it writes to a layer lasts until it changes it again or calls `reset`. Any
  event other than `update` and any part other than `audio`, `pointer` and `clicks` make the
  package ask for `scripts2`.

The inputs a module uses come from what it declares (`uses`) or, without declaration, from what it
imports from `reiwand` (`audio` and `input`). The manifest also has to list them in `inputs`
(`package.md`): `pack check` tells you which one is missing and in which part of `scene.json`.
