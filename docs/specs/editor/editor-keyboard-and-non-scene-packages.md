# Editor keyboard and non-scene packages and The own transition of a scene

On this page: editor keyboard and non-scene packages, the own transition of a scene.

## Editor keyboard and non-scene packages

The editor is navigated with Tab: the tabs of each group and their lists, the canvas bar, the
canvas, the fields of what is chosen, the problems and the timeline. The canvas takes focus with Tab
and shows the focus mark (only when it arrives by keyboard, not by a click); with a layer chosen, the
arrows move it one pixel and Shift with an arrow ten, and Esc leaves the canvas and returns focus to
the previous control (if a point was being chosen, Esc first cancels that choice). The app's check
(`SceneKeysCheck`, unit `keys.editor`) opens a scene, goes through each tab with Tab, fails if a
control or one of those regions is not reached, and moves a layer with the arrows on the canvas.
The keys of the workbench itself (F6 and Shift+F6 between groups, Ctrl+Tab between tabs, F11,
Ctrl+1 to Ctrl+9 for the layouts, Alt for the menus and the tab menu with Shift+F10) are in "Editor
frame" ([editor frame](editor-frame.md)). The Timeline, while it holds the keyboard, plays and
pauses with Space, goes to the start and end with Home and End, and moves the playhead with Left and
Right by 0.1 s (1 s with Shift).

The layer list is a single Tab stop: on entering it shows a focus frame on the chosen row (or around
the list if there is none). Up and Down choose the previous or next layer and the properties panel
follows it, as with a click; Home and End go to the first and the last; Delete removes it by the
same path as "Remove" in the menu (it is undone with Ctrl+Z); Menu or Shift+F10 opens that menu. The
next Tab goes through the buttons of the chosen row (hide, lock, solo) and leaves the list. Each row
is 30 px: a handle to drag it, a thumbnail (the picture of an image layer, from the `picture` the
outline gives it, or the icon of its kind), its name, what it is (its kind, or its number of
effects) when the list is wide enough, and the eye and the lock; the button that shows it alone
appears while that is on, under the pointer or on the chosen row. The unit `SceneLayerKeysCheck`
goes through it with the keys. On entering "Create", the marked row takes focus (unless focus is on
the side menu), so the keys of the help bar and the "Enter" of the main button work without a prior
click.

Packages that are not scenes (image, video, web page and preset) use the top bar of the editor
(`SceneTop`), as dense as a scene's: back, the title with its state, the menus as words (File, Edit,
View and Window: no Layer, and View holds only the light or dark editor, as there is no canvas to
zoom), the six layouts, Undo, Redo, "Set as wallpaper" and "Publish…", which
goes through `SceneUse.publish()` as in a scene; "View as wallpaper" only appears in the types that
can be seen alone (image, video and web page). The button of the data is called "Wallpaper
details…", the preview panel is called "See as" (with the levels "Slow computer" to "Very fast
computer") and the user properties of a web page are called "Options". The status bar says
whether it is saved, its problems, the kind with the layout in use ("Web page · Wallpaper") and the
keys of the focused panel. The unit `PackageTopCheck`
checks the bar's buttons, the panels each type shows, that it opens in Wallpaper without the panels
of a scene, and that Try comes into view.

"Capture" saves what is seen also when a preset is chosen in "Try": the image or the clip carry the
preset's values, with the loose values that were tried on top.

## The own transition of a scene

*Options › Transitions* has two rows, *When it comes in* and *When it leaves*; each shows the
duration of the transition the scene carries, or *Make a transition…*. Both open the same tool
(`SceneTransitionState`, in `page.settings.passage`):

- each change is the editor action `transition` (`TransitionEdit`: `way`, `make`, `effect`,
  `duration_ms`, `curve`, `params`, `remove`) and the query `transition` returns the two directions
  with the shape of their curve (`shape`, 49 samples), the `name` and the `fields` of the
  descriptor, `plays` per quality level and the validation `problem` in a person's words, plus the
  transition `effects` of the package, the named curves (`curves`) and the `limits`. The action
  makes the same checks as loading the scene, so it never writes what loading rejects; a rejected
  value leaves the scene as it was and the panel shows the message next to the duration;
- `make` copies the descriptor of the other direction or writes the initial effect
  `transitions/opening.{json,wgsl}` (a radial opening with a `center` point and an `edge` width); the
  files remain after undoing;
- the canvas shows the crossing between a previous wallpaper (a plain color or an image the author
  chooses) and a still image of the scene at the editor's clock, drawn by the renderer at the level
  of *See as*; with *Less motion*, or at a level where the transition does not play, it shows the
  simple fade the user would see. Point values (`vec2`) are handles on the canvas, reached with Tab
  and moved with the arrows;
- the canvas bar has *Try the transition*, *Previous wallpaper* and *Ready*; the footer has
  *Progress* and the curve editor, with named curves and two Bézier handles that write a curve of
  its own; the right panel has the effect, the duration, the numeric values and the notes about the
  quality level and about less motion. While the panel is shown, the inspector's problem list stays
  hidden.
