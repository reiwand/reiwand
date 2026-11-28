# Motion effects

On this page: the effects that move a layer.

## The effects the program brings

Each effect carries its name as it appears in the app, its cost and its parameters, with their
default values and the range the program accepts. Ranges are checked on validation:
`/layers/2/effects/0/params/radius is 300, expected 0..=128`.

Some facts that apply to several:

- `tilt_shift` blurs outside a band or, if you give it a `depth`, according to a depth map.
- `glitch` stays still until you set `bursts` or `beat`.
- `film_grain` only changes over time if `fps` is greater than 0.
- `depth_parallax` costs nothing while it does not move anything (no orbit, with the pointer far
  away or with `strength` at 0). `overscan` enlarges the layer so the edges are not seen.
- Effects with a phase (`phase`) are `linear` images whose brightness shifts the time of each
  point, so neighboring parts do not move together.
- `gradient` images are a single row, and are read from left to right.

## Motion

**`caustics`** (Caustics, light). Moving lights like the sun through water.

Parameters: `color` (color, default `#cfefff`); `scale` (number, default 2; 0.1 to 16); `speed` (number, default 0.5; 0 to 8); `strength` (number, default 0.6; 0 to 4); `perspective` (number, default 0; 0 to 1).

**`clouds`** (Clouds and fog, medium). Clouds or fog drifting by, made with noise.

Parameters: `color` (color, default `#ffffff`); `shade` (color, default `#a7b0c2`); `opacity` (number, default 0.8; 0 to 1); `coverage` (number, default 0.45; 0 to 1); `softness` (number, default 0.3; 0.01 to 1); `size` (number, default 320; 8 to 16384); `speed` (`[x, y]`, default `[6, 0]`; -1024 to 1024); `warp` (number, default 0.8; 0 to 4); `octaves` (number, default 5; 1 to 8); `height` (number, default 1; 0 to 1).

**`electric`** (Electric, light). Thin, crackling lines where two shifting noises cross.

Parameters: `color_a` (color, default `#66ccff`); `color_b` (color, default `#ffffff`); `bounds` (`[x, y]`, default `[0.4, 0.6]`; 0 to 1); `scale` (number, default 2; 0.1 to 16); `smoothness` (number, default 0.1; 0 to 1); `speed` (number, default 1; 0 to 8); `multiply` (number, default 1; 0 to 8); `blend` (blend mode, default `add`).

**`fire`** (Fire, light). Lifts and flickers the flames already in the image, and colors them by brightness with a gradient.

Parameters: `speed` (number, default 1; 0 to 8); `strength` (number, default 8; 0 to 64); `scale` (number, default 3; 0.1 to 16); `gradient` (image).

**`flow`** (Flow, light). Warps the image along a flow map; without a map it uses a flow field of its own.

Parameters: `map` (image); `strength` (number, default 8; 0 to 512); `speed` (number, default 0.25; 0 to 8); `direction` (`[x, y]`, default `[1, 0]`; -1 to 1); `noise` (yes/no, default no); `phase` (image).

**`fluid`** (Fluid, heavy). Smoke, fire or liquid simulated over the layer, with point and line emitters and the pointer. Each emitter is `{ "from", "to", "angle", "strength" }`, in fractions of the layer (-2 to 4); `angle` from -360 to 360 (-90 is up) and `strength` from 0 to 64; up to 64 emitters.

Parameters: `render` (choice, default 0): 0 gradient, 1 emitter color, 2 the image itself, 3 distortion only; `gradient` (image); `color` (color, default `#ffffff`); `velocity_decay` (number, default 0.98; 0 to 1); `dye_decay` (number, default 0.97; 0 to 1); `push` (number, default 1; 0 to 8); `emitters` (list of emitters); `strength` (number, default 1; 0 to 8); `blend` (blend mode, default `normal`); `perspective` (number, default 0; 0 to 1); `opaque` (yes/no, default no).

**`glitch`** (Glitch, light). Digital glitch with shifted rows and split colors. It stays still until you set `bursts` or `beat` (a beat holds it for `length` seconds).

Parameters: `bursts` (number, default 0; 0 to 10); `beat` (yes/no, default no); `length` (number, default 0.2; 0.02 to 2); `rows` (number, default 24; 1 to 256); `threshold` (number, default 0.7; 0 to 1); `shift` (number, default 24; 0 to 512); `split` (number, default 4; 0 to 64); `jitter` (number, default 2; 0 to 64); `speed` (number, default 12; 0 to 60).

**`scroll`** (Scroll, light). Scrolls the image and repeats it at the edges (or not, with `repeat_x` and `repeat_y` off).

Parameters: `speed` (`[x, y]`, default `[0.01, 0]`; -4 to 4); `repeat_x` (yes/no, default yes); `repeat_y` (yes/no, default yes).

**`shake`** (Shake, light). Shakes the image along a direction or a painted flow map, with noise or with a periodic push.

Parameters: `strength` (number, default 0; 0 to 256); `speed` (number, default 2; 0 to 60); `direction` (`[x, y]`, default `[1, 1]`; -1 to 1); `noise` (number, default 0; 0 to 1); `friction` (number, default 0; 0 to 1); `bounds` (`[x, y]`, default `[1, 1]`; 0 to 1); `phase` (image); `flow` (image); `motion` (choice, default 0): 0 noise, 1 periodic push; `side` (choice, default 0): 0 both ways, 1 forward only, 2 backward only. The periodic push is a soft push with the return set by `friction`, not the exact sawtooth of the package effect `flow_shake`; when migrating a package you can keep its effect.

**`spin`** (Spin, light). Rotates the part of the image near a center, with less force toward `radius`.

Parameters: `center` (point, default `[0.5, 0.5]`); `angle` (number, default 0; -360 to 360); `speed` (number, default 0; -720 to 720); `radius` (number, default 0.5; 0 to 2).

**`sway`** (Sway, light). Swings the image like something hanging, or with `foliage` makes leaves tremble.

Parameters: `amplitude` (number, default 6; 0 to 256); `speed` (number, default 0.4; 0 to 10); `stiffness` (number, default 2; 0 to 8); `spread` (number, default 1.5; 0 to 64); `swing` (yes/no, default no); `angle` (number, default 3; 0 to 90); `pivot` (`[x, y]`, default `[0.5, 0]`; -1 to 2); `double` (yes/no, default no); `phase` (image); `foliage` (yes/no, default no); `ratio` (number, default 1; 0.1 to 10).

**`trail`** (Motion trail, medium). Leaves fading copies of what the previous effects drew, the same at any frame rate.

Parameters: `persistence` (number, default 0.8; 0 to 0.99).

**`water_rings`** (Water rings, light). Expanding rings, like drops in water.

Parameters: `amplitude` (number, default 4; 0 to 128); `wavelength` (number, default 48; 2 to 4096); `speed` (number, default 32; -1024 to 1024); `falloff` (number, default 400; 1 to 16384); `center` (`[x, y]`, default `[0.5, 0.5]`; -1 to 2).

**`water_ripple`** (Water ripples, medium). Water waves that react to the pointer and to clicks.

Parameters: `strength` (number, default 32; 0 to 256); `damping` (number, default 0.96; 0.5 to 0.97); `radius` (number, default 24; 1 to 256); `cursor` (number, default 1; 0 to 8); `click` (number, default 1; 0 to 8); `beat` (number, default 0; 0 to 8); `specular` (number, default 0.15; 0 to 2); `perspective` (number, default 0; 0 to 1); `reflection` (image); `tint` (color, default `#ffffff`); `walls` (image); `speed` (number, default 1; 0 to 4).

**`waves`** (Waves, light). Distorts the image with one or two waves.

Parameters: `amplitude` (number, default 8; 0 to 256); `frequency` (number, default 4; 0 to 64); `speed` (number, default 1; -20 to 20); `direction` (number, default 0; -360 to 360); `count` (integer, default 1; 1 to 2); `angle2` (number, default 90; -360 to 360); `perspective` (number, default 0; 0 to 1); `phase` (image).
