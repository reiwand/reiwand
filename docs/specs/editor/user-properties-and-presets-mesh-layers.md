# Mesh layers

The tools for mesh layers edit the file of the layer as a whole, whatever its number of parts, as
long as the scene draws it (at most 64 primitives, as for any mesh layer); a mesh drawn by several
nodes gets one of its own for each of them on the first edit. Bones are
the joints of its skins and the nodes without a mesh, except attachment points (`extras.point`);
the parts of a mesh without a skin are bound to the bones as soon as there is one. Each edit writes
one skin for every part, with the vertices
at their rest place in the layer and each bone bound where it rests, keeps the other contents of the
file (pictures, materials, blend shapes, animations, names) and drops what nothing uses any more.
The skin records in `extras.weights` whether its weights are `auto` (they follow the bones: adding,
moving or removing a bone binds every vertex again) or `painted` (they stay; a new bone starts with
no weight and a removed one leaves its weights to its parent). A file whose skin says nothing keeps
its weights, as painted. When the folder is saved, the files of earlier edits that the history
still names wait in the state folder of the editor (next to the saved history, outside the scene
folder) and come back when undo or redo names them again, and those that nothing names any more
are deleted, so the folder and the package keep only the meshes in use; a new file never takes the
name of one that waits.

The inspector of the `mesh` field shows the bones as a tree: a click picks a bone, also on the
preview, a double click or the pencil renames it, and a bone dropped on another hangs from it, or
from none when dropped below the list. *Place bones on the preview* drags their heads. *Paint
weights on the preview* paints the picked bone with the brush, its strength, size and soft edge,
keeps the bones that are ticked as locked, and colours the triangles of the mesh by the weight of
the picked bone (from blue for none to red for all of it, blended between the vertices) or by the bone
that leads each triangle, with its colour, drawn as one picture of at most 384 pixels a side that
is made again only after an edit; *Auto weights* runs `auto_weights`. The inspector also picks the clip the scene plays (`animations`)
and `hit` bones and `depth` with its scale and back-face hiding, as `set` on those fields, which
`reiwand-ctl scene act DIR set` does as well. *Pick them on the preview* under the bones that take the pointer
makes a click on a bone of the preview add it to `hit` or take it out, and the bones in `hit` are
drawn with a thick ring; while a layer has `depth` and its tools are shown, a knob on a line at the
bottom of the preview drags the scale (0 to 10) with the value over it, and the preview follows the
drag as one undoable step (the same change as the slider of the inspector, `parts` with `depth`). The blend shapes have sliders that only change the preview, and
an expression is saved from them, shown to be changed and saved again, renamed or removed; each expression also has a slider of its own, 0 to 1, that mixes the expressions in the preview. *Sculpt a blend shape on the preview* picks an existing shape or names a new one and brushes the mesh (push away, pull in, smooth or erase, with its strength, size and soft edge) with `shape_sculpt`; the preview shows the shape at full weight while it is sculpted and one stroke is one undoable step.

*Make a chain on the preview* takes a click on the bone a chain starts from and one on the bone it
ends at, and adds it with `ik`. Each chain is listed with what it reaches for (the pointer, with a
note that it needs the pointer permission, a point, a bone or point of the mesh, or another layer),
a button that bends it the other way, its weight and a button that limits or frees each of its
bones. On the preview a dashed line joins the end of each chain to its target, a chain that reaches
for a point has a ring that is dragged to move it, and each limit is an arc around its bone from the
smallest to the largest angle, with a dot at each end that is dragged to change it. *Pick bones
that swing on the preview* adds or removes the body of the bone clicked, and each body is listed
with its mode, sliders for its stiffness, damping, inertia, gravity and wind, and ticks that bound
its turn and how far its head slides, each with a slider from its smallest to its largest value
(`free` when unticked); on the preview each body has a ring at its head, an arrow for its gravity
with a square at its tip that is dragged up or down to set it, and a bounded turn is an arc from
its smallest to its largest angle with a dot at each end that is dragged to change it. *Draw a rope on the preview* turns a drag
into a `physics` rope of up to 9 new bones that hangs from the picked bone, or the nearest one.
*Simulate* plays the preview with the physics, or holds it without (the preview drops the
`physics` of the layer, only there), and *Reset* sends the layer once without its bodies so that
they start again from the pose. The rules that hand a bone over to another parent are listed with
the bone it moves, a picker for its new parent and a weight slider, a button that removes one, and
pickers for the bone and parent of a new one, all with `bone_rule`. The wind has sliders for its angle, strength and gusts and *Show
on the preview* draws it as an arrow from the middle of the canvas whose tip is dragged; all of
them run `wind`. *Attach layers on the preview* lists the other layers as chips: one dragged onto a
bone of the mesh follows it, one dropped elsewhere on the mesh follows a new point there, and a
picked chip does the same with a click; what the layer inherits is ticked below, the layers that
follow the mesh are listed with a button that lets them go, and a dotted line joins each of them to
its bone or point, ending in a ring that is dragged to move the layer along it (`nudge`). The
attachment points of the mesh are listed with their name, which is edited in place, the bone they
hang from and a button that removes them; *Add a point* places one in the middle, and on the
preview each point is a square that is dragged to move it, all with `point`. The parts of a mesh of several parts are listed from back to front: a part is
dragged by its handle to a new place, which writes the `order` of every part with `parts`, ticked
to be shown, and given the part it only shows over; the picked part and the part it is cut to are
outlined on the preview.

In the inspector every field that names a bone, a point, a part or a blend shape offers those of
its mesh layer: a single bone (`physics`, `rules`, the `attach` of a layer, which names the bones
and points of the layer it follows) or part (`clip_to`) is a list to pick from, a list of bones
(`chain`, the `bones` of animation layers, `hit`) a set of ticks in the order of the skeleton, a
map keyed by bone or part (`limits`, the `bones` of clips, `parts`, `shapes`) a list to add a key
from with its fields below, and the `target` of a chain a list of the pointer, the bones and
points, the other layers and a point. The schema marks these fields with `x-ref` set to `bone`,
`point`, `part`, `shape` or `reach`; the names come from the file of the mesh, read again only when
it changes.

*Edit clips on the timeline* swaps the timelines under the preview for the clips of the layer: a
clip is picked, added, renamed or removed, and given its length and loop; the playhead scrubs,
plays and loops the preview, which plays that clip alone with the sliders of the blend shapes. The
rows are the channels of the bones that have keys and of the picked bone, the order of the parts
that have keys and every blend shape with a slider that keys its weight at the playhead; a key is
picked, dragged to move it (with Ctrl to copy it), given any easing of the scene format, bent with the same two Bézier handles as the keyframes of a timeline (the curve button opens the plot of the curve to the next key of its row; the same applies to the rows of a 3D model) or removed. On the preview the
picked bone has handles to move, turn and scale it at the playhead, in the space of its parent as
the clip keys it, and *Key* writes, with `key`, the channels the handles changed, or the whole pose
of the bone at the playhead when none did. The timeline shows the seconds from the start of the
clip, its `from`, and the keys are written where the playhead is. The lane of events above the rows marks each event of the clip: a double click on the lane adds one at that second, a mark is dragged to move it, a click on it seeks to it and opens its name, to rename it or remove it (all with `clip` and its `event`). Only the preview sees
the clip and the sliders chosen here; the scene keeps its own `animations` and `shapes`.
