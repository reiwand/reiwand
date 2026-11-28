# The 3D camera and animated models

On this page: the camera, its paths and animated models.

## The 3D camera and its paths

`camera3d` at the root of the scene (`camera3d`) is the camera of models, 3D groups and tilted
layers: `fov` (1 to 179 degrees, may be bound), `near` and `far` planes, `eye`, `center` (both may be
bound) and `up`; `paths` (at most 16 `{ "id", "keys", "ease", "loop" }`, each with 2 to 256 keys `{
"t", "eye", "center", "up", "fov" }`, where `t`, `eye` and `center` are required, in increasing time) play one after another or at random
(`order`), joined by a `fade` `{ "to": "black" | "cross", "duration" (0 to 10 s) }`.
`camera3d` is written whole (`null` only when no layer is a model, a mesh or a 3D group: such a scene
has no 3D camera and the loader refuses a scene with 3D content and `null`); the editor suggests the camera
at `[0, 0, 5]`, looking at the origin with `up` `[0, 1, 0]`, a
`fov` of 60 degrees, `near` 0.1 and `far` 1000; depth is reversed (1 near, 0 far) in 32-bit floats.
Scripts that use `camera` read `scene.camera3d` and set `{ "fov", "eye", "center", "up" }` for the next
frame (`fov` 1 to 179, points within a million units); what they set wins over the paths.

Paths (`camera_paths`) fly the camera through their keys on a centripetal Catmull-Rom curve, which
passes through each key and never loops on keys close together; `up` and `fov` follow the same curve,
and keys without them keep those of the camera. An open path holds its first key until its first `t`
and lasts until its last key; a path with `loop` closes back on its first key one mean gap after its
last key, without a jump in place or heading, and `ease` shapes the time along one pass. One open path
plays once and holds its last key, asking for no more frames; one looping path loops for good; several
paths play one after another, each once (a looping one for one turn), `sequential` in their order or
`random` in a new order each round, and then start again. `fade` joins them: `black` darkens the last
half of `duration` of a path and brightens the first half of the next one (the whole picture of the
scene fades to black, flat layers included, as one black layer over everything that costs one
full-canvas draw at every quality level and only while it is not clear), and `cross` draws the last
`duration` of a path over the start of the next one, mixing both pictures of the models: the start of
the next path is rendered once and kept while its models, lights and camera stay the same, so a cross
fade costs one more render of the models when it begins and one mix a slice each frame. Moving paths ask
for frames, within `max_fps` and at most 30 a second at the lowest quality, and `scene bench` counts as
needed only the frames that pace asks for.

## Animated models

A model with any of `clips`, `animations`, `ik`, `physics`, `rules`, `shapes`, `expressions` or
`root_motion` (`model_animation`) is read with its skeleton: the joints of its skins and the nodes that
carry meshes become bones, and its vertices stay in the space of their skin (at most 4 bones a vertex,
weights adding up to 1 within 0.02) or of their node. Clips, animation layers (weight, rate, additive
layers, fades and bone masks), rules, blend shapes and expressions work as on meshes, mixing the
animations of the file; the keys of a clip may also give a bone `angles` (degrees around x, then y, then
z) and `shift` (in units of the model, along x, y and z) added to its rest pose. The palette is worked out
on the CPU at most once a frame and only for models that are shown, and the GPU skins each vertex with
it in the vertex shader; blend shapes of the file (positions, normals and, when the file gives them, tangents, so that relief
maps light a changed shape right) are mixed there too, the 8 heaviest for each primitive; a primitive
whose shapes carry tangents keeps three values a vertex for each shape instead of two. A model without
any of these fields bakes the weights its file sets for each mesh (or node) into its vertices once, when
it is read, and costs nothing more while it plays. A still skinned model draws its rest pose, as the file places it. At most
256 bones and parts, 1 024 nodes and 4 194 304 blend shape values a model.

`physics` swings bones in the plane of x and y of the model, counting one unit of the model as 100
pixels for `gravity`, `wind` and `translation`, and freezes at the lowest quality, where bones keep the
pose of their animation. Chains of `ik` on models take every target meshes take: `bone:` of the model, and a point `[x, y]` in
pixels of the box of the layer, `layer:` and `cursor`, which are seen through the camera and met on the
plane through the root of the chain that faces it, then reached in the plane of x and y of the model. With `root_motion`, the root
bone (the first bone, from the top of the skeleton, that an animation of the file moves) keeps the pose it
has when its clips start, and what the clips do to it moves the layer instead, along x, y and z and turning
where the clips turn it, adding up each turn of a repeating clip so that a walk walks on and a turning walk
walks a circle. Every playing animation layer moves the layer by its weight and fade, mixed as their poses
are: a layer replaces the motion of those before it by its weight and an additive one adds its own, and a
layer whose bone mask leaves the root out moves nothing. The work is a few samples and products of the
root bone per playing layer and frame, the same at every quality and tier, with nothing allocated.
A model that moves in its stage renders the stage again only on the frames where its pose changed.
