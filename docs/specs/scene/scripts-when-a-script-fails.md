# When a script fails, Listening to the audio from a script and Script declarations

On this page: when a script fails, listening to the audio from a script, script declarations.

## When a script fails

A function that throws writes the error to the console and the frame goes on with the other modules;
a module whose functions throw in three frames in a row stops alone, with its timers, and the
screen reports "A script of this scene stopped and the scene goes on without it: <module>: <error>"
in the language of the desktop, as does `reiwand-ctl state`. A module that does not load stops the same
way. A frame that runs out of time or memory fails as a whole, and after three such frames in a row
the scripts of the scene are off and the screen says so; the scene keeps drawing without them. On
phones the scripts run in a thread of their own with the same limits, and the diagnostics of the app
say when a module stopped or the scripts are off.

## Listening to the audio from a script

A scene lists at most 4 `listens` at its root, each `{ "kind", "unit", "window", "reference", "hold_silence" }` with
the fields of an audio binding (`window`, `reference` and `hold_silence` required with `surge`, refused otherwise).
`audio.heard(slot, count)` returns the 64 bands (or 16 or 32, averaged) of entry `slot` as a `Float32Array`,
read as that kind and unit would give them, and throws a `RangeError` for a slot the scene does not list. A
`surge` slot rises above 1 when the music returns after quiet and decays back. `audio.bass`, `mid`,
`treble`, `level`, `bands()` and `buffers()` stay relative in decibels. The script needs the `audio` input.

## Script declarations

Each entry of `scripts` may declare what its module needs from the engine, so that an engine without
it refuses the package instead of running the module wrongly, and so that its inputs are known
without reading its text. `reiwand-ctl scene act DIR import_script` and the editor propose them from the
module's exports, imports and calls.

| Field | Meaning |
|-------|---------|
| `events` | the functions the module exports and the events it handles, each once: the lifecycle above (`update` is the only one older engines call), the pointer events of `solid` layers and `hit` bones `click`, `down`, `up` (input `clicks`), `enter`, `leave` and `move` (input `pointer`), delivered with `scene.layer(id).on(event, handler)` as `{ type, layer, local, scene }` with the touched bone, and `mediaStatus`, `mediaPlayback`, `mediaProperties`, `mediaThumbnail` and `mediaTimeline` (input `media`) |
| `uses` | the parts of the `reiwand` module it uses, each once: `audio`, `pointer` and `clicks` (those inputs), `timers`, `layers` (`spawn`, `destroy` and `order`; `script_layers`), `effects`, `particles` (it names `particles2` and gives each particles layer particles of its own), `math`, `console`, `shared`, `media` (the `media` object and its colors, input `media`; `media_info`, which engines run later), `storage` (`storage`; `script_storage`), `geometry` (`scene.mesh(pool)`; needs `pools`; `script_geometry`), `sounds` (`sound(id)` with `play`, `pause`, `stop`, `playing` and `volume`; `script_sounds`), `animations` (`layer.video`, `layer.anim(clip)` and, on `mesh` and `model` layers, `layer.bone(name)`, `bones`, `shape`, `impulse`, `hit` and `attachment`; texture frames and model clips come with the layers that have them; `script_animations`), `camera` (`scene.camera`, `camera3d`, `parallax`, `background`, `ambient` and `sky`; `script_camera`), `bloom` (`scene.bloom`, which may turn on a glow the scene does not declare; `script_camera`; the static cost of the scene, `reiwand-ctl pack check` and the cost per level of the editor count that glow at every level, while the running scene counts it only while a script asks for it; the editor adds `bloom` to the uses of a script that writes `.bloom`), `shortcuts` (`openShortcut(prop)`, which asks to open what the user chose for that `shortcut` property and returns `true` only inside the handler of a `click`, at most four properties a frame; see "User properties and presets"; `script_shortcuts`), `hierarchy` (`layer.parent`, `layer.children`, `layer.world` and `layer.setParent(id, keep)`; `script_hierarchy`) and `folders` (`folder(prop)`; `script_folders`), described in "Parents, the camera, shapes, storage and folders" |
| `params` | at most 32 settings of the module `{ "type": "number" \| "bool" \| "text" \| "color" \| "vec2", "default", "min", "max" }` that values bound to it with `fn` and `params` fill in; ids `[a-z0-9_]{1,32}`. They live only here: the module does not export them, `scene act DIR import_script` does not propose them and keeps them when it imports the module again, and the `script` action of the editor writes them |

Any event but `update`, any part but `audio`, `pointer` and `clicks`, and `params` name `scripts2`;
handling pointer events needs a `solid` layer or `hit` bones. A module must declare `timers`,
`layers` and `effects` to use them: the engine refuses the call otherwise. A module that declares
nothing gets the same interface; its uses and inputs are those of what it imports from `reiwand`
(`audio` the sound and `input` the pointer, never words that only appear in its comments or texts),
and the editor lists what it uses without declaring. Effect uses may carry a `name` (`[a-z0-9_]{1,32}`, unique in their layer) by which scripts find
them (`scripts2`). `pools` maps at most 16 ids to `{ "type": "mesh", "max" (1 to 256 shapes),
"max_vertices" (3 to 65536), "texture", "size" }`, the shapes scripts with `geometry` may build
(`script_geometry`).

```json
"scripts": [{ "path": "scripts/bars.js", "events": ["init", "update", "click"], "uses": ["audio", "layers"],
              "params": { "speed": { "type": "number", "default": 1, "min": 0, "max": 4 } } }],
"templates": { "bar": { "blend_space": "canvas", "id": "bar", "type": "solid", "color": "#ffffff", "size": [8, 100] } }
```
