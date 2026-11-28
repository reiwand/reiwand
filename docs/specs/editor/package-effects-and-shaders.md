# Package effects and shaders and History

On this page: package effects and shaders, history.

## Package effects and shaders

The effects section of the assets panel creates a package effect from a template (*New effect*: a name
and the template `color`, `distort`, `passes`, `material` or `particle`, the `shader` action with `new`)
and, for each effect, opens its WGSL code in the Code panel and the form of its descriptor.

- Code (`SceneShaderBody.qml` in `SceneCodePanel`) edits the WGSL with the same base as scripts. Half a
  second after the last text change, and only then, it checks the effect as the readers do
  (`wgsl::check`, the same `Descriptor::packaged` that the engine uses when loading an effect: the
  helper process with its limits, naga, the work limits and, for a material, its construction in the
  context of the 3D renderer) on a separate thread (`ShaderCheck`, off the interface thread; an old
  result is discarded). It shows the error with its line and column, marks the line in the text and
  jumps to it; with *Preview while typing*, each text that compiles is written (`shader` action with
  `write`, as a gesture: consecutive steps are undone together) and the preview draws it over the
  chosen layer; *Use on the selected layer* adds the effect to that layer. Ctrl+S saves even if the
  text does not compile.
- The descriptor form (`SceneEffectForm.qml`) lists the parameters with their type, default value,
  minimum, maximum, unit (degrees for a dial), the image tools of a texture or mask (`tools`: gradient
  ramp, gray brush, direction brush, depth map; if not declared, those given by its id apply, which the
  form shows) and which handle each receives on the canvas (`point`, `corner`, `vector` or `angle`, and
  its partner: a point `<root>from` with `<root>to`, `start` with `end`, `a` with `b`), which is what
  the canvas draws for each use of the effect; the passes with their input, scale, inputs and target;
  the declared cost and the one that results per quality level, with and without `optional` (the
  `effect_cost` query), and the fields `mask_mode`, `fuse` and `reach`. Each change is a `shader`
  action (`param`, `pass`, `lighter` or `field`).
- The *Lighter variant for low quality levels* section of the form (`SceneEffectLighter.qml`) adds or
  removes the effect's light variant: its passes (the same rows as those of the effect, with
  `lighter: true`), its declared cost and from which level it replaces the effect (only where the
  effect would be omitted, or at Q0, at Q1 and below, at Q2 and below). On creating it, it writes the
  entry `fs_<effect>_lite` in the WGSL if missing. The live check also builds the variant's entries
  and rejects one that is not cheaper than the effect, and the section shows which form each level
  draws (full, light variant, still or omitted). The inspector's cost panel (`SceneEffectCost`) says at
  each level whether the effect is drawn with its light variant, next to the preview per quality
  level.
- The materials of a model with a package shader show their parameters: numbers, colors and, for their
  images, a picker of the package's images and of `white`, `black`, `noise` and `normal_flat`
  (`scene3d` `material` with `values`). *New material shader* uses the `material` template, and each
  material shader of the package opens the same code view.

The queries `shader_source` (the descriptor and the WGSL of an effect, its parsed form, the parameter
types and the form fields) and `shader_check` (with `wgsl` or `descriptor` not saved, if given: `ok`,
`error`, `line`, `column`, `stage`, `units`, `uses`, `handles`, `lighter`, the cost of the light
variant, and `levels`, the form and cost of each level) live in `wgsl` of `crates/reiwand-editor`.
A text of more than 64 KiB does not reach the checker.

## History

Each action that changes something is one step `{label, undo, redo}`. Only gestures merge: the
app marks the changes it sends while a slider or a canvas handle is dragged, and such a change
merges into the previous gesture with the same label, recorded less than 500 ms before, when its
operations only replace values that step set; while a press holds a gesture open (`Project::hold`,
the `hold` of the app on press and release, which every slider of the scene editor sends, and the
canvas for every left press on it, so each handle of the canvas holds it while pressed), its
first change starts a step of its own and the next ones merge into it however long the drag pauses,
and an action that is not part of the gesture closes it; an undo or redo while it is held makes the
rest of the drag a step of its own. Dragging is one step, while typed values, moves,
insertions, removals and every `reiwand-ctl scene` command are steps of their own. A new step drops
the steps that were undone; an action that changes nothing once the computed fields follow it,
such as unticking an input the scene uses, records no step. The history keeps at most 500 steps
and 64 MiB of values, dropping the oldest first.

The history is stored with the unsaved document in
`$XDG_STATE_HOME/reiwand/editor/<hash of the folder>.json` (mode `0600`, folder `0700`),
together with a hash of the files it started from. The app writes it every 30 seconds while
something changed and when the editor closes; `reiwand-ctl scene` writes it after each command, so
undo and redo work across commands and between the app and the command line. When a folder opens
with unsaved changes based on the same files, the app offers to recover or discard them, and
`reiwand-ctl scene recover [--discard]` does the same; until then, closing the editor or running
another `reiwand-ctl scene` command keeps them as they were. Saving refuses to overwrite files that changed
since the folder was opened. The file goes away when the editor closes with everything saved or
the history is empty; a new one deletes those untouched for 30 days and keeps the 32 newest.
`reiwand-app --check` keeps them in a temporary folder.
