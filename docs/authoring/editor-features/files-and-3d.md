# Layers from files and 3D models

On this page: layers from files and importing 3D models.

## Layers from files

The *Add a layer* menu offers *Layer from a file* (image, video, sound or `.glb` mesh: it chooses
the file, imports it as an asset and adds the layer in a single step, as
`reiwand-ctl scene act DIR add`), *Light* with its four types, each with reasonable starting values.
Lights are seen over the preview as markers that are dragged and the chosen one shows its handles:
the radius ring, the height, the rotation, the edges of a spot's cone, the two ends of a tube and the
arrow of a directional light, and on request the box that is redrawn when it moves; if their values
have keys, the drag creates the key at the current instant. The light's inspector chooses the type
and the color, adjusts intensity, radius, height, length and cone, draws the falloff of the light
with distance and leaves it solo or mutes it in the preview. When you choose the canvas, all the
lights are listed with those two switches, how many there are against those each equipment class
keeps, the colors of the ambient light and the sky with a sphere lit by them and the quality selector
from Q0 to Q3, which here says whether the light is computed once or how much it costs in each frame.
Image, video and text layers have their lighting part: the normal, reflection, roughness and glow
maps, their sliders, *Generate normals* from the image (with strength, blur and a view of the relief
before saving it as a package image) and a brush that paints roughness or glow in gray over the
preview with its heat view. Point lights and spots have *Light the air* with density and samples
sliders, the glow curve and what each quality level does; spots choose what they project (images,
videos and the layers that can be projected) and the preview draws the image inside the cone, where
it falls on the canvas. The particle tools have their light part: receiving lights, the normal map
with a view of the particle lit from the front and the back, and refraction with its map, its amount
and the displacement drawn over the preview; the budget says what each level costs and what phones
cut. In the terminal the same is done by `reiwand-ctl scene act DIR light`,
`scene act DIR particles`, `scene ask DIR lights` (the handles, the box and the counts, or the cost
with `--cost`), `scene act DIR lighting`, `scene act DIR normals`, `scene act DIR paint_map` and
`scene act DIR ambient`.

## 3D models

3D models are added with *3D model from a file*, which reads the `.glb` before importing it and
reports its triangles, materials, textures, bones and clips and what each equipment class receives,
or with *3D model* and *3D group* from the layer menu. The preview of a scene with models has an
authoring view with its own camera, which is saved with the project but only used by the preview
(like the debug view and the tone-map split): the right button rotates it, the middle one (or Shift)
pans it and the wheel moves it closer, while the scene's camera is drawn as a pyramid with handles
for the eye and the point it looks at. The chosen model shows arrows in x, y and z to move it, one
handle to rotate it and another to scale it; a click on its box chooses it. A model's inspector shows
its counts, the reflection (`env`) with a thumbnail and a materials panel with a sample sphere, the
shader, the light, the double side, the shadows, the colors, the numbers and the thumbnails of its
maps; a group's makes it 3D and that of a flat layer inside it turns on *Hide behind nearer models*.
When you choose the canvas the 3D camera appears (field of view and roll live, *Set the camera from
the view*, *Look through the camera*) and the bright colors (HDR, tone mapping and a before-and-after
slider over the preview); the quality selector says how many triangles, 3D groups and texture memory
each level costs and whether it fits in each equipment class. In the terminal the same is done by
`reiwand-ctl scene act DIR scene3d` and `scene ask DIR cost3d`.

To animate a model, its inspector has an animation panel: the layer's clips with a button to open
them on the clips timeline (with rows of 3D rotations and offsets per bone, blend shape weights and
the event marks), *New clip from the file* with the `.glb`'s animations, *Root motion*, swinging
bones with preset settings (hair, tail, cloth, antenna, chain, jelly), a slider per blend shape and
the animation layers (clip, weight, speed, fades, additive mode and bone mask), which now also appear
under the meshes' bones. In the authoring view the chosen model draws its bones, which are chosen
with a click, and the path it travels with the root motion; the chosen bone shows a ring that rotates
it (by the edge) or tilts it (by the inside) and a square that moves it, and each drag puts a key in
the clip at the playback cursor. Swinging bones draw the fan of their rotation limit with two handles
to change it, and their panel has each bone's wind, the limits and the wind arrow over the preview.
The move, rotate and scale handles start from the model's value at that moment: if it has keys they
put a key and if it follows a link they do not touch it. Each material chooses fur (layers per
quality, length, density and a panel to comb it), foliage (sway, speed, the wind arrow and the mask
of the vertex colors over the preview) or chroma key (key color, tolerance, feather and the matte
over the preview); *New material shader* writes a template and each package shader says whether it
builds or at which line and column it fails, with a notice that it runs on the graphics card. The
lights panel turns on shadows per light, with the map size and what each equipment class draws, the
bias and a view that paints in red what is left in shadow and draws the maps' volumes. The 3D camera
panel edits paths: keys taken from the current view, which are dragged over the curve drawn in the
authoring view (the eye and the point they look at, each at its own depth) and in their own row of
the scene's timeline, loop, easing, order, fade and a button to fly through them from the camera. The
cost panel adds per level the shadow maps, the fur layers and the skinned vertices. In the terminal:
`reiwand-ctl scene act DIR animate`, `scene act DIR clip`, `scene act DIR key`,
`scene act DIR scene3d` and `scene act DIR physics`.
