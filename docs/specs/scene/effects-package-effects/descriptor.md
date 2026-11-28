# The descriptor

A package may bring at most 32 effects of its own (`custom_effects`): `effects` in `scene.json`
maps an id (`[a-z0-9_]{1,32}`, not the id of a built-in effect) to `{ "descriptor": "<path>.json",
"wgsl": "<path>.wgsl" }`, two files of the package of at most 64 KiB each (256 KiB of shaders for
all of them), and layers use the id
like any effect. Readers parse the descriptor with the fields above, require its `cost`, compile
the shader with the contract below and refuse it with `/effects/<id>: <reason>` when it does not
compile, lacks an entry point or an `override` its options name, or declares bindings of its own;
then they check every use of it like the uses of built-in effects. A descriptor may list at most 2
`audio` inputs, each `{ "kind", "unit", "window", "reference", "hold_silence" }` with the fields of an audio binding
(`kind` and `unit` always; `window` from 0.05 to 60, `reference` and `hold_silence` required with `surge` and refused
otherwise). The shader reads the 64 bands of input `n` with `heard_band(n, i)`, 0 to 63, as the binding of that kind
would give them (a `surge` is above 1 when the band rises after quiet and decays back); reading it adds the audio
input like `band()` does, which stays relative in decibels. The inputs an effect needs are
found in the compiled shader (reading `frame.time`, the pointer or the audio, also through helper
functions) and added to its `uses`, so that the scene asks for them without the author listing
them. Package descriptors may also use `stage`: `material` for the shader of a model material, whose
WGSL defines `fn material(s: Surface) -> Surface`, or `particle` for a particle renderer
(`render.shader`), whose WGSL defines `fn particle(p: Sprite) -> Sprite` for each corner of each
particle; such descriptors have no passes, and the scene may only use them where their stage fits.
`Surface` has `position`, `normal`, `uv`, `albedo`, `emissive`, `roughness`, `metallic` and
`occlusion`; `Pixel` has `sample`, `color`, `brightness`, `alpha`, `uv`, `age` and `life`; `Sprite` has `position`, `velocity`, `corner`, `uv`, `color`, `size`, `rotation`, `age`
and `life`. A material shader has at most 8 parameters (`param(0u)` to `param(7u)`), none of them a
`lut`, `mask` or `emitters`, and samples up to 4 images: its `texture` parameters, in the order the
descriptor lists them, are read with `image0(uv)` to `image3(uv)` (a repeating, filtered read of
the picture, each counted as a texture read of the stage function in its cost). A material of the
scene names each image by the id of a picture asset of the package or `white`, `black`, `noise` or
`normal_flat`, as a literal (not bound to a property); one it does not name reads the `prefill`
of the descriptor. Readers also build a material shader the way the 3D renderer does, with only
`frame`, `param`, the images and the helpers of the material prelude, and refuse it with
`it does not build as a material shader: wgsl:<line>:<column>: <reason>` when it uses what only
layer effects have (`input_tex`, `sample`, `tap`, `mask_at` and the like). A material shader the
graphics card still cannot build is left out like an effect: its models are shaded with `pbr`
and `left_out` names it (`the effect <id> was left out: its models are shaded with pbr instead,
since <reason>`). Package descriptors may also use:

| Field | Meaning |
|-------|---------|
| `params` | at most 24; besides the kinds above, `emitters` (a list of at most 64 point and line emitters, given to the shader as a picture of three texels each, read with `textureLoad`, and its count in the slot, 0 while no emitter has a strength above 0; the list may be keyframed, where two keys with as many emitters blend each emitter of the same shape and a point that becomes a line jumps, follow a property through a `map` of lists, or be set by a script, but follows no sound, pointer or other number), `int` (whole numbers), `enum` (with `values`: 1 to 64 `{ "value", "label" }`), `blend` (one of the 34 blend modes, given to the shader as its number in the order of the list), `position` (a point of the layer the editor lets the author place; any parameter may also have `unit`: `degrees`, which makes a `float` an angle the editor turns on a dial on the canvas) and `mask` (a picture painted per channel, with `channels` 1, 3 or 4 and `paint`, the color or number a mask without a picture has); a `texture` may name its authoring tool with `tool`: `gray`, painted in grey with the mask brush, or `flow`, a flow map whose red and green hold a direction (0.5 is still, alpha 0 leaves the picture unread) painted with a comb that points each texel along the stroke drawn over it); a `texture` or `mask` whose value (its `prefill`, the value an effect use gives it or a value of its `map`) names no asset but is `white`, `black`, `noise` or `normal_flat` reads that generic picture, and the editor offers those four next to the assets; every parameter may have a `label` and a `help` per language, of at most 256 characters, and `yields_to`, another parameter that, while not zero, takes over: this one then asks for no input of its `uses`, and `waits_for`, another parameter that must not be zero for this one to ask for the input of its `uses` (the emitters of `fluid` wait for its `strength`, so a fluid at strength 0 rests once it settles) |
| `targets` | at most 8 named textures `{ "scale" (0 to 2 of the layer size), "format" }` with `rgba8` (prefilled) or `rgba16f`, kept between passes of one frame |
| `options` | at most 8 variants `{ "id", "values", "prefill", "when_bound" }`, each a WGSL `override` of the shader with 1 to 16 whole numbers or flags; a use sets them through `params`, as literals or bound to a property, and every combination it may need (at most 8 per use) is built when the scene loads, never during a frame; `when_bound` turns an option on while the named texture parameter has a picture; only an option that declares `when_bound` accepts `null` in the `params` of a use, meaning "chosen by the binding": it takes the `when_bound` value while the texture is bound and its `prefill` otherwise, and any other option refuses `null` |
| `compose` | `true` draws the layer over what lies below it first; the effect reads that picture as its input and its result replaces the layer's box on the scene |
| pass `target` | writes the pass into that target instead of the next texture; targets start transparent each frame, so passes that write them with a `blend` add up |
| pass `copy` | a pass without a shader that copies one input (`"copy": "target:history"`) into its output, resizing it when the sizes differ |
| pass `swap` | a pass without a shader that exchanges two targets of the same scale and format (`"swap": ["front", "back"]`), for ping-pong passes |
| pass `blend` | `replace`, `normal`, `add` or `multiply` onto its output, to accumulate without a copy |
| pass `vertex`, `grid` | a vertex entry point and a grid of 1 to 64 cells a side over the layer, to bend it (`vertex_effects`) |
| pass `inputs` | at most 9, also `backdrop` (what is already drawn under the layer, which then draws on its own), `target:<name>` and `asset:<mask parameter>`; they are bound to `input_tex` and then `aux0` to `aux7` |
