# Colliders, trails and the look

On this page: walls, ropes and trails, the look, drawing order, child systems and cost.

## How to put up walls (colliders)

**Where it is.** The system's inspector, **Colliders** block. They are drawn over the preview: in red where the particles die and in green where they stick.

**Steps**

1. Open the **Add a collider** list (up to 8) and choose: **Canvas** (the canvas edge), **Plane**, **Circle**, **Box**, **Segment**, **Layer** or **Picture mask**.
2. Drag the handles on the canvas. A plane has a handle at its point and another that turns the side where the particles stay. The circle has center and radius; the box, two corners; the segment, its ends. The layer has a handle at its center: drop it on another canvas layer for the particles to collide with that layer. The mask has a handle on the top edge: drag it along the edge to set the opacity from which it is solid. The canvas edge has nothing to adjust and carries no handle.
3. Choose what happens on collision: **Bounce**, **Slide**, **Stop**, **Stick** or **Delete**.
4. With **Bounce**, adjust the bounce and the **Friction**.
5. On circle and box, check **Keep particles inside** so they do not leave (the drawing becomes a dashed line).
6. In **Layer**, choose the layer they collide with. In **Picture mask**, choose the **Image** and the opacity from which it is solid (**Solid from opacity N**).
7. **Remove this collider** deletes it.

**In the terminal.** The `particles` actions: `collider` and `response`. Also `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).

## How to make ropes and trails

**Where it is.** The system's inspector, **Ropes and trails** block.

**Steps**

1. In **Texture**, choose an image from the scene (or **No texture**).
2. Check **Join particles with a rope**. Adjust **Width**, **Points between particles**, **Texture repeats**, **Texture scroll** and **Smooth curve**.
3. If an emitter releases more than one particle per step, the rope piles up. The editor warns and offers **One particle a step**, which fixes it.
4. Check **Leave a trail**. Adjust **Steps**, **Width**, **Kept points**, **Points between kept points**, **Texture repeats** and **Texture scroll**. **Show the texture along the trail** puts the texture over the trail.
5. The preview updates while you move the controls.

**In the terminal.** The `particles` actions: `strand` (rope or trail), `texture` and `pace`. Also `reiwand-ctl scene act DIR particles` and `rope` to print the advice. See [actions](../actions.md).

## How to change the particles' look

**Where it is.** The system's inspector, **Look** block, and the **Light** and **Bend what is behind the particles** blocks.

**Steps**

1. **Cut out the alpha of the texture**: check the box and adjust the two-handle range and the **Opacity**.
2. Choose where the sprites face: **Face the screen**, **Along the motion** or **At a fixed angle** (with **Turn N°**).
3. **Perspective**: turns on depth. The **Focal length** controls how much depth shrinks; on the canvas there is a ring with a handle. **Depth** and **Depth speed** are two-handle ranges.
4. In **Light**, check **Take the lights of the scene** and choose a **Normal map**. A thumbnail shows a sprite lit from the front and the back (each sprite receives up to 8 lights on both faces).
5. In **Bend what is behind the particles**, choose a normal map, the units (proportional to the size or in pixels, up to 4096) and the amount, and how it paints its color: **Only bend what is behind**, **Multiply what is behind by their color** (tints it) or **Add their color over what is behind** (lights it). Check **Show the displacement over the preview** to see how much each particle displaces. A note says which qualities leave it out.

**In the terminal.** The `particles` action, keys `cutout`, `orient`, `perspective`, `light` and `shader`. Also `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).

## How to choose the drawing order and what layers may change

**Where it is.** The system's inspector, **Drawing order and what layers may change** block.

**Steps**

1. In **On top**, choose which particles are drawn on top: **In no set order**, **The newest**, **The oldest**, **The largest**, **The smallest** or **The lowest**.
2. Set the number of times a texture's frames repeat in a life (0 is only once).
3. In the checkbox list, uncheck the values the layers must not change; they stay locked.

**In the terminal.** The `particles` action, keys `render` and `settings` (with `lock`). See [actions](../actions.md).

## How to draw a different system on each layer

**Where it is.** The system's inspector, the **Render settings for** selector.

**Steps**

1. Choose **The system, for every layer** or **Only the layer NAME**.
2. With a layer chosen, your first change gives it a render of its own, copied from the system. The panel says how it differs.
3. **Use the render of the system again** undoes it.

The selector covers these parts of the render: **Stretch**, **Frames**, **Frames per second**, **Blend frames**, **Brightness**, **Cycles**, **Drawing order**, **Rope**, **Trail**, **Texture**, **Cutout**, **Orientation**, **Angle**, **Shader**, **Light**, **Normal map**, **Refraction**, **Size over life** and **Color over life**.

**In the terminal.** The `particles` action, key `scope` (with `layer` and `own`). The panel has no controls for size and color over life: they are written with the `over` key, or with `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).

## How to touch up a layer over its system

**Where it is.** The inspector of a particle layer, **This layer from its system** block.

**Steps**

1. Move the controls you want: **Opacity**, **Size, times**, **Share of the particles**, **Speed, times**, **Life, times**, **Pace of time, times** and **Brightness**. Those the system locked say “locked by the system”.
2. **Tint** changes the color. Each control has a button to go back to **As the system draws it**.
3. The canvas control points belong to this layer: dragging one gives the layer its own.
4. **Draw everything as the system does** removes all the touch-ups.

**In the terminal.** The `layer` action, key `instance`. See [actions](../actions.md).

## How to make a system start others

**Where it is.** The parent system's inspector, children block (at the end).

**Steps**

1. Choose when the child starts: **All the time**, **When a particle starts**, **When a particle ends** or **Following each particle**.
2. Click **New child system**.
3. On each child, **Child NAME takes from its parent:** check **Color**, **Size**, **Alpha** or **Speed** (and the depth speed when parent and child have perspective). **Every step while it follows** repeats the inheritance.
4. **Remove the child** deletes it.
5. **Particle shader** chooses a package effect to draw with; if the engine cannot build it or it is too slow, it is left out and the particles are drawn without it.

**In the terminal.** The `particles` actions: `create` (with `parent` and `on`), `child`, `inherit` and `shader`. See [actions](../actions.md).

## How to measure a system's cost

**Where it is.** The system's inspector, **Particle budget** block.

**Steps**

1. If a preview is running, the block says how many particles are alive, out of how many slots, and whether the graphics card or the processor simulates them. It also says at what quality the preview draws.
2. Without a preview, the counts are an estimate by the editor (a note says so).
3. Check **Simulate on the processor and show the particles over the preview** to see the particles of the editor's computation over the preview. When you click one, the flock's neighbor radius is drawn.
4. Choose the **Preview quality**: **Q0, lowest**, **Q1**, **Q2** or **Q3, highest**. You see what each level keeps (births, neighbors, trail and rope points).
5. **Restart systems** starts only the particles over, without touching the rest of the scene.
6. For each equipment class (phones and old computers fall into the two lowest) there is a row with the slots, the trail points, the neighbors and the rope spans.

**In the terminal.** `reiwand-ctl scene act DIR particles` prints the budget. `reiwand-ctl scene preview DIR --quality Q --restart` does the quality and the restart. See [actions](../actions.md).
