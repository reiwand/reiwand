# Actions from the command line and Scripts and templates

On this page: actions from the command line, scripts and templates.

## Actions from the command line

Rule: every editor action is reached with `reiwand-ctl scene act`. Each editor operation is a variant
of the `Action` enum of `crates/reiwand-editor` (or a query of `rigs::query`), and the command line
has no subcommand of its own per tool. Dedicated subcommands exist only for what a person or a script
does by hand: create, import, preview, capture, check, pack, sign, publish and the repositories. A
new editor feature enters as an `Action` with its unit test in `crates/reiwand-editor`, and the
integration scripts exercise it with `scene act`.

- `reiwand-ctl scene act DIR ACTION [JSON]` applies an action to the scene folder, just like the
  application: the same validation, the same undo history and the same error messages. `JSON` is the
  argument object of the variant; it can be passed as an argument, with `--file PATH` or on standard
  input if it is `-`. Without `JSON`, the arguments are `{}`. It prints the ids of what was added, or
  with `--json` the operations that changed the document.
- `reiwand-ctl scene act DIR batch JSON` takes a JSON list of objects `{"action": {arguments}}` and
  applies them as a single undo step (`Project::act_all`): if one fails, no change remains in the
  project or the history, and undo or redo cannot be part of the list.
- `reiwand-ctl scene actions` lists each action with a line; `scene actions NAME` prints its arguments
  (name, type, whether they are required or optional, values of enumerations and default values) and
  an example that works, with the lines that prepare its scene in front; with `--json` it prints the
  JSON schema of the action. The schema comes from the types with `schemars` (variants of nested
  enumerations and variants without data appear in the list, and the fields that the action requires
  even though the type gives them a default value appear as required), and the examples are in
  `manual/examples.txt`. A test of `crates/reiwand-editor` (`manual`) fails if an `Action` variant is
  not in the list or has no description, and a `reiwand-ctl` test runs each example of each action
  and each query on a prepared scene and fails if any is rejected or answers `null`.
- When `set`, `add` or any action leaves the scene with a problem that it did not have (a value out of
  range, a resource that does not exist), the action is applied and `reiwand-ctl` prints on the error
  output a `problem: PATH: MESSAGE` or `warning: ...` line for each one, the same as those of the
  editor's problem list; with `--json` it brings them in `problems`.
- `reiwand-ctl scene ask DIR QUERY [JSON] [--layer ID]` makes an editor query (the same ones the
  application asks with `Project::ask`) and prints the answer as JSON; `scene actions --queries
  [NAME]` lists and describes them, with a working example each. The `history` query gives the steps
  that undo and redo would go through and whether there are unsaved changes. `--inside TEMPLATE`
  applies to the three commands.

## Scripts and templates

The Scripts panel (`SceneScriptsPanel`) lists each module of `scripts` in the order in which it runs:
its size, how many values follow it, a notice when what the code uses differs from what the scene
declares, and buttons to move it up or down, edit its code, open the file in the system application
and remove it. The attach button offers the `.js` files of `scripts/` that the scene does not yet
list. Unfolding a module shows the check of the code against its declaration and its settings.

- The check (`scripts::audit`) reads the code and reports what it uses and the entry does not
  declare, and what the entry declares and the code does not use, for `events`, `uses` and `params`
  (the settings that are read as `params.name` or `params["name"]`). The parts that the code cannot
  show (`clicks`, `geometry`, `animations`, `shortcuts`, `hierarchy`) are never reported as unused.
  The *Declare what it uses* button runs `script` with `declare`.
- Scripts that move a mesh (`.bone(`, `.shape(` and `.impulse(` of a layer) count as `animations` in the
  declaration. The code view offers *Insert a call that moves a mesh*: a mesh or model layer, the type
  (bone, blend shape or pushing a body), the bone or shape among those the file has (the bodies for
  pushing) and the call (turn, move, scale, read, release, give weight, push); it inserts at the
  cursor, for example `scene.layer("id").bone("arm").rotation = 0;`, with the query
  `{"script_calls": {"layer": ID, "kind", "name", "what"}}` of `rig` (without `kind` it returns the
  bones, shapes and bodies). The check adds `rig`: each bone or shape that the code names with a text
  and that the meshes cited by the code (all those of the scene if it cites none) do not have, with its
  line, and the code view lists it under the check.
- The settings of a module are a table with one row per setting: its id, its type, its default value
  (a field for numbers and texts, a checkbox, the color picker or two numbers) and, for numbers, the
  minimum and the maximum. Each change is a `script` action with `param`; if the module rejects a
  value, the row goes back to how it was.
- The Code panel (`SceneCodePanel`) edits the module: it saves with the button or with Ctrl+S
  (`script` with `write`), shows the check over the text and opens the file in the system
  application. The Reference panel (`SceneApiPanel`) lists the scripts API (`scripts::reference`:
  the parts of the `reiwand` module, the functions that a module exports and the functions that a
  value follows) with a search, and the call that moves a mesh, which it inserts in Code. With the application open, the size and modification date of the
  modules are looked at once per second, only when the scene lists scripts: if a file changes outside
  the application, the text of an unmodified code view is read again, the check is repeated and the
  preview restarts the scene with the new code, because the document it receives carries a mark of the
  modules (`scene.ext.editor.scripts`).
- The link menu of each value has *Follow a script*: for each module, *every frame* (the module's
  `update`) and each function it exports besides the events (`scripts::functions`); a function of a
  module with settings is linked with `params` filled with their default values.
- The templates section of the Scripts panel lists each template of `templates` as a tree of its layers. Pressing a layer
  selects it, so the inspector shows its fields; the preview button shows the template alone as a copy
  draws it, and each template can be renamed or removed. *Make a template* of the layers bar takes the
  selected layer and those marked with Ctrl.

The queries `scripts::listing` (the modules with their functions, settings, the values that follow
them and their check, and the free files of `scripts/`), `scripts::audit`, `scripts::reference`,
`scripts::stamp`, `scripts::changes` and `templates::tree` live in `crates/reiwand-editor`; the
application reaches them with the `code` query of the scene editor.
