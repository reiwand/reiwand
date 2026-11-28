# Test and debug

### `console`

`console.log`, `info`, `warn`, `error` and `debug` write lines with this form:

```
log scripts/traffic_light.js: state: red
error scripts/traffic_light.js: TypeError: opacity takes a number (scripts/traffic_light.js line 12)
```

First the level, then the module and then the text. An error ends with the file and the line where
it happened. `console` always works, but `pack check` warns ("uses console but does not declare
it") until you add `"console"` to `uses`. An object is written as JSON. Going over 20 lines per
second discards the extra ones, and each line is cut to 512 characters.

### Where it shows

- **In the editor**, in the *Console* panel, while the preview is on. The inputs that the script
  uses and that the preview does not have granted show up in *Problems*.
- **In the terminal**, with the preview of the scene on an output:

  ```
  reiwand-ctl scene preview my-scene -o DP-1 --console
  ```

  It says on which outputs it shows the scene and then writes the script lines as they come out
  and the error when it changes, until you interrupt it with Ctrl-C. It works on your real screens
  and on test screens. The scene only runs while something is looking at that output, so use a
  screen that is in view. `reiwand-ctl scene preview
  my-scene --stop` ends the preview (if the app had *Show on the desktop* on, it also turns it
  off) and says on which outputs it ended it.
- **For a single frame, without a desktop or daemon**, `reiwand-ctl scene render my-scene -o view.png
  --time-of-day 15:42` runs the scripts, with that wall time, and writes their lines and errors to
  the error output, with the prefix `script:`:

  ```
  script: log scripts/clock.js: hello 15:42
  ```

  Before the first frame it gives the scripts a few steps to start, so each line comes out more
  than once. It does not simulate sound; the player only with `--media track.json`, the same file
  as `scene preview --media`. The scripts run in the same isolated process that the daemon uses,
  never in that of `reiwand-ctl`; if it cannot start, it draws without scripts and says
  `script stopped: …` on one line. The capture of `scene thumbnail --capture` and the thumbnails
  of the desktop library also run the scripts, each in its own isolated process. `scene bench`, on
  the other hand, does not. The phone's poster does not run scripts: it uses the preview of the
  package and, if there is none, a drawing without scripts with the initial text.
- If the scene has a script error, `reiwand-ctl state` prints it next to the output.
- **In the service log** (`reiwandd`): what `console` writes comes out at `debug` level
  (`RUST_LOG=reiwandd=debug`) and a module that stops comes out at `warn` level.

### What happens when it fails

| What happens | What you see |
|--------------|--------------|
| A function throws an error | `error <module>: <error>` in the console; the frame goes on with the other modules. |
| A module fails three frames in a row | It stops with its timers. The screen says "A script of this scene stopped and the scene goes on without it: `<module>`: `<error>`". |
| A module does not load (syntax error, an `import` that does not exist) | It stops the same way, without running anything. `pack check` and `pack check` detect it before it runs (below). |
| A frame runs over its time or memory | That whole frame fails. Three in a row turn off the scripts: "The scripts of this scene failed three times in a row and are off; the scene goes on without them." |

The texts of JavaScript's own errors and those of the interface (`opacity takes a number`) come out
in English. The most common ones:

| Message | What happened |
|---------|---------------|
| `opacity takes a number` (and other `... takes ...`) | You wrote a value of another type into a field of a layer. |
| `<module> uses timers without declaring it in scene.json` | `timers` (or `layers`, `effects`...) is missing from `uses`. |
| `Error resolving module './x.js' ... only reiwand, reiwand/math and the scripts of the scene can be imported` | The `import` points to something that is not `reiwand`, `reiwand/math` or a script of `scripts`. |
| `invalid property name at a.js:1:26`, `scripts/a.js:4: unexpected token in expression: ';'` (and similar) | Syntax error, with file and line (and sometimes column). It is the line where the engine noticed, which may be one after the typo. |
| `the script used more than 4ms of CPU time` | The frame took too long; on load the limit is 100 ms. |
| `out of memory` | The script went over 32 MiB. |
| `<path>: names <fn>, which the script does not export` | A value tied with `fn` names a function that the module does not export. |

### What `validate` sees and what it does not

`reiwand-ctl pack check` checks `scene.json`, the manifest, the inputs and also **the scripts,
without running them**: it compiles each module and says, with the file and the line, a syntax
error or a script that `scene.json` names and is not in the folder (the editor puts it in the
*Problems* panel, and `pack build` refuses to build):

```
/scene/scripts/0/path: scripts/traffic_light.js:4: unexpected token in expression: ';'
Error: 1 problem
```

`pack check` prints them as `problem:` lines and ends with an error. What is only known when
running is not seen: a function that throws an error, a value of the wrong type (`opacity takes a
number`), a loop that takes too long. For that, look at the console with the preview or with
`scene render` before packaging (`package.md`).

To test a script that reacts to something you do not have at hand:

- `reiwand-ctl input grant org.example.my-scene pointer` grants the `pointer` input to the scene
  being shown (the id is that of its manifest, `org.example.<folder>` if you did not change it; it
  prints nothing, and `input status` shows the permission), and `reiwand-ctl input touch DP-1
  0.8,0.2` puts an invented pointer at that fraction of the screen (`--stop` removes it). With a
  `console.log` of `input.cursor.scene` in `update`, the preview console of a desktop with Plasma
  prints `log scripts/follow.js: cursor [1536,216] true`: 0.8 and 0.2 of a 1920 × 1080 canvas.
  Without a pointer that the desktop follows (a session without KWin) the script sees the center
  and `false`;
- `reiwand-ctl input status` says which inputs the scene of each screen uses and which it has
  granted;
- `reiwand-ctl scene preview my-scene --media track.json` invents a player track;
- `reiwand-ctl scene preview my-scene --values settings.json` tries property values without saving
  them;
- `reiwand-ctl scene preview my-scene --restart` starts the scene and the scripts from scratch.
