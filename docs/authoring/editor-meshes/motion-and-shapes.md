# Bone motion, blend shapes and parts

On this page: physics, the pointer, parent rules, blend shapes, order, attached layers and posing.

## How to make bones swing: physics, rope and wind

**Where it is:** the mesh's inspector, **Bones that swing**, **Wind**.

**Steps:**

1. Click **Pick bones that swing on the preview** and click a bone to make it swing; another click on it stops it.
2. Each bone with physics appears in the list with its mode (**Spring**, **Rigid** or "Rope") and five sliders: "Stiffness", **Damping**, "Inertia", "Gravity" (from -10 to 10) and "Wind" (how much the wind affects it).
3. Check **Bound its turn, in degrees** and **Bound how far its head slides, in pixels** to set limits with a two-ended slider (they start at -30 and 30).
4. On the preview each bone has a ring at its head and a gravity arrow: drag the square at the tip up or down to set it (**Drag up or down to set the gravity of NAME**). A bounded turn is an arc with a point at each end.
5. **Draw a rope on the preview**: drag along where the rope hangs. It gets up to 9 bones that hang from the chosen bone or the nearest one.
6. **Simulate** plays the preview with the physics, or leaves it still. **Reset** launches it again from the pose.
7. In **Wind**, adjust **Angle** (from -180 to 180 degrees), **Strength** (from 0 to 1000 px/s) and **Gusts**. **Show on the preview** draws the wind as an arrow from the center of the canvas and its tip is dragged. **No wind** turns it off. If the wind follows a link or keyframes, a warning is shown and setting it here makes it fixed.

**In the terminal:** `physics` (`layer`, `bones`, `mode`, `stiffness`, `damping`, `inertia`, `gravity`, `wind`, `rotation`, `translation`, `free`, `rope`, `parent`, `remove`) and `wind` (`angle`, `strength`, `gusts`, `off`). The `wind` query gives the arrow.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","cells":12,"id":"doll"}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"body","at":[0.5,0.7]}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"tail","parent":"body","at":[0.2,0.8]}'
reiwand-ctl scene act my-scene physics '{"layer":"doll","bones":["tail"],"mode":"spring","stiffness":0.4,"gravity":3}'
reiwand-ctl scene act my-scene physics '{"layer":"doll","bones":["rope"],"rope":[[0.3,0.2],[0.3,0.35],[0.3,0.5]],"parent":"body"}'
reiwand-ctl scene act my-scene wind '{"angle":90,"strength":120,"gusts":0.4}'
```

## How to choose the bones that receive the pointer

**Where it is:** the mesh's inspector, **Bones that take the pointer**.

**Steps:**

1. Click **Pick them on the preview**.
2. Click a bone to make it receive the pointer; another click releases it. Those that receive it are drawn with a thick ring.
3. You can also choose a bone in the list below and click **Add**; each row has **Remove**.

The scene's scripts know which of these bones the pointer touched, if the scene has the pointer permission.

**In the terminal:** `set` on the mesh layer's "Hit" field, with a list of bone names as the value (for example, `{"pointer":"/layers/0/hit","value":["hand"]}` if the mesh is the first layer).

## How to hand a bone to another parent with a rule

**Where it is:** the mesh's inspector, **Rules that hand a bone to another parent**.

**Steps:**

1. In **New rule** choose the **Bone** and the **New parent**, and click **Add a rule**.
2. Each rule is listed as "bone to parent". Change its parent, adjust its **Weight** (from 0 to 1) or click **Remove**.

**In the terminal:** `bone_rule` (`layer`, `bone`, `parent`, `weight`, `index`, `remove`).

## How to sculpt a blend shape and save expressions

A blend shape is a change of the mesh (a smile, a blink) that is blended with a weight. An expression is a saved set of weights.

**Where it is:** the mesh's inspector.

**Steps to sculpt:**

1. Click **Sculpt a blend shape on the preview**. The button is enabled when a shape is chosen or a name is typed in the next step.
2. In **Shape** choose an existing one, or **New shape…** and type its **Name** (the shape is created with the first stroke).
3. Choose the **Brush**: **Push away**, **Pull in**, **Smooth** or **Erase**. Adjust **Strength**, **Brush size** and **Soft edge**.
4. Hover over the mesh: a ring shows the brush radius and changes when you move **Brush size**. Drag over the mesh; the stroke is marked while you do it. The preview shows the shape at full weight while you sculpt. Each stroke is a step.

**Steps for expressions:**

1. When the mesh already has blend shapes, **Blend shapes in the preview** has a slider per shape, from -1 to 1. They only change the preview. **Reset the preview** turns them off.
2. Type a name in **Name of a new expression** and click **Save the sliders**.
3. In **Expressions**, each one has three buttons: **Show in the preview to edit**, **Keep the sliders in this expression** (to save it again) and **Remove**. When editing one, a field appears to **Rename**.
4. **Expressions in the preview** has a slider per expression, from 0 to 1, that blends them.

**In the terminal:** `shape_sculpt` (`layer`, `shape`, `mode` with `push`, `pull`, `smooth` or `erase`, `by`, `radius`, `falloff`, `points`) and `expression` (`layer`, `name`, `shapes`, `to`, `remove`).

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","cells":12,"id":"doll"}'
reiwand-ctl scene act my-scene shape_sculpt '{"layer":"doll","shape":"smile","mode":"push","by":10,"radius":0.1,"points":[[0.5,0.5]]}'
reiwand-ctl scene act my-scene expression '{"layer":"doll","name":"happy","shapes":{"smile":1}}'
```

## How to order a mesh's parts

If the mesh has more than one part, the inspector lists them.

**Where it is:** the mesh's inspector, **Parts, from back to front**.

**Steps:**

1. Drag a row's handle to another place to change the order.
2. Each part's **Shown** checkbox shows or hides it.
3. Click the part's name to mark it: it is drawn with its outline over the preview.
4. The list on the right of each row says **Over everything** or **Only over ANOTHER PART**: the part is only seen over the other one, which is also drawn with its outline.

**In the terminal:** `parts` (`layer`, `order`, `show`, `hide`, `clip`, `unclip`).

## How to attach layers to a mesh

An attached layer follows a bone or a point of the mesh.

**Where it is:** the mesh's inspector, **Layers that follow this mesh** and **Attachment points**.

**Steps:**

1. Click **Attach layers on the preview** (another layer is needed). Below, a chip appears for each other layer.
2. Drag a chip onto a bone: the layer follows it. Drop it on another place of the mesh: an attachment point is created there. Or click a chip and then click on the preview.
3. Check what the layer inherits: position, rotation and scale ("Position", "Rotation", "Scale").
4. A dotted line joins each layer with its bone or point, and ends in a ring: drag it to move the layer along what it follows.
5. The **Layers that follow this mesh** list says, for example, **hat follows arm**; each row's button releases it.
6. In **Attachment points** each point has its name (edited there), the bone it hangs from (**No bone** if it hangs from none) and **Remove the point**. **Add a point** puts one at the center. On the preview each point is a square that is dragged.

**In the terminal:** `attach` (`layer`, `to`, `bone` or `point`, `inherit`, `nudge`, `detach`) and `point` (`layer`, `name`, `at`, `parent`, `to`, `remove`).

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","cells":12,"id":"doll"}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"body","at":[0.5,0.7]}'
reiwand-ctl scene act my-scene point '{"layer":"doll","name":"hand","parent":"body","at":[0.9,0.4]}'
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"hat"}'
reiwand-ctl scene act my-scene attach '{"layer":"hat","to":"doll","bone":"body","inherit":["position","rotation"]}'
reiwand-ctl scene act my-scene attach '{"layer":"hat","nudge":[10,0]}'
```

## How to pose a bone with the handles

To animate a mesh, a bone is posed at an instant of a clip and saved as a key.

**Where it is:** the mesh's inspector, **Edit clips on the timeline**.

**Steps:**

1. Click **Edit clips on the timeline**. If the mesh had no clip, an empty one is created. The clip is chosen, played and looped as explained in [editor-animation](../editor-animation.md).
2. Choose a bone: click its name in the list or its point on the preview.
3. Put the playhead on the second you want. The chosen bone shows three handles: a circle at its head to **Move the bone**, a point on its axis to **Turn the bone** and a square to **Scale the bone**. The values are measured in the parent's space, as the clip stores them.
4. Drag the handles. Only the preview changes.
5. Click **Key**. It saves the channels you moved; if you moved none, the bone's whole pose at that second. **Key** is off if no bone is chosen.
6. **Back to the timelines** closes the clips mode.

The channels' **Key**, the curve, the keys that move and the clip's events are in [editor-animation](../editor-animation.md). **Clip in the preview** chooses which clip the mesh's preview plays.

**In the terminal:** `clip` (`layer`, `id`, `add`, `length`, `loop`) and `key` (`layer`, `clip`, `bone`, `time`, `rotation`, `position`, `scale`, `ease`). The `pose` query gives the posed mesh and `keys`, a clip's keys.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","cells":12,"id":"doll"}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"arm","at":[0.8,0.4]}'
reiwand-ctl scene act my-scene clip '{"layer":"doll","id":"wave","add":true,"length":1}'
reiwand-ctl scene act my-scene key '{"layer":"doll","clip":"wave","bone":"arm","time":0.5,"rotation":30}'
reiwand-ctl scene ask my-scene pose '{"layer":"doll","clip":"wave","time":0.5}'
```
