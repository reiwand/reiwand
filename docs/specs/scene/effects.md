# Effects

An effect use is `{ "effect", "enabled", "mask", "params", "optional" }`. The built-in effects are
the descriptors in `crates/reiwand-scene/effects/`, which list every parameter with its type, range
and prefilled value (`prefill`). `texture` parameters take the id of an image asset and `lut` parameters the id of a
`.cube` asset, both as literals, and a `texture` parameter may also take a layer (`{ "layer" }`,
see "Values"). The mask of an effect takes a texture, a layer or a shape, like the mask of a layer,
with `channel`, `invert`, `feather` and `strength`; a layer or shape mask (`effect_masks`) is drawn
over the box of the layer, so a shape is centered on it and sized in its pixels, and a layer masks
the effect where it lies on the screen. At lower quality levels (on battery, with software
rendering or when the frame budget is exceeded) medium and heavy effects run their inner passes at
half or a quarter of their size, use the cheaper settings of their descriptor or are dropped,
`optional` ones first; the pass that writes the result of an effect keeps its full size. An effect
dropped at the current level needs no input, and neither does one whose `enabled` or input
parameter is bound to a property that currently turns it off.

Positions such as `center` and `source` are fractions of the layer (of the canvas on an adjustment
layer, which also places its masks and images there), and a `follow` flag hands them to the pointer
while it is on the output and eases them back within a quarter second after it leaves.
`tilt_shift` blurs outside a band, or by depth when `depth` names an image. `depth_parallax` shifts
each point by `(depth - focus) × strength` pixels, times `scale` on each axis (−1 to 1, so a parallax may move less or the other way up and down than across), towards the pointer (`follow`; with `smooth` the pointer as `canvas.parallax` eases it, with its `delay`, so the depth moves together with the parallax of the layers) or along a slow
orbit (`orbit`, which also takes over while the pointer is away), searching its `depth` image (a
`linear` asset, bright is near) in 8 to 24 `steps` (prefilled 16; 24 costs half as much again at Q3 only), at most 12 at Q2 and 8 at Q1, so that near
parts slide over far ones without tearing; `overscan` enlarges the layer to keep its edges out of
sight (still pictures with a parallax of their own use depth layers computed once instead, in which what a near object uncovers is filled with background; see "Depth layers" in `dbus.md`). While it would move nothing (no orbit, no `overscan` and the pointer away, centred or not
followed, or a `strength` of zero) the frame leaves it out, so it costs no pass; the phone, whose
pointer only follows touches, shows it this way most of the time. Still images with a depth map
move with the same search. `glitch` stays still until
`bursts` or `beat` is set (a beat keeps it on for `length` seconds), and `film_grain` only changes
over time with an `fps` above zero. `blur_motion` streaks along `angle` for `length` pixels, or,
with `follow`, along the way the layer moved: its speed in canvas pixels per second times
`shutter` seconds, at most `length` pixels; the scene draws one more frame once the layer stops so
that the streak goes away.

Blurs and distortions treat the edges of the screen like the edges of the picture: a layer that
reaches past the screen, or fills it, keeps the color of its outermost visible pixels instead of
fading into the empty space around it, on all four sides.

`tint` mixes `color` into the layer with a `blend` mode (prefilled multiply) by `amount` (0 to 1,
times the alpha of the color), or with `mode: 1` colorizes it: the color takes the brightness of each
point and `blend` does not apply. `blend_texture` blends `texture` (a picture or a layer) over the
layer with `blend` by `amount` times the alpha of the texture, placed on the layer, moved by `offset`
and repeated by `scale` (fractions of the layer); a layer may chain several, each one a pass, and the
effect never makes the layer more opaque. `light_shafts` draws beams of its own, made of noise
across their direction: `direction` 0 sends them along `angle` (degrees, 90 downwards), 1 from
`center` and 2 from the corner of the layer nearest to `center`; `exponent` and `smoothness` (0 hard
edges, 1 soft) shape them, `feather` fades their far end, `intensity` multiplies them and `color` or a
`gradient` (read from the start of the beams to their end) colors them. They add light, so they also
show on a transparent `solid` layer, and the scene only draws new frames for them while `speed` is
not zero.
