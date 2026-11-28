# Layouts

The editor comes with six layouts, in this order and under Ctrl+1 to Ctrl+6 (`benchfactory.rs`;
the shares are those of a 1920 × 1008 workbench). Each holds only the panels its kind of work uses;
every other panel starts closed (in its `closed` list) and Window, the canvas bar's "Open … again"
or anything that brings a panel into view (a click on the problems of the status bar, the curve
button…) opens it where Window would:

| Layout | Tree |
|---|---|
| Scene | Assets and Scripts (300) · the canvas over Timeline and Problems (168) · Inspector and Effects (500) over Layers (380) |
| Particles | Layers (360) over Particles (320) · the canvas over Curves and Console (270) · Inspector (600) over Cost and See as (390) |
| Scripts | Scripts (280) · Code and Reference over Console and Problems (250) · the canvas (392) over Inputs and Try (580) |
| Simulation | Try (560) over Screens and Problems (300) · the canvas · See as (420) over Inputs (400) |
| Animation | a row of Layers and Assets (300), the canvas and Inspector and Effects (380), 540 high, over a row of Timeline and Curves (560) |
| Wallpaper | Options and Texts and languages (300) · Wallpaper sheet · How it will look (260) over the canvas (400) over Review and Problems (420) |

A panel a layout keeps closed comes back next to the panels it is with in the layout in use or,
when that layout does not hold it, where the full tree of Scene (`benchfull.rs`, which places every
panel once) puts it: with its companions there, or on its side.

The top bar shows the six, each a word with its blueprint (18 × 12 px) and its name in Red Hat
Display, the one in use in Frost and bold with the accent mark under it; when one of the author's
layouts is in use it follows them. When the bar is short of room only the layout in use keeps its
name. A click on a layout, or its key, changes the workbench to it; Ctrl+7, Ctrl+8 and Ctrl+9 choose
the author's first three layouts. Each layout keeps the changes the author makes to it (moved,
stacked, resized or closed panels), apart from the others, and "Reset this layout" (Window and the
canvas bar's "More") gives back the layout in use as it came, or, for one of the author's, as it was
saved. "Save as new layout…" (`SceneLayoutName`) asks for a name and keeps the workbench as it is
under it, as a layout of the author's that is then in use; a name is refused, with its reason in the
dialog, when it is empty, starts or ends with a space or is the name of another of the author's
layouts. "Rename or remove your layouts…" (`SceneLayoutOwn`) lists them with their blueprints,
"Rename…" and "Remove"; "Remove" asks again in its row ("Remove it", coral, or "Cancel"), since it
cannot be undone, and removing the layout in use goes back to Scene. A scene just made opens in
Scene; a scene opened again opens in the layout chosen last (the choice belongs to the window, not
to the scene). A web, image, video or preset package opens in Wallpaper (`benchkits::opening`), in the
same workbench (see "Packages that are not scenes").

The rest is the workbench built by `SceneDocks`: a
tree whose inner nodes are splits (a row or a column, with a share of its length for each child) and
whose leaves are tab groups (`SceneGroup`), each with its panels and the one it shows. There are
twenty-three panels: the canvas (`SceneStage`, with the canvas bar at the top and, if the scene has no
layers, the card "Add a layer" with image, text, time or date and particles, below the notices and
the canvas bar, scrolled when the group is too short for it), Layers, Assets, Options, Try,
the Inspector (`SceneFields`), Effects (`SceneEffectsPanel`), Timeline (`SceneFoot`), Curves
(`SceneCurves`), Problems, See as (`SceneLookPanel`), Inputs (`SceneInputs`), Screens
(`SceneScreensPanel`), the Wallpaper sheet (`SceneSheet`), Texts and languages (`SceneTextsPanel`), How
it will look (`SceneCoverPanel`), Review (`SceneReviewPanel`), Particles, Cost, Scripts, Code, Reference
and Console (see their sections); the tree is that of the layout in use (see "Layouts": the panels it holds; the rest are closed), and the usual layout of a split is that of the layout in use as it came. Each
panel is a unit (`SceneUnit`)
created once and reassigned to the body of the group that holds it, so it keeps its state when it
moves. The canvas is alone in its group and is never stacked with other panels; its strip shows
the tab "Canvas", which cannot be dragged but has the tab menu (menu key, Shift+F10 or right click):
"Move left", "Move right", "Move up" and "Move down" trade its place with what is next to it that way
(`benchcanvas.rs`: at the nearest split of that direction, with its column and share), "Detach into
a window" takes it to a window of its own, and "Stack with…", "Detach the whole group" and "Close
the panel" are off for it. A tab strip is 32 px high, darker than the group, with each tab
an icon and its name (13 px) and a line after it; the current tab takes the colour of the group and
the mark of place, in the accent only in the group that has the keyboard focus. The tabs stay on one
line: those that do not fit go, in their order, into the "⋯" at the end of the strip (the tab shown
always keeps its place), which then lists them with "Move or close …" for the tab shown; otherwise
"⋯" opens the menu of the tab the group shows. Each group clips what it holds below its strip,
with the room of a focus ring around the body, so nothing a panel lays out draws over the strip or
another group.

Three things change what the workbench shows without changing the layout or saving anything. F11
(and "Maximize (F11)" in the tab menu) enlarges the panel that holds the keyboard to the whole
workbench, without edges, and F11 again gives the workbench back. "Detach into a window" in the tab
menu takes the panel (never the canvas) out of the workbench into a window of its own, with "Return
to the workbench"; closing the window returns it too, to its place in the layout, and the window
keeps its size for the next time the same panels are detached, also after the app is closed
(`editor.conf`, `[windows] sizes`: JSON, the panels joined with "+" to `[width, height]`, written
half a second after the size settles; not read or written during the checks; on Wayland the desktop
chooses where it appears). Below 1500 px of window width, when the first child of the layout is a side without the
canvas, that side folds into a rail along the left edge, as wide as a tab strip, with one icon per
panel; a click (or showing the panel from a menu or the status bar) unfolds it over the workbench in
a group of its own, beside the rail, and a click again folds it; the rail's edge does not move. The
status bar lists F6 and F11 among the keys of every panel. F6 and Shift+F6 take the keyboard to the
tab shown in the next or previous group, in the order of the tree (left to right, top to bottom; the
rail is skipped); Ctrl+Tab and Ctrl+Shift+Tab show the next or previous tab of the group that has the
keyboard; Esc folds a panel unfolded from the rail. Window has "Detach into a window" and "Maximize"
(or "Back to the workbench"), with F11 beside it, for the panel that has the keyboard.

The tree, its operations and its sizes are in Rust (`apps/reiwand-app/src/bench*.rs`, the QML type
`Workbench`); `SceneLayout` holds the tree in use. Each group has a floor (the widest floor of its
panels: the canvas `sideRight` by half of it, the Inspector `sideRight`, the others `sideLeft`, and two
canvas bars of height, or, for the timeline while it is unfolded, its tab strip and the whole
timeline); a split gives each child its share and takes what a child needs to reach its floor from
the others in proportion to what they have above theirs; when the floors do not fit, the child that
holds the canvas keeps its floor (or half of the split, whichever is smaller) and the others shrink
below theirs in proportion to their floors. A group that shows the folded timeline takes exactly its
tab strip and the timeline bar, and its edge does not move. The edges between the children of a
split (`SceneGrip`) are dragged, moved with the arrows (16 px) and returned with a double click or
Enter to the place they have in the usual layout (to the middle of the two when the usual layout has
no such split); a drag never takes a child below its floor. In a group too narrow for the timeline
bar on one row (`SceneTimeBar`), its easing and keyframe buttons go to a second row under play, the
time and the playhead slider.

A tab is dragged to a new place (`SceneDrop`). Over the group under the pointer it draws a cross of
five targets (`SceneDropTarget`, each a small plan of the part it takes): the middle one stacks the
panel in that group as its last tab, and the four around it split the group in two, halves of
equal share, with the panel on that side; letting go anywhere else on a group stacks it too. At the
middle of each edge of the workbench a target opens the panel as a new group along that whole edge
(a quarter of it). Only targets that change the layout are drawn: the canvas group has no middle
target, the panel's own group has none either and offers its sides only when it holds other panels.
Before the drop, the zone that the panel will take is painted with the place it will have after the
drop (worked out from the tree it would make), with "With …", "Left of …", "Right of …", "Above …",
"Below …" or "Along this edge of the window". The tab menu (menu key, Shift+F10 or right click,
`SceneTabMenu`) offers "Move left", "Move right", "Move up" and "Move down": the panel joins the
nearest group on that side in the tree, passing over the canvas; if it passed over the canvas and
nothing lies beyond, it opens as a new group along that edge of the whole workbench (a quarter of
it); if nothing lies on that side, it splits its group in two and opens in the half of that side;
when none of this applies (alone in a group at the edge) the choice is disabled. "Stack with…" lists
the other groups but the canvas, named by their panels, and the panel joins the one chosen as its
last tab. "Close the panel" takes it out of the workbench (the canvas is never closed); for each
closed panel the canvas bar's "More" shows "Open … again", which brings it back as the last tab of
the first panel of its usual group that is open, or, when none is, alone on its usual side: along the
left or right edge of the workbench (a quarter of it) or below the canvas (a quarter of the canvas).
A group left without panels closes and a split left with one child gives its place to it; "Window ›
Reset this layout" (also in "More") goes back to the layout as it came, with its panels open.

The layouts are a view preference, with no editor action or `reiwand-ctl` command. They are saved,
400 ms after the last change, in `~/.config/reiwand/workbench.json` as `{"format": 8, "content":
{"chosen", "factory", "own"}}` (`benchdesk.rs`, `benchstore.rs`): `chosen` is the layout in use,
`{"factory": "scene"}` (or `particles`, `scripts`, `simulation`, `animation`, `wallpaper`) or
`{"own": "<name>"}`; `factory` holds the six layouts by those names, each a workbench `{"root",
"closed"}` with the author's changes; `own` lists the author's layouts in their order, each
`{"name", "saved", "view"}`, where `saved` is the workbench as it was saved (what "Reset this
layout" gives back) and `view` the one in use. In a workbench `closed` lists the closed panels and
each node is `{"split": {"dir", "shares", "nodes"}}` or `{"group": {"panels", "shown"}}`. Every
field is written, like every format (`explicit-values.md`). Reading refuses a file of another
format, a missing field (named by its path,
`content.factory.scene.root.split.nodes[0].group.shown`), a tree that breaks the rules (a split of
fewer than two nodes, shares that are not one per node, above 0 and adding up to 1, a split inside
another of the same direction, an empty group, a panel twice or neither in a group nor closed, the
canvas with other panels or closed, a shown panel that is not in its group), a name of the author's
that is empty, starts or ends with a space or is taken twice, and a `chosen` layout of the author's
that is not in `own`, naming the place. An older file is brought up a format at a time
(`benchmend.rs`, `benchstore.rs`): format 1 (one tree) gets an empty `closed`; format 2 (one
workbench, `{"root", "closed"}`) becomes the Scene layout, the five others as they come; formats 3,
4 and 5 get Effects and Curves, then See as, Inputs and Screens, then the four panels of the
wallpaper closed in every workbench whose tree lacks them, and formats 6 and 7 as described below.
A file of this format that does not read whole is read layout by layout: a broken layout of the
six goes back to how it comes, a broken one of the author's is left out, a `chosen` layout that is
gone becomes Scene, the others are kept, and the notice names what was lost; the file is written
again with the next change. Only a file that is not a set of layouts at all, or of a newer format,
is refused: the editor opens with the six layouts as they come, in Scene, shows the reason in its
notice and does not write over the file until "Reset this layout", which writes them. A missing file is the six layouts as they come, but for the Scene layout
while the `arrangement` key that earlier versions kept in `window.conf` (group `editor`:
`{"places": {"left", "right", "bottom"}, "sizes"}`) describes one (`benchold.rs`): its left, right
and bottom panels in groups on those sides of the canvas, at their sizes (within a third of the
workbench), with the panels it did not know closed; panels it names twice or does not know are left
out, and one that names none gives the Scene layout as it comes. Those layouts are written to the
file at once, and only then is the key removed from `window.conf` (its other keys and lines stay,
written beside it first); a key that cannot be removed is converted again next time, to the same
layout, and a file that cannot be written leaves the key where it is.

During a drag of a layer or a handle over the canvas (action `layer` with `grip` or `transform`, as a
gesture), each step applies the action, sends the scene to the preview and publishes only the handles
(`gizmos`, from `outline::moving`); the scene outline (`view`) and the property rows are not rebuilt.
They are rebuilt once when the pointer is released (`hold`) or when the drag stays still for 120 ms,
and the drag remains a single undo step. What the saved scene needs is saved first and says so
("Saved before setting it as the wallpaper.", "Saved before publishing.").

"View as wallpaper" asks on which screen when there is more than one, minimizes the window and leaves
a fixed system notification (`org.freedesktop.Notifications`, without a timeout) with "Back to the
editor", "Keep it" (sets it as the wallpaper for real) and "Stop the preview". If the view ends by
itself (the daemon cuts it or crashes), the notification changes to "The wallpaper view ended" with
"View it again". Without a notification service the window returns and the notice stays inside it.
"Set as wallpaper" asks for the screen the same way and notifies ""Title" is your wallpaper." with
"Undo", which sets back what each screen had. The Assets sections (scripts, templates, effects and
shape groups) fold and remember their state between sessions.
