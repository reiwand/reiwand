# Built-in effects

The desktop and the phone draw a set of built-in effects, new ones and new parameters of effects
that already existed. With the default values everything looks exactly as before.

## New effects

The desktop and the phone also draw:

- gradient blending;
- pointer reveal;
- displacement by a map or by a noise of its own;
- caustics, glints, fire and spin;
- transformation;
- pulse (also to the beat of a sound band);
- local contrast;
- motion trail;
- VHS;
- fluids;
- lens flare;
- electric crackle.

See also the `tint`, `blend_texture` and `light_shafts` effects in
[Scene features](scene-features.md).

## Fluids

Fluids have as many point and line emitters as needed, up to 64. Emitters:

- are placed by the editor with handles on the canvas, and turned and strengthened from the tip of
  their arrow;
- are animated with keyframes on the timeline;
- follow a property (the canvas shows those of the current value or those that the inspector
  chooses for each value, and each emitter accepts its ends written out);
- change from a script.

They stay still while their strength is 0.

## New parameters of existing effects

| Effect | New parameters |
| --- | --- |
| Light rays | Directional or starburst. |
| Chromatic aberration | Four modes. |
| Grain | In black and white or with another blend mode. |
| Blur | Mixed with the image. |
| Iris and lens | Background visible. |
| Reflection | Tilted. |
| Displacement | Non-repeating. |
| Shake | With noise, friction and limits. |
| Glow | With blending, gradient and four styles (line, double line, ring or flash, chosen in the editor among drawings of each). |
| Sway | Double sway. |
| Swirl | Elliptical. |
| Ripples | With perspective, reflection, tint, walls and speed. |
| Waves | Double waves with perspective and a per-pixel offset (`phase`) that keeps neighboring areas from moving at once. |

## Performance

- Time-dependent effects stop asking for frames when their speed is zero.
- The trail and the fluids stop as soon as they settle.
- Fluids freeze at the lowest quality, and medium effects are omitted there, as when running on
  battery.

> **Note:** The blend modes `tint` and `diffuse_light` are approximations whose formulas may
> change.

## Feature names

They are named `effect.<id>` and `effect.<id>.<parameter>`, so earlier engines reject with the
newer-version notice the packages that use them.
