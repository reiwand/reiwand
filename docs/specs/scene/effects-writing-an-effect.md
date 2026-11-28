# Writing an effect

A built-in effect is two files in `crates/reiwand-scene/effects/`, found when the crate is built:
`<id>.json`, its descriptor, and `<id>.wgsl`, its fragment entry points. Nothing else changes.

```json
{
  "id": "nudge",
  "name": { "en": "Nudge" },
  "params": [{ "id": "distance", "type": "float", "prefill": 8, "min": -64, "max": 64 }],
  "mask_mode": "strength",
  "fuse": "uv",
  "reach": "distance"
}
```

```wgsl
fn uv_nudge(uv: vec2<f32>) -> vec2<f32> {
    return uv - px(param(0u).x) * vec2<f32>(1.0, 0.0) * mask_at(uv);
}
```

| Field | Meaning |
|-------|---------|
| `category` | a word such as `blur`, `color`, `distort`, `light`, `motion`, `blend`, `stylize` or `transform` (any other word falls back to its capitalized form); groups the effect in the catalog browser of the editor and in `reiwand-ctl scene catalog` |
| `description` | a short `Text` (plain, or per locale like `name`) of what the effect does, shown next to its cost in the catalog browser and in `reiwand-ctl scene catalog` |
| `uses` | `time`, `cursor`, `audio`: the scene keeps rendering while time runs, the pointer moves or sound plays; otherwise the effect costs nothing after its first frame |
| `pace` | when a time effect changes: `{ "rate": "<float parameter>" }` (that many times a second), `{ "sweep": ["<speed>", "<gap>", "<phase>"] }` (a band crosses the layer once every `1 + gap` turns and waits off it in between), `{ "steps": [<high>, <low>] }` (1 to 240 times a second, the second number at Q0) or `"outside"` (only while the pointer is off the output); the scene then sleeps until the picture changes |
| `mask_mode` | `mix` (prefilled) blends the result with the input by the mask; with `strength` the shader scales itself by `mask_at(uv)`, as distortions should, and must leave the picture as it is where the mask is zero: a single-pass effect masked by a picture (not a layer or a shape, not inverted, feathered or weakened) is then drawn only over the box of the lit texels of its mask channel, grown by `reach` and kept inside the visible part of the layer, and left out while that box is off screen |
| `params` | at most 16 of `float`, `vec2`, `color`, `bool`, `texture` and `lut`, with `id`, `prefill`, `min`, `max` and `uses` (`time`, `cursor`, `clicks` or `audio`; clicks come with the pointer), an input needed only while the parameter is not zero (a `follow` flag needs the pointer only when set, a `vec2` while either part is not zero), and `tools`, the editor tools that make the picture of a `texture` or `mask` parameter (`ramp`: a gradient stops editor that writes a ramp image, `gray`: a grey brush, `flow`: a direction brush for flow maps, `depth`: make a depth map) or `["table"]` on a `lut`; without `tools` the editor picks them by the id (`gradient` and `ramp` get `ramp`, `phase`, `walls` and `reflection` get `gray`, `map` gets `gray` and `flow`, `depth` gets `gray` and `depth`, a `lut` gets `table`), and the built-in effects declare theirs; parameter *n* is `param(nu)`, in `.x`, `.xy` or `.rgba` (linear) |
| `passes` | `{ "entry", "scale", "inputs", "output" }`; `scale` is a fraction of the layer size, and lower quality levels shrink every pass but the last; `inputs` (prefilled `["previous"]`) name `previous`, `input`, `source` (the layer before its effects), `mask`, `pass:N`, `state`, `backdrop` (what is already drawn under the layer, which then draws on its own) or `asset:<texture parameter>`, bound to `input_tex` and then `aux0` to `aux3` |
| `cost` | `{ "taps", "passes", "scale" }`; taps × passes × scale² units, light up to 1, medium up to 6, heavy above |
| `min_fps` | 1 to 240: the frame rate at which the effect still looks smooth; while the only things that move in a scene are effects that declare it, the scene runs at the highest of their `min_fps` instead of the full rate (never above the usual limits) |
| `velocity` | a `bool` parameter that, when set, gives the shader the speed of the layer in `fx.velocity.xy` (canvas pixels per second) and keeps the scene drawing while it moves |
| `fuse` | `uv` or `color` for single-pass effects, see below |
| `reach` | a float parameter, in pixels, by which the effect grows the layer bounds |
| `bounded` | texture or mask parameters outside whose lit texels (any of red, green and blue above zero) the effect leaves the picture as it is; the engine then draws a single-pass effect only over the box of those texels, grown by `reach` |
| `state` | `{ "scale", "settle", "fade" }`, persistent textures for simulations, see below |
| `solve` | `homography`: the first four parameters are `vec2` corners (top left, top right, bottom right, bottom left, as fractions of the layer) and the three slots after the parameters hold the rows of the matrix that takes a layer point back to the unit square; such an effect is not fused |
| `quality` | overrides per level, `{ "q1": { "params": { "taps": 8 }, "scale": 0.5 } }`, applied at that level and below; a number is an upper limit for the value the scene sets, other values replace it, `scale` shrinks the inner passes and the state, and `"freeze": true` keeps an effect that would be dropped at that level: its state stops and it shows its last picture without asking for frames |
| `lighter` | a cheaper variant of the effect for low quality levels, `{ "passes": [...], "cost": {...}, "up_to": "q1" }`, see "Lighter variants" below |

## Lighter variants

```json
"lighter": {
  "passes": [{ "entry": "fs_blur_lite", "scale": 0.5 }, { "entry": "fs_blur_lite_mix", "inputs": ["previous", "input"] }],
  "cost": { "taps": 2, "passes": 2, "scale": 0.5 },
  "up_to": "q0"
}
```

An effect may carry a lighter variant that the engine draws instead of leaving the effect out, so
that a heavy scene still looks close to itself on a slow device. Its `passes` are entry points of
the same shader file, written as the passes of the effect are (same inputs, targets, state and
limits); its `cost` is its own, and both the declared cost and, for a package effect, the measured
one must be lower than those of the effect, or the reader refuses the descriptor. Everything else
(parameters, mask mode, `reach`, `state`, `quality` overrides, `uses`) is that of the effect. An
effect that is fused (`fuse`) cannot have one. A descriptor without `lighter` behaves as before.

At each quality level the engine picks one form of each use, the same at an automatic level and at
one the user fixed: at `up_to` and below (if it is given) the variant whenever it fits that level;
otherwise the full effect when it fits the level, and the variant where the level would leave the
effect out (medium effects at Q0, `optional` ones at Q1, as in the table of costs below); an effect
whose variant does not fit either keeps its `freeze` or is left out as before. The hard budget of a
view tries the variant of a use that does not fit before it leaves the use out, and notes it as
`the effect <id> on <layer> was drawn with its lighter variant: <reason>` instead of `... was left
out: <reason>`; `reiwand-ctl pack check` prints the same notes. The cost of a scene at a level counts
the form drawn there. Pipelines of variants are built when the scene loads, as those of the full
effects are, and choosing one allocates nothing while frames are drawn. The editor shows the form
each level draws in the cost of an effect (the query `effect_cost` gives `form`: `full`,
`lighter`, `frozen` or `out`, per level). A package effect with a lighter variant names the feature
`lighter_effects`.

Every shader starts from one prelude. `frame` holds `time`, `dt`, `resolution`, `cursor` and
`cursor_delta` (fractions of the output), `buttons` (the pressed pointer buttons as a bit mask),
`inside` (1 while the pointer is on the output), `presence` (`inside` eased over a quarter second),
`bass`, `mid`, `treble`, `level`, `beat`,
`since_beat` (seconds since the last beat), `seed`,
`quality`, 64 audio bands (`band(i)`) and `glide` (the pointer as `canvas.parallax` eases it with its `delay`, from −1 to 1 on each axis, 0 in the middle; reading it needs the pointer). The samplers are `samp` (clamped) and `wrap` (repeating);
`lut_tex` is the 3D texture of the first `lut` parameter, read with `lut(rgb)`. Helpers: `tap(uv)`,
`sample(t, uv)`, `px(pixels)`, `mask_at(uv)`, `layer_uv(uv)` and its inverse `from_layer(uv)`,
`asset_at(t, uv)` (an asset placed on the layer, such as a depth map), `cursor_uv()`,
`screen_uv(uv)`, `to_linear` and `from_linear` (they convert between the values the effect reads and linear light according to `canvas.blend_space`: with `linear`, the values already are linear and both return their input unchanged; with `srgb` they decode and encode the sRGB curve), `straight`, `oklab`, `from_oklab`, `gauss`, `hash`,
`hash2`, `noise1`, `value_noise`, `gradient_noise`, `fbm(p, octaves)`, `bayer(pixel)`,
`layer_extent()` (the size of the layer in output pixels), `time_phase(t, uv)` (the brightness of a
`phase` picture at that point), `outside_layer(uv)` (the point, or `OUTSIDE_UV` when it is off the
layer; a `uv_<id>` function that returns `OUTSIDE_UV` leaves that point transparent) and
`relief(depth, sampler, uv, shift, focus, steps)`, the depth search of `depth_parallax`.
`fx.texel` is the size of the first input, `fx.info.z` the pass index, `fx.area` the effect
rectangle in output pixels, `fx.clip` the part of it that is on the screen (as fractions, where
`sample` and `tap` stop reading) and `fx.velocity` the speed of the layer when the descriptor
names a `velocity` parameter. Effects may not declare bindings of their own.

An effect with `fuse` defines `fn uv_<id>(uv) -> vec2<f32>`, where its output reads its input, or
`fn color_<id>(color, uv) -> vec4<f32>` on premultiplied colors, and gets the single pass
`fs_<id>`. Consecutive light fused effects of a layer without masks and with a fixed `enabled` run
as one generated pass, compiled when the scene loads, so their helper functions start with the
effect id.

`"state": { "scale": 0.25, "settle": 3 }` gives the effect two textures (16-bit float where the GPU
allows) at that fraction of the layer size, kept between frames. A pass with `"output": "state"`
writes the next one once per 1/60 s of scene time (at most four times a frame, so a simulation
keeps its speed at any frame rate; `fx.info.y` counts these steps) and `state` reads the latest.
They start transparent, restart when the layer size changes and are dropped when the scene leaves,
stops or pauses. `settle` keeps rendering that many
seconds after the pointer moves, a button is pressed or the sound stops, so ripples fade before the
scene goes still, and also after the layer or the effects before it last changed. `fade` names a
float parameter by which the state is kept each step: the scene then also renders until a picture
kept that way has faded below 1/512 (at most 60 s). In a built-in effect, passes with `"output":
"step"` before the pass that writes the state run with it on each step, into 16-bit textures of
their own, so that a simulation can solve in several passes; package effects use targets instead.

`cargo test -p reiwand-scene` checks every descriptor and compiles its shader. A GPU test for an
effect goes in `crates/reiwand-render/tests/effects/<id>.rs`, also found automatically:

```rust
use super::*;

#[test]
fn nudge_moves_the_image() {
    let Some(out) = Fx::new("nudge", json!({ "distance": 8 })).input(halves(RED, BLUE)).shot() else { return };
    out.check(&[((36, 16), RED), ((44, 16), BLUE)]);
}
```

`Fx::chain(effects)`, `.adjust()` (effects on an adjustment layer), `.image(id, image, linear)`,
`.lut(id, text)` and `.quality(level)` set up the scene. `.run()` returns a `Run` whose `runtime`
(time, audio) and `pointer([x, y])` change between `shot()`s, and whose `stats()` count passes and
new textures. A `Shot` has `at`, `check`, `mean` and `diff`.
