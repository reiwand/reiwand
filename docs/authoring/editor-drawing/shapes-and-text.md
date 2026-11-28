# Shapes, gradients and text

On this page: shapes, their handles, gradients and texts.

## How to add a shape and choose its figure

**Where it is:** **Layers** tab, **Add a layer** button, **Shape** entry.

**Steps:**

1. Click **Add a layer** and choose **Shape**. The layer starts as a rectangle.
2. With the layer chosen, look at the inspector. The "Shape" field is a list of the figures: rectangle, ellipse, ring, polygon, star and line. Each has its own fields (size, radius, sides, points, thickness…).
3. The fill color is the "Fill" field. The outline is the "Stroke" group: click **Add** to have it.

**In the terminal:** `add_layer` with `"kind":"shape"`, and `set` to change the figure or any field. See [actions](../actions.md).

## How to change a shape with the canvas handles

While a shape layer is chosen, the preview draws its handles on top. They are moved with the mouse and each drag is saved as a step.

**Where it is:** in the preview, over the chosen shape.

**Steps:**

1. Choose the shape layer in the layer list, or click the shape in the preview.
2. Drag a handle. Hover over it to see its help.
3. Release. The change is left as a step.

The handles that change the size are slightly inside the layer's box, so as not to cover the layer's grips. The square ones change measurements (corners and edges); the round ones change a number (rounded corners, sides, points, depth of the points, ring thickness).

| Figure | Handles | What each does |
|--------|---------|----------------|
| Rectangle | width, height, size (corner), radius | **Set the width**, **Set the height**, **Resize; hold Shift to keep the proportions**, **Round the corners** |
| Ellipse | width, height, size | the same, without radius |
| Ring | width, height, size, thickness | also **Set the thickness of the ring** |
| Polygon | size, sides | drag the sides handle around the figure: **Drag around the shape to set the number of sides** (from 3 to 64) |
| Star | size, points, depth | **Drag around the shape to set the number of points** (from 2 to 64) and **Set the depth of the points** (from 0.01 to 1) |
| Line | start, end, thickness | **Move the start of the line**, **Move the end of the line**; when one end moves, the other stays where it was |
| Any but line, with an outline | outline thickness | **Set the width of the outline** |
| Any, with a gradient | start and end of the gradient | **Start of the gradient, or its center when it is radial** and **End of the gradient, or the edge when it is radial** |

The corner radius does not exceed half of the shorter side. The outline and gradient handles only appear when the layer has that outline or that gradient.

**Shortcut:** holding **Shift** while dragging the size handle keeps the proportions.

**In the terminal:** the `layer` action with the `figure` variant and `shape`: `{"figure":{"shape":{"layer":"ID","handle":"NAME","to":[X,Y]}}}`. `handle` is the name of the handle (`width`, `height`, `size`, `radius`, `band`, `sides`, `points`, `inner`, `start`, `end`, `stroke`, `gradient_from`, `gradient_to`) and `to` is a point of the canvas in pixels. With `"even":true` the proportions are kept. There is no terminal query that gives the position of the handles.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"box"}'
reiwand-ctl scene act my-scene layer '{"figure":{"shape":{"layer":"box","handle":"size","to":[1200,700]}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"shape":{"layer":"box","handle":"radius","to":[1100,300]}}}'
```

## How to add a gradient and edit its stops

**Where it is:** the shape layer's inspector, "Gradient" group.

**Steps:**

1. In the "Gradient" group click **Add**. The shape is filled with a gradient.
2. In the gradient bar, click a point to add a stop. There are between 2 and 4. Drag a stop to move it.
3. Below the bar, each stop has its position (from 0 to 1), a color button and the **Remove the stop** button. You cannot leave fewer than 2.
4. The checkbox of the "Radial" field changes the gradient to radial. Drag its two handles over the shape, which have another color than the others: the first is the center and the second the edge (in a linear one, the start and the end).
5. To remove the gradient, click the **Remove** button of the "Gradient" group.

The bar says: **Click the bar to add a stop, at most 4, and drag a stop to move it**.

**In the terminal:** `layer` with `figure` and `stop`: `make` creates the gradient with two stops (and `radial`), `clear` removes it, `add`, `move`, `color` and `remove` edit the stops. The stops are kept sorted.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"box"}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"box","stop":{"make":{"radial":false}}}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"box","stop":{"add":{"at":0.5,"color":"#ff8800"}}}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"box","stop":{"move":{"index":1,"at":0.3}}}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"box","stop":{"color":{"index":0,"color":"#112233"}}}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"stop":{"layer":"box","stop":{"remove":{"index":1}}}}}'
```

## How to write a text with line breaks

**Where it is:** **Add a layer** > **Text**; then the "Text" field of the inspector.

**Steps:**

1. Add the text layer. It starts with its name as the text.
2. Type in the "Text" field. It is a multi-line box: Enter writes a line break, which counts as one line of the box.
3. Over the preview, the text's box is drawn with the real size it will have, with the same font, the same line wrapping, the same spacing and the outline and shadow margins. Above its upper left corner you read the size and the lines, such as **800 × 120 px, 2 lines**.

**In the terminal:** `add_layer` with `"kind":"text"` and `set` on the text's field.

## How to write a text in several languages

**Where it is:** text layer, the "Text" field of the inspector, and below it the **Language, such as es or pt-BR** box.

**Steps:**

1. Type the text in the package's main language.
2. Below the field, type the code of another language (`de`, `pt-BR`) and click **Translate**. A row appears per language, each with its multi-line box, and the new part starts from the main language's text.
3. Type the text of each row. Whoever looks sees the one in their language and, if it is not there, the one in the main language. **Remove this language** deletes a row; the layer's last text cannot be removed.
4. To see the scene in another language, choose the language in **Language of the preview** (preview panel). It also applies when saving an image or a clip.

If the text follows a value (a time, a date, a property) it has no languages. In the problems list a warning appears if the layer lacks the text in the main language. A script that writes the layer's text writes it in all the languages.

**In the terminal:** `translate` with `"pointer":"/scene/layers/0/text"`, the language and the text, or with `"add":true` to start from the main language's text; `reiwand-ctl scene render DIR -o view.png --language de` draws the scene in a language.

## How to set the text width with the handle

**Where it is:** preview, right edge of the text's box.

**Steps:**

1. Choose the text layer. The box is seen with a dashed line if the text has no fixed width, and continuous if it has.
2. Drag the square on the right edge. Help: **Drag to wrap the text at a width**. The text is split into lines at that width; the left edge does not move.
3. To go back to not wrapping, double-click the square. Help of the already wrapped handle: **Set the width of the text; double-click to stop wrapping**.

The handle writes the "Width" field of the layer's "Wrap" group.

**In the terminal:** `layer` with `figure` and `text_width`: `to` is the point of the canvas where the right edge falls; with `"remove":true` the wrapping is removed.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"title"}'
reiwand-ctl scene act my-scene layer '{"figure":{"text_width":{"layer":"title","to":[900,100]}}}'
reiwand-ctl scene act my-scene layer '{"figure":{"text_width":{"layer":"title","remove":true}}}'
```
