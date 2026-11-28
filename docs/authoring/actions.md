# Editor actions and queries from the command line

Everything you can do with the scene editor you can also do from the command line. Each editing operation is an **action**. Each lookup about the scene is a **query**. This page explains how to use them. The pages linked at the end describe each one, with its arguments and a working example. To work with the graphical editor, see [editor](editor.md).

## How to call an action

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text"}'
reiwand-ctl scene act my-scene hide '{"id":"text","hidden":true}'
```

- `my-scene` is the scene folder (the one with `manifest.json` and `scene.json`). Create it with `reiwand-ctl scene new my-scene`.
- The second word is the action name, such as `add_layer` or `hide`.
- The last argument holds the action's arguments, written as a JSON object in single quotes. If the action needs no arguments you can leave it out or write `{}`.
- With `-` instead of the JSON, the command reads the arguments from standard input. With `--file PATH`, it reads them from a file.
- Each call is one step that can be undone, just like in the editor.

What it prints:

- Actions that create something print its id (`add_layer` prints the layer id). The others print nothing.
- With `--json`, they print the list of low-level operations that changed the document.

## Several actions in one step

The special action `batch` takes a list of actions and applies them together, as a single undo step. If one fails, none is applied.

```sh
reiwand-ctl scene act my-scene batch '[{"add_layer":{"kind":"solid","id":"background"}},{"hide":{"id":"background","hidden":true}}]'
reiwand-ctl scene act my-scene undo
```

## Undo, redo and review

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"shape"}'
reiwand-ctl scene history my-scene
reiwand-ctl scene act my-scene undo
reiwand-ctl scene act my-scene redo
reiwand-ctl pack check my-scene
```

`scene history` lists the steps that `undo` and `redo` walk through. `pack check` checks the scene and exits with code 1 if something is wrong. Some actions (such as `set` and `add`) accept values that break a scene rule, and then they print a `problem: PATH: MESSAGE` line to the error output for each problem that change adds (with `--json`, they return them in `problems`). It is a good idea to validate when you finish. `scene ask my-scene history` gives the same as `scene history`, as JSON.

## How to read errors

An error is printed to the error output, with the text `Error:` and the cause, and the command exits with a non-zero code.

| Code | When it happens | Example |
| --- | --- | --- |
| 1 | The action is valid but cannot be applied to this scene | `Error: there is no layer "zzz"` |
| 2 | The call is badly written: an action or query that does not exist, a missing argument, or broken JSON | the `hidden` field is missing |

Error messages are in English. When the action name does not exist, the message lists all the valid names. If an action fails inside `batch`, the scene stays as it was.

## How to find out which actions exist

```sh
reiwand-ctl scene actions
reiwand-ctl scene actions hide
reiwand-ctl scene actions --json
```

- With no name, it lists all the actions, one line each.
- With a name, it shows its arguments (whether they are required or optional, and their type) and a working example. If the example needs something in the scene, the lines before the last one set it up; they run in order on a freshly created scene, with the example files (`picture.png`, `sheet.png`, `sound.ogg`, `model.glb`, `figure.glb`, `pieces.glb`, `video.webm`) in the folder you run from. A program test runs every example it prints.
- With `--json`, it prints the list, or the JSON schema of the arguments, as JSON.
- With `--queries`, it does the same for queries: `reiwand-ctl scene actions --queries` and `reiwand-ctl scene actions --queries wind`.

Some actions are called with a single variant: the argument is an object with one key, which is the variant name. For example `effects` is called with `{"add":{...}}`, `{"set":{...}}` or `{"remove":{...}}`. The pages below describe each variant.

## How to ask a query

```sh
reiwand-ctl scene ask my-scene sounds '{}'
```

- The query answers with JSON on standard output. A query does not change the scene.
- The final JSON holds the query's arguments; with `-` or `--file` they are read as for actions.
- Many queries are about one layer. The layer is given in the arguments, or with `--layer ID` when the arguments do not name it.
- If something needed to answer is missing (a layer with no bones, for example), the answer is usually `null` or an empty list. It is not an error.
- `--inside TEMPLATE` makes the scene that is edited or queried the layers of a script template.

## The examples on these pages

Each example is in a block that runs on a scene freshly created with `reiwand-ctl scene new my-scene`. The lines in a block go in order: the first ones set up what the last one needs. The example files that are named (`picture.png`, `sheet.png`, `sound.ogg`, `model.glb`, `figure.glb`, `pieces.glb`, `video.webm`) are in the folder you run from; use your own files under another name.

Coordinate names, units and ids are explained at the start of each page, in "Before you start".

## Index


Each name links to its description.

### [Layers, assets and canvas](actions-layers.md)

Actions: [set](actions-layers/before-you-start.md#set), [add](actions-layers/before-you-start.md#add), [remove](actions-layers/before-you-start.md#remove), [move](actions-layers/before-you-start.md#move), [add_layer](actions-layers/layers.md#add_layer), [remove_layer](actions-layers/layers.md#remove_layer), [duplicate_layer](actions-layers/layers.md#duplicate_layer), [move_layer](actions-layers/layers.md#move_layer), [group](actions-layers/layers.md#group), [ungroup](actions-layers/layers.md#ungroup), [hide](actions-layers/layers.md#hide), [lock](actions-layers/layers.md#lock), [solo](actions-layers/layers.md#solo), [import](actions-layers/assets.md#import), [import_assets](actions-layers/assets.md#import_assets), [asset](actions-layers/assets.md#asset), [rgb](actions-layers/layers.md#rgb), [rename](actions-layers/layers.md#rename), [paste](actions-layers/layers.md#paste), [layer](actions-layers/place-and-shape-a-layer.md#layer), [canvas](actions-layers/canvas.md#canvas), [film](actions-layers/assets.md#film), [sprites](actions-layers/assets.md#sprites)

Queries: [copy](actions-layers/queries.md#copy), [unkept](actions-layers/queries.md#unkept), [asset_uses](actions-layers/queries.md#asset_uses), [film](actions-layers/assets.md#film), [sprites](actions-layers/queries.md#sprites), [touch](actions-layers/queries.md#touch), [shape_handles](actions-layers/queries.md#shape_handles), [text_box](actions-layers/queries.md#text_box), [font_list](actions-layers/queries.md#font_list), [capture_view](actions-layers/queries.md#capture_view)


### [Time, package, properties and scripts](actions-time-and-package.md)

Actions: [keyframe](actions-time-and-package/timeline-and-keys.md#keyframe), [import_effect](actions-time-and-package/shaders.md#import_effect), [remove_effect](actions-time-and-package/shaders.md#remove_effect), [import_script](actions-time-and-package/scripts-and-templates.md#import_script), [translate](actions-time-and-package/package.md#translate), [thumbnail](actions-time-and-package/package.md#thumbnail), [shot](actions-time-and-package/package.md#shot), [add_property](actions-time-and-package/user-properties.md#add_property), [move_property](actions-time-and-package/user-properties.md#move_property), [rename_property](actions-time-and-package/user-properties.md#rename_property), [remove_property](actions-time-and-package/user-properties.md#remove_property), [condition](actions-time-and-package/user-properties.md#condition), [add_preset](actions-time-and-package/user-properties.md#add_preset), [manifest](actions-time-and-package/package.md#manifest), [property](actions-time-and-package/user-properties.md#property), [timeline](actions-time-and-package/timeline-and-keys.md#timeline), [script](actions-time-and-package/scripts-and-templates.md#script), [template](actions-time-and-package/scripts-and-templates.md#template), [pool](actions-time-and-package/scripts-and-templates.md#pool), [uses](actions-time-and-package/scripts-and-templates.md#uses), [preset_steps](actions-time-and-package/queries.md#preset_steps), [shader](actions-time-and-package/shaders.md#shader), [undo](actions-time-and-package/shaders.md#undo), [redo](actions-time-and-package/shaders.md#redo)

Queries: [palette](actions-time-and-package/queries.md#palette), [clocks](actions-time-and-package/queries.md#clocks), [pools](actions-time-and-package/queries.md#pools), [uses](actions-time-and-package/scripts-and-templates.md#uses), [shader_source](actions-time-and-package/queries.md#shader_source), [shader_check](actions-time-and-package/queries.md#shader_check), [strings](actions-time-and-package/queries.md#strings), [flashes](actions-time-and-package/queries.md#flashes), [credits](actions-time-and-package/queries.md#credits), [curves](actions-time-and-package/queries.md#curves), [key_clip](actions-time-and-package/queries.md#key_clip), [fires](actions-time-and-package/queries.md#fires), [cues](actions-time-and-package/queries.md#cues), [review](actions-time-and-package/queries.md#review), [signature](actions-time-and-package/queries.md#signature), [tried](actions-time-and-package/queries.md#tried), [levels](actions-time-and-package/queries.md#levels), [base](actions-time-and-package/queries.md#base), [media](actions-time-and-package/queries.md#media), [clock_sample](actions-time-and-package/queries.md#clock_sample), [clock_help](actions-time-and-package/queries.md#clock_help), [history](actions-time-and-package/queries.md#history), [script_uses](actions-time-and-package/queries.md#script_uses)

### [Meshes, bones and animation](actions-meshes.md)

Actions: [mesh_from_picture](actions-meshes/before-you-start-with-meshes-bones-and-a.md#mesh_from_picture), [bone](actions-meshes/bones-and-weights.md#bone), [mesh_depth](actions-meshes/before-you-start-with-meshes-bones-and-a.md#mesh_depth), [weights](actions-meshes/bones-and-weights.md#weights), [auto_weights](actions-meshes/bones-and-weights.md#auto_weights), [clip](actions-meshes/clips-and-keys.md#clip), [key](actions-meshes/clips-and-keys.md#key), [edit_keys](actions-meshes/clips-and-keys.md#edit_keys), [clip_event](actions-meshes/clips-and-keys.md#clip_event), [shape_sculpt](actions-meshes/blend-shapes-and-expressions.md#shape_sculpt), [expression](actions-meshes/blend-shapes-and-expressions.md#expression), [ik](actions-meshes/kinematics-and-physics.md#ik), [physics](actions-meshes/kinematics-and-physics.md#physics), [collider](actions-meshes/kinematics-and-physics.md#collider), [bone_rule](actions-meshes/kinematics-and-physics.md#bone_rule), [wind](actions-meshes/kinematics-and-physics.md#wind), [attach](actions-meshes/parts-and-attachments.md#attach), [point](actions-meshes/parts-and-attachments.md#point), [parts](actions-meshes/parts-and-attachments.md#parts), [animate](actions-meshes/play-and-blend-animations.md#animate)

Queries: [rig](actions-meshes/queries.md#rig), [weights](actions-meshes/bones-and-weights.md#weights), [pose](actions-meshes/queries.md#pose), [keys](actions-meshes/queries.md#keys), [handles](actions-meshes/queries.md#handles), [motion](actions-meshes/queries.md#motion), [script_calls](actions-meshes/queries.md#script_calls), [outline](actions-meshes/queries.md#outline), [wind](actions-meshes/kinematics-and-physics.md#wind), [wander](actions-meshes/queries.md#wander)

### [Effects and masks](actions-effects.md)

Actions: [effect_rule](actions-effects/a-layer-s-effects.md#effect_rule), [paint_mask](actions-effects/masks-and-painted-maps.md#paint_mask), [comb_flow](actions-effects/masks-and-painted-maps.md#comb_flow), [ramp](actions-effects/masks-and-painted-maps.md#ramp), [depth_map](actions-effects/masks-and-painted-maps.md#depth_map), [effects](actions-effects/a-layer-s-effects.md#effects), [transition](actions-effects/masks-and-painted-maps.md#transition)

Queries: [normals](actions-effects/queries.md#normals), [map](actions-effects/queries.md#map), [ramp](actions-effects/masks-and-painted-maps.md#ramp), [depth_model](actions-effects/queries.md#depth_model), [rules](actions-effects/queries.md#rules), [effect_cost](actions-effects/queries.md#effect_cost), [level](actions-effects/queries.md#level), [effect_handles](actions-effects/queries.md#effect_handles), [mask_view](actions-effects/queries.md#mask_view), [mask_picture](actions-effects/queries.md#mask_picture), [transition](actions-effects/queries.md#transition)

### [Particles, lights and sound](actions-particles-and-lights.md)

Actions: [particles](actions-particles-and-lights/particles.md#particles), [light](actions-particles-and-lights/lights-and-ambient-light.md#light), [lighting](actions-particles-and-lights/lights-and-ambient-light.md#lighting), [normals](actions-particles-and-lights/lights-and-ambient-light.md#normals), [paint_map](actions-particles-and-lights/lights-and-ambient-light.md#paint_map), [ambient](actions-particles-and-lights/lights-and-ambient-light.md#ambient)

Queries: [particles](actions-particles-and-lights/particles.md#particles), [budget](actions-particles-and-lights/queries.md#budget), [lights](actions-particles-and-lights/queries.md#lights), [falloff](actions-particles-and-lights/queries.md#falloff), [hemisphere](actions-particles-and-lights/queries.md#hemisphere), [lighting](actions-particles-and-lights/lights-and-ambient-light.md#lighting), [air](actions-particles-and-lights/queries.md#air), [projection](actions-particles-and-lights/queries.md#projection), [swarm_light](actions-particles-and-lights/queries.md#swarm_light), [spread](actions-particles-and-lights/queries.md#spread), [meter](actions-particles-and-lights/queries.md#meter), [response](actions-particles-and-lights/queries.md#response), [sounds](actions-particles-and-lights/queries.md#sounds), [wave](actions-particles-and-lights/queries.md#wave), [bands](actions-particles-and-lights/queries.md#bands)

### [3D scene](actions-3d.md)

Actions: [scene3d](actions-3d/3d-scene-how-it-works.md#scene3d)

Queries: [model_info](actions-3d/queries.md#model_info), [stage3d](actions-3d/queries.md#stage3d), [materials3d](actions-3d/queries.md#materials3d), [cost3d](actions-3d/queries.md#cost3d), [shadows3d](actions-3d/queries.md#shadows3d), [paths3d](actions-3d/queries.md#paths3d), [material_check](actions-3d/queries.md#material_check), [bone3d](actions-3d/queries.md#bone3d), [bound3d](actions-3d/queries.md#bound3d), [surface3d](actions-3d/queries.md#surface3d), [depth3d](actions-3d/queries.md#depth3d)
