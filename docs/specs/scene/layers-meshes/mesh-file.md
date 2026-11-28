# The mesh file

A `mesh` layer (`mesh`) draws a binary glTF of flat parts (positions with an optional depth, UVs and,
for bending, `JOINTS_0` and `WEIGHTS_0` with a skin) bent by its skeleton, such as a character cut
into limbs. A scene has at most 16 `mesh` and `model` layers. The editor cuts such a mesh out of a
picture of the scene, along a grid or its outline, and places its bones (see `editor.md`).

Positions are in pixels from the middle of the box the layer fills, x to the right and y up as glTF
writes them, and UVs start at the top left of the picture. Every node of the file is a joint: the
bones of skins, the nodes that carry a mesh without a skin, which then move as one piece, and
nodes without a mesh, such as attachment points (the editor marks the ones it places with
`extras.point`), which follow their parent and never count as the end of a bone. The rest pose is the transform of each node; vertices
follow at most 4 bones whose weights add up to 1 (within 0.02, then made exact). The engine skins
the vertices on the GPU into a picture of the box at the pixels it covers on the screen, with its transform, parents and camera, up to the pixels of
its picture (half of them at the lowest quality and three quarters at the next; a mesh drawn larger
than its box is skinned in steps of half an octave). When the skin has fewer than about three
quarters of the pixels of its texture on each side, the texture gets mipmaps and is read with
trilinear filtering, so the edges of a character stay smooth at the lowest quality instead of
showing steps, and draws that picture like
any other layer, with its transform, effects, masks and blend mode; it draws it again only when the
pose, the blend shapes, the order of the parts or a video texture changed, and a still or settled
skeleton asks for no frames. A hidden `mesh` layer that masks or feeds an effect of another layer is
posed and drawn every frame like a shown one, so a mask that is a mesh follows its clips. A picture mask of a `mesh` layer, its own `mask` or the `mask` of one of its effects, is read over the sheet, not over the box: the engine adds, when the scene loads, one hidden copy of the mesh for each such picture, with the picture as its texture and without tint, light, mask or effects, and points the mask at it with the channel it had (prefilled luminance), so a mask painted over the picture of the mesh stays on the same parts as they move. Each copy costs one more posed mesh of the size of the layer, drawn in the same pass as the mesh it copies, and counts in the cost like a layer that masks others, but not among the 16 `mesh` and `model` layers; each copy spawned from a template with such a mask brings its own hidden copies, and the bones a script drives on the original are driven alike on its copies. Parts that lie out of the box in the rest pose get a margin of skin around the box, up to half of the box on each side, so they are not cut and the mesh keeps its size (the box does not squeeze it); parts that a pose moves further out than the rest pose are cut at the edge of that margin, so `size` leaves them
room.

Files are refused that have a bone cycle or a node with two parents, a vertex whose weights do not
add up to 1 or that follows a bone of no skin, an index out of its vertices, a value that is not a
number, sparse values out of their buffer views or with indices that are not whole numbers or name no element (sparse accessors are read: their values replace those of the elements they name, over zeros when the accessor has no buffer view of its own), primitives other than triangles (strips and fans are read as triangles),
animation keys out of order, a mesh that names a blend shape twice, more than 1 024 nodes, 65 536
vertices, 1 048 576 triangle corners (three for each triangle, strips and fans included), 256 bones
and parts, 64 drawn primitives, 524 288 blend shape vertices or 4 194 304 animation values, or 128 MiB
of meshes for all the mesh layers of a scene; meshes are read whole, once, when the scene is, and a
file is refused as soon as it passes a limit.
