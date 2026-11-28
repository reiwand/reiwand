# The camera and lights

On this page: the scene camera, camera paths, lights and projected images.

## How to set and adjust the scene camera

Where it is: choose the canvas; in the inspector, **3D camera** part.

Steps:

1. Move the **Field of view** and **Roll** sliders; the preview changes while you move them.
2. Look at the text “From … toward …”, which gives the point the camera looks from and the point it looks at.
3. Place the authoring view where you want and click **Set the camera from the view**: the scene camera takes that position.
4. With **Look through the camera** you check the result; **Back to the camera view** returns the authoring view.

You can also move the camera with handles over the preview (with **Look through the camera** off):

- **Camera**: a knob on the camera's eye. Drag to move it in the plane of the view; with Ctrl, along the line of sight.
- **Aim**: a square at the point the camera looks at.
- **Roll**: a ring around its line of sight. Drag along the ring to tilt the camera.
- **Orbit**: drag to turn the camera around what it looks at, keeping the distance.
- **Dolly**: drag up to bring it closer to what it looks at and down to move it away.

In the terminal: `scene3d` with `camera` (`eye`, `center`, `up`, `fov`, `roll`, `from_view`) and with `grab` (`orbit`, `dolly`, `roll`, `plane`); `scene ask stage3d` lists the handles.

The **2D camera** (zoom, rotation and pan of a flat scene) is something else; it is in [editor-canvas](../editor-canvas.md).

## How to make a camera path

A path is a list of view keys: each key stores where the camera is and where it looks, and the camera travels from one to the next.

Where it is: canvas, **3D camera** part, **Camera paths** block.

Steps:

1. Place the view where the path should start (you can use **Set the camera from the view**).
2. Type a name in **Name of a new path** (lowercase letters, digits and underscore, up to 32 characters) and click **Add from the view**.
3. In the path's row you see its name and its duration, the **Loop** checkbox and the remove button. Below, its keys.
4. Move the view to another place and click **Key the view at N s** in that row to add a key at the current time.
5. To remove a key click **Remove this key** on its button.
6. If there are several paths, choose **In order** or **At random** for the order, and among **Cut**, **Through black** and **Crossing** for the change from one to the next; the number box next to it gives the duration of that change in seconds.
7. Click **Fly through the paths from the camera** to see the flight from second zero.

In the preview each path draws its curve, a knob at each key (the eye at that moment) and a square at the point it looks at, joined by a dashed line. Drag a knob to move the eye of that key; with Ctrl, along the line of sight. The label is **Path N at X s** and **Aim at X s** on the square. A **Camera now** box marks where the paths put the camera at the current time.

The scene's timeline has a lane per path with its keys: drag one to another time, or click to go to that moment in the preview.

In the terminal: `scene3d` with `path` (`add`, `key`, `remove`, `set`, `order`, `fade`) and the query `paths3d`.

## How to add and adjust a light

There are four types of light: **Point**, **Spot**, **Tube** and **Directional**.

Where it is: the **Add a layer** button, **Light** submenu: **Point light**, **Spot**, **Tube light** and **Directional light**.

Steps:

1. Choose the type in the menu. The light enters as a new layer (inside the group if you had a group chosen).
2. In the inspector, **Light** part: change the type with the list (**Point**, **Spot**, **Tube**, **Directional**) and the color with the picker next to it.
3. Adjust the sliders:
   - **Intensity**.
   - **Radius**: how far it reaches.
   - **Height**: the distance to the canvas plane.
   - **Length**: tube only.
   - **Bright middle** and **Cone**: spot only, the angle of the strong part and that of the whole beam.
   - **Tilt toward the canvas**: on the spot and the directional, how much it looks toward the canvas.
4. The **Light against the distance** graph shows how the light falls off as you move away.
5. Checkboxes at the end: **Show the part it redraws when it moves**, **Light the preview with it alone** (solo) and **Leave it out of the preview** (mute).

Over the preview each light has a mark. Drag it to move it (**Drag to move the light**). The chosen light draws its handles:

- A ring with a knob on its radius.
- A square above it whose distance gives the height (**Height N**: drag up or down to raise or lower it).
- On the spot: a knob that rotates it (**Drag to turn the light**), the edges of the outer cone and of the bright middle with a knob on each (**Drag to widen or narrow the cone** and **the bright middle of the cone**), and a knob on the direction line for the tilt (**Tilt N°**).
- On the tube: a knob at each end that moves it, rotates it and lengthens it.
- On the directional: an arrow with a knob that rotates it.

If the value has keys or the light is animated, the handles put a key at the current time, like on the other layers. Lights also appear in the timeline. The lights' handles and the lights panel use only the first 3D group: with several 3D groups in the scene they show the placement of the first one.

In the terminal: the `light` action (`add`, `set`, `remove`, `solo`, `mute`). The `lights` query lists the lights and `falloff` gives the falloff curve. See [actions](../actions.md).

## How to see all the lights and their limit

Where it is: choose the canvas; in the inspector, **Lights** part.

What you see:

- A row per light with its name, its type and the **air** and **projects** marks if it uses them. A click chooses the light. Each row has **Solo** and **Mute** buttons.
- “N of at most M lights”: how many lights the scene has against the 12 allowed.
- Per equipment level, how many lights are kept and how many are left out.
- The **Preview quality** selector (**Q0, lowest** to **Q3, highest**) with the **Restart the preview** button. Below, per quality level, whether the light is computed once at the start of the scene (Q0) or how many lights per pixel are computed in each frame. A separate line counts the same for phones.

In the terminal: query `lights` and `lighting` with `time` and `quality`.

## How to make a spot project an image

Where it is: the spot's inspector, **Projects** part.

Steps:

1. Choose the spot.
2. In **Projects**, choose an image or a video from the scene, or a layer that can be projected (no loop). **Nothing** turns it off.
3. The preview draws inside the cone, with a dashed outline, what falls on the canvas. For a video the playhead's frame is shown. A group, a shape, a spectrum or a layer with effects or a mask is seen as the engine draws it at the playhead, cut to what they cover.
4. If the beam runs along the canvas, the image is drawn out to the light's radius, dimmer, with a notice.

Only the point and the spot, and not the tube, offer air; only the spot projects.

In the terminal: the `light` action with `projection`; query `projection`.
