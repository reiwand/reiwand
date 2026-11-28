# Problems and previews

On this page: the problems list, the preview, the desktop and quality levels.

## How to read the problems list

Where it is: under the canvas, next to **Timeline**, in the **Scene** layout. The title says **Problems (N)** with the number, or **No problems** and then the list is two lines tall.

Each row is a problem, with an icon and the message in your language.

- Error icon (red): it has to be fixed. While there is one, the package cannot be created.
- Warning icon (yellow): it prevents nothing, but it is worth a look.

What the warnings and errors say:

- Scene values that are not valid, or that point to something that does not exist (a layer, a file, a timeline, a property).
- Package data that is missing or wrong: title, license, authors, credits, package texts that cannot be found.
- Which platforms refuse the package or play it without some feature, in the format “<platform>: refused: …” or “<platform>: plays without …”.
- `assets` files that no part of the scene uses, or a signature left by an earlier build.
- A suggestion about flashes, if the scene flickers a lot.
- A script that uses something (sound, pointer…) without declaring it: **… uses … but does not declare it; declare it in its uses**.
- A script that does not compile (a syntax error, with its file and line) or that `scene.json` names and is not in the folder. The editor compiles the scripts without running them, like `reiwand-ctl pack check`.

Steps:

1. Click a row. The editor chooses the layer, property or preset where the fault is and marks the field with a red line in the inspector.
2. Fix the value.
3. Some rows have a **Fix** button on the right. Clicking it does the fix for you (for example, declaring what a script is missing) and counts as a step that can be undone.

Below the list a notice appears while the preview runs: **A script wants … but it is not granted
here, so that part of the scene is off while you try it.**

The **Console** panel shows, while the preview runs, the error of a script that stopped (with *Go to
the line*, which opens it in **Code**) and what the scripts write with `console.log`: *All*,
*Warnings* or *Errors*, errors in red and warnings in amber; clicking a line opens its script.

In the terminal: `reiwand-ctl pack check DIR` lists the same problems with the field's pointer and exits with 1 if something is wrong. With `--json` it gives one object per problem. `reiwand-ctl scene preview DIR --console` shows the scripts' console. See [cli](../cli.md).

## How to view the scene in the preview and on the desktop

Where it is: the canvas and **Show on the desktop** in the top bar.

The canvas shows the scene drawn by the daemon, as it would look as a wallpaper. It is redrawn with each change, even if you have not saved it. The playhead's time decides which second is shown; if the time strip is paused, the image stays still at that second.

Steps to see it on your real screens:

1. Click **Show on the desktop**. The switch turns on and all your screens show the scene without applying it; over the canvas the notice **This scene is shown on the desktop as a preview and is not applied. Stop the preview when you are done.** appears, with the **Stop the preview** button.
2. Click it again (or use that button) to turn it off. It also turns off when you close the editor, and if the preview is ended from outside (`reiwand-ctl scene preview DIR --stop`, another scene on the screen), the editor says so and turns the switch off.

Steps to test the mouse:

1. Click **Test the pointer and clicks** in the canvas bar.
2. Move the mouse over the canvas and click. The scene receives a simulated pointer: a ring is seen where it is, red while you press, and each click launches a pulse from where it landed. That way you see how far a click reaches.
3. Turn the switch off to go back to editing with the mouse.

In the terminal: `reiwand-ctl scene preview DIR` shows the saved folder for 60 seconds (`--stop` ends it, `-o` chooses the output). It needs the daemon. For an image without a desktop, `reiwand-ctl scene render`.

## How to view the scene at each quality level

The daemon draws the scene at one of four levels: **Q0, lowest**, **Q1**, **Q2** and **Q3, highest**. A low level draws smaller and leaves out what weighs most; that way the scene stays smooth on a weak computer or on battery. The editor lets you see how it looks at each one.

Where it is: the **Preview level** button in the canvas bar. It opens a panel. It closes with **Escape**.

Steps:

1. Open the panel.
2. In **Preview quality** choose the level. The preview is redrawn right away. **Restart** starts the scene and the particles over from the beginning.
3. Check **On battery** to see it as the daemon draws it on battery: it lowers Q3 to Q2 and draws at most 60 frames per second.
4. Check **Less motion** to see it with reduced motion: the parallax and the camera shake stay still.
5. Read the summary that appears below the checkboxes: **Drawn as Q2 at … % of the size, with … effects left out**. If the scene has glow above white, it says whether that level draws it. If there are effects or particles that follow the pointer, it names them so you can test them with **Test the pointer and clicks**.
6. Below the summary there is a line for each effect: **<effect> on <layer>: <n> units of work** or **… left out of the scene at this level**. Those that are left out appear in bold.

To compare two levels, or with and without glow:

1. In **Compare with** choose the second level. Check **Without the glow of the canvas** to compare the same level without the glow.
2. Hold down **Hold to see the other side**. While you hold it, the preview shows the second look; when you release, it goes back to the first.

To see the glow split live over the canvas, click **Compare the glow** in the canvas bar (it only appears if the scene has glow). The canvas is split with a line: on the left it is seen with the glow and on the right without it. Drag the line's handle to move the cut. Click the button again to go back to the normal view. In the terminal: `reiwand-ctl scene preview DIR --without-bloom`.

To save what you see:

1. Optionally check **Move a pointer across and click** (a simulated pointer crosses the canvas and clicks about halfway through the clip, so that what follows the pointer moves in the file too) and **Split the picture with the other side** (the left with the level above and the right with that of **Compare with**).
2. Click **Save a picture…** (one frame of the second where the playhead is; PNG, JPG or WebP) or **Save a clip…** (90 frames at 30 per second; an animated GIF or, with another extension, an image sequence).
3. Below it appears **Saved: <file>**.

Check **Set the time the clock shows** and choose the hour and minutes so that clocks and scripts see that time in the file (unchecked, your computer's). If the preview's simulated player is on, the file shows it with its cover. All this is drawn from what you have in the editor, saved or not, with the same things you see in the preview: the solo layer, the particle seed and the scripts.

In the terminal: `reiwand-ctl scene preview DIR --quality Q [--battery] [--calm] [--without-bloom] [--restart]` and `reiwand-ctl scene render DIR --out FILE [--time-of-day TIME] [--media TRACK.json] [--quality Q] [--battery] [--calm] [--frames N] [--pointer X,Y[:X,Y]] [--click] [--against Q] [--without-bloom] [--split F]`. See [cli](../cli.md).
