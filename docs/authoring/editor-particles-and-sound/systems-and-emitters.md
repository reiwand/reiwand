# Particle systems and emitters

On this page: adding systems, shapes, emitting from layers, rates and bursts.

## How particles work

A **particle system** is the recipe: how many are born, from where, how they move, how long they live and how they are drawn. A **particle layer** draws a system on the canvas. A system can exist without a layer (for example, so another one starts it). Several layers can draw the same system, each with its own touch-ups.

A system's controls appear in the inspector when you choose that system. Some have tools over the preview (handles you drag). The handles are in canvas pixels and respect the layer's transform.

While a system or a particles layer is chosen, the canvas bar names its handles: **Emitter handles**, **Force arrows** (pulls, distances and swirls), **Control points** and **Colliders**. Each word turns those handles off and on, to see the particles without them or to reach one under another; they are all on when the app starts, and the choice is not saved.

## How to add a particle layer

**Where it is.** The **Add a layer** button of the layer list.

**Steps**

1. Click **Add a layer**.
2. Choose **Particles**.
3. The layer is born with a system that already works: snow falling from top to bottom, visible from the first frame, with gentle turbulence and starting colors and sizes.
4. In **Start with…** in the inspector choose **Snow**, **Sparks**, **Rain** or **Dust** to fill in the emitters, the life and the forces all at once with a starting point; then adjust it to your taste.

**In the terminal.** The `add_layer` action with `kind` equal to `particles`, and the `kit` variant of `particles` for the starting point. See [actions](../actions.md).

## How to open a layer's system

**Where it is.** The particle layer's inspector.

**Steps**

1. Select the particle layer.
2. Click the **Edit the particle system** button (snowflake icon, at the top of the inspector).
3. The inspector switches to show the system's settings.

The layer itself shows the **This layer from its system** block, explained below.

**In the terminal.** `reiwand-ctl scene act DIR particles` prints what the editor draws for that system. See [actions](../actions.md).

## How to see, create, copy, rename and remove systems

**Where it is.** The **Particle systems** button (snowflake icon, icon only) below the layer list. Its help says “Create, copy, rename and remove the scene's particle systems”. It shows **Particle systems of the scene**. It also appears at the top of the inspector of any system.

**Steps**

1. Click the **Particle systems** button. If there are none, it reads **The scene has no particle systems yet.**
2. Each row has the system's name in a field and four buttons:
   - **Edit this system**: opens it.
   - **Copy this system**: creates a copy with a new name and opens it.
   - **Add a system that this one starts**: creates a child system and opens it.
   - **Remove the system**.
3. To rename, change the text of the field and confirm. Names allow lowercase letters, digits and underscore, up to 32 characters.
4. To create one without a layer, click **Add a system without a layer**.

If any script still names the old name, the editor warns about it with each line and the **Rewrite the exact names** button.

**In the terminal.** The `particles` action with the keys `create` (with `from` to copy or `parent` for a child), `delete`, `rename` and `scripts`. Also `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).

## How to adjust a system

**Where it is.** The system's inspector, **Settings of this system** block.

**Steps**

1. **Most particles alive**: the cap of particles at once (from 1 to 200,000).
2. **Seed**: the number that decides the randomness. Type one, click **Pick a random seed** or click **Use the seed of the preview** (it appears when the preview's is another) to fix the one you see now.
3. **Seconds simulated before it shows**: how many seconds the system runs before being seen, so it does not start empty.
4. **Particles move with**: **The layer** (if you move the layer, they move with it) or **Where they were born** (the layer and the control points only place the particles when they are born).

**In the terminal.** The `particles` action, key `settings`. See [actions](../actions.md).

## How to choose an emitter's shape

A system has from one to four emitters. Each is a block **Emitter 1**, **Emitter 2**, etc., under **Emitters**. **Add an emitter** adds one at the end and **Remove this emitter**, in each block, removes it; a system keeps at least one emitter. When one is removed, the following ones take its number: whatever names an emitter by its number outside the system is not rewritten, and if the change leaves the scene broken, the check rejects it.

**Where it is.** The system's inspector, **Emitters** block.

**Steps**

1. In the emitter, open the list of shapes and choose one:
   - **Point**: they are born at a point.
   - **Box**: inside a rectangle.
   - **Circle**: inside a circle.
   - **Line**: along a line.
   - **Edge**: at the edge of the canvas.
   - **Around a point**: along an arc around a control point.
   - **Between two points**: along a line between two points.
   - **From a layer**: from the pixels of another layer.
2. On the canvas the shape's handles appear. Drag them: the corner of a box, the center and radius of a circle, the ends of a line.
3. In **Around a point** and **Between two points**, choose in the lists which control point they follow (or **a point**, fixed), and type how many particles are born in the number on the right (from 1 to 4096). An end tied to a control point moves that point when you drag it.

**In the terminal.** The `particles` action, key `shape`. See [actions](../actions.md).

## How to emit from the pixels of another layer

**Where it is.** An emitter with the **From a layer** shape.

**Steps**

1. Choose the **Layer** that serves as the source (never the one that draws this system).
2. **Threshold**: particles are born at the pixels more opaque than that value. The preview marks those pixels.
3. Check **Its color** so that each particle is born with the pixel's color.
4. Check **Its motion** so that it inherits the layer's motion.
5. **Read again**: **Once** or **When the layer changes**.

If the layer cannot be read pixel by pixel (a 3D model or a dynamic text like a clock), the preview marks only its box and the engine emits from its opaque pixels.

**In the terminal.** The `particles` action, key `layer`. Also `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).

## How to adjust an emitter's rate

**Where it is.** Inside each **Emitter** block.

**Steps**

1. The slider gives the rate in “N a second”. If the rate follows timeline keys or a link, it reads **Rate follows keyframes or a link** and the control is locked: click **Fixed rate** to replace it with a number.
2. Check **Most particles started in one step** and set the number, so that a big jump in rate does not release many at once.
3. Check **Follow the pointer** for the emitter to follow the user's pointer.
4. Check **Only while sound plays** for it to emit only if there is sound.
5. **Speed taken from the emitter**: what fraction of the pointer's speed the particles inherit (from 0 to 1). It works with **Follow the pointer**.

**In the terminal.** The `particles` actions: `settings` (with `emitter` and `rate`), `pace` and `emit`. See [actions](../actions.md).

## How to give an emitter bursts and pauses

**Where it is.** Inside each **Emitter**: a short timeline and the **Bursts** list.

**Steps for the timeline**

1. Drag the start to **Start after N s**.
2. Check **Stop after a while** and drag the end to **Stop after N s**.
3. Check **Emit in bursts and pauses** to alternate emission spans and pauses. Drag the ends of the spans and the pauses: the top row is the shortest duration and the bottom one the longest.
4. The line shows “N s shown” according to its scale. Each drag is a single history step.

**Steps for the bursts**

1. Click **Add a burst** (up to 8 per emitter).
2. Type how many particles it releases.
3. Choose when it fires: **on time**, **on the beat** (the sound's pulse) or **on a click**.
4. With “on time”, set **After N s** and, if you check **Repeat**, **Every N s**. Over the line they are seen as a dot (the first shot) and squares (the repeats); dragging them changes those times.
5. **Remove this burst** deletes it.

**In the terminal.** The `particles` action, key `emit`. See [actions](../actions.md).
