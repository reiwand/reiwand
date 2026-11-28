# The model layer and materials

On this page: the model layer, its materials and shaders.

## The model layer

A `model` layer (`model3d`) draws a binary glTF model with metallic-roughness materials. Alone it
renders into its own box (prefilled `size`, the canvas) with depth, and then composes like any
layer, with its effects, masks, blending and `transform`; a `group` with `space: "3d"` renders all its
models in one pass with a shared depth into a box the size of the canvas, placed where a child of the
group without a `transform` would be, which then composes as one layer in the place of the group. Models
also take `clips`, `animations`, `ik`, `physics`, `rules`, `shapes`, `expressions` and `hit` as meshes
do (`model_animation`, see "Animated models" below), and:

| Field | Meaning |
|-------|---------|
| `model` | the `.glb` asset |
| `position`, `rotation`, `scale` | three numbers each (or one for all three), in the units of the model and degrees (turned around x, then y, then z), which may be bound |
| `root_motion` | `true` lets the animation of the root move the layer; needs `clips` |
| `env` | a picture reflected by the materials, read as an equirectangular panorama: blurred for the light around and sharper for shiny materials; the lowest quality keeps only the light around |
| `materials` | per material name of the file, at most 64 of `{ "shader", "color", "emissive", "lit", "double_sided", "cast_shadows", "receive_shadows", "cull": "back" \| "front" \| "none", "depth_test", "depth_write", "blend": "opaque" \| "mask" \| "blend" \| "add", "maps", "params" }` |

## Materials and shaders

The materials of the file give the color, metal, roughness, normal, occlusion and glow maps with their
factors (`KHR_materials_emissive_strength` multiplies the glow), the alpha mode and cutoff, and both
sides; `KHR_materials_unlit` draws a material without light. A material of the scene multiplies its
color by `color`, replaces the glow by `emissive`, picks `shader` and overrides the rest; `lit: false`
draws it unlit, `blend` or an `opacity` below 1 draws it after the opaque ones, farthest first.
Primitives without a material take a white one. Textures may be PNG, JPEG or WebP pictures of at most
4096 pixels; phones and the T0 and T1 tiers draw them at most 2048 pixels wide.

`shader` is always written: `pbr` (the editor suggests it; `params` `metallic`, `roughness`, `occlusion` 0 to 1 and `normal_scale`
0 to 10), `unlit`, `fur` (`layers` 1 to 32, `length` 0 to 100, `density` 0 to 1), `foliage`
(`strength` and `speed` 0 to 10, swaying with `canvas.wind`), `chroma` (`key` color, `threshold` and
`softness` 0 to 1) or the id of a package effect with `stage: "material"` (`material_shaders` for all
but `pbr` and `unlit`); `fur` also takes `comb_x`, `comb_y` and `comb_z` from −1 to 1, and all but `unlit`
take the numbers of `pbr`. Every parameter of `fur`, `foliage` and `chroma` listed here is written (the
editor suggests `layers` 12, `length` 10, `density` 0.5, `comb_x`, `comb_y` and `comb_z` 0, `strength` and
`speed` 1, `key` `#00ff00`, `threshold` 0.4 and `softness` 0.1); the numbers of `pbr` may stay out to keep
those of the model file. Shadows of lights and materials name `shadows`; both `cast_shadows` and
`receive_shadows` are `true` unless set, which changes nothing until a light casts shadows.
`maps` replaces maps of the file with pictures of the scene: `{ "base", "orm", "normal",
"occlusion", "emissive" }` (and gives fur its `comb` map), each the id of a picture asset; a replaced map keeps the texture
coordinates, the transform and the sampler of the slot it replaces (the first set unmoved,
repeated and smooth where the file had no map) and should be `linear` for
`orm`, `normal` and `occlusion`. A picture that is not loaded leaves the map of the file.

- **`fur`** draws its primitives again as shells pushed out along the normal, `layers` of them (12 by
  default) at Q3, three quarters at Q2, half at Q1 and a quarter at Q0 (at least one); `length` is how
  far the outer shell stands, in hundredths of the radius of the model (prefilled 10), `density` how
  close the strands grow (0.5), and the comb bends them towards that direction of the model, more at the
  tips. `maps.comb` names a `linear` picture over the first texture coordinates that combs each place
  on its own: its red, green and blue are the direction in the space of the model (0.5 is none, 0 and 1
  are −1 and 1) and its alpha how much it replaces the comb of the material; the vertex stage of the shells
  reads one texel of it per vertex (of a 1-pixel picture without a comb map), at every quality level and
  tier. Strands thin out towards the
  tips and shade darker at their roots; the shells are cut, not blended, so they need no sorting.
- **`foliage`** sways the vertices with `canvas.wind`: the red of the vertex colors (`COLOR_0`) of the
  file says how much each vertex moves (black stays), or, without vertex colors, the height within the
  model. `strength` (prefilled 1) scales the sway, which reaches a twentieth of the radius of the model
  for a wind of 100 pixels a second, and `speed` (1) its pace. Without wind, at the lowest quality or
  when the wind stays still at nothing, foliage stays still and asks for no frames.
- **`chroma`** makes the colors close to `key` (prefilled green, an sRGB color like the others,
  compared with the color of the surface in linear light) transparent: closer than `threshold`
  (0.4) is cut, and the cut fades over `softness` (0.1) beyond it.
- **A material shader of the package** (`stage: "material"`, checked and limited like every package
  shader) gives `fn material(s: Surface) -> Surface`, which runs for each pixel after the maps are read
  and before the light, and may change the place, normal, color, glow, roughness, metal and occlusion.
  It may read `frame`, `param(0u)` to `param(7u)` (its parameters in their order, set by `params`), the
  noise and color helpers, but no pictures. A shader that cannot be built is drawn as `pbr`, and the
  stage renders again each frame while it reads time, audio or the pointer.
