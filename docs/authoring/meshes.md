# Meshes, bones and animation

A `mesh` layer draws an image bent by a skeleton, like a character cut into pieces that moves its
arms. 3D models (`model`) use the same animation. These pages explain the mesh, the bones and
everything that moves them: clips, animation layers, inverse kinematics, physics, rules, blend
shapes and attachment points.

The example is [`examples/mesh-with-bones`](examples/mesh-with-bones/scene.json), which passes
`reiwand-ctl pack check`. For 3D models and lights see [3d.md](3d.md).

Meshes, by topic:

- [The mesh layer and its parts](meshes/layer-and-parts.md): the mesh layer and its parts.
- [Clips, bones and expressions](meshes/motion.md): clips, animation layers, reaching targets, physics, rules and blend shapes.
- [Attached layers and models](meshes/attaching.md): layers on a bone, touching a bone and animated models.

## Not covered here

- Scripts that push bones (`impulse`) and those that set the pose: [scripts.md](scripts.md).
- How each editor tool looks (skeleton, weights, depth painting): [editor.md](editor.md) and
  [actions.md](actions.md).
