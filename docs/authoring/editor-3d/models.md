# Models and moving in 3D

On this page: adding models, 3D groups, the handles and the view.

## How to add a 3D model

Where it is: the **Add a layer** button of the layers panel, the **Layer from a file** entry, the **3D model from a .glb file…** option.

Steps:

1. Click **Add a layer**, open **Layer from a file** and choose **3D model from a .glb file…**.
2. In the **3D model from a file** dialog, click **Pick a binary glTF file** and choose a `.glb` file.
3. Before accepting, the dialog tells you how many triangles, materials, textures, bones and clips the file has. For each equipment level it says whether it **fits** or is **too heavy**, what maximum texture size it keeps and how much texture memory it uses.
4. Accept. The file is imported into the scene and a model layer with the file's name is added, in a single step; **Undo** removes both things at once. If the file cannot be read, the dialog says why and **OK** stays disabled. If you had a group chosen, the model goes into it.

If you have no model at hand, the app ships one of its own: the example scene **Turning crate** (`solids`), with a crate, its light and its shadow. In **Create**, the **Learn from an example** tab, choose it and click **Edit a copy…** and you will have a 3D scene ready to try; its model, `crate.glb`, is public domain (CC0) and is in the copy's `assets/`, so it can also be imported into other scenes. The files in `scenes/solids/assets` of the repository are the same.

FBX and OBJ files are not read directly: open them in Blender and export them as binary glTF (`.glb`).

If the layer menu already has an imported `.glb` file, the **3D model** entry in the list of types also works: it lets you choose the file among those the scene already has.

What you will see: the layer appears in the list with the model icon and the inspector shows the **3D model** part with the file's numbers and, per equipment level, whether it fits.

Layer settings:

- **Reflects**: choose a scene image that the model reflects on its shiny surfaces (or **Nothing**). A thumbnail is shown next to it.
- **Look at it in the authoring view**: frames the model in the authoring view (see below).
- To move, rotate and scale use the handles of the authoring view or the position, rotation and scale fields of the inspector.

In the terminal: the `scene3d` action with `model` (`add`, `set`, `remove`); `scene ask model_info` reports the same as the dialog. See [actions](../actions.md).

## How to gather models, lights and flat layers in a 3D group

Where it is: the **3D group** layer type, and, on a flat layer inside a 3D group, the **Hide behind nearer models** checkbox.

Steps:

1. Choose a group. The inspector offers **3D group: models, lights and flat layers share depth**; turn it on so the group is 3D.
2. Put inside it the models, lights and flat layers you want.
3. On a flat layer of that group, check **Hide behind nearer models** so that a model passing in front covers it.
4. On a particle layer, a note below the checkbox says whether each particle hides on its own or whether the whole layer hides as a plane, and why (for example, because the system has no perspective). At the lowest quality the whole layer always goes as a plane.

In the terminal: `scene3d` with `group`, `space` and `depth_test`; query `depth3d`.

## How to move, rotate and scale a model with the handles

Where it is: the preview. As soon as the scene has a model, the preview becomes an authoring view with its own camera and the handles are drawn over it.

What is drawn:

- Each model has a box. A click on the box chooses it.
- The chosen model shows three arrows **X**, **Y** and **Z** (red, green and blue). Each one moves it in that direction. The model stays under the pointer whatever the angle of the view.
- Three rings, **Turn X**, **Turn Y** and **Turn Z**. Drag along the ring to rotate the model around that direction of the scene.
- A **Depth** square: drag up to move the model away along the line of sight and down to bring it closer.
- A **Turn** knob: sideways it rotates the model and up or down it tilts it.
- A **Size** square at the corner of the box: drag it away from the model to enlarge it and inward to shrink it.

The handles start from the values the model has at the current time.

Steps:

1. Choose the model with a click on its box or in the layer list.
2. Drag the handle you need.
3. Release. The position, rotation or scale is saved as a single undo step.

If the value has keys, the handle puts a key at the current time (with the **Write over keyframes** switch on, it puts a fixed value over the keys and the links). If the value follows a link to a property or to a script, the handle looks dimmed and warns: “The position follows a binding; drag with Shift to replace it with a value”. With Shift while you drag, the link is replaced by the value. For scale, a link can only be changed from the inspector.

Shortcuts: there are no keyboard shortcuts of their own for the handles. The modifiers are Shift (replace a link) and, on the handles of the camera, the lights and the paths, Ctrl (see below).

In the terminal: `scene3d` with `grab` (which handle, from where to where and the time), or `scene3d` with `model` `set` to write position, rotation and scale directly. This second form refuses to write over keys or links. See [actions](../actions.md).

## How to move something in a plane or in depth

The handles of the camera, of 3D lights and of path keys move in two ways:

- Normal drag: they move in the plane of the view, that is, parallel to the screen.
- Drag with Ctrl: they move along the line of sight, toward or away from the viewer.

For a model there is no plane handle of its own: use the arrows, which move in a single direction, and the **Depth** square.

In the terminal: `scene3d` with `grab`. The handles are `x`, `y`, `z`, `turn_x`, `turn_y`, `turn_z`, `plane`, `depth`, `orbit`, `dolly` and `roll`.

## How to move around the view without touching the scene camera

There are two cameras. The authoring view is yours alone: it is for looking around while you work and does not show in the wallpaper. The scene camera is the one the viewer will see.

Where it is: the preview, when the scene has models.

Steps:

1. Drag with the right button to rotate the view around its look-at point.
2. Drag with the middle button, or with Shift and the right button, to pan it.
3. Turn the mouse wheel to move closer or farther.

The preview is drawn with that view. The scene camera is seen as a volume of dashed lines with its handles (see below). Select the canvas to see the **3D camera** panel, which repeats this tip.

To go back: in the **3D camera** panel click **Back to the camera view**. With **Look through the camera** on, the preview shows what the scene camera sees.

In the terminal: `scene3d` with `view` (`orbit`, `pan`, `zoom`, `frame`, `reset`, `through`).
