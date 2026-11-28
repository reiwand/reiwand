# Skeleton meshes

`mesh` layers deform an image with a skeleton. They are already drawn on the desktop and on the
phone. This page explains drawing, clips, inverse kinematics, bone physics, depth, attachments and
script control.

## Drawing

The GPU deforms the vertices with up to four bones each and the heaviest blend shapes. The
resulting image is composed like any other layer, with its effects, masks and blend modes.

Performance:

- A still skeleton or one that has finished does not ask for frames.
- The pose is computed only for visible layers and without reserving memory on each frame.
- At the lowest quality the mesh is drawn at half resolution and moves at 30 frames per second at
  most.
- The mesh texture is read with mipmaps and linear filtering when drawn smaller, so the edges of
  the character stay smooth, without steps.

## Clips and animation layers

Clips take the animations of the file, or keys of their own in the scene's JSON: rotation, position
and scale of each bone, weight of each shape and order of each part. They play looped, back and
forth or once.

Animation layers are mixed with weight, speed, fade in and out, additive layers and layers limited
to some bones.

Rules pass a bone to another parent without it jumping. Expressions group shapes, and parts are
ordered and clipped over one another.

Scripts control the clips with `scene.layer("fox").anim("wave")` (`play`, `pause`, `stop`, `seek`,
`rate`, `weight` and the `ended` notice) if they declare `animations`.

## Inverse kinematics

Inverse kinematics chains turn their bones until they reach:

- a point;
- a bone;
- another layer;
- the pointer (only with the pointer permission, which the app announces like any other use of the
  pointer).

How they solve:

- Exact for two bones, whose limits can bend them to the other side.
- In longer chains, by rounds from the pose of their clips (which keep their curve) that never bend
  a joint to the other side, or from an even arc if the pose is straight, bends to the other side or
  does not reach the target.

Each frame starts from its own pose, the same live, after jumping or when starting over. Chains have
limits per bone, weight and the direction in which all joints bend, which the editor inverts with a
tap on the chain.

## Bone physics

Bone physics simulates springs, pendulums and ropes with one step per drawn frame as long as the
interval between frames:

| Frame rate | Step |
| --- | --- |
| 120 fps | 1/120 s |
| 60 fps | 1/60 s |
| 30 fps | 1/30 s |

This holds also on battery, so it moves the same at those rates. An interval of more than 1/30 s is
split into equal steps of 1/30 s at most, and a frame catches up at most a quarter of a second.

The simulation has inertia, damping, gravity, rotation and stretch limits, and the wind of
`canvas.wind` with gusts (`canvas.gusts`, from 0 to 1). The wind is taken at the instant of each
step, the same at any frame rate.

- A body still for half a second stops asking for frames.
- At the lowest quality the bones stay in the animation pose.

## Depth

With `depth` the depth of the vertices is projected with the perspective of `canvas.perspective`.
This also applies under the tilt of the layer and that of the groups and parents that contain it,
always along the face of the layer as they turn it. Faces seen from behind are hidden if asked.

Drawing order:

1. First the opaque pixels of all pieces are drawn, which mark the depth.
2. Then the soft ones, which only consult it.

So the soft edges of a near piece show what is behind without a halo in any order, even if the
pieces cross. Among pieces at the same depth, the one that comes later covers the soft edges of the
previous.

In addition the pieces sort themselves from back to front by the average depth of their vertices in
the pose, blend shapes included, so that soft over soft blends in order. This can be turned off per
layer with `depth.sort: false`.

The mesh panel in the editor turns on depth and adjusts its perspective, the hiding of faces and
this order. `reiwand-ctl scene act FOLDER parts` does the same with `--depth-scale`, `--cull`,
`--sort` and `--flat`.

## Attachments

Any layer is attached to a bone or a node of a mesh with `attach`. It follows its position,
rotation and scale of the same frame. What it does not inherit from the bone it keeps taking from
the camera, which brings it closer and turns it like the rest of the scene.

## Bones in scripts

Scripts read and move bones with `scene.layer("fox").bone("arm")`: local or world rotation, position
and scale, for the next frame and before physics, so the bone returns with its spring when released.

| Function | Use |
| --- | --- |
| `shape` | Set blend shapes. |
| `impulse` | Push bones. |
| `hit` | Look for the nearest bone. |
| `attachment` | Read attachment points. |

The bones of `hit` receive the pointer, and the events say which bone was touched.
