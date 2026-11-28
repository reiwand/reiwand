# Layer outlines on the canvas

The canvas draws the outline of each layer where the scene draws it, with its parents, groups,
anchor, origin, skew, parallax and 2D camera, from the editor query `seats::seats`, which places
every layer with the same code as the renderer (`Runtime::seats` of `reiwand-scene`: the frame its
position lives in and its whole transform); groups get the box around their layers. The selected
layer gets handles: the corners scale it from its pivot (Shift keeps the proportions), the middles
of the edges stretch one side or, with Ctrl, skew it along that edge, a round handle above turns
it, the dot of the pivot moves its origin without moving the layer, and a depth gizmo drawn in the
perspective of the scene tilts and lifts it: a dashed ring around the pivot in the plane of the layer,
the upright axis whose end tilts it around the width (`transform.tilt[0]`), the level axis whose end
tilts it around the height (`tilt[1]`) and a line out of the layer from the middle of its bottom edge,
along the direction `z` moves it, whose diamond lifts it (`transform.z`). The gizmo comes from
`seats::depth` (the outline field `Gizmo.depth`: `pivot`, `ring`, `height`, `width`, `foot`, `lift`
and `reach`, its length in canvas pixels, 0.35 of the shorter side of the layer, from 40 to 400),
which projects every point with the plane the renderer leans the layer by (`Seated.plane` of
`Runtime::seats`, `Plane::lean`, the same code as the scene draws with). The `tilt` grip (`{ "tilt":
{ "axis": 0 } }` or `1`) picks the angle, −89 to 89 degrees, that puts the end of that axis nearest to
the pointer, the one closest to the current angle when two are as near, and `lift` the depth that puts
the end of the line nearest to it. Each handle sends the `grip` of the `layer` action with the
point it is dragged to, so the corner or edge it holds stays under the pointer whatever the
parents do. With *Magnets* (on by default) a dragged layer snaps to the edges and the middle of the
canvas and of the other layers, showing a guide where it snaps, and Alt places it freely; the arrow
keys nudge the selected layer by a pixel, by ten with Shift, as one step per press. The canvas takes the keyboard focus (the `SceneFocus` signal) when a layer is picked in the layer list, when a layer is tapped or dragged and when the empty canvas is tapped, so the arrows work at once; a change of the selection that is not one of those gestures leaves the focus where it is. *Fit to the
canvas* covers the canvas with it, fits it inside or stretches it (`fit`). *Follow a layer* picks
its `parent` with a tap on another layer of the canvas, keeping it where it is (`parent` with
`keep`), and *Stop following* removes it the same way; while a layer follows another a dashed line
joins their pivots, and the list of the `parent` field of the inspector only offers the layers it
may follow, without the ones that would close a loop or pass 8 layers (`seats::parents`). Other
tools of the canvas build on the same place: `Seat::at` takes a point of the box of a layer, in
fractions, to the canvas, and `Seat::fraction` brings it back; the overlay `SceneSeat.qml` does the
same for a layer named by id and sends the grips. A tilted or lifted layer, or one under a tilted
parent, also gets a dashed outline where the scene projects it (`Gizmo.corners`, through the same
perspective as the scene), and its handles sit where the scene shows them (`Gizmo.handles`: the
projected corners, middles of the edges and pivot). The grips bring the pointer back onto the layer:
`turn`, `scale`, `skew` and `size` onto the plane of the layer (`Seat::unlean`), and `move` and
`origin` onto the plane the pivot moves in, the layer at its depth without its own tilt
(`Seat::unlift`), so whatever they hold lands exactly under the pointer. Taps, drags and the hover of a layer take only the points inside its outline (the same polygon the canvas draws, so a skewed layer is hit exactly as drawn), not the rectangle around it. The *2D camera* button of the toolbar adds the camera of the
scene (`canvas.camera`) or removes it, and switches between *View through the camera*, where the
canvas and every handle show the scene as the screen does (`seats::seats` places the layers through
the camera), and *View the whole canvas with the camera frame*, where the preview and the handles
leave the camera out and the frame of what the camera shows is drawn over the canvas, from the outline
field `camera2d` (`seats::lens::frame`: `center`, `zoom`, `rotation`, `corners`, the point `top` of
the knob that turns it, and `whole`; `null` without a camera), computed from the camera the renderer
itself uses (`Runtime::lens` of `reiwand-scene`) without its shake. The frame has a knob in the middle
that moves the camera, one per corner that zooms it and one above that turns it, each through the
`camera_grip` of the `canvas` action, so a keyed camera gets a key at the playhead or, with
*Overwrite*, its plain value. A script that moves the camera through `scene.camera` still moves the
preview in either view. The inspector of a layer shows its parallax depth (`parallax`) with a slider per side, across and up
and down, or one for both while *Same depth across and up and down* is on, and a *Parallax range* of
±1, ±3 or ±10 that widens by itself to hold the stored value; each drag is one step of the `depth` of
the `layer` action. While the canvas has parallax, *Try the parallax* in the toolbar puts a simulated
pointer on the canvas (the outline field `probe`), and, with *Follow the tilt of a phone*, a square
whose place from the middle of the canvas is the tilt; dragging them sends `probe` and the outlines and
handles of every layer shift by its depth, and the grips keep working on the shifted layers. While
it is on, the chosen layer gets a handle at its middle displaced by its depth (a quarter of the
canvas width is 1, from -10 to 10; the same depth across and up and down moves along the diagonal
and Shift sets them apart, a depth that follows a binding or keys has none), and choosing the canvas
gives its parallax a handle for `offset` (from -1 to 1 of half the canvas on each side); both send
`depth` of `layer` and `parallax` of `canvas` as drags. The
preview stream itself still follows the real pointer. The toolbar also offers
guides for 16:9, 21:9, 32:9, 4:3, 16:10 and 9:16 screens. *Screens* previews the scene spread over
several screens: two or three side by side, two stacked, or the user's own layout as the daemon
reports it (`GetState().outputs[x].mm`). It frames the part of the canvas that each screen shows
and darkens what the frames between them (`layout.bezel_mm`) and the edges of the layout hide, and
its *Spread over every screen* box sets or removes `display.span: "all-outputs"` in the manifest,
as an undoable step. The parts come from the editor query `{"spread": {"layout", "bezel",
"monitors"}}` (`rigs::query`), which answers `{spread, canvas, screens, bounds, parts: [{id, rect}]}`
with each rect in canvas pixels, or `error` for an unknown layout or screens without a name or a
size (at most 64). `reiwand-ctl scene preview DIR --layout dual|triple|vertical|mine --out FILE
[--time S]` writes the same view as a picture, without a desktop: the scene at its poster time, or
at second `S` of the scene act DIR canvas`--time` is 0 or more (as the editor shows the moment its
preview is at), each screen's part at its physical place and the frames between them. Dragging a layer that is not locked
selects it and moves it, as one gesture. The handles write with the `layer` action, as
`reiwand-ctl scene act DIR layer` does: a value with keyframes takes a keyframe at
the playhead instead, and so does each part of a position or scale keyed on its own (`tracks`),
while its plain parts take the new value; a part that follows a link refuses the drag and says so.
*Write over keyframes* in the canvas toolbar makes them write a fixed value over keyframes and
links (`replace`). A drag that starts outside the layers, or any drag with
Shift, draws a rectangle that selects the layers it touches and picks them as Ctrl does in the
layer list, ready for *Group*. Ctrl and the wheel zoom, the wheel or a drag with the middle or
right button pans.
