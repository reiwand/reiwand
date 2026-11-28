# Models and 3D

The Add layer menu offers 3D model (once the scene has a `.glb` asset), 3D group and 3D model from a
file, a dialog that reads a binary glTF file before it is imported and tells its triangles,
materials, textures, bones and clips, and for each device tier whether it fits, the largest texture
it gets (2048 px on phones and on T0 and T1) and the texture memory, with a note on converting FBX and
OBJ with Blender; accepting imports the file and adds a model layer, inside the selected group.

A scene with models draws over the preview an authoring view with its own camera, kept in
`ext.editor.view3d`, which only the preview reads: dragging with the right button turns it around its
target, with the middle button or Shift slides it and the wheel brings it closer, and the preview
renders through it while the scene camera shows as a dashed frustum with a handle on its eye, a square
on the point it looks at, both moved in the plane of the view, and a line toward its up. Each model
shows its box, which selects it with a click; the selected model shows three arrows along x, y and z
that move it along that direction (the point of the axis closest to the pointer, so the model stays
under it whatever the angle of the view), three rings around x, y and z with a knob each that turns it
around that axis of the scene, a Depth square that moves it along the line of sight (up pushes it
away, down brings it closer, the model staying under the pointer), a knob above it that turns it
(sideways around y, up and down around x) and a square on its box corner that scales it. These handles
start from the values the model has at the current time and write `position`, `rotation` and `scale`:
a keyed value gets a key at the current time, a value bound to a property or a script keeps its
binding and its handle is dimmed (a drag with Shift replaces the binding with the value), and
`scene3d` `model` `set` refuses to write over keys or a binding. The arrows, rings, Depth and the
handles of the camera, of camera path keys and of lights all send `scene3d` `grab` (`what`: `model`
with `layer`, `eye`, `center`, `key` with `path`, `key` (its time) and `aim`, or `light` with `layer` and
`aim`; `handle`: `x`, `y`, `z`, `turn_x`, `turn_y`, `turn_z`, `plane`, `depth`, `orbit`, `dolly` or `roll`;
`from` and `to` in pixels of the canvas as `stage3d` reports them; `time`, the playhead, where a keyed
value gets its key; `replace`, which writes over a key or a binding). The editor works out the move
with the camera of the renderer: the ray through each pixel, the axis or plane it meets and the
angle around the axis, so the result lands where the renderer then draws it.

The inspector of a model layer shows the counts and the tier verdicts of its file, a Reflects picker of
the pictures of the scene (`env`) with a thumbnail, Look at it in the authoring view, and a Materials
part with a row per material of the file: a ball shaded with the material and the light around and from
above of the scene (as in the light tools), the shader picker (physical, unlit or a package effect),
lit, both sides, casts and takes shadows, tint and glow colors, sliders for metal, roughness, relief and
occlusion, and the thumbnails of the maps of the file (color, metal and roughness, normals, occlusion,
glow). The inspector of a group turns it 3D, and the inspector of a flat layer inside a 3D group
toggles Hide behind nearer models (`depth_test`); on a particles layer a note under it says, from the
`depth3d` query, whether each particle hides on its own (and that the lowest quality hides the whole
layer as one plane) or why the whole layer hides as one plane: its system has no perspective; for refracting particles it adds that their bending hides
too but can still pull in a few pixels of a nearer model next to it (`caveat: refract_edges`).

Selecting the canvas also shows the 3D camera: sliders for the field of view and the roll that change
the preview while they move, where it looks from and toward, Set the camera from the view, Look
through the camera and Back to the camera view. In the authoring view the scene camera shows a knob on
its eye and a square on the point it looks at, which move in the plane of the view or, with Ctrl,
along the line of sight; a ring around its line of sight with a Roll knob that turns its `up`; an Orbit
knob that swings the eye around the point it looks at, keeping the distance; and a Dolly square that
brings the eye closer (up) or backs it away (down) along the line of sight; and Bright colors: a switch for `hdr`, the tone map
picker and a Before and after slider that keeps `ext.editor.tonemap_split`, so the preview shows the
part on the left without the tone map. The quality picker of the lights panel lists for each level the
triangles, the 3D groups, the work units of the models and whether reflections of pictures are drawn,
and for each tier the quality it starts at, whether the models fit and their texture memory.
`reiwand-ctl scene act DIR scene3d`, `group3d`, `material`, `camera3d`, `view3d`, `tonemap`, `model-info` and
`cost3d` do the same from the terminal, and `group3d … show` prints the handles the preview draws.

The inspector of a model with a skeleton or clips adds an Animation part: its clips with their length and
animation, On the timeline (the clip timeline of meshes, whose rows for a model key `angles` and `shift`
of the chosen bone and the weights of its blend shapes, and which shows the events of the clip as marks
that seek to them), New clip from the file with the animations of the file and their length, Root motion,
Swinging bones (a bone picker and the presets, with the bodies the layer has), a slider per blend shape
for its weight, and Animation layers: per layer the clip it plays, On, Adds, move up, remove, sliders
for weight, speed, fade in and fade out, and a bone mask picker (Only these bones); the same Animation
layers part shows under the bones of a mesh. In the authoring view the selected model draws its bones
(a line from each bone to its parent and a knob that picks the bone for the timeline and the pickers)
and, with root motion, the path the layer will walk ahead from where it stands, mixing every playing
animation layer by its weight (adding the additive ones), over two turns of its longest clip and in all three
directions, turning where the clips turn. The picked bone shows a ring: dragging
the ring turns the bone around the view axis, dragging inside it tips the bone like a trackball, and a
square in its middle moves it in the plane of the view; each drag keys `angles` or `shift` of that bone
in the chosen clip at the playhead (a model without clips gets `clip_1`). A swinging bone draws the fan
its rotation bound allows in its own plane, with Least and Most knobs that drag the bound, and under
Swinging bones each body shows its wind slider, the bounds of its turn and slide and Show the wind on
the preview, which draws the arrow of the wind of the scene. A body with a `translation` bound draws the
box its head may slide in, along x, y and z of the model around the head it is pinned to, and the picked
bone gets Least and Most knobs on its x edge that drag the bound. Colliders lists the colliders of the
model: Add a collider on the picked bone, a capsule along the bone or a sphere at its head, a radius
slider and Remove; the view draws each one as dashed circles, with a knob on the rim that drags the
radius. Chains that reach lists the IK chains of the model with New chain from the parent of the picked
bone to it, the target (a point, the pointer, a bone of the model or a layer), the weight and Remove; a
point target shows a knob in the view that drags it, in pixels of the box of the layer. `rigs::query` answers `bone3d` (`layer`,
`bone`, `clip`, `time`, `clip_time`, `from` with the values the drag started at, and `turn` in degrees,
`tilt` `[x, y]` in degrees or `slide` `[x, y]` in pixels) with the `angles` and `shift` to key, and
`bound3d` (`layer`, `bone`, `point`) with the degrees of the bound under a point, and
`surface3d` (`at` a point of the canvas in pixels, `time`) with the model surface under it in the
authoring view (`layer`, `material`, `uv`, `point`, `normal`, `unit`: the size of a pixel of the canvas
there, and `color` of its surface), or `null`; the cast follows the skinned pose of the time but not
the blend shapes;
`reiwand-ctl scene act DIR key` and `scene act DIR physics`
do the same from the terminal.

A material row picks Fur, Foliage or Chroma key besides Physical and Unlit and the material shaders of
the package: fur shows sliders for shells, length and density, the shells each quality level draws,
a comb pad for the comb of the whole material (dragged across for x and y, a slider toward the viewer)
and Comb on the model, a brush of the authoring view: while it is on, a drag over the model draws the
stroke with a ring of the brush size and, when released, combs the fur where it passed (`comb`, one
step of undo per stroke), with sliders for the brush size and the strength, Erase, and Clear the combing;
foliage shows sway and speed, the
wind of the canvas with its arrow; chroma shows a key color
picker, Pick the key color from the model (a click on the model in the authoring view sets the key,
`key`), and tolerance and feather sliders. Every material row has a Debug view: none, the sway mask of
the vertex colors, the matte (its alpha after the key) or where shadows fall (lit white, shadowed red,
unlit materials white); only that material shows it and the others are drawn as they are. Debug views
live in `ext.editor.debug3d`, which only the author preview reads: a shown or packed wallpaper never
draws them. A shader of the package shows a warning that it
runs on the graphics card within the limits of package effects. Under the materials, New material shader
writes a template, and every material shader of the package shows whether it builds, or its error with
the line and column of its file. *Edit the WGSL code…* opens the shader in the code view of package shaders, with
its live checks, its marked failing line and its undoable save (`shader` `write`), and *Open in your
editor…* in that view hands its file to the editor of the desktop.

The lights panel shows Lights in 3D: per light a switch that places it in 3D where it shows now or
sends it back to the plane (`scene3d` `light`). In the authoring view every light shows a knob where it
lights the models (dimmed while it stays on the plane: dragging it places it in 3D) and, but for point
lights, a square on the point it shines at with a dashed line between them; both move in the plane of
the view or, with Ctrl, along the line of sight, keying `position3d` and `aim3d` at the playhead when
they are keyed (`grab` with `what` `light`). `stage3d` lists each light with `at`, `aim`, `space` and
whether its fields are keyed or bound. The lights panel shows Shadows: per point, spot or directional light a switch, the size of its map
(automatic or 256 to 4 096) with the size each tier draws, on phones and at Q0, and a bias slider, and
Show where shadows fall, which draws lit places white and shadowed ones red in the preview and the
dashed frusta of the maps in the authoring view. The 3D camera panel shows Camera paths: per path its
length, Loop, the easing, remove, its keys (a click removes one) and Key the view at the current time,
Add from the view, the order, the fade with its length, and Fly through the paths from the camera, which
looks through the scene camera and plays from the start. In the authoring view each path draws its
curve, a knob on each key that moves the eye of that key and a square on the point it looks at, both
moved in the plane of the view at their own depth or, with Ctrl, along the line of sight (`grab` with
`what` `key`), a dashed line between them, and a square where the
paths put the camera at the current time. The scene timeline shows a lane per path with its keys, which
drag to another time (`key` with `from`, `camera-path … key PATH T --from T` in the terminal) and seek
the preview with a click; the panel of the timelines takes the presses that no lane takes, because the
Breeze style turns a drag that nothing accepts into a move of the whole window, which kept the keys of a
lane from being dragged. The cost lines add the shadow maps, the fur shells, the fur triangles each
shadow map draws (`fur_shadow_triangles`; `left_out` says when fur casts its skin only) and
the skinned vertices of each quality level and the shadowed lights of each tier. `reiwand-ctl scene
animate`, `clip`, `key`, `material`, `shadows`, `camera-path`, `new-material`, `debug3d` and `cost3d`
do the same from the terminal.
