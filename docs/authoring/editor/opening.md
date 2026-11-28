# The editor and its window

On this page: what the editor touches, the window and opening a scene folder.

## What a scene is and what the editor touches

A scene is a folder. Inside there is a file with the package data (title, license, credits, properties), a file with the scene (layers, effects, particles, clocks) and an `assets` folder with the images, videos, sounds and models it uses. The editor changes that folder; there is no editor-specific format.

While you edit, the changes live in the editor's memory. They do not reach the files until you click **Save**. The terminal is different: each `scene act` command saves immediately.

## What the window looks like

The top bar shows the title of the scene or package, with no other word before it, next to the way back to the page you came from (**‹ Create**, for example). The window has five areas.

- **Top bar**: the way back (**‹ Create** or the page you came from), the title with its state (**Saved** or **Unsaved ·** with **Save now**), the menus **File**, **Edit**, **Layer**, **View** and **Window**, in the middle the layouts (**Scene**, **Particles**, **Scripts**, **Simulation**, **Animation** and **Wallpaper**, each with a small drawing of its groups before its name; the one in use is lit and underlined), then **Undo**, **Redo** and, on the right, **View as wallpaper**, **Set as wallpaper** and **Publish…**. In a window narrower than 1500 px the five menus fold into **Menu**, and **Undo** and **Redo** stay in **Edit** and in their keys. Packages that are not scenes (image, video, web page and preset) have the same bar, with its menus (no **Layer**) and layouts; **View as wallpaper** is not in a preset, which cannot be viewed on its own.
In the **Scene** layout, the one a new scene opens in:

- **Left group**, with two tabs:
  - **Assets**: the scene's files ([canvas](../editor-canvas.md)). The words above the list (**All**, **Images**, **Sounds**…) show one kind of asset, and **Search for an asset** finds one by its name.
  - **Scripts**: the scene's scripts, what they declare and the `scene.spawn` templates ([scripts](../scripts.md)).

  The other panels (**Options**, **Try**, **Code**, **Console**, **Cost**…) start closed in this layout: **Window** opens each one, and they are already open in the layouts made for that work (**Try** in **Simulation**, **Code** in **Scripts**…).
- **Center**: at the top the canvas bar, in the middle the canvas with the preview, and below it a group with two tabs: **Timeline** ([animation](../editor-animation.md)) and **Problems**, with its number.
- **Right side**: the **Inspector** tab, which shows the fields of whatever you have chosen, and below it **Layers**, the list of layers and its tools ([canvas](../editor-canvas.md)). Each layer is one row with a small picture of it, its name, what it is and the buttons that hide and lock it.
- **Status bar**, at the bottom: whether the scene is saved, how many problems it has (a click opens **Problems**), how the preview is seen (**See as: …**), the size of the canvas with the layout in use and, on the right, the keys of the panel that has the keyboard: the arrows, **Shift** and **Esc** on the canvas; the arrows, **Home**, **End**, **Del** and the menu key in Layers; the arrows, **Home** and **End** in Assets; **Tab** in the others. **Alt** is always there: it takes you to the menus.
- **Notices**, above everything else: a red message with the editor's error if something fails, a green message with the result of the last thing you did (with a cross to close it) and the recovery notice (see [How to save and recover changes](creating-and-saving.md#how-to-save-and-recover-changes)).

The editor takes its colors from the thumbnail of the scene or package you edit (the app's own colors while it has none), and gives the app its colors back when you leave it. It is dark; **View** › **Light editor**, also in Settings › **Wallpapers** › **Making wallpapers**, draws it as dark ink on light paper in the same hues. Only the editor changes, and the choice stays for every scene and package until you change it again.

Each menu opens with a click, with **Alt** and its underlined letter (**Alt+F** for **File**, **Alt+E** for **Edit**, **Alt+L** for **Layer**, **Alt+V** for **View**, **Alt+W** for **Window**), or with **Alt** alone, which takes you to **File**: the arrows move between the menus, **Down** opens one, and **Esc** returns to where you were. While a menu is open, **Left** and **Right** open the one next to it. The menus gather what the panels and bars already do: **File** saves and imports (the same as the buttons in Assets) and opens **Capture**; **Edit** undoes and redoes, naming the step, and renames, duplicates, copies, pastes or removes the chosen layers; **Layer** adds a layer and groups, ungroups, hides, locks, raises or lowers the chosen ones; **View** zooms, fits, tests the pointer and opens the guide, several screens and **See as**, and at its end turns on **Light editor** (or back to **Dark editor**); in a package that is not a scene, it holds only that; **Window** lists the panels, with a mark on those that are open (choosing one closes it or opens it again), then the layouts with their drawings and keys, **Save as new layout…**, **Rename or remove your layouts…** and **Reset this layout**.

The window is a workbench: each area is a group of panels with its tabs, and the groups sit side by side or one above the other. The panels adjust to the size of the window, and none goes below its smallest size; the canvas never shrinks below its own. To change the size of two groups, drag the edge between them (a double click returns it to its usual place) or choose it with **Tab** and move it with the arrow keys (**Enter** also returns it). While the timeline is folded, the group that shows it keeps the height of its bar.

To move a panel, drag its tab. Over the group under the pointer a cross of five targets appears: the middle one adds the panel to that group as one more tab, and the four around it split the group in two and put the panel on that side. At the middle of each edge of the window a target puts the panel along that whole edge. Before you let go, the place the panel will take is painted, with what it will do (**With Layers**, **Below Inspector**, **Along this edge of the window**…). Letting go anywhere else on a group adds it as one more tab. With the keyboard, choose the tab and press the menu key (or **Shift+F10**, or a right click) for **Move left**, **Move right**, **Move up** or **Move down**: the panel joins the next group on that side, passing over the canvas; when there is no group past the canvas it opens on its own along that edge of the window, and when there is none on that side at all it opens next to its group, which is split in two. **Stack with…** in the same menu lists the other groups by their panels and adds the panel to the one you choose. A choice that cannot do anything is greyed out. A group left with no panels closes. The canvas always stays alone in its group; its group shows the **Canvas** tab, which cannot be dragged, but the canvas moves and goes to a window of its own with the menu of that tab (see below). Each tab shows an icon before its name, and **⋯** at the end of each strip opens the menu of the tab that the group shows. When a group is too narrow for all its tabs, those that do not fit go into **⋯**, which then lists them.

The tab of the canvas is not dragged, but it has the same menu (right click, menu key or **Shift+F10**): **Move left**, **Move right**, **Move up** and **Move down** trade its place with what is next to it on that side (the Timeline under it goes with it), and it is never stacked with other panels.

**Detach into a window**, in the menu of a tab, puts that panel in a window of its own, which you can place anywhere, also on another screen, and which opens again at the size you gave it, also after you close the app; **Detach the whole group** takes all the tabs of the group with it (not the group of the canvas). The canvas can go to a window of its own too. **Return to the workbench** in that window, or closing it, puts the panel back where it was. **Maximize (F11)** in the same menu, or **F11**, makes the panel that has the keyboard fill the window; **F11** again, or **Back to the workbench** (in the tab menu **Back to the workbench (F11)**), returns.

**Close the panel**, in the same menu, takes the panel out of the window (the canvas cannot be closed). To bring it back, choose it in **Window**, or open **More** in the canvas bar and choose **Open** and its name **again** (for example **Open Assets again**; each closed panel has its line): it returns next to the panels it is usually with in the layout in use, or on its usual side if they are closed too. **Reset this layout** opens every panel again in its usual place.

### Layouts

A layout is a way of placing the panels for one kind of work. Each shows only the panels that work uses; the others start closed and **Window** opens any of them. The editor comes with six, in the top bar and in **Window**, each with a small drawing of its groups (the filled box is the canvas):

| Layout | Key | What it puts where |
|---|---|---|
| **Scene** | **Ctrl+1** | Assets and Scripts on the left, the canvas in the middle with Timeline and Problems below it, Inspector and Layers on the right |
| **Particles** | **Ctrl+2** | Layers over Particles on the left, the canvas with Curves and Console below, Inspector over Cost and See as on the right |
| **Scripts** | **Ctrl+3** | Scripts on the left, Code (with Reference) in the middle with Console and Problems below, the canvas on the right with Inputs and Try below it |
| **Simulation** | **Ctrl+4** | Try over Screens and Problems on the left, a large canvas in the middle, See as over Inputs on the right |
| **Animation** | **Ctrl+5** | Layers and Assets, the canvas and Inspector at the top, and a wide Timeline with Curves at the bottom |
| **Wallpaper** | **Ctrl+6** | Options and Texts and languages on the left, the Wallpaper sheet in the middle, How it will look over the canvas over Review and Problems on the right |

Click a layout, or press its key, to change the window to it: the scene stays as it is, only the panels move. Each layout remembers what you change in it (moved, stacked, resized or closed panels), without touching the others, and **Window** › **Reset this layout**, also in **More** in the canvas bar, returns the layout in use to how it came. A new scene opens in **Scene**; a scene you open again opens in the layout you used last.

**Window** › **Save as new layout…** keeps the panels as they are now under a name of your choice, and that layout is then in use; **Ctrl+7**, **Ctrl+8** and **Ctrl+9** choose your first three. A name cannot be empty, start or end with a space, or repeat the name of another of your layouts. **Window** › **Rename or remove your layouts…** lists them with **Rename…** and **Remove**; removing one asks again, since it cannot be undone. **Reset this layout** returns one of yours to how you saved it.

The editor keeps the layouts, with your changes and your own layouts, in `~/.config/reiwand/workbench.json`. A file from an earlier version is brought up to date with your arrangement kept, and the panels that version did not have start closed (the **Window** menu opens them). If one layout in the file is broken, only that one goes back to how it comes (one of your own is left out) and the editor says which; your other layouts are kept. If the file cannot be read at all, or was written by a newer version, the editor says what and where, opens with the layouts as they come and leaves the file as it is: what you change is not saved until you choose **Reset this layout**, which writes the layouts over it. The first time, before that file exists, the **Scene** layout takes the panels and sizes that earlier versions kept in `window.conf`, and once they are saved in `workbench.json` the old setting is removed from `window.conf`. Pages of packages that are not scenes (image, video, web page and preset) keep their layouts apart, one file per kind next to it (`workbench-image.json`, `workbench-video.json`, `workbench-web.json`, `workbench-preset.json`): moving a panel on a web page package changes only the layouts of web pages, never those of your scenes.

### The canvas bar

Almost all its buttons show only the drawing; the name appears when you hover. From left to right:

- **Zoom out**, the zoom percentage, **Zoom in**, **Fit** (returns to 100 % and centers) and **Test the pointer and clicks** (a switch; see [How to view the scene at each quality level](checking-and-viewing.md#how-to-view-the-scene-at-each-quality-level)).
- **Guide:** a dropdown with **No guide** and the ratios 16:9, 21:9, 32:9, 4:3, 16:10 and 9:16. It draws the frame of that ratio over the canvas.
- **Canvas and clock**, **2D camera**, **Try the parallax**, **Magnets**, **Fit to the canvas**, **Follow a layer** and **Stop following** ([canvas](../editor-canvas.md)).
- **Preview level** (see below).
- **Write over keyframes**: a switch. Off, the canvas handles add a key at the playhead to animated values and do not touch linked values. On, they write a fixed value over the keys and the links.
- **Screens:** a dropdown to see the scene spread over several screens (**One screen**, **Two side by side**, **Three side by side**, **Two stacked**, **My screens**). With more than one screen, the **Spread over every screen** checkbox appears, which saves that setting in the package. The **Screens** panel, in the **Simulation** layout, offers the same choices and, with more than one screen, a map of them: the thumbnail of the scene cut into what each screen shows, with the screen's name.

With the mouse on the canvas: the wheel scrolls the view, **Ctrl** and the wheel zoom in or out, and dragging with the middle or right button moves the view.

A right click opens a menu with what can be done with what is underneath: over one or several chosen layers (in the list or on the canvas) it offers duplicate, group, hide, lock, rename, raise or lower, copy, paste and remove; over the empty canvas, paste and **Add a layer**; over a key or a row of the timeline and over a file in **Assets**, what their buttons already offer. If the layer you clicked was not chosen, it is chosen first; if it was one of several, the menu acts on all of them, and removing several is undone in one go. The menu key or **Shift+F10** open the same menu over what is chosen, and **Esc** closes it.

If the canvas does not show the scene, read what it says. **The preview appears here while reiwandd runs.** means the daemon is missing. **Install the KPipeWire QML module to see the preview here, or use Show on the desktop.** means your system lacks that module.

## How to open a scene folder

Where it is: the **Create** page of the app's menu.

Steps:

1. If the project is under **Continue with yours** or **All my projects** (at the end of that row), choose it and click **Keep editing**. That list has the projects you opened and those in the `reiwand` folder of your Documents (or of your home folder if you have no Documents).
2. If it is not there, click **Open a project…** and choose the project's folder: the one with its `manifest.json` (and, if it is a scene, its `scene.json`). If you choose another, the notice tells you what was expected.
3. The app looks at what is inside. A scene or an assets package opens in this editor. A web page, a preset, an image or a video opens on the package page ([package](../package.md)).

If the folder has unsaved changes from an earlier session, the editor says so when you open it (see [How to save and recover changes](creating-and-saving.md#how-to-save-and-recover-changes)).

In the terminal: `reiwand-ctl scene open DIR` describes the folder. All the `scene` commands take the folder as the first argument. See [cli](../cli.md).
