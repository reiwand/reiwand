# How particles start and the forces

On this page: control points, starting values, colors, noise, remaps, pulls, flocks and force limits.

## How to use control points

**Control points** are marks (`p0` to `p7`) that shapes, forces and colliders can use.

**Where it is.** The system's inspector, **Control points** block.

**Steps**

1. Click **Add a control point** (up to 8).
2. Drag it on the canvas. With a particle layer selected, the point you drag belongs to that layer only; with no layer, it belongs to the system.
3. Check **Follows the pointer** for it to follow the user's pointer.
4. Use the angle (**Turn N°**) to rotate it.
5. **Remove** deletes the last point in the list (the first one cannot be removed).

**In the terminal.** The `particles` action, key `point`. Also `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).

## How to set what each particle starts with

**Where it is.** The system's inspector, **Starting ranges** block.

**Steps**

1. Each row is a range with two handles: **Life in seconds**, **Speed**, **Direction**, **Size**, **Spin**, **Spin** (speed) and **Alpha**. Each particle takes a value between the two ends.
2. **Curve**: with the slider you tilt the distribution. Below 1 values near the high end are chosen more often, above 1 near the low one.
3. At each end you can leave **Low end set here** or **Follows SETTING**, so that end follows a user setting (see [editor-properties](../editor-properties.md)).

**In the terminal.** The `particles` action, key `start`. Also `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).

## How to choose the starting colors

**Where it is.** **Starting ranges** block, below the rows, **Starting colors** part.

**Steps.** Choose a mode in the list:

- **Between two colors**: two color pickers; each particle takes one between the two.
- **Hues, saturation and value**: three ranges with two handles (**Hue**, **Saturation**, **Value**). Check **Pick the hue among steps** for the hue to jump among a number of fixed values.
- **From a palette**: a list of colors with **Add a color** and **Remove the color**, and three noise controls (**Hue noise**, **Saturation noise**, **Value noise**) that deviate each particle a little.
- **Along a gradient**: a gradient bar to edit the stops.

**In the terminal.** The `particles` action, key `start`, with color `ranges`. See [actions](../actions.md).

## How to turn or offset the birth with a noise

**Where it is.** The system's inspector, **Starting directions and offsets** block.

**Steps to turn the directions**

1. Check **Turn starting directions with a noise**.
2. Adjust **Direction**, **Turn by up to**, **Noise speed**, **Spread between particles** and **Phase**.
3. The preview draws a fan with the possible directions, and rays that are the first particles born with the current seed. With an arrow you drag the direction, and with a handle on the edge you open or close the fan.
4. Click **Keep this seed** so that each start of the scene draws what you see.
5. In **Sound on the rays**, choose a band (**No sound**, **Bass**, **Mids**, **Treble**, **Level**, **Beat**) for the noise to move with the sound. A slider says how much that band sounds for the preview's rays. With **Sound multiplies the phase**, the sound moves the phase.

**Steps to offset births**

1. Check **Move births along a fractal noise**.
2. Adjust **Distance**, **Pattern size**, **Detail**, **Spread** and **Drift**.
3. Choose which way it moves: **Across**, **Down**, and which side (**Both sides**, **Positive side**, **Negative side**).
4. Over the preview you see arrows from the birth places to where the noise takes them, and a ring with a handle for the distance.

**In the terminal.** The `particles` actions: `turbulent` and `offset`. Also `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).

## How to remap values

A **remap** turns an input (for example, randomness or sound) into a particle value (for example, the size).

**Where it is.** Two blocks of the inspector:

- **Starting values from inputs**: computed at birth (up to 8).
- **Values changed as particles live**: applied at every step (they also accept the `age` input).

**Steps**

1. Click **Add a remap**.
2. Choose the **input** in the first list and the **output** in the second. The lists show the names as they are: inputs such as `random`, `life`, `speed`, `daytime`, `runtime`, `cursor:x`, `cursor:y`, `audio:bass`, `audio:mid`, `audio:treble`, `audio:level`, `distance:p0` (and `age` in the second block); outputs such as `size`, `alpha`, `color`, `speed`, `life`, `rotation`, `spin`, `frame`.
3. With the two ranges of handles you set which part of the input is measured and which part of the output it reaches.
4. Check **Clamp the input** or **Clamp the output** so it does not go past those ends.
5. Choose how it combines with the current value: **Set**, **Multiply**, **Add** or **Subtract**.
6. Choose the shape: **Straight**, **Sine wave** or **Noise**. A graph shows the output curve against the input.
7. **Remove this remap** deletes it.

**In the terminal.** The `particles` action, key `remap` (starting values) or `force` with `remap` (values during life). Also `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).

## How to add pulls, distances and swirls

**Where it is.** The system's inspector, **Pulls, distances and flocks** block.

**Steps**

1. Open the **Add a force** list and choose one:
   - **Keep a distance**: the particles stay at a fixed distance from a point.
   - **Keep between two points**: they stay between two points.
   - **Pull towards a point**.
   - **Swirl around a point**.
   - **Circle around a point**.
   - **Slow down near a point**.
2. Choose the point: a control point, the pointer or a fixed point (**a fixed point**, with its coordinates in system pixels).
3. Adjust the **Strength** (or **Speed kept** in the slowing) and, where it applies, **Weaker with distance**: **As the force does by default**, **Not at all**, **In proportion** or **With the square**.
4. Each reach (radius, inner edge, outer edge) has a slider. On the canvas it is seen as a dashed ring with a handle on top; the force has another handle at its center.
5. On the distance forces, the **Distance N px** control sets the distance; on the canvas a ring is seen with a handle on the target and another on the ring.
6. **Remove this force** deletes it.

**In the terminal.** The `particles` action, key `force`. Also `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).

## How to make particles gather and drift like embers or dust

**Where it is.** **Noise fields** block.

**Steps**

1. Press **Add a noise field**. It adds a `noise_field` force with every field written: strengths
   from 20 to 60, phases from 0 to 200 px, swirls about 200 px across, a drift of 10 px a second,
   across and down.
2. Drag the two ends of **Strength** apart so that each particle pushes at its own strength, and
   those of **Phase** so that each reads the noise a different way ahead.
3. **Size of the swirls** sets `scale`, **Drift** how fast the noise slides, and the three
   **Axes** sliders how much it pushes across, down and in depth (depth only moves with a
   perspective).
4. **Remove the noise field** deletes it.

**In the terminal.** The `particles` action, key `force`, with `{"noise_field": {...}}`. See
[actions](../actions.md).

## How to make particles move in a flock

**Where it is.** **Pulls, distances and flocks** block.

**Steps**

1. Check **Move in flocks**.
2. Adjust **Neighbour radius**, **Separation**, **Alignment**, **Cohesion** and **Top speed**.

**In the terminal.** The `particles` action, key `boids`. See [actions](../actions.md).

## How to limit when each force acts

**Where it is.** **When in the life each force acts** block. It only appears if the system has forces.

**Steps**

1. Check **FORCE: only in part of the life** on the force you want to limit.
2. Drag the drawing's handles: from left to right is a particle's life; the force appears, acts fully and fades where the shape rises, holds and falls.
3. Uncheck the checkbox for the force to always act.

**In the terminal.** The `particles` action, key `window`. Also `reiwand-ctl scene act DIR particles`. See [actions](../actions.md).
