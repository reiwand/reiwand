# 3D animation and cost

On this page: animating models, bones, bright colors and what 3D costs.

## How to animate a model

Where it is: the model layer's inspector, **Animation** part (only if the model has a skeleton, clips or blend shapes).

Steps:

1. The list shows the layer's clips with their duration and where they come from. **On the timeline** opens that clip for editing (see [editor-animation](../editor-animation.md)).
2. To bring an animation from the file, open **New clip from the file…** and choose one.
3. Turn on **Root motion: the clip moves the layer** if the clip should move the model. In the authoring view the path it will travel forward is drawn, blending all the animation layers by their weight.
4. If the model has blend shapes, each one has a **Shape N** weight slider between -1 and 1.

### Animation layers

The **Animation layers** part blends several clips at once. First add a clip. For each layer:

- The clip it plays, and **On** and **Adds** (adding on top of the others instead of blending).
- Move up, move down and remove.
- Sliders **Weight**, **Speed**, **Fade in** and **Fade out**.
- **Only these bones**: limits the layer to some bones.
- **Add a layer playing…** creates a new layer.

In the terminal: the `animate` actions (`layer`, `root`, `preset`, `morph`), `clip` and `key`; query `motion`.

## How to edit a model's bones

Where it is: the authoring view, with the model chosen.

The chosen model draws its bones: a line from each bone to its parent and a knob on each. A click on the knob chooses the bone for the timeline and for the selectors. The chosen bone shows:

- A ring: drag the ring to rotate the bone in the view; drag inside to tilt it like a trackball.
- A **Move** square in the center: it moves it in the plane of the view.

Each drag puts a key in the chosen clip at the current time. A model without clips gets the clip `clip_1`.

In the terminal: the `key` action (bone angles and offset in a clip); query `bone3d`.

### Swinging bones

Where it is: **Animation** part, **Swinging bones** block.

Steps:

1. Choose a bone in the selector.
2. Click a preset: **Hair**, **Tail**, **Cloth**, **Antenna**, **Chain** or **Jelly**. The bone starts swinging like that kind of thing.
3. Each swinging bone has its mode list (**Spring**, **Rigid**, **Rope**), the remove button and the sliders **Stiffness**, **Damping**, **Inertia**, **Gravity** and **Wind**, plus the limits of its rotation and its sliding.
4. **Show the wind on the preview** draws the scene's wind arrow.
5. In the authoring view, a swinging bone draws the fan its rotation allows, in its own plane, with **Least** and **Most** knobs that drag the limit. If it has a sliding limit, it draws the box in which its head can move and the chosen bone has **Least** and **Most** knobs on its edge.

Swinging stops at the lowest quality level, where the bones keep their animation.

In the terminal: the `physics` actions (with rotation limit) and `wind`.

### Colliders

Where it is: **Animation** part, **Colliders** block.

Steps:

1. Choose a bone. The button says **Add a collider on NAME**.
2. Choose **Capsule along the bone** or **Sphere at its head**, and click add.
3. Adjust the width with the slider, or in the authoring view by dragging the knob on the edge (**Drag to set how wide the collider is**). The collider is drawn with dashed circles.
4. **Remove** deletes it.

Swinging bones are left out of the colliders: all of them in the two highest quality levels, the first two in the second lowest and none in the lowest.

In the terminal: the `collider` action.

### Chains that reach

Where it is: **Animation** part, **Chains that reach** block.

Steps:

1. Choose a bone that has a parent. The button says **New chain from A to B**.
2. Choose the target: **A point**, **The pointer**, a bone of the model or a layer.
3. Adjust the **Weight**; **Remove** deletes the chain.
4. If the target is a point, drag it in the view (**Drag the point the chain reaches for**).

In the terminal: the `ik` action.

### Sculpt a blend shape

Where it is: **Animation** part, **Sculpt a blend shape** block.

Steps:

1. Type a name in **Name of the shape**.
2. Click **Sculpt on the view** and choose **Push**, **Pull** or **Smooth**.
3. Adjust **Size** and **Strength** and drag over the model in the authoring view. Each stroke moves the points under the brush along the perpendicular to the surface, or evens them out, and is saved in a file next to the model. **Undo** removes the stroke.
4. The shape then has its weight slider.

In the terminal: the `shape_sculpt` action and `scene3d` with `sculpt`.

## How to keep colors brighter than white

Where it is: canvas, **Bright colors** part.

Steps:

1. Turn on **Keep colors brighter than white**.
2. Choose the **Tone mapping**: **None**, **Filmic (ACES)** or **Soft (AgX)**.
3. Move **Before and after**: the preview shows the left part without tone mapping, to compare.

Phones keep 8-bit colors before tone mapping. On HDR screens, colors brighter than white are kept and gently bent up to each screen's brightness, instead of this mapping; the editor's preview shows the normal image. If any output has HDR on, the panel says “HDR is on for: …”.

In the terminal: `scene3d` with `tones` (`hdr`, `tonemap`, `split`).

## How to know what 3D costs

Where it is: canvas, **Lights** panel, after the **Preview quality** selector. The **3D models** part appears and a line per quality level.

What it counts:

- For each quality level: the triangles, the 3D groups, the units of work of the models and whether image reflections are drawn (**with reflections** or **no reflections of pictures**).
- For each equipment level, at what quality it starts, whether each model **fits**, **shows its poster** or is **too heavy**, the texture memory and how many lights cast shadows.
- The shadows (shadow maps of N megapixels), the fur layers and the fur triangles that each shadow map adds, and the vertices with skin and swinging bones.
- The lights with air, the pixels the shafts copy, the lights on particles and the refraction.

When the cost is too much, lower the quality level, remove lights or reduce fur layers and texture sizes.

In the terminal: queries `cost3d`, `lighting` and `budget`. See [actions](../actions.md).
