# Layers

To move, scale or rotate a layer, see [Place and shape a layer](place-and-shape-a-layer.md#place-and-shape-a-layer). The list of layers is in the app's layers panel.

## `add_layer`

Adds a layer of a given kind, at the end of the list (the very top) or wherever you ask. It prints the id of the new layer. In the app it is **Add a layer**, and **Layer from a file** imports the file in the same step.

| Argument | Type | What it means |
|-----------|------|---------------|
| `kind` | text (required) | Layer kind. One of those in the table below. |
| `id` | text | Id of the layer. If missing, it comes from `name` or the kind (`image`, `image_2`...). If it is already in use, the action fails. |
| `name` | text | Name used to build the id. On a `text` layer it is also the text it shows (default `Text`). |
| `source` | text | Where what the layer shows comes from. See below. |
| `file` | text | File (path from where you run the command) that is imported as an asset and used in the new layer. It must be of the layer's kind: image, video, sound or `.glb`. Only valid for `image`, `video`, `sound`, `mesh` and `model`. |
| `into` | text | Id of a group to put the layer in. Without it, it goes in the main list. |
| `at` | integer | Position in the list: 0 is the very bottom. If it is greater than the length, it goes at the end. |

| `kind` | What it creates | What `source` is |
|--------|----------|------------------|
| `image` | An image. | Id of an image asset. Without it, the first one there is. |
| `video` | A video. | Id of a video asset. Without it, the first one there is. |
| `sound` | A sound. | Id of a sound asset. Without it, the first one there is. |
| `text` | A text. | Not used. |
| `shape` | A shape (a 480 by 320 px rectangle). | Not used. |
| `solid` | A flat color that fills the canvas. | Not used. |
| `spectrum` | An audio spectrum (960 by 240 px). | Not used. |
| `particles` | A layer that draws a particle system. | Name of the system. If it does not exist, one is created with default values. Without it, it uses the layer's id. |
| `group` | A group of layers. | Not used. |
| `adjustment` | An adjustment layer: its effects change what is underneath. | Not used. |
| `mesh` | An articulated mesh from a `.glb` file. | Id of a `.glb` asset. |
| `model` | A 3D model from a `.glb` file. | Id of a `.glb` asset. |
| `group3d` | A 3D group. | Not used. |
| `light` | A light. | `point`, `spot`, `tube` or `directional`. Without it, `point`. |
| `shapes` | Shapes that scripts draw from a pool (`pool`). | Id of the pool. Without it, the first one. You must create one first with [`pool`](../actions-time-and-package/scripts-and-templates.md#pool). |

If you ask for a kind that needs an asset and there is none, the action says what to import first.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"text","name":"Title","at":0}'
reiwand-ctl scene act my-scene add_layer '{"kind":"group","id":"ornament"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","into":"ornament"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"light","source":"spot"}'
```

## `remove_layer`

Removes a layer and everything that named it from outside: parents, masks, effect parameters, bone bindings, triggers. It is rejected while a particle system takes particles from that layer. In the app it is **Remove**.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the layer. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene act my-scene remove_layer '{"id":"background"}'
```

## `duplicate_layer`

Copies a layer right above the original, with new ids (`id_copy`). If the layer is a group, it also copies the ones inside. It prints the id of the copy. With `template` it does not make a new layer: it saves the layer as a template so a script can repeat it (see [`template`](../actions-time-and-package/scripts-and-templates.md#template)). In the app it is **Duplicate** and **Make a template**.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the layer that is copied. |
| `template` | yes/no | `true` saves it in `/scene/templates` under the name `id_template` (or `id_template_2`...). Only image, text, shape, solid, spectrum, mesh and group are valid. Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"title"}'
reiwand-ctl scene act my-scene duplicate_layer '{"id":"title"}'
reiwand-ctl scene act my-scene duplicate_layer '{"id":"title","template":true}'
```

## `move_layer`

Moves a layer to another place: to another position in its list, or to another group.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the layer. |
| `to` | integer (required) | Final position in the destination list (0 is the very bottom). If it is greater than the length, it ends up at the very top. |
| `into` | text | Id of a destination group. Without it, the layer goes to the main list. |

The group can be above or below the layer in the list. A group cannot be put inside itself or inside what it contains: the action is rejected.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"group","id":"ornament"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"title"}'
reiwand-ctl scene act my-scene move_layer '{"id":"title","to":0,"into":"ornament"}'
reiwand-ctl scene act my-scene move_layer '{"id":"title","to":0}'
```

## `group`

Puts several layers in a new group, which takes the place of the lowest one. It prints the group's id.

| Argument | Type | What it means |
|-----------|------|---------------|
| `ids` | list of texts (required) | Ids of the layers. They must be in the same list (all in the main one or all in the same group). |
| `id` | text | Id of the group. Default `group`, `group_2`... |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"title"}'
reiwand-ctl scene act my-scene group '{"ids":["background","title"],"id":"sign"}'
```

## `ungroup`

Dissolves a group and leaves its layers in its place. It is rejected if the group has something of its own that would be lost (opacity, transform, blend, mask, effects, parallax, visibility or clipping). In the app it is **Ungroup**.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the group. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"group","id":"ornament"}'
reiwand-ctl scene act my-scene ungroup '{"id":"ornament"}'
```

## `hide`

Hides a layer or shows it again. On a sound layer it mutes the sound. When hiding, it saves what the value was before and gives it back when showing.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the layer. |
| `hidden` | yes/no (required) | `true` hides, `false` shows. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene act my-scene hide '{"id":"background","hidden":true}'
reiwand-ctl scene act my-scene hide '{"id":"background","hidden":false}'
```

## `lock`

Marks a layer as locked, or removes the mark. The mark belongs to the editor (it is kept in `/scene/ext/editor/locked`) and only the app's canvas handles and drags respect it: a locked layer cannot be moved or stretched with the mouse. It protects nothing else: actions (`set`, `layer`, `move_layer`, `remove_layer`...), inspector fields and command-line commands still change a locked layer. In the app it is **Lock**.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the layer. |
| `locked` | yes/no (required) | `true` locks, `false` unlocks. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene act my-scene lock '{"id":"background","locked":true}'
```

## `solo`

Makes the editor's previews show only one layer (and what contains it), or all of them again. It does not change the scene that is saved. In the app it is **Solo**.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text | Id of the layer that stays alone. Without it, the solo ends. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"background"}'
reiwand-ctl scene act my-scene solo '{"id":"background"}'
reiwand-ctl scene act my-scene solo '{}'
```

## `rename`

Changes the id of a layer, timeline, asset or particle system and fixes everything that names it: parents, masks, effect parameters, values that follow a timeline, fields that name an asset, layers that use a system. Scripts are not rewritten: the command prints each script line that names the old id so you can change it by hand. In the app it is **Rename the layer** and **Rename…** on each asset.

| Argument | Type | What it means |
|-----------|------|---------------|
| `kind` | text (required) | What is renamed: `layer`, `timeline`, `asset` or `system`. |
| `id` | text (required) | Current id. |
| `to` | text (required) | New id: 1 to 32 lowercase letters, digits or `_`, and free. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene rename '{"kind":"layer","id":"sky","to":"background"}'
reiwand-ctl scene act my-scene rename '{"kind":"asset","id":"picture","to":"landscape"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"dust"}'
reiwand-ctl scene act my-scene rename '{"kind":"system","id":"dust","to":"rain"}'
reiwand-ctl scene act my-scene timeline '{"id":"intro","add":true}'
reiwand-ctl scene act my-scene rename '{"kind":"timeline","id":"intro","to":"entry"}'
```

## `paste`

Pastes layers copied with `reiwand-ctl scene ask FOLDER copy`, in this scene or another. The layers get free ids (the same id if it is free, otherwise `id_2`...); the assets, systems and effects they use are added if missing, with their files. In the app they are **Copy the layers** and **Paste the layers**.

| Argument | Type | What it means |
|-----------|------|---------------|
| `clip` | object (required) | What `scene ask FOLDER copy` prints: `layers` (the layers) and, if needed, `assets`, `particles`, `effects` and `from` (the folder they were copied from, to find the files). |
| `into` | text | Id of a group to paste into. |
| `at` | integer | Position to paste at (0 is the very bottom). Without it, the very top. |

```sh
reiwand-ctl scene act my-scene paste '{"clip":{"layers":[{ "blend_space": "canvas", "id":"greeting","type":"text","text":"Hello","size":96,"color":"#ffffff"}]}}'
reiwand-ctl scene act my-scene add_layer '{"kind":"group","id":"ornament"}'
reiwand-ctl scene act my-scene paste '{"clip":{"layers":[{ "blend_space": "canvas", "id":"greeting","type":"text","text":"Hello","size":96,"color":"#ffffff"}]},"into":"ornament","at":0}'
```

To get a real `clip`, copy a layer from the scene:

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"greeting"}'
reiwand-ctl scene ask my-scene copy '{"ids":["greeting"]}'
```

## `rgb`

Makes an adjustment layer send what it captures to your machine's RGB lighting, or stop doing so, and chooses the zone it captures. Turning it on asks for the package's `rgb` permission; turning it off (if no other layer uses it) removes it. It only works for `adjustment` layers.

| Argument | Type | What it means |
|-----------|------|---------------|
| `layer` | text (required) | Id of an adjustment layer. |
| `on` | yes/no | `true` sends to the RGB lighting, `false` stops sending. |
| `size` | list of 2 numbers | Width and height in pixels of the zone around the layer (from 1 up to twice the canvas). If the layer has no position, it is put in the middle of the canvas. |
| `whole` | yes/no | `true` captures the whole canvas (removes `size`). Default `false`. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"adjustment","id":"adjust"}'
reiwand-ctl scene act my-scene rgb '{"layer":"adjust","on":true}'
reiwand-ctl scene act my-scene rgb '{"layer":"adjust","size":[400,200]}'
reiwand-ctl scene act my-scene rgb '{"layer":"adjust","whole":true}'
```
