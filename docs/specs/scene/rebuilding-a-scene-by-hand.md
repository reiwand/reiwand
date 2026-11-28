# Rebuilding a scene by hand

An author who rebuilds a wallpaper made with another program writes its parts with these fields;
the scene format has a field for each of them, and the features above say which ones an engine
draws today.

| The original has | In `scene.json` |
|------------------|-----------------|
| layers placed by their center with Y up from the bottom left, angles in counterclockwise radians and opacity from 0 to 100 | `transform.position` with `origin: [0.5, 0.5]`, y = canvas height − y, degrees = −radians × 180 / π, `opacity` / 100 |
| a scene of one picture without effects | a package of `kind: image` |
| a full-screen layer that applies effects to what is below it | an `adjustment` layer; with a region and its own transform, `size` and `redraw` |
| a flat color layer | a `solid` layer |
| an orthographic projection of a given size and the user's alignment | `canvas.size` and `canvas.fit` (`fill` covers the screen) |
| a parent that moves a layer without grouping it | `parent` and `inherit` |
| a layer color and a brightness above 1 | `color` and `brightness` (with `canvas.hdr`, so that light above white is kept) |
| a texture without mipmaps, with sharp pixels or repeated outside its edges | the asset's `mipmaps`, `filter: nearest` and `wrap` |
| a sprite sheet or an animated GIF that plays by itself | the asset's `atlas` or `rects` with `fps` or `durations`, or `animated: true` |
| a text with a maximum width and number of rows, vertical alignment or an opaque background | `wrap` (`width`, `lines`, `ellipsis`), `valign`, `line_height`, `background` and `padding` of the text |
| a font collection | the `.ttc` asset with `face` |
| a clock or a date written by a script | `{ "clock" }` and `{ "date" }` values, which wake the scene only when the text changes |
| the time of day used as a number | `{ "clock": "day" }`, or `engine.daytime` in scripts |
| keyframes with Bézier handles, or each axis animated on its own | keys with `{ "in", "out" }` and `{ "x", "y" }` or `{ "r", "g", "b", "a" }` parts |
| an animation added to the value it starts from | `base` of the value |
| camera parallax with an amount, a delay and a share for the pointer, and a depth per axis | `canvas.parallax` and `parallax: [x, y]` of the layers |
| the blending of a material or of particles | the layer's `blend` |
| colors mixed as encoded values | `canvas.blend_space: srgb` |
| an effect mask painted in an editor | the effect's `mask`, a `linear` picture |
| an effect masked by another layer or by a shape | the effect's `mask` with `layer` or `shape` |
| an effect that reads what another layer draws | a `texture` parameter set to `{ "layer" }` |
| an effect that copies what lies below the layer | a pass that reads `backdrop` |
| blend modes such as linear light, pin light, hard mix or divide | `blend` of the layer, and `blend_mode()` in shaders |
| several properties moved by one animation | the same `timeline` on each value |
| an animation that counts frames | `frames` and `fps` of the timeline |
| a looping animation that blends its end into its start | `wrap: true` on a `repeat` timeline |
| named moments of an animation that show layers, play sounds or start other animations | `events` of the timeline, and `trigger` or `play_on` of `event:<name>`; scripts get them through `timelineEvent` |
| a script that changes the rate of an animation, stops it or reads the frame it is on | `timeline(id).speed`, `stop()`, `time` and `duration` (a frame is `time` × `fps`), and `on("ended", …)` |
| a sound object with several files played in turn or at random, with silences between them | `sounds` with `mode` and `pause` |
| background music, a whole song or MP3 files | `.mp3` assets, and `stream: true` for anything longer than 60 s |
| a sound played by an animation event | `play_on: "event:<name>"` |
| a text measured in points | `size` in pixels, points × 4 / 3 |
| a texture chosen with a checkbox or a list | `{ "prop", "map" }` of a `bool` or `combo` property |
| a texture the user replaces | a `file` property with `default_asset` |
| shader variants chosen before compiling | `options` of a package effect |
| shader uniforms shown as controls | `params` of a package effect |
| per-instance overrides of a particle system | `instance` of the `particles` layer, which may be bound to sound or the pointer |
| changes over the life of a particle | `render.size_over_life`, `color_over_life` and `init.alpha` |
| a deformable picture with bones | a `mesh` layer with a `.glb` exported from a rigging tool |
| a 3D model in FBX or OBJ | a `model` layer with the model converted to `.glb` |
| a wallpaper that runs a program | a `web` package exported with WebGL or WebAssembly; packages never run programs |

## Effects with an equivalent

Effects that other programs ship as built-ins become these effects. The built-in effects are written
from scratch, with the parameters written here, so an imported value is converted rather than copied.
Positions are fractions of the layer counted from the top left, angles are degrees that turn
clockwise, and sizes are pixels of the layer unless the row says otherwise; a value given as a
fraction of the picture becomes pixels when multiplied by the width of the layer, and an angle
counted counterclockwise changes its sign.

| The original effect | Built-in effect | Parameters to convert |
|---------------------|-----------------|-----------------------|
| a blur of the whole layer | `blur` | `radius` in pixels |
| a blur away from a center | `blur_radial` | `center`, `strength` (0 to 1) |
| a blur along a direction | `blur_motion` | `angle`, `length` in pixels |
| a color made transparent | `chroma_key` | `key`, `tolerance`, `softness`, `distance`, `keep` |
| edge detection | `edges` | `threshold`, `width` in pixels, `color` |
| a fisheye or lens bulge | `lens_distort` | `k1` (above 0 bulges), `center` |
| a four-point perspective | `perspective` | `top_left`, `top_right`, `bottom_right`, `bottom_left` |
| a shake | `shake` | `strength` in pixels, `speed`, `direction` |
| eyes that move | `iris` | `range` in pixels, `center` |
| water that flows along a painted map | `flow` | `map` (a `linear` picture), `strength` in pixels, `speed` |
| parallax from a depth map | `depth_parallax` | `depth` (a `linear` picture, bright is near), `strength` in pixels, `focus`, `reach` |
| a mirrored reflection | `reflection` | `line` (from the top), `opacity` |
| drifting clouds | `clouds` | `size` and `speed` in pixels, `coverage` |
| an opacity mask painted on the layer | the layer's `mask` | `texture`, `strength` |
| foliage that sways | `sway` | `amplitude` in pixels, `pivot`, `speed` |
| a line of light that crosses the layer again and again | `shine` | `angle`, `width` (fraction of the layer), `speed` (crossings a second), `gap` |
| rays from the bright parts of the picture | `god_rays` | `source`, `exposure`, `decay` |
| color fringes | `chromatic_aberration` | `amount` in pixels |
| film grain | `film_grain` | `amount`, `size` in pixels |
| a twirl | `twirl` | `angle`, `radius` in pixels, `center` |
| a picture that scrolls | `scroll` | `speed` in layers a second |
| water waves | `waves` | `amplitude` in canvas pixels, not layer pixels (a layer drawn at scale 3 with amplitude 20 still moves the picture by about 20 pixels of the canvas), `frequency` (waves across the layer), `speed`, `direction` |
| ripples where the pointer touches water | `water_ripple` | `strength`, `radius` in pixels |
| a tint with a blend mode | `tint` | `color`, `blend`, `amount` |
| up to six pictures or layers blended with modes | `blend_texture`, one use each | `texture` (an asset or `{ "layer" }`), `blend`, `amount` |
| animated beams of light | `light_shafts` | `direction`, `angle`, `center`, `color`, `gradient`, `exponent`, `intensity`, `feather`, `smoothness`, `speed` |

A test reads this table and checks that every effect and parameter in it exists.

`chroma_key` measures how far each pixel is from `key` in the OKLab color space. `distance` is
required: 0 (`chroma only`) compares only the color plane, ignoring lightness, which suits a green
or blue screen under uneven light but cuts every grey, white and black when the key is neutral; 1
(`full color`) adds lightness to the distance, so a white, grey or black key removes only that
tone. `keep` is also required (0 to 1): the opacity that the removed color keeps, 0 making it fully
transparent. Both must be written in every use of the effect, a scene without them is refused
(`/layers/N/effects/M/params` lacks the required parameter), and the editor writes them when it adds
the effect. `spill` pulls the color of the key out of the edges and does nothing for a neutral key.
The `chroma` material of a model already measures the full color distance and needs neither.
