# Effects

An effect changes how a layer looks: it blurs it, tints it, makes it ripple. These pages explain how
effects are put on, which effects the program brings, how they are limited with a mask and how to
write one of your own.

The examples come from [`examples/stacked-effects`](examples/stacked-effects/scene.json) and from
[`examples/own-effect`](examples/own-effect/scene.json), and they pass `reiwand-ctl pack check`.

Effects, by topic:

- [Putting effects on a layer](effects/on-a-layer.md): the effect list of a layer and its masks.
- [Motion effects](effects/motion.md): the effects that move a layer.
- [Light and color effects](effects/light-and-color.md): glow, light, shadows and color.
- [Blur, distortion and style effects](effects/blur-and-style.md): blur, distortion, style, blending and transformation.
- [Your own effect: the descriptor](effects/own-descriptor.md): what an own effect declares.
- [Your own effect: the shader](effects/own-shader.md): the shader, trying it, other shaders and own transitions.

## Not covered here

- Effects of particles and of 3D materials: [particles-and-sound.md](particles-and-sound.md) and
  [3d.md](3d.md).
- The effects editor, the mask brush and the ramp editors: [editor.md](editor.md).
