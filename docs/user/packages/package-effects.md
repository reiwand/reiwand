# Package effects

A package can bring effects of its own, written in WGSL. The desktop and the phone draw them. This
page explains what an effect can do, how Reiwand checks and compiles it, and how its cost is
measured.

## Structure of an effect

Each effect is a JSON descriptor and a WGSL shader with up to 24 passes. A pass can:

- write to the next texture or to named targets at their scale (at 8 or 16 bits per channel);
- copy them;
- swap them to alternate between two (ping-pong);
- accumulate with per-pass blending (replace, normal, add or multiply);
- deform the layer with a vertex shader over a grid of up to 64 × 64 cells;
- read up to 8 textures with their size.

An effect with `compose` reads the layer already drawn over what is underneath, and its result
replaces that area.

Variants (`options`, WGSL `override` constants) are all compiled when the scene loads, never in the
middle of a frame. The parameters and variants come out as controls in the editor.

## Inputs of the shader

The shader receives:

- the time of day and the frame time;
- the pointer and its previous position;
- the screen size;
- the spectrum of 16, 32 or 64 bands, mixed or per channel;
- the parallax;
- the color and opacity of the layer.

It has helper functions of its own: rotation, gray, HSV and the blend modes.

A mask without an image takes its color from `paint`. A texture can use the generic `white`,
`black`, `noise` or `normal_flat`.

## Wake rules

The inspector of each effect use shows its rules for waking the scene: which input each parameter
asks for, which it waits for and which it yields to. In the package's effects it changes them with
selectors, like `reiwand-ctl scene act FOLDER effect_rule`.

## Checks before the driver

Since the shader comes from the package, naga checks it before it reaches the driver:

| Rule | Limit |
| --- | --- |
| Bindings | No bindings of its own; all resources bounded. |
| Loops | Integer-counter loops that advance a fixed step up to a fixed limit without overflowing. |
| Expression depth | At most 256 operations deep. |
| Operations per pixel | At most 1,048,576. |
| Length | 32,768 operations long, counting each call in full. |
| Variables | 16 KiB per pixel. |

The shader is compiled in separate threads with a deadline of 10 seconds. If the driver rejects it
or it does not finish in time, the layer is drawn without the effect. The app, the tray,
`reiwand-ctl state` and the phone's diagnostics note it, without treating it as a failure.

## Crash protection

1. Before checking or compiling an effect for the first time, each program notes it as "on trial".
2. It deletes the note when it finishes or when its first frames pass.
3. A note left by a crashed or hung process counts as a failure, like a GPU that stops.
4. With two failures the effect is left out.

It remembers by the hash of the effect, so a new version is tried again. *Try again* in the app, in
the phone's diagnostics or `reiwand-ctl retry-effects` try it once more.

## Cost and speed

The engine measures on the GPU each frame, each effect (the built-in ones too) and the simulation
of the particles. `reiwand-ctl scene bench` shows the time as `gpu` and the particles part on the
simulation line.

A package effect is only left out for being slow when the package effects of a frame exceed the
limit by themselves. It is never left out because of what the layers or the built-in effects cost;
the quality level takes care of that.

Savings:

- The targets of package effects are cleared in their first pass.
- Adjustment layers read what is underneath without copying it. A stack of four package effects at
  4K costs about 10 % less.
- Depth parallax (`depth_parallax`) draws nothing while it would not move the image (no orbit or
  `overscan` and with the pointer outside, at the center or not followed), which on the phone is
  almost always.
- A single-pass effect with an image mask in `strength` mode is only drawn over the box where its
  mask is not zero, and nothing while that box is off the screen.
- A package effect can declare with `bounded` which textures limit where the image changes.

Each pass, each texture read and each shader operation count in the cost of the scene even if the
descriptor says less. `reiwand-ctl pack check`, `pack build` and the editor warn when a scene does
not fit at quality 2 on an old integrated GPU or a phone one.

## Porting shaders

`docs/specs/scene.md` explains how to port a shader from HLSL or GLSL to WGSL, with an equivalence
table.
