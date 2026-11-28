# Sprites, rest rules and shortcuts

On this page: sprite sheets, when a scene wakes up and the shortcut summary.

## How to animate a sprite sheet

A sprite sheet is an image with several frames, which play as a frame-by-frame animation.

Where it is: the assets panel, the **Cut sprites…** button (grid icon) in an image's row. **Cut the sprites of** the asset's name opens.

Steps:

1. To cut in a grid, set **Rows**, **Columns**, **Margin**, **Spacing** and **Frames (0 for all)**; choose the reading order (for example **Left to right, top to bottom**) and click **Cut the grid**.
2. To draw a frame by hand, drag over the image. Drag a frame to move it and its corner to resize it.
3. In the strip below, the preview plays the frames. **Pause** / **Play** stops it. **Frames per second** sets the speed and **Frames played** how many of the strip are used.
4. Choose a frame in the strip and use **Earlier** or **Later** to change its order, or **Remove the frame** to delete it. **Remove all frames** empties the sheet.

Drawing and editing the frames are also in [drawing](../editor-drawing.md).

In the terminal: the `sprites` action (`grid`, `rects`, `add`, `change`, `remove`, `reorder`, `play`, `clear`). See [actions](../actions.md).

## How to decide when a scene wakes up (an effect's rest rules)

A scene sleeps when nothing in it needs to move. A package effect says which parameters need to wake the scene and why.

Where it is: the inspector of an effect, rules field.

What you see: a row per parameter with three selectors.

1. **Asks for** (the first): which input the parameter needs while it is not zero. Options: **Nothing**, **Time**, **The pointer**, **The sound**, **Clicks**.
2. **Waits for** another parameter (or **Waits for nothing**): the parameter only asks when the other one is not zero either.
3. **Yields to** another parameter (or **Yields to nothing**): the parameter steps aside when the other one asks.

About the built-in effects: their rules are shown, but they cannot be changed. The text says: **A built-in effect keeps these rules; an effect of the package changes them here.** If the effect always asks for something, it reads “Always asks for: ...”. The scene sleeps when no parameter asks for anything.

In the terminal: the `effect_rule` action (`effect`, `param`, `uses`, `waits_for`, `yields_to`). See [actions](../actions.md) and [effects](../editor-effects.md).

## Shortcut summary

There are keyboard shortcuts only inside the **Curve editor**:

| Shortcut | What it does |
| --- | --- |
| Click | Chooses a key |
| Ctrl or Shift + click | Adds or removes a key from the selection |
| Alt + drag | Scales the values of the chosen keys |
| Ctrl+C | Copy |
| Ctrl+V | Paste at the playhead |
| Delete | Remove the chosen keys |
| F | Frame the chosen ones |
| Home | Frame everything |
| Wheel | Zooms the time in or out |
| Ctrl + wheel | Zooms the value in or out |
| Shift + wheel | Scrolls the time |
| Middle button + drag | Moves the view |

In the clips strip, Ctrl when starting to drag a key copies it instead of moving it.
