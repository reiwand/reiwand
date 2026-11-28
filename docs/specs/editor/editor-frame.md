# Editor frame

The editor page fills the top bar with its own content (`topBar` of the page, see `shell.md`) and
asks for the dense bar of the workbench (`dense`: 44 px instead of `topHeight`, the measures in
`DeskLook`): back, the title, its state ("Saved" or "Unsaved ·" with "Save now"), the menus as words
(`SceneMenus`), in the middle the layouts (`SceneLayoutWords`), undo and redo; on the right "View as
wallpaper", "Set as wallpaper" and "Publish…". Back ("‹ Create") is Mist at rest and Frost under the
pointer or the keyboard. While the editor is in view, the whole page and the dialogs it opens use
the denser measures of `Look` (`desk`, set by `SceneStatus`): 13 px text, tool buttons 26 px at 12.5
px, fields and combo boxes 26 to 28 px, menu options 30 px, switches 30 × 16 px; the rows of the
canvas bar are 34 px. The Inspector is drawn on the colour of its group, with no lighter card. The
other packages have the same top bar and status bar (see below), over the same workbench. Below the workbench a 28 px status bar
(`SceneStatus`) says whether the scene is saved, its problem count (a click shows Problems, opening
it again if it was closed), "See as: …" as the canvas bar says it, the canvas size with the name of
the layout in use ("1920 × 1080 · Scene") and, on the right, the keys of the panel that holds the
keyboard focus, as key caps with what each does: the canvas (arrows, Shift by 10 px, Esc), Layers
(up and down, Home and End, Del, the menu key), Assets (up and down, Home and End), Tab in the
others, F11 to enlarge it, and always Alt for the menus. Only keys that work in that panel without a click first are
listed.

The menus are File, Edit, Layer, View and Window (`SceneMenuWords`), each a word with an underlined
letter taken from the "&" of its translated name. Alt and that letter opens it; Alt pressed and let
go alone puts the focus on the first word, where Left and Right move, Down opens and Esc returns the
focus where it was; while a menu is open Left and Right open its neighbour. Below 1500 px of window
width the words fold into one "Menu" that lists them, and undo and redo leave the bar (they stay in
Edit and their keys). The entries add no tool: File has "Save now" (Ctrl+S), the four imports of
Assets and "Capture…"; Edit has undo and redo, named with the step they undo or redo, then rename,
duplicate, copy, paste and remove from the menu of the chosen layers; Layer has "Add a layer" and
the rest of that menu (group, ungroup, hide or show, lock or unlock, raise, lower); View has zoom
in, zoom out, fit, test the pointer, the guide, several screens and "See as…"; Window lists the
panels other than the canvas, marked when open, and choosing one closes it or opens it again (as
"Close the panel" and "Open … again"), then every layout with its blueprint (30 × 20 px) and its
key, the one in use in bold, then "Save as new layout…", "Rename or remove your layouts…" (disabled
while the author has none) and "Reset this layout". An entry that cannot act is disabled. The rows
of these menus, and of the other menus of the editor (`SceneMenu`), are 30 px high with 13.5 px
type.

The blueprint of a layout (`SceneBlueprint`, from `Workbench.blueprint`) draws one box per group in
18 × 12 px in the bar and 30 × 20 px in Window and in "Your layouts", from the shares of the tree
alone (no floors), cut on whole pixels so that the boxes tile the drawing, every group at least one
pixel each way; the group of the canvas is filled.

The layouts are in [Layouts](editor-layouts.md), and each panel in [Editor panels](editor-panels.md).
