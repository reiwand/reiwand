# Adding and arranging layers

On this page: adding, grouping, ordering, renaming, duplicating and parenting layers.

## How to add a layer

Where it is: **Layers** tab, the **Add a layer** button.

Steps:

1. Open the button. The menu lists the types: **Image**, **Video**, **Text**, **Shape**, **Solid color**, **Sound spectrum**, **Particles**, **Sound**, **Group**, **Adjustment**, **Mesh**, **3D model**, **3D group** and **Shapes from a pool**.
2. The types that need a file (image, video, sound, mesh, 3D model) are off with the notice **import a file of this kind first** while the scene has no asset of that type. **Shapes from a pool** asks first for a pool (**add a pool first**). A shapes layer has no handles on the canvas or preview of what the pool contains, because the shapes are created by scripts when they run.
3. If there is a single asset of the type, the layer uses it. If there are several, a **Which asset** menu lets you choose.
4. If you had a group chosen, the layer goes into that group.
5. **Light** opens another menu: **Point light**, **Spot**, **Tube light** and **Directional light**.
6. **Layer from a file** chooses a file from disk and creates the layer with it: **Picture…**, **Video…**, **Sound…**, **Mesh from a .glb file…** or **3D model from a .glb file…**.
7. **Mesh from a picture…** creates a mesh from an image asset. See [drawing](../editor-drawing.md).
8. The **Particle systems** button in the same row opens the management of the scene's particle systems (see [particles and sound](../editor-particles-and-sound.md)).

Each layer gets an id of its own, which you can change (see [How to change the id of a layer](#how-to-change-the-id-of-a-layer)).

In the terminal: `add_layer` (with `kind`, and `file` or `source`) and `light` with `add`. See [actions](../actions.md).


```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"title"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"group","id":"sky"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background","into":"sky"}'
reiwand-ctl scene act my-scene move_layer '{"id":"title","to":0}'
reiwand-ctl scene act my-scene light '{"add":{"light":"spot"}}'
reiwand-ctl scene act my-scene rename '{"kind":"layer","id":"title","to":"name"}'
reiwand-ctl scene act my-scene duplicate_layer '{"id":"name"}'
reiwand-ctl scene act my-scene ungroup '{"id":"sky"}'
```

## How to start with a clock, a date or endless motion

Where it is: **Layers** tab, the **Starting points** button (the stopwatch icon, next to **Add a layer**).

Steps:

1. Open the button. **Add a clock or a date** lists formats such as `HH:mm`, `hh:mm a` or `EEEE d MMMM yyyy`, each with how it looks now. Choosing one creates a text layer that follows the clock; if you had a group chosen, it goes into it.
2. **Keep the chosen layer moving** rotates the layer endlessly (**Rotate right**, **Rotate left**) or moves it and brings it back endlessly (**Drift to the right and back**, **Drift to the left and back**). Linear keys with repetition are left on the layer's rotation or position: you can change them afterwards in the keys.
3. When you type the text of a text layer, **Show the time or the date…** appears below. Typing `{time}` does not show the time: the text comes out as is. That button puts the clock or the date in its place with a format to choose.

In the terminal: the `layer` action with the `start` variant, which has `clock` and `forever`. See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene layer '{"start":{"clock":{"format":"HH:mm","id":"time"}}}'
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","id":"wheel"}'
reiwand-ctl scene act my-scene layer '{"start":{"forever":{"layer":"wheel","motion":"spin"}}}'
```

## How to group and ungroup layers

Where it is: **Layers** tab, the **Group the layers**, **Ungroup**, **Move into a group** and **Move out of the group** buttons.

Steps:

1. Choose a layer and mark others with Ctrl (see [How to choose several layers](choosing-and-moving.md#how-to-choose-several-layers)). Click **Group the layers**: a group is created with all of them.
2. Choose a group and click **Ungroup** to dissolve it. Its layers stay where they were.
3. To take a layer into an existing group: choose it, click **Move into a group** and choose the group in the menu.
4. **Move out of the group** returns the chosen layer to the main list.

A group moves, rotates and hides all its layers at once. The layers inside are seen indented in the list. See [layers](../layers.md).

In the terminal: `group`, `ungroup`, `move_layer` (with `into` to enter a group).

## How to order layers

Where it is: **Layers** tab. The list shows at the top the layer that is drawn above all the others.

Steps:

1. Choose the layer and click **Raise** or **Lower**: it changes one place within its group.
2. Or drag the handle on the left of its row (**Drag to move the layer**) to another row. A line marks the spot. When you release, the layer takes the place of that row, within the same group as it.
3. The buttons are off if the layer is already at the very top or bottom.

In the terminal: `move_layer` with `id` and `to` (position) and, if needed, `into`.

## How to change the id of a layer

Where it is: **Layers** tab, the **Rename the layer** button.

Steps:

1. Choose the layer and click the button. **Rename X** opens.
2. Type the id you want. Only lowercase letters, numbers and underscore are valid, up to 32 characters.
3. Accept. The links, parents, masks, effects, triggers and timelines that name it follow the chosen id.
4. If any script names the layer, the dialog lists the script lines that are not rewritten; fix them by hand.

In the terminal: `rename` with `kind` `layer`. It works the same for assets, timelines and particle systems.

## How to duplicate, remove, copy and paste layers

Where it is: **Layers** tab, the **Duplicate**, **Remove**, **Copy the layers** and **Paste the layers** buttons.

Steps:

1. **Duplicate** makes a copy of the chosen layer with a free id.
2. **Remove** deletes the chosen layer. If another layer followed it and could not be left where it was, a notice says so; check where it ended up.
3. **Copy the layers** copies the chosen layer and those marked with Ctrl, together with what they use, to the clipboard as text.
4. **Paste the layers** (active when the clipboard has copied layers) pastes them with free ids and copies their files. If you had a group chosen, they go into it.
5. Copy and paste work between scenes: copy in one editor window and paste in another.

Shortcut: none; they are buttons.

In the terminal: `duplicate_layer` and `remove_layer`. `paste` receives the copied content in `clip`; copying to the clipboard is done only by the app.

## How to make a layer follow another (parent)

Where it is: the canvas bar, the **Follow a layer** and **Stop following** buttons. Also the **Parent** field of the inspector.

Steps:

1. Choose the layer that should follow.
2. Turn on **Follow a layer** and click on the canvas on the layer that will be its parent. The layer does not move from its place; from now on it moves with its parent.
3. A dashed line joins the layer's pivot with that of its parent.
4. Escape cancels the choice.
5. **Stop following** (visible if the layer has a parent) releases the layer without moving it.
6. You can also choose the parent in the inspector's **Parent** field.

Shortcut: Escape to cancel (`SceneOverlays.qml`).

In the terminal: `layer` with `parent` (with `keep` so as not to move the layer).
