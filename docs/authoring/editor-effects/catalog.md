# The effect catalog and parameters

On this page: the catalog, parameters and points on the canvas.

## Effect catalog

The table lists the effects the program ships with, in the order and with the names of the **Add an effect** dialog. The second column is the id used in the terminal.

### Blend

| Name | Id | What it does |
|---|---|---|
| Gradient blend | `blend_gradient` | Blends in a texture using a gradient map to control where it shows. |
| Blend texture | `blend_texture` | Blends a texture over the picture with a chosen blend mode. |
| Dissolve | `dissolve` | Dissolves the picture away in a grainy pattern. |
| Reveal | `reveal` | Reveals or hides the picture through a growing shape. |

### Blur

| Name | Id | What it does |
|---|---|---|
| Blur | `blur` | Blurs the picture by a fixed radius. |
| Motion blur | `blur_motion` | Blurs the picture in one direction, as if it moved fast. |
| Radial blur | `blur_radial` | Blurs outward from a center point, as if zooming. |
| Local contrast | `local_contrast` | Sharpens local contrast without affecting overall exposure. |
| Tilt-shift | `tilt_shift` | Blurs out of a focus band, as in a tilt-shift photo. It can use a depth map. |

### Color

| Name | Id | What it does |
|---|---|---|
| Chroma key | `chroma_key` | Removes a chosen color, making it transparent. |
| Color adjustments | `color_adjust` | Adjusts exposure, contrast, saturation and hue. |
| Color grading | `color_lut` | Grades color using a LUT image. |
| Posterize | `posterize` | Reduces the picture to a few color levels. |
| Pulse | `pulse` | Pulses brightness, color, opacity or a tinted light over time. |
| Tint | `tint` | Tints or colorizes the picture with a chosen color. |

### Distort

| Name | Id | What it does |
|---|---|---|
| Chromatic aberration | `chromatic_aberration` | Splits color channels apart near the edges, like a cheap lens. |
| Depth parallax | `depth_parallax` | Shifts layers by a depth map as the pointer moves, for a 3D-ish parallax. |
| Displace | `displace` | Offsets pixels using a map of normals or offsets. |
| Lens distortion | `lens_distort` | Barrel or pincushion distorts the picture like a lens. |
| Perspective | `perspective` | Warps the four corners of the picture into a new quad. |
| Pixelate | `pixelate` | Blocks the picture into large square or hexagonal pixels. |
| Twirl | `twirl` | Twists the picture around a center point. |

### Light

| Name | Id | What it does |
|---|---|---|
| Glow | `bloom` | Glows bright areas of the picture outward. |
| Light rays | `god_rays` | Draws light rays spreading from a bright source. |
| Iris | `iris` | Darkens outside a circle that follows the pointer, like a spotlight. |
| Lens flare | `lens_flare` | Draws a lens flare from a light source. |
| Light shafts | `light_shafts` | Draws soft colored light shafts across the picture. |
| Reflection | `reflection` | Adds a wavy reflection below the picture. |
| Shine | `shine` | Sweeps a shine or glint band across the picture. |
| Vignette | `vignette` | Darkens the corners of the picture. |

### Motion

| Name | Id | What it does |
|---|---|---|
| Caustics | `caustics` | Animates shifting light patterns like sunlight through water. |
| Clouds and fog | `clouds` | Draws drifting procedural clouds or fog. |
| Electric | `electric` | Draws animated electric arcs. |
| Fire | `fire` | Draws animated fire using a color gradient. |
| Flow | `flow` | Distorts the picture along a flow map over time. |
| Fluid | `fluid` | Simulates a fluid that reacts to the pointer. |
| Glitch | `glitch` | Glitches the picture with shifted rows and color splits. |
| Scroll | `scroll` | Scrolls the picture, repeating at the edges. |
| Shake | `shake` | Shakes the picture along a direction or a painted flow map, by noise or a periodic push. |
| Spin | `spin` | Spins the picture around a center point. |
| Sway | `sway` | Sways the picture like something hanging, with stiffness and spread, or makes foliage tremble. |
| Motion trail | `trail` | Leaves a fading trail of past frames. |
| Water rings | `water_rings` | Draws expanding rings like drops on water. |
| Water ripples | `water_ripple` | Simulates ripples on water that react to the pointer and clicks. |
| Waves | `waves` | Distorts the picture with one or two wave patterns. |

### Stylize

| Name | Id | What it does |
|---|---|---|
| CRT screen | `crt` | Simulates an old CRT screen with scanlines and curvature. |
| Edges | `edges` | Outlines edges, with an optional sketch look. |
| Film grain | `film_grain` | Adds animated film grain. |
| Glitter | `glitter` | Scatters small sparkling points over the picture. |
| VHS | `vhs` | Simulates a worn VHS tape with noise and color smear. |

### Transform

| Name | Id | What it does |
|---|---|---|
| Transform | `transform` | Offsets, scales, rotates and skews the picture. |

## How to adjust an effect's parameters

**Where it is:** inspector, with the effect chosen (see [How to see the effects of a layer](basics.md#how-to-see-the-effects-of-a-layer-reorder-them-and-remove-them)).

**Steps:**

1. Each parameter is a field with its name. Hover over the name to see its help.
2. A number has a slider and a box to type it. A parameter in degrees also has a dial. A color has its picker. An image is chosen from a list of the package's images.
3. The **Default** button (the back icon) leaves the parameter as it comes from the factory.
4. The menu next to each field that can be animated lets you put a keyframe, follow a user property, the pointer or the sound, or leave it fixed. See [editor-animation](../editor-animation.md) and [editor-properties](../editor-properties.md).
5. A parameter that follows something shows a **Follows …** button; click it to edit the source.

**In the terminal:** the `effects` action with `set` and `params`. See [actions](../actions.md).

## How to place an effect's points on the canvas

Some parameters are a place or a direction: the center of a twirl, the source of some rays, the corners of a perspective. Instead of typing numbers, you place them by dragging.

**Where it is:** the canvas. The handles appear while the effect, or one of its parameters, is chosen in the inspector.

**Steps:**

1. Choose the effect in the inspector.
2. Look at the canvas: each parameter with a handle draws its own over the layer, with the layer's motion, parent and origin already applied.
3. Drag the handle:
   - A **point** (circle): **Place this point**.
   - A **corner** (square), like the four of the perspective: they are joined into a quadrilateral.
   - A **direction** (neutral-colored arrow, with the handle at the tip): **Aim the direction**. It starts from the effect's first point or, if it has none, from the layer's center.
   - An **angle** (circle around a point, with the handle on the edge): **Turn the angle around its point**. Parameters in degrees also have a dial in the inspector.
   - Two paired points (for example start and end) are joined with a line. The handle at each end is **Place this end; hold Shift to move the whole line**. The square handle in the middle, **Move the whole line**, moves both ends at once.
4. Each drag is a single **Undo** step.

In a package effect's descriptor, each parameter says which handle it receives (see [How to edit the descriptor of an effect](package-effects.md#how-to-edit-the-descriptor-of-an-effect)).

**Shortcut:** hold **Shift** while dragging an end to move the whole line (`SceneEffectPointOverlay.qml`).

**In the terminal:** the `effects` action with `grip` (the parameter and the point to take it to). See [actions](../actions.md).
