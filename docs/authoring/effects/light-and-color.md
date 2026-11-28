# Light and color effects

On this page: glow, light, shadows and color.

## Light

**`bloom`** (Glow, medium). Makes the light parts glow outward. With `tint` it tints the glow.

Parameters: `threshold` (number, default 0.8; 0 to 4); `intensity` (number, default 1; 0 to 8); `radius` (number, default 16; 0 to 128); `knee` (number, default 0.5; 0 to 1: the part of the threshold where the cutoff is softened); `tint` (color, default `#ffffff`); `saturation` (number, default 1; 0 to 4: 0 gives a gray glow and 4 a vivid one); `capped` (yes/no, default no: caps the brightest part); `limit` (number, default 1; 0 to 16, only with `capped`: the cap of each channel before the blur); `levels` (integer, default 1; 1 to 8: how many blurs, each twice as wide as the last, are added up); `scatter` (number, default 0.5; 0 to 2, only with `levels`: the weight of each wider level); `unit` (list, default canvas pixels; or screen pixels: which one `radius` is read in).

**`god_rays`** (Light rays, medium). Rays of light coming out of a bright source.

Parameters: `source` (`[x, y]`, default `[0.5, 0.15]`; -1 to 2); `threshold` (number, default 0.7; 0 to 4); `density` (number, default 0.8; 0 to 1); `decay` (number, default 0.96; 0.5 to 1); `exposure` (number, default 0.6; 0 to 4); `color` (color, default `#fff4e0`); `samples` (number, default 12; 4 to 16); `follow` (yes/no, default no); `mode` (choice, default 0): 0 the same offset moving away from the center, 1 the same offset along `angle`, 2 star; `angle` (number, default 90; -360 to 360); `noise` (number, default 0; 0 to 1); `edges` (integer, default 6; 2 to 16).

**`iris`** (Iris, light). Darkens outside a circle that follows the pointer, like a spotlight. It needs the pointer.

Parameters: `range` (number, default 12; 0 to 512); `follow` (number, default 0.05; 0 to 1); `center` (`[x, y]`, default `[0.5, 0.5]`; -1 to 2); `saccades` (yes/no, default yes); `rate` (number, default 1; 0.1 to 10); `background` (yes/no, default no).

**`lens_flare`** (Lens flare, medium). A core, a halo and reflections along the line that goes from a light to the center.

Parameters: `source` (point, default `[0.3, 0.3]`); `follow` (yes/no, default no); `intensity` (number, default 1; 0 to 4); `ghosts` (integer, default 4; 0 to 8); `halo` (number, default 0.3; 0 to 1); `color` (color, default `#ffffff`).

**`light_shafts`** (Light shafts, light). Soft, colored shafts of light made with noise, which add light (they also show over a transparent `solid` layer).

Parameters: `direction` (choice, default 0): 0 linear, 1 the same offset moving away from the center, 2 from the corner; `angle` (number, default 30; -360 to 360); `center` (point, default `[0.5, 0]`); `color` (color, default `#fff2cc`); `gradient` (image); `exponent` (number, default 1; 0.1 to 8); `intensity` (number, default 1; 0 to 4); `feather` (number, default 0.3; 0 to 1); `smoothness` (number, default 0.5; 0 to 1); `speed` (number, default 0.2; -8 to 8).

**`reflection`** (Reflection, light). A wavy reflection under the image.

Parameters: `line` (number, default 0.6; 0 to 1); `opacity` (number, default 0.6; 0 to 1); `fade` (number, default 1; 0 to 1); `amplitude` (number, default 2; 0 to 128); `frequency` (number, default 12; 0 to 512); `speed` (number, default 0.5; -20 to 20); `angle` (number, default 0; -360 to 360).

**`shine`** (Shine, light). Passes a band of brightness, a ring or a sparkle across the image.

Parameters: `angle` (number, default 30; -360 to 360); `width` (number, default 0.12; 0 to 1); `softness` (number, default 0.6; 0 to 1); `intensity` (number, default 0.4; 0 to 4); `color` (color, default `#ffffff`); `speed` (number, default 0.25; -4 to 4); `gap` (number, default 1; 0 to 8); `phase` (number, default 0; 0 to 1); `blend` (blend mode, default `add`); `gradient` (image); `style` (choice, default 0): 0 line, 1 double line, 2 ring, 3 sparkle.

**`vignette`** (Vignette, light). Darkens the corners.

Parameters: `intensity` (number, default 0.35; 0 to 1); `radius` (number, default 1; 0 to 2); `softness` (number, default 0.65; 0 to 1); `roundness` (number, default 1; 0 to 1); `center` (`[x, y]`, default `[0.5, 0.5]`; -1 to 2); `color` (color, default `#000000`).

## Color

**`chroma_key`** (Chroma key, light). Removes a color and makes it transparent, like a green screen.

Parameters: `key` (color, default `#00ff00`); `tolerance` (number, default 0.1; 0 to 1); `softness` (number, default 0.08; 0 to 1); `spill` (number, default 0.5; 0 to 1); `distance` (required; 0 compares only the chroma and suits green or blue screens with uneven light, 1 compares the full color, with its lightness, and is what removes a white, a gray or a black without taking the other tones); `keep` (required, number from 0 to 1: the opacity the removed color keeps, 0 leaves it fully transparent). The editor writes both when adding the effect; if they are missing from a file, the scene is rejected.

**`color_adjust`** (Color adjustments, light). Adjusts exposure, contrast, saturation and hue.

Parameters: `exposure` (number, default 0; -5 to 5); `contrast` (number, default 1; 0 to 4); `saturation` (number, default 1; 0 to 4); `hue` (number, default 0; -180 to 180).

**`color_lut`** (Color grading, light). Color grading with a `.cube` table.

Parameters: `lut` (table); `intensity` (number, default 1; 0 to 1).

**`posterize`** (Posterize, light). Reduces the image to a few color levels.

Parameters: `levels` (number, default 6; 2 to 64); `dither` (number, default 0.5; 0 to 1).

**`pulse`** (Pulse, light). Pulses the brightness (0), the color (1), the opacity (2) or a tinted light (3), with time, with noise or with a band of the sound (`audio_band`, from 1 to 64; it needs the audio input only then).

Parameters: `what` (choice, default 0): 0 brightness, 1 color, 2 opacity, 3 tinted light; `speed` (number, default 1; 0 to 16); `bounds` (`[x, y]`, default `[0, 1]`; 0 to 4); `tint_low` (color, default `#000000`); `tint_high` (color, default `#ffffff`); `noise` (number, default 0; 0 to 1); `noise_speed` (number, default 1; 0 to 16); `phase` (image); `audio_band` (integer, default 0; 0 to 64).

**`tint`** (Tint, light). Tints the image with a color (mode 0) or colorizes it (mode 1: the color takes the brightness of each point).

Parameters: `color` (color, default `#ff8800`); `blend` (blend mode, default `multiply`); `amount` (number, default 1; 0 to 1); `mode` (choice, default 0): 0 tint, 1 colorize.
