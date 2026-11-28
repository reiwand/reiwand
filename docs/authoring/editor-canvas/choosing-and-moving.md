# Choosing and moving layers

On this page: choosing layers, moving, scaling, skewing, rotating, tilting and fitting them.

## How the canvas works

- The canvas is the scene's preview. It appears while the wallpaper daemon (reiwandd) is running. If there is no preview, the canvas says so with a text over the background.
- Over the preview the **handles** of the chosen layer are drawn: the outline, small squares at the corners and sides, a circle to rotate it and another for its pivot.
- Clicks and drags are measured in pixels of the scene's canvas, not in screen pixels. What you see enlarged or reduced is still worth the same.
- If the value you move has keyframes, see [How to write over keys](view-and-canvas.md#how-to-write-over-keys-or-add-a-key).

In the terminal, the editor has no canvas, but everything you do with the handles is an action: `layer` with `transform` or with `grip`. See [actions](../actions.md).

## How to choose a layer

Where it is: on the canvas and in the **Layers** tab.

Steps:

1. Click the layer on the canvas. The layer whose outline contains the point is chosen. Layers draw their click area with the real shape of their outline, so a rotated or tilted layer only responds inside its shape.
2. Or click its row in the **Layers** tab.
3. To choose the canvas itself (size, background, glow and parallax), click an empty place on the canvas. The inspector changes to show **Canvas**.

A chosen layer is drawn with the full outline and the handles. A **locked** layer can be chosen, but shows no handles and cannot be dragged (see [How to lock, hide and isolate layers](view-and-canvas.md#how-to-lock-hide-and-isolate-layers)).

In the terminal: there is no “choose”; each action names its layer by its id.

## How to choose several layers

The canvas handles work with one chosen layer. For several layers at once, mark them with Ctrl and click in the layer list or on the canvas.

Where it is: the **Layers** tab or the canvas.

Steps:

1. Click a layer to choose it.
2. Hold Ctrl and click other rows, or other layers on the canvas, to mark them. Ctrl and click on an already marked row unmarks it. A click without Ctrl leaves only the chosen layer.
3. With the selection made, use **Group the layers**, **Make a template** or **Copy the layers**. Those three buttons act on the chosen layer and those marked with Ctrl.

Dragging on the canvas moves only the chosen layer. There is no box selection or moving several layers with one drag; for that, group them and move the group.

Shortcut: Ctrl + click on the layer's row (`SceneLayerRow.qml`).

In the terminal: `group` receives the list of ids; `template` with `make` receives the list of ids. See [actions](../actions.md).

## How to move a layer

Where it is: on the canvas, with the layer chosen.

Steps:

1. Drag the layer from any point of its outline. If it was not chosen, the drag chooses it and moves it.
2. Release. The whole drag counts as a single step of the history.
3. To move it bit by bit, click on the canvas or in the list and press the keyboard arrows. An arrow moves 1 pixel; with Shift, 10 pixels.
4. To type the exact position, use the **Position (px)** field of the **Transform** group in the inspector.

Shortcut: arrows, and Shift with arrows for 10 pixels (`SceneOverlays.qml`).

In the terminal: `layer` with `transform` and `position` sets the exact position; with `grip` and `nudge` it moves one step, like the arrows.


```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"sign"}'
reiwand-ctl scene act my-scene layer '{"transform":{"layer":"sign","position":[960,540],"rotation":15}}'
reiwand-ctl scene act my-scene layer '{"grip":{"layer":"sign","grip":"nudge","to":[10,0]}}'
reiwand-ctl scene act my-scene layer '{"grip":{"layer":"sign","grip":{"scale":{"corner":[1,1],"even":true}},"to":[1200,700]}}'
```

## How to adjust with magnets and guides

Where it is: the canvas bar, the **Magnets** button (on by default) and the **Guide:** list.

Steps:

1. With **Magnets** on, when you drag a layer its edges and its center snap to the edges and the center of the canvas and of the other layers. A colored line marks each alignment.
2. Hold Alt while dragging to place it without snapping.
3. Magnets only affect movement with the mouse. The scale and rotation handles do not snap.
4. In **Guide:** choose a ratio (**No guide**, 16:9, 21:9, 32:9, 4:3, 16:10 or 9:16). A frame of that ratio is drawn inside the canvas, to see what stays in view on screens of that shape. The guide is not saved in the scene.

Shortcut: Alt while dragging (`SceneGizmo.qml`).

In the terminal: `layer` with `grip` accepts `snap` (distance in pixels) when the grip is `move`.

## How to scale and stretch a layer

Where it is: on the canvas, the square handles of the chosen layer.

Steps:

1. Drag a corner handle to scale from the pivot. With Shift the proportions are kept.
2. Drag a handle in the middle of a side to stretch only that side.
3. Or type **Scale** in the **Transform** group of the inspector.

Shortcut: Shift when dragging a corner (`SceneGizmoKnobs.qml`).

In the terminal: `layer` with `grip` and `scale` (option `even` to keep proportions), or `transform` with `scale`.

## How to skew a layer

Where it is: on the canvas, the handles in the middle of the sides.

Steps:

1. Hold Ctrl and drag the handle in the middle of a side. The layer skews along that edge.
2. You can also type the two-value **Tilt** field without a unit in the **Transform** group. That is the skew. The other field called **Tilt**, in degrees, is the tilt in depth (see [How to tilt and bring a layer closer in depth](#how-to-tilt-and-bring-a-layer-closer-in-depth)).

Shortcut: Ctrl when dragging a side's handle (`SceneGizmoKnobs.qml`).

In the terminal: `layer` with `grip` and `skew`, or `transform` with `skew`.

## How to rotate a layer and move its pivot

Where it is: on the canvas. The rotation circle is above the top edge of the layer. The pivot is a circle of another color at the center of rotation.

Steps:

1. Drag the rotation circle: the layer rotates around the pivot.
2. To change the center of rotation and scale, drag the pivot circle. The layer stays where it is; only its **Origin** changes. On a tilted layer the image may shift a few pixels when you move the pivot.
3. Or type **Rotation (°)** and **Origin** in the **Transform** group.

In the terminal: `layer` with `grip` and `turn` or `origin`, or `transform` with `rotation` and `origin`.

## How to tilt and bring a layer closer in depth

Where it is: on the canvas, near the pivot of the chosen layer. There is a ring of dots and three small handles joined to the pivot by lines.

Steps:

1. Drag the end of the vertical axis toward the pivot or away from it: it tilts the layer about its width.
2. Drag the end of the horizontal axis: it tilts the layer about its height.
3. Drag the diamond-shaped handle along the line that comes out of the layer: it brings it closer to the viewer or moves it away. That value is the **Depth (px)**.
4. Each handle shows the current degrees or pixels in its help. The same values are in the **Transform** group of the inspector (**Tilt** in degrees and **Depth (px)**).

A tilted layer is drawn with its projected outline, with a dashed stroke.

In the terminal: `layer` with `grip` and `tilt` (with `axis` 0 or 1) or `lift`, or `transform` with `tilt` and `z`.

## How to fit a layer to the canvas

Where it is: the canvas bar, the **Fit to the canvas** button (active if a layer is chosen and it is not locked).

Steps:

1. Choose the layer.
2. Open the button and choose **Cover the canvas** (fills the canvas, cropping what is left over), **Fit inside the canvas** (it is seen whole) or **Stretch to the canvas** (fills without respecting the proportion).

In the terminal: `layer` with `fit` (`cover`, `contain` or `stretch`).


```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"background"}'
reiwand-ctl scene act my-scene layer '{"fit":{"layer":"background","fit":"cover"}}'
```
