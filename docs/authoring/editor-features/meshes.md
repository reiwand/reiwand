# Meshes, clips and chains

On this page: meshes from pictures, bones, clips, chains, physics and the pointer.

## Meshes and bones

It also cuts out a mesh from a scene image (“Mesh from a picture” in the add-layer menu), with a
grid of cells or following the outline of the opaque part, and gives it bones, just like an
imported multi-part `.glb`. The inspector of the `mesh` field shows the bones as a tree: a click
chooses a bone (also on the preview), a double click renames it (and the scene follows it in clips,
chains, physics, rules, pointer and attached layers), and a bone dropped onto another hangs from
it; when removing a bone or hanging it from another, in the same step the keys, chains, physics
bodies, rules, `hit` bones and attached layers that named it or that stopped being chained are
removed, so the scene stays valid. On the preview the bones are dragged, the vertices' depth is
painted, which brings them nearer to or farther from the viewer, and the chosen bone's weights are
painted with a brush that adds, subtracts, smooths or normalizes, with strength, size, soft edge
and locked bones; the mesh is colored by that bone's weight or by the bone that rules each part,
and each vertex ends up with four bones at most and weights that add up to 1. While they are not
painted, the weights follow the nearest bones; once painted they are kept when the bones change,
and *Auto weights* computes them again. Each change saves a new mesh in `meshes/`, so that undoing
returns to the previous one; when saving, the earlier meshes wait outside the folder, next to the
editor's history, and come back when a step names them, so the package carries only the meshes in
use.

## Clips of a mesh

*Edit clips on the timeline* swaps the timelines below the preview for the layer's clips: a clip is
chosen, created, renamed or removed, its duration and its loop are set, and the playhead walks
through, plays and repeats the preview with that clip, in seconds from the start of the clip (its
`from`), where the keys are also written. There are rows per channel of each bone, for the order of
the parts and for each blend shape, with a slider that stores its weight; keys are dragged (with
Ctrl they are copied), deleted and given step, line or smooth curve. On the preview the chosen bone
has handles to move, rotate and scale it at that instant, and *Key* saves into the clip's keys of
the scene JSON, the same ones the engine plays, only the channels the handles changed, or the
bone's whole pose (with the file's motion) if none changed. Blend shapes have sliders that only
change the preview, and with them expressions are created, changed, renamed and removed.

## Chains, physics and the pointer

The same inspector has tools over the preview for the rest of the character. *Create a chain on the
preview* asks for a click on the bone where it starts and another on the bone where it ends; each
chain chooses what it reaches (the pointer, a point, a bone or a mesh point, or another layer),
bends toward the other side, has a weight and limits per bone. The point target is a ring that is
dragged and each limit is an arc around the bone with two points that set its minimum and maximum
angle. Swinging bones are chosen with a click on the preview, with their mode and sliders for
stiffness, damping, inertia, gravity (an arrow whose tip is dragged) and wind, and checkboxes that
bound their rotation (an arc with two points that are dragged) and how much their head slides;
*Draw a rope* creates, along the stroke, up to 9 new chained bones that hang from the chosen bone
and swing like a rope, and *Simulate* and *Restart* run the physics on the preview or return it to
the rest pose. The wind of `canvas.wind` is an arrow dragged from the center of the canvas, with its
angle, its strength and its gusts (`canvas.gusts`). To attach layers, a layer is dragged onto a
bone, or onto the mesh to create a new point there (a node without a mesh from the file, hung from
the bone that rules at that place), choosing whether it follows position, rotation and scale, and a
line joins each attached layer with its bone or point and ends in a ring that is dragged to move it
along it. The mesh points are listed with their name, which is edited right there, the bone they
hang from and a button to remove them, and on the preview they are squares that are dragged; a point
created for a single layer goes away with it when the layer stops following it. Points never count
as the end of a bone, so they do not change the chains or the physics. The parts are ordered by
dragging them in a list from back to front, hidden and clipped to be seen only over another part,
whose outline is marked on the preview. The outlines of the parts, the bones and points and the
handles of chains and physics are seen on any mesh the editor shows; a mesh that several nodes draw
gets, on the first edit, a mesh of its own for each one (`<mesh>_<node>`), so their bones and
weights are also edited here, and each primitive is matched to its part by the node and the
primitive it comes from. In the general inspector, every field that names a bone, a point, a part or
a blend shape (chains, targets and limits, physics, rules, bone masks of the animation layers, `hit`,
`attach` and `parts`) offers those of the mesh in a list.

From the terminal the same is done by `reiwand-ctl scene act DIR mesh_from_picture`, `bone` (with
`--rename`, `--root` and `--remove`), `mesh-depth`, `weights` (weights brush with `--mode`, `--by`,
`--radius`, `--falloff` and `--lock`), `auto-weights`, `clip` (with `--add`, `--remove`, `--rename`,
`--length` and `--loop`), `key` (keys of bones, shapes and order, with `--ease`, `--remove`, `--from`
and `--copy`), `expression`, `ik` (`add ROOT TIP`, `set N` and `remove N`, with `--target`,
`--limit BONE MIN MAX`, `--bend` and `--weight`), `physics` (`add BONE...` with `--mode`,
`--stiffness`, `--damping`, `--inertia`, `--gravity`, `--wind`, `--rotation`, `--translation` and
`--free`; `rope NAME X,Y;X,Y;...` with `--parent`; `remove BONE...`), `wind` (`--angle`,
`--strength`, `--noise` and `--off`), `attach` (`--to`, `--bone`, `--point X,Y` or a node,
`--offset`, `--nudge`, `--inherit` and `--detach`), `point` (attachment points with `--at`,
`--parent`, `--root`, `--rename` and `--remove`) and `parts` (`--order`, `--hide`, `--show`,
`--clip`, `--unclip`, `--depth-scale`, `--cull`, `--sort` and `--flat`); `bones`, `part-outlines`,
`vertex-weights` and `keys` print the bones and points, the outline of each part, the weights of
each vertex and the keys of a clip.
