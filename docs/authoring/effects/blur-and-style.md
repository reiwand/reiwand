# Blur, distortion and style effects

On this page: blur, distortion, style, blending and transformation.

## Blur

**`blur`** (Blur, medium). Blurs the image with a fixed radius. `blend` and `amount` mix the blurred image with the sharp one.

Parameters: `radius` (number, default 8; 0 to 128); `blend` (blend mode, default `normal`); `amount` (number, default 1; 0 to 1).

**`blur_motion`** (Motion blur, medium). Blurs in one direction, as if moving fast. With `follow` it does so according to the real movement of the layer, in pixels per second times `shutter`, up to `length`.

Parameters: `angle` (number, default 0; -360 to 360); `length` (number, default 16; 0 to 256); `samples` (number, default 12; 2 to 32); `follow` (yes/no, default no); `shutter` (number, default 0.05; 0 to 0.5).

**`blur_radial`** (Radial blur, medium). Blurs outward from a center, like a zoom.

Parameters: `center` (`[x, y]`, default `[0.5, 0.5]`; -1 to 2); `strength` (number, default 0.1; 0 to 1); `samples` (number, default 8; 2 to 16); `follow` (yes/no, default no).

**`local_contrast`** (Local contrast, medium). Adds contrast around edges without changing the overall exposure; it is a good idea to mark it `optional`.

Parameters: `radius` (number, default 8; 1 to 64); `amount` (number, default 0.5; 0 to 4); `luma_only` (yes/no, default yes).

**`tilt_shift`** (Tilt-shift, medium). Blurs outside a focused band, like a miniature photo; with `depth` it blurs according to a depth image.

Parameters: `focus` (number, default 0.5; 0 to 1); `width` (number, default 0.15; 0 to 1); `softness` (number, default 0.25; 0 to 1); `angle` (number, default 0; -360 to 360); `radius` (number, default 12; 0 to 64); `depth` (image).

## Distortion

**`chromatic_aberration`** (Chromatic aberration, light). Splits the color channels near the edges, like a cheap lens.

Parameters: `amount` (number, default 3; -64 to 64); `center` (`[x, y]`, default `[0.5, 0.5]`; -1 to 2); `mode` (choice, default 0): 0 grows with the distance from the center, 1 the same offset along `angle`, 2 the same offset moving away from the center, 3 grows with the square of the distance; `angle` (number, default 0; -360 to 360).

**`depth_parallax`** (Depth parallax, medium). Shifts each point according to a depth map, toward the pointer (`follow`) or in a slow orbit (`orbit`), for a 3D effect. `depth` is a `linear` asset where light is near. `reach` is required: `circle` (0) clips the pointer direction to a circle, so in a corner each axis reaches 0.707 of the edge value; `square` (1) lets x and y each go from −1 to 1 on their own, like the parallax of layers, and in a corner the relief moves as much as at the edge. The editor adds it as `circle` and draws the path of the pointer. While it would not move anything it costs no pass.

Parameters: `depth` (image); `strength` (number, default 24; 0 to 256); `focus` (number, default 0.5; 0 to 1); `steps` (number, default 16; 8 to 24); `overscan` (number, default 1; 1 to 1.25); `follow` (yes/no, default yes); `orbit` (yes/no, default no); `scale` (`[x, y]`, default `[1, 1]`; -1 to 1); `smooth` (yes/no, default no).

**`displace`** (Displace, light). Displaces pixels with a normal map (mode 0) or a displacement map (mode 1), or with noise if you give no map.

Parameters: `map` (image); `mode` (choice, default 0): 0 normals, 1 displacements; `scale` (number, default 1; 0.1 to 16); `strength` (number, default 8; 0 to 256); `scroll` (`[x, y]`, default `[0, 0]`; -4 to 4); `ratio` (number, default 1; 0.1 to 10).

**`lens_distort`** (Lens distortion, light). Barrel or pincushion distortion, like a lens.

Parameters: `k1` (number, default 0.1; -1 to 1); `k2` (number, default 0; -1 to 1); `fit` (yes/no, default yes); `center` (`[x, y]`, default `[0.5, 0.5]`; -1 to 2); `background` (yes/no, default no).

**`perspective`** (Perspective, light). Warps the four corners of the image toward another quadrilateral.

Parameters: `top_left` (`[x, y]`, default `[0.04, 0]`; -2 to 3); `top_right` (`[x, y]`, default `[0.96, 0]`; -2 to 3); `bottom_right` (`[x, y]`, default `[1, 1]`; -2 to 3); `bottom_left` (`[x, y]`, default `[0, 1]`; -2 to 3).

**`pixelate`** (Pixelate, light). Turns the image into large square or hexagonal pixels.

Parameters: `size` (number, default 8; 1 to 256); `hex` (yes/no, default no).

**`twirl`** (Twirl, light). Twists the image around a center.

Parameters: `angle` (number, default 60; -1080 to 1080); `radius` (number, default 256; 0 to 16384); `center` (`[x, y]`, default `[0.5, 0.5]`; -1 to 2); `aspect` (number, default 1; 0.1 to 10); `inner` (number, default 0; 0 to 1); `noise` (number, default 0; 0 to 1); `repeat` (yes/no, default no).

## Style

**`crt`** (CRT screen, light). An old tube screen, with scan lines and curvature.

Parameters: `curvature` (number, default 0.1; 0 to 1); `pitch` (number, default 4; 2 to 64); `scan` (number, default 0.3; 0 to 1); `phosphor` (number, default 0.15; 0 to 1); `vignette` (number, default 0.25; 0 to 1); `glow` (number, default 0.15; 0 to 1).

**`edges`** (Edges, medium). Draws the edges, with an optional sketch look.

Parameters: `strength` (number, default 1; 0 to 1); `threshold` (number, default 0.25; 0 to 4); `width` (number, default 1; 0.5 to 8); `color` (color, default `#000000`); `sketch` (yes/no, default no); `paper` (color, default `#ffffff`).

**`film_grain`** (Film grain, light). Film grain; it only changes over time if `fps` is greater than 0.

Parameters: `amount` (number, default 0.05; 0 to 1); `size` (number, default 1.5; 0.5 to 16); `fps` (number, default 0; 0 to 60); `mono` (yes/no, default no); `bias` (number, default 0; -1 to 1); `blend` (blend mode, default `overlay`).

**`glitter`** (Glitter, light). Small star-shaped sparks on a grid.

Parameters: `density` (number, default 0.3; 0 to 1); `size` (number, default 2; 0.5 to 16); `speed` (number, default 1; 0 to 8); `color` (color, default `#ffffff`).

**`vhs`** (VHS, light). A worn video tape: a rolling tracking line, broken rows, snow, streaks and smeared colors.

Parameters: `strength` (number, default 0.5; 0 to 2); `speed` (number, default 1; 0 to 8); `noise` (number, default 0.3; 0 to 1); `aberration` (number, default 2; 0 to 16); `band` (number, default 1; 0 to 1); `artifacts` (number, default 0; 0 to 3); `invert` (yes/no, default no).

## Blending

**`blend_gradient`** (Gradient blend, light). Blends another image with a gradient map that says where it shows; without a map, from left to right.

Parameters: `texture` (image); `map` (image); `progress` (number, default 0.5; 0 to 1); `softness` (number, default 0.1; 0 to 1).

**`blend_texture`** (Blend texture, light). Puts an image over the layer with a blend mode. You can chain several; it never makes the layer more opaque.

Parameters: `texture` (image); `blend` (blend mode, default `multiply`); `amount` (number, default 1; 0 to 1); `offset` (`[x, y]`, default `[0, 0]`; -2 to 2); `scale` (`[x, y]`, default `[1, 1]`; 0.01 to 16).

**`dissolve`** (Dissolve, light). Dissolves the image into a grainy pattern.

Parameters: `progress` (number, default 0; 0 to 1); `size` (number, default 48; 1 to 4096); `edge` (number, default 0.06; 0 to 1); `glow` (color, default `#ffa040`).

**`reveal`** (Reveal, light). Shows another image through a circle or an ellipse that can follow the pointer.

Parameters: `texture` (image); `shape` (choice, default 0): 0 circle, 1 ellipse; `radius` (number, default 0.15; 0 to 1); `softness` (number, default 0.3; 0 to 1); `follow` (yes/no, default no); `invert` (yes/no, default no); `center` (point, default `[0.5, 0.5]`).

## Transformation

**`transform`** (Transform, light). Moves, scales, rotates and skews the image after the previous effects, inside its box.

Parameters: `offset` (`[x, y]`, default `[0, 0]`; -2 to 2); `scale` (`[x, y]`, default `[1, 1]`; 0.01 to 16); `angle` (number, default 0; -360 to 360); `skew` (`[x, y]`, default `[0, 0]`; -89 to 89); `pivot` (point, default `[0.5, 0.5]`).
