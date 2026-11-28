# Shadows, lights and 3D groups

On this page: shadows, lights, flat layers in 3D groups and when models render again.

## Shadows and lights

Shadows are maps the models are drawn into from the light: 2 048 pixels a side for a directional light,
fitted around the models of the stage, 1 024 for a spot light within its cone and a cube of six faces of 512 for a
point light (read as one cube map, so its edges filter across faces without seams), all at most 4 096 (1 024 on phones, 2 048 on T2) and half of that at the lowest quality;
`shadow_resolution` picks another size within those caps. Each pixel of a material that takes shadows
reads its map four times around its place and blends them (percentage closer filtering), lifted along its
normal by `shadow_bias`. Desktop computers (T3) draw shadows of the first 2 lights that cast them, T2 of
the first one, and T0 and T1 of none. A map is drawn again only when its light, a caster, a pose or a
swaying material changes, so a moving camera over still models reuses it. Primitives drawn with `blend`
or `add` and materials with `cast_shadows: false` cast none; cut-out (`mask`) primitives cast a
shadow only where they are drawn, since the map pass reads the alpha of their color map, a cost
only they pay. Fur casts the shadow of its length at Q2 and Q3: each shadow map draws its skin and
one more shell at six tenths of the length, cut to its `density` (as many triangles again as the
skin for each face of the map, only when the map is drawn again); at Q0 and Q1 it casts its skin only.

Light layers light the models as they light flat layers: each light without `position3d` stands on
the plane through the point the camera looks at, under the point of the box where it is drawn, moved
toward the viewer by its `z`, with its radius and length in pixels of that plane; a light with
`position3d` stands at that point among the models whatever the camera does, so it can stand behind or
beside a model, and with `aim3d` a spot or a directional light shines from it toward that point (its
radius and length are still pixels of the plane through the point the camera looks at). Both cost
nothing more at any quality level: the place of each light is worked out once a frame on the CPU.
Directional lights shine along their heading, and the light around and from above (`canvas.ambient` and `canvas.sky`) comes from below and
from the up of the camera. At most 12 lights count (8 on T0 and T1). A scene without lights, light
around or sky and without `env` lights its models with a soft light from above the camera on the left,
so that a model never shows black.

## Flat layers in 3D groups

A flat layer inside a 3D group is drawn in front of the models in its order unless it sets
`depth_test`: it then stands on the plane through the point the camera looks at, moved toward the
viewer by its `transform.z` in pixels of that plane, so that models in front of the plane cover it.
The models are drawn in slices between such planes, at most 4 different ones (more layers share the
nearest), and the layers between their slices. Particle layers count as flat layers there: with
`depth_test` a system without `perspective` stands on its plane. A system with `perspective` takes
no plane: each particle stands at the depth of its own place, the plane of the layer times
`(f + z) / f`, and hides pixel by pixel behind the models in front of it. For that the stage also
draws the depth of its opaque models once into a picture of its own size (one more pass of their
triangles, without colors, only while such a layer is in the group, and kept while nothing moves),
which the particles read; lit particles (`render.lit` or a layer with `lighting`) , particles with a shader of their own,
refracting particles, trails (short and long, each point at the depth its particle had when it
was marked) and ropes (each knot at the depth of its particle) read it too. Refracting particles hide their bending where a nearer model
covers them, but the picture they bend is the finished picture of the group, which holds no depth:
where a visible particle sits beside a nearer model its offsets can still pull in a few pixels of that
model. Child systems, at any depth of the family, are drawn with their layer and tested the same
way. Systems without `perspective` stand on the plane of the layer as before, and so does every system at the lowest quality, which skips that pass. A plain group inside a 3D group (no
effects, mask, other blend, opacity below 1, color, brightness, `clip`, tilt, `z` or `depth_test`)
lets its layers join the slices one by one, and a plain 3D group inside another 3D group, directly or
through plain groups, shares its depth: its models and its flat layers are drawn with those of the
outer group, with the camera and the box of the outer group. A 3D group inside another that is not
plain still draws its own picture, and the models of the outer one then draw alone.

## When models render again

A model whose values do not move renders once and its picture is kept; a 3D group renders again only
when a model, the camera, a light, the light around or the size changes, so that a still scene asks
for no frames. The work of a model counts the triangles it draws, fur shells included, toward the
cost of the scene, and again for each shadow map they are drawn into (the first two lights with
`shadows`, six maps for a point light). How many triangles make a unit depends on the device: the
tier of discrete GPUs on a computer (T3) counts one unit for every 200 000, the ratio measured on a
discrete graphics card, where 60 000 triangles took 0.29 units (a full-screen pass of 1920×1080 at
0.016 ms) and a directional shadow map about as much as the triangles themselves. Phones, whatever
their tier, and the lower tiers that phones reach (T0, T1 and T2, also on computers) keep the more
prudent one unit for every 60 000 until the same measurement exists for a phone. The editor, the
validator and `pack check` warn with the prudent unit; the daemon, the phone and `reiwand-ctl scene bench`
choose the quality with the unit of their device. The test `the_3d_stage_costs_on_this_gpu` of
`crates/reiwand-render/tests/models` (run with `--ignored`) prints those numbers for any graphics card,
and `scripts/phone-bench.sh costs` builds it for a phone and runs it there over adb. Device tiers take at most 50 000 triangles (T0), 150 000
(T1), 500 000 (T2) and 2 000 000 (T3), skins of at most 256 bones and shadows of 0, 0, 1 and 2 lights;
the editor and the validator tell which tiers a scene fits. A tier shows the poster of a scene whose
models draw more triangles than it takes at the lowest quality, where fur keeps a quarter of its
shells; other heavy scenes run there at a lower quality, or as their poster when even the lowest
costs too much. A model inside a 3D group with an `opacity` below 1 is drawn see-through, blended in
linear light after the opaque models, farthest first: its depth is drawn first, so only its nearest
surface shows and its own hidden faces and parts stay hidden, at the cost of drawing its triangles
twice; the lowest quality skips that depth and lets them show through. A lone model keeps its depth and
is faded as a whole.
