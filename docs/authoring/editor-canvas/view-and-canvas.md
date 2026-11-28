# The view and the canvas

On this page: handles, zoom, locking and hiding, keys, canvas size, glow, parallax, the pointer and a 2D camera.

## How to change the size of what an adjustment captures

Where it is: on the canvas, when you choose an **Adjustment** layer that has a size of its own.

Steps:

1. The rectangle the layer captures is drawn, with handles at the corners and sides. A text says **Captures X × Y px; the most is ...**.
2. Drag a handle: it changes the size of the captured part and the layer keeps its scale. The normal scale handles are not drawn on this layer.

See [effects](../editor-effects.md) for the rest of the adjustment layers.

## How to move the handles of shapes and texts

Where it is: on the canvas, when you choose a **Shape** or **Text** layer.

- A shape shows handles of its own: width, height, size (with Shift it keeps the proportions), rounded corners, ring thickness, number of sides or points, depth of the points, start and end of a line, outline thickness and the ends of the gradient. Each handle explains in its help what it does (**Set the width**, **Round the corners**...).
- A text shows a handle on the right edge to set its wrapping width (**Set the width of the text; double-click to stop wrapping**) and a label with **X × Y px, N lines**.

How to draw and edit shapes and texts is in [drawing](../editor-drawing.md).

In the terminal: `layer` with `figure` (`shape` or `text_width`).

## How to zoom in, zoom out and scroll the canvas

Where it is: the canvas bar, on the left: **Zoom out**, the percentage, **Zoom in** and **Fit**.

Steps:

1. **Zoom in** and **Zoom out** change the zoom in steps of a quarter. The zoom goes from 10 % to 1600 %.
2. **Fit** returns to 100 % and centers the canvas.
3. With the mouse: Ctrl and wheel change the zoom (10 % per notch); the wheel alone scrolls the view; dragging with the middle or right button also scrolls it.

Zoom and scroll only change what you see in the editor; they are not saved in the scene.

Shortcut: Ctrl + wheel for zoom (`SceneCanvas.qml`).

In the terminal: it has no equivalent.

## How to lock, hide and isolate layers

Where it is: in each row of the **Layers** tab there are three buttons: **Hide**, **Lock** and **Solo**.

Steps:

1. **Hide**: the layer stops being drawn and its name looks dimmed. Click it again to show it.
2. **Lock**: the layer cannot be moved with the mouse, shows no handles, and the canvas bar buttons that act on the chosen layer (**Fit to the canvas**, **Follow a layer**, **Stop following**) turn off. It is for not moving a background by accident.
3. **Solo**: the scene shows only that layer. When you release the button, all are shown.

A button that is not activated looks fainter than an active one.

In the terminal: `hide` (with `hidden`), `lock` (with `locked`) and `solo` (without `id` to remove the isolation).


```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"sign"}'
reiwand-ctl scene act my-scene hide '{"id":"sign","hidden":true}'
reiwand-ctl scene act my-scene lock '{"id":"sign","locked":true}'
reiwand-ctl scene act my-scene solo '{"id":"sign"}'
reiwand-ctl scene act my-scene solo '{}'
```

## How to write over keys or add a key

Where it is: the canvas bar, the **Write over keyframes** button.

When a value of the layer (position, rotation...) has keyframes, the canvas handles cannot just change it. What they do depends on this button:

- Off: the handles add a key at the playhead to animated values and do not touch linked values.
- On: the handles write a fixed value over the keys and the links.

In the terminal: the `layer` actions accept `at` (instant of the key) and `replace` (write over).

## How to change the canvas size and the scene clock

Where it is: the canvas bar, the **Canvas and clock** button.

Steps:

1. Open the button. In **Screen size** choose a size from the list: HD, Full HD, QHD, 4K UHD, 5K, Ultrawide 21:9, Super ultrawide 32:9, 16:10, 4:3 or Phone 9:19.5. The active size has a mark.
2. **Use the playhead as the still frame (x s)** takes the instant where the playhead is as the scene's still image.
3. **A new seed each time the scene starts** makes whatever uses random numbers different at every start. Off, it is always the same.

The other canvas options (background, glow and parallax) are in the inspector when you choose **Canvas**. See [scene](../scene.md).

In the terminal: `canvas` with `size`, `clock` (with `poster`, `seed` or `fresh`).

## How to turn on the canvas glow and parallax

Where it is: inspector, when you click an empty place on the canvas (**Canvas**).

Steps:

1. **Brightness** switch: **Bright parts glow**. When you turn it on, **Threshold**, **Strength** and **Radius** appear.
2. **Parallax** switch: **Layers move in depth**. When you turn it on, **Depth**, **Delay**, **Pointer** and **Offset** appear (two sliders, one for each direction).
3. Each value, except **Delay**, has a link button so that it follows a source instead of a fixed number.
4. **Follow the tilt of a phone** checkbox: the parallax also reacts to the phone's tilt.
5. Each layer decides how much it moves with its **Parallax depth:** field in the layer's inspector.

In the terminal: `canvas` with `bloom` and `parallax`.

## How to try the parallax on the canvas

Where it is: the canvas bar, the **Try the parallax** button. It only appears if the canvas parallax is on.

Steps:

1. Turn the button on. A **Pointer** circle appears in the center of the canvas.
2. Drag it: each layer moves according to its parallax depth, as if the pointer were there.
3. If the parallax follows the phone's tilt, a **Tilt** square also appears and a dashed line from the center. Drag it away from the center to tilt the simulated phone toward that side.
4. With a layer chosen and the button on, a green **Depth** circle appears with a dashed line from the layer's center. Drag it to set the layer's parallax depth: each quarter of the canvas width is worth 1, from -10 to 10. If the depth is the same across and up, the circle moves diagonally and writes the same value in both; hold Shift to set them separately. A depth that follows a link or keyframes has no handle.
5. Even if the button is off, if you choose **Canvas** a **Parallax center** circle appears with a line from the canvas center: drag it to set where the parallax rests (**Offset**), from -1 to 1 in each direction.
6. Turn the button off to return to the normal view.

In the terminal: `canvas` with `probe` (`pointer`, `tilt` or `off`) and with `parallax` and its `offset`, and `layer` with `depth`.

## How to test the pointer and clicks

Where it is: the canvas bar, to the right of **Fit**, the **Test the pointer and clicks** button.

Steps:

1. Turn it on. While it is on, the mouse over the preview stops editing and becomes a simulated pointer: a circle marks where it is and a red ring expands at each click.
2. Move and click over the preview to test what reacts to the pointer (links to the pointer, clicks, entering and leaving an area).
3. Turn it off, or take the mouse out of the preview, and the simulated pointer goes away.

In the terminal: `reiwand-ctl input touch` with the position and the buttons; see [cli](../cli.md).

## How to use a 2D camera on the canvas

Where it is: the canvas bar, the **2D camera** button.

Steps:

1. Open the button and choose **Add a 2D camera**.
2. With the camera added, the same menu offers **View through the camera** (the canvas shows the scene as the camera sees it), **View the whole canvas with the camera frame** and **Remove the camera**.
3. In the whole view the camera's frame appears with three handles: a corner for the zoom (**Camera zoom**), a **Camera** circle at the center to move it and another to rotate it.

In the terminal: `canvas` with `camera`, `camera_grip` and `camera_view`.
