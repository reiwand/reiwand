# Masks, maps and glow

On this page: masks and their handles, painted maps, gradients, depth maps and glow.

## How to limit an effect, or a layer, with a mask

A mask decides where the effect is seen. Where the mask is full the effect acts completely; where it is empty it does not act. It can be put on an effect or on the whole layer.

**Where it is:** inspector. With an effect chosen the field is called **Mask of the effect:**; with a layer chosen, **Mask of the layer:**.

**Steps:**

1. Choose the effect (or the layer).
2. In **Mask of the effect:** choose the source:
   - **None**: removes the mask.
   - **A picture**: uses an image or video of the package (it starts with the first one; change it in **Picture:**). The package needs to have one.
   - **A layer**: uses another layer of the scene (it starts with the first one; change it in **Layer:**). The effect acts where that layer is on the screen. There has to be another layer.
   - **A shape**: draws a shape. Choose which in **Shape:**: **Rectangle**, **Ellipse**, **Ring**, **Polygon**, **Star** or **Line**. It starts centered on the layer and at half its size.
3. **It looks like this:** shows a thumbnail of the mask.
4. Adjust the mask:
   - **Channel** (only with a picture or layer): which channel is read, **Brightness**, **Red**, **Green**, **Blue** or **Alpha**. With a picture it starts at **Brightness**; with a layer, at **Alpha**.
   - **Invert: apply the effect where the mask is empty**.
   - **Soft edge** (only with a shape): how many pixels the edge is feathered; a slider and a box.
   - **Strength**: from 0 to 1; how much the mask weighs.
5. The canvas draws the mask over the layer while the effect, the layer or the mask is chosen: the layer's box in dashed stroke and, with a picture, the channel being read tinted over the layer; with a layer, the masking layer framed; with a shape, its outline filled where the effect acts.

**In the terminal:** the `effects` action with `mask` (for the whole layer, without `index`); also `reiwand-ctl scene act DIR effects`. See [actions](../actions.md).

## How to change a mask's size and edge with the handles

**Where it is:** the canvas, with a shape mask chosen (the effect, the layer or its mask). Picture and layer masks have no handles.

**Steps:**

1. Choose the layer or the effect whose mask is a **shape**.
2. Drag the square handle at the corner: **Drag to size the mask**. On a line it is the end.
3. Drag the point beyond the edge: **Soft edge … px: drag away from the shape to soften its edge**. It is the same value as the inspector's **Soft edge**; the canvas draws the range as two dashed outlines, one larger and one smaller than the shape.
4. Depending on the shape there is one more handle:
   - **Ring**: **Drag to widen or narrow the ring**.
   - **Rectangle**: **Drag to round the corners**.
   - **Star**: **Drag to sharpen or blunt the points**.
5. Each drag is a single **Undo** step.

The canvas repeats what the inspector says: “The canvas shows the mask over the layer: drag the handles of a shape to size it and the point next to its edge to soften it.”

**In the terminal:** the `effects` action with `mask_grip` (which handle and where it is taken). See [actions](../actions.md).

## How to paint a mask, a gray map or a direction map

Some effects read an image that you paint: a gray map that says where something acts, or a direction (flow) map that says where it moves. The brush writes into a package image; you do not need to have one beforehand. The mask brush paints in coordinates of the layer's box: on a mesh it matches the sheet only in the rest pose; outside it, pause the clip to paint.

**Where it is:** inspector, on the effect's image parameter (for example `map`, `phase` or that of a package effect that declares a mask parameter). The paint button appears next to the parameter.

**Steps for a gray map:**

1. Choose the effect and find the image parameter.
2. Click **Paint in grey** (on a mask parameter, **Paint**). The button stays pressed.
3. Adjust:
   - **Erase**: the brush paints 0.
   - **Brush size**.
   - **Gray** (or **Value**, on a mask parameter): how much the brush paints, from 0 to 1.
   - Only on a mask parameter: **Hardness** and, if the image has several channels, the channel dropdown (**Red**, **Green**, **Blue**, **Alpha**).
4. Paint over the layer in the preview. The channel you paint is seen tinted. Each stroke is an **Undo** step. If another effect or layer uses the same image, the first stroke creates a copy of its own for the parameter you are painting and the other stays the same.
5. Click the button again to stop painting.

**Steps for a direction map:**

1. Click **Paint the direction**.
2. **Flatten** makes what you painted stand still. **Strength** is how much it moves.
3. Drag over the layer in the direction it should move: each stroke writes the direction it took, and the rest stays still.

**In the terminal:** `reiwand-ctl scene act DIR paint_mask` (stroke in fractions of the layer), or the `paint_mask` and `comb_flow` actions. See [actions](../actions.md).

## How to make a color gradient

**Where it is:** inspector, on the image parameters that are a gradient (for example the `gradient` of fire or of light shafts) and on those of a color table (that of **Color grading**).

**Steps:**

1. Choose the effect. A gradient bar appears below the image selector.
2. Click the bar to add a stop (up to 8). Drag a stop to move it.
3. Below the bar, each stop has its position, a color picker and **Remove the stop**. At least one stays.
4. When you stop editing, the editor makes the gradient image and puts it in the parameter. It is a single **Undo** step. If the parameter already names a gradient made here, it is made again.

In a color table the dark tones take the first stops and the light ones the last.

**In the terminal:** the `ramp` action (stops, format `png` or `cube`, parameter). See [actions](../actions.md).

## How to make a depth map

Some effects, like **Depth parallax** and **Tilt-shift**, read a depth map: a gray image that says how far each part is.

**Where it is:** inspector, below the effect's `depth` parameter.

**Steps:**

1. Choose the effect.
2. In the dropdown choose the image the depth comes from.
3. Click **Make a depth map**. The map is saved in the package and the parameter uses it.
4. If the button is not available, the notice next to it says what is missing: the depth model or the library that runs it.
5. You can also paint the map by hand with **Paint in grey** (above).

**In the terminal:** the `depth_map` action; the `depth_model` query says whether the model is ready (`reiwand-ctl scene ask DIR depth_model '{}'`). See [actions](../actions.md).

## How to make the scene glow

**Where it is:** the scene's inspector. To get to it, click a place on the canvas where there is no layer.

**Steps:**

1. Click a place on the canvas where there is no layer. The inspector shows the scene's settings.
2. In **Brightness:** turn on **Bright parts glow**.
3. Adjust **Threshold:** (how bright something must be to glow), **Strength:** and **Radius:**.
4. To compare, open **Preview level**, check **Without the glow of the canvas** and hold down **Hold to see the other side**: the preview shows the other side of the comparison without the glow. Or click **Compare the glow** in the canvas bar to see the canvas split live, with the glow on the left and without it on the right.

If only one group of layers should glow, use the **Glow** effect on a layer, or on an adjustment layer.

To keep colors brighter than white, the same inspector has **Bright colors**: **Keep colors brighter than white**, the **Tone mapping** (**None**, **Filmic (ACES)** or **Soft (AgX)**) and **Before and after**, which splits the preview to compare.

**In the terminal:** the `canvas` action with `bloom`; also `reiwand-ctl scene act DIR canvas`. Tone mapping: `reiwand-ctl scene act DIR scene3d`. See [actions](../actions.md).
