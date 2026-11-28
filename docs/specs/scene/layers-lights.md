# Lights

A `light` layer lights the layers that have `lighting` (`lights`); it draws nothing, so it takes only
`id`, `type`, `transform` (its position, its rotation for the direction of spot, tube and directional
lights, and `tilt`), `anchor`, `visible`, `parent`, `inherit` and these fields. It follows parents,
groups and bones (`attach`) and animates with keyframes like other layers. A scene has at most 12
lights; on T0 and T1 devices (phones and older computers) only the first 8 in the order of the layers
light anything, and the others are named in the notes of what was left out (`the layer … was left
out`), as the effects are.

| Field | Meaning |
|-------|---------|
| `light` | `point` (prefilled), `spot`, `tube` or `directional` |
| `color`, `intensity` | the color (prefilled white) and 0 to 100 (prefilled 1); both may be bound |
| `radius` | 1 to 100000 px (prefilled 512), where the light ends; may be bound |
| `z` | height above the canvas, −100000 to 100000 px (prefilled 100); may be bound |
| `cone` | spot lights only: `[inner, outer]` angles from 0 to 180 degrees (prefilled `[30, 60]`), full in the middle and dark past the outer angle |
| `length` | tube lights only: 1 to 100000 px (prefilled 256) |
| `volumetric` | point and spot lights only: `{ "density" (0 to 1, prefilled 0.2), "samples" (4 to 64, prefilled 16), "shafts" (0 to 1, prefilled 0) }` lights the air (`volumetric_lights`), see below |
| `texture` | spot lights only: the asset id of a picture or video, or `{ "layer": id }`, that the light projects (`light_textures`); at most 4 spot lights of a scene project, see below |
| `shadows` | point, spot and directional lights: `true` casts shadows of models on models whose materials take them; at most 2 lights per scene (`shadows`), see "Models and 3D" |
| `shadow_resolution` | with `shadows`: 256 to 4096 pixels a side of its map (for point lights, of each of its six faces) instead of the size picked for its kind |
| `shadow_bias` | with `shadows`: 0 to 1 (prefilled 0.2), how far surfaces lift away from their own shadow; raise it against stripes, lower it when shadows leave their casters |
| `position3d` | `[x, y, z]` in the units of the models (may be keyed or bound): where the light stands among the models, in the space of every 3D group and lone model; without it the light stands on the plane of the camera (see "Models"). Flat layers keep using the place of the light layer |
| `aim3d` | `[x, y, z]` (may be keyed or bound): the point a spot or directional light shines at, or the direction of a tube, among the models; without it the heading of the layer, turned with the camera, gives the direction |

A spot or directional light with no rotation points down the canvas and turns clockwise with
`rotation`; the first number of `transform.tilt` (−89 to 89 degrees) dips it into the canvas, so
`[90, 0]` would point straight at it. A tube light lies along the x axis of its layer, `length`
long, and lights like a row of point lights. The light of a point, spot or tube light fades as
(1 − d²/r²)² with the distance d in three dimensions, `z` included, and ends at the radius r; a
directional light reaches everywhere with the same strength.

Each pixel of a lit layer adds, for every light, the color of the light times its intensity, its
fade, its cone and the cosine between the normal of the pixel and the direction to the light, and a
highlight that is sharp on smooth surfaces and spread out on rough ones; the ambient light comes on
top, the color of `canvas.ambient` on surfaces facing down and of `canvas.sky` (the ambient one when
unset) on surfaces facing up, and black when neither is set. A layer without a normal map faces the
viewer, and so takes the mix of both halfway. A layer turned with `tilt` or lifted with `z` is lit in its own plane: its normal, and the normal map laid over it, turn with the plane, and each pixel takes the distance to the light from where the plane holds it in depth (as if seen straight on, without the narrowing of the perspective), so a wall turned away from a lamp darkens and its near edge takes more light than its far one; a mesh layer is lit like a picture of its posed parts, in the plane of its box. `lighting` takes:

| Field | Meaning |
|-------|---------|
| `normal` | a `linear` normal map: red to the right, green up and blue towards the viewer, from 0 to 1 for −1 to 1, laid over the layer as its picture is and turned with the layer |
| `roughness`, `roughness_map` | 0 to 1 (prefilled 0.5), times the red of a `linear` gray picture |
| `reflection`, `env` | 0 to 1 (prefilled 0): how much of `env`, a picture read by the normal as a sphere, the layer reflects; without `env` it reflects what is drawn below it, moved by its normals |
| `emissive`, `emissive_map` | light of its own from 0 to 16 (prefilled 0), times the red of a `linear` gray picture; it shows even in the dark |
| `strength` | 0 to 1 (prefilled 1): how much of the lit color replaces the plain one |

The numbers may be bound. Lighting is worked out in the same pass that draws the layer, for at most
12 lights and only where a light reaches. While the lights stay still, lit layers are drawn once and
kept like other still layers; when one moves, only the box its radius covers is drawn again. At the
lowest quality level the lighting is worked out once, when the scene starts, the level or the size
of the screen changes or the scene is loaded again, and lit layers are kept as still pictures: lights that move stay where they started and ask for no
frames. Scripts read and set `scene.ambient` and `scene.sky` as colors, like `scene.background`.

A `volumetric` light glows in the air within its radius, added where the light layer is among the
layers: the layers below it show through the glow and the layers above it cover it. Inside a group
drawn apart (with effects, a mask, an opacity under 1 or a cache) the glow goes between the children
of the group, where the light is, and the group is then drawn again each frame instead of kept; the
picture of such a group, and of a tilted group, takes in the whole glow of its lights, and a tilted
group leans the glow with its layers; when a script reorders the layers the glow follows the new order. Each pixel
gathers, at `samples` points on the line from the pixel towards the light, the fade, the cone and
the projected picture of the light there; `density` is the part of the line it walks and also how
strongly the air glows, and the glow fades to nothing at the radius. With `shafts`, each of those
points also takes how bright the layers below the light are there, so their dark parts hold back the
glow and cast shafts: 0 leaves the glow as it is and 1 lets only bright parts glow. Dark and bright
are measured against the brightest of five points around the light, so a background that is evenly
dark still glows and only what is darker than around the light holds it back. The shafts read a copy
of the layers below made at the size of the air, not of the screen. The air is worked
out at half the size of the screen (Q3 and Q2) or a quarter (Q1, and on T0 and T1 devices), with
half the samples at Q1, and not at all at Q0; it is kept while the light and, with `shafts`, the
layers below it stay still, and drawn again only in the box of its radius when one of them moves.

A spot light with a `texture` multiplies its color by the picture, which fills the outer cone as a
projector does, upright and seen from behind the light. A video plays on and asks for frames while it
plays; a `{ "layer": id }` projects that layer as it is drawn, with its effects, even when it is
hidden (`layer_textures`). The projected layer must not take light itself, hold or read a layer that
does, or the scene is refused as a loop. Lit layers and the glowing air take the projected picture,
while lit particles take only the color of the spot; layers lit by a still picture stay kept, a video or a moving layer redraws them in
the box of the light.
