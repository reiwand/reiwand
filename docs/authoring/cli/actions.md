# The editor's actions from the terminal

On this page: `scene act`, `scene actions`, `scene ask` and other useful commands.

## The editor's actions: `scene act`, `scene actions` and `scene ask`

Everything the app's editor can do with a scene (add a layer, move an asset, animate a property,
change the manifest…) is an **action**, and it is applied to a folder with three commands, the
same ones the app uses. This page only introduces them; `actions.md` explains each action and its
arguments.

```
reiwand-ctl scene actions
reiwand-ctl scene actions hide
reiwand-ctl scene act my-scene hide '{"id": "date", "hidden": true}'
reiwand-ctl scene ask my-scene lights
```

- **`scene actions`** lists the actions with one line each (`set`, `add_layer`, `hide`…). With the
  name of one, it prints its arguments and an example:

  ```
  hide: Show or hide a layer
    hidden: boolean (required)
    id: string (required)

  Example:
    reiwand-ctl scene act DIR hide '{"hidden":false,"id":"text"}'
  ```

  `--json` prints the list, or the JSON schema of an action. `--queries` does the same with the
  questions of `scene ask`.
- **`scene act DIR ACTION [JSON]`** applies an action and saves the folder. The arguments are a
  JSON object (`{}` if omitted); with `-` they are read from standard input and with `--file PATH`,
  from a file. It prints nothing if it succeeds; with `--json` it prints the operations that
  changed the document. The `batch` action applies a list `[{"action": {arguments}}, …]` as a
  single undo step.
- **`scene ask DIR QUERY [JSON]`** asks the editor a question about the folder (the lights, the
  cost of the particles, a layer's skeleton…) and prints the answer in JSON.

An action that would leave the scene with an error is rejected and changes nothing. Each step can
be undone with the `undo` action (and redone with `redo`).

## Other useful commands

| Command | What it is for |
|---------|----------------|
| `scene bench clock.wpak --frames 300` | Draws a package, or the folder of a scene project, off screen and reports the cost of each frame (without running the scripts). |
| `scene recover my-scene` | Restores the unsaved changes the app left for a folder (`--discard` discards them). |
| `pack new-assets`, `pack parts` | Make and list asset packages with parts of a scene (`package.md`). |
