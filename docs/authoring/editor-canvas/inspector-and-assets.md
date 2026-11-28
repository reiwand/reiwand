# The inspector, templates and assets

On this page: the inspector, pointer input, templates, assets packages and managing files.

## How to use the inspector

Where it is: the right column.

What it shows: the title is the id of the chosen layer, or the path of what is chosen (for example **Canvas** or a timeline). Below there is a row of buttons and then the fields.

Buttons in the row:

- **Up**: chooses what contains the chosen item.
- **Raise** and **Lower**: change the place of an item in a list (a key, an effect, an emitter...).
- **Add an effect**: opens the layer's effect list (see [effects](../editor-effects.md)). It does not appear on sound layers.
- **Remove**: deletes the chosen list item.
- **Edit the particle system**: on a particle layer, opens its system (see [particles and sound](../editor-particles-and-sound.md)).

Each field shows its name (with the unit in parentheses if it has one) and, when you hover over the name, a help. Depending on the field there is a slider, a number box, a color picker, a list of options, a checkbox or two boxes for a pair of values.

Steps:

1. Choose a layer. Change a field by typing or moving the slider. A long drag counts as a single history step.
2. An optional field that already has a value shows **Default**: it removes the value and goes back to the factory value. An optional group of fields shows **Add** to create it.
3. The button to the right of each field (◇, ◈, ◆ or ⛓) animates it or links it. See [animation](../editor-animation.md) and [properties](../editor-properties.md).
4. A linked field shows a **Follows X** button; click it to switch to editing the link.

The inspector comes from the scene format: if the format gains a field, it appears by itself. The fields of each layer type are in [layers](../layers.md).

In the terminal: `reiwand-ctl scene fields DIR POINTER` lists the fields the inspector shows, as JSON. To change one, the `set` (value), `add`, `remove` or `move` action on a JSON pointer. See [actions](../actions.md).

## How to make a layer receive the pointer

Where it is: the inspector of a group, image, video, text, shape, solid color, spectrum or mesh layer, **Pointer** section.

Steps:

1. Choose one of three options: **Lets the pointer through** (the default), **Takes the pointer over its whole box** or **Takes the pointer where it is opaque**.
2. With the third, an **Opaque from X %** slider appears: the pointer counts only where the layer is more opaque than that percentage.
3. With a layer that takes the pointer chosen, the canvas draws the area of each layer that takes the pointer: a continuous line for the whole box, a dashed one for the threshold (with its percentage) and a bolder one for the chosen layer.
4. Those areas enable clicks and the pointer entering and leaving in timelines, sounds and scripts. The pointer permission is needed. To test it without touching the real screen use [How to test the pointer and clicks](view-and-canvas.md#how-to-test-the-pointer-and-clicks).

In the terminal: `layer` with `touch` (`takes`: `off`, `all` or `alpha`; and `alpha`).


```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"button"}'
reiwand-ctl scene act my-scene layer '{"touch":{"layer":"button","takes":"alpha","alpha":0.3}}'
```

## How to make a template

A template is one or several layers saved aside, which are not seen in the scene. Scripts draw copies of them when they want (see [scripts](../scripts.md)).

Where it is: **Layers** tab, the **Make a template** button, and the **Templates** list in the **Assets** tab.

Steps:

1. Choose a group, image, text, shape, solid color, spectrum or mesh layer, and mark others with Ctrl if you want.
2. Click **Make a template**. The layers are copied into a template; the originals stay in the scene.
3. The template appears in the **Templates** list of the **Assets** tab, with its name and its layers. Each layer is chosen with a click and edited with the inspector.
4. If any script names the template, its name appears as **name · named by** followed by the scripts.
5. Among the buttons of the **Layers** tab a list appears (only if there are templates): **The scene** or **Template X alone**. Choose a template to see it alone on the canvas, as a copy of it is drawn, and edit its layers with the layers panel and the canvas tools. Choose **The scene** to go back.
6. In the **Templates** list each template has three buttons: **Show it alone**, **Rename…** and **Remove**. When renaming, the **Rewrite the exact names in the scripts** checkbox changes the name in the scripts that name it with that exact text; the lines that name it another way are listed and not rewritten.

In the terminal: `template` with `make`, `rename` (with `rewrite`) or `remove`. Seeing a template alone has no equivalent.


```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"star"}'
reiwand-ctl scene act my-scene template '{"make":{"layers":["star"],"name":"spark"}}'
reiwand-ctl scene act my-scene template '{"rename":{"name":"spark","to":"glow"}}'
```

## How to share parts with an assets package

An assets package is a `.wpak` file with layers, particle systems, scripts and effects that other authors can bring into their scene, with the license and credit of each file.

Where it is: **Assets** tab, the **Make an assets package…** and **Import from an assets package…** buttons.

To create a package:

1. First choose the layer you want to share (it is marked by itself).
2. Click **Make an assets package…**. In the dialog fill in **Title:**, **Package id:**, **License:**, **Credit as:** (your name, as other authors will credit you) and **Folder:**.
3. With **Preview:** and **Choose a picture…** you choose an image for the package.
4. The list shows the scene's layers, particle systems, scripts and effects. Check the ones you want. Each checked part accepts a **Title of the part** and a **Thumbnail…**.
5. Click **Make the package**. The scene is saved and a folder is created with the files and the credits. **Open it in the editor** opens that folder; the package is built from there.

To bring parts from a package:

1. Click **Import from an assets package…**. The dialog lists the assets packages installed in the library: choose one. You can also type the path of a `.wpak` file and click **Use path**, or choose it with the file picker.
2. The dialog shows the title, the version, the license and the authors, and each part with its thumbnail and, for each file, its license and its credit.
3. Check the parts you want and accept. The parts and their files are copied into your scene, which then works without the package; the credits and the origin are written by themselves. It is a single history step.

In the terminal: `import_assets` (with `package` and `parts`). The `reiwand-ctl pack new-assets` and `reiwand-ctl pack parts` subcommands do the same as the two dialogs; see [package](../package.md) and [cli](../cli.md).

## How to import files by dragging them

Where it is: over the canvas and over the list of the **Assets** tab.

Steps:

1. Drag files from the file explorer to the window. When passing over, a frame with a text appears.
2. Over the canvas (**Drop files to add them as layers**): each image (png, jpg, jpeg, webp, gif, bmp, ktx2), video (webm, mp4, mkv) or sound (ogg, opus, flac, wav, mp3) is imported and added as a layer. Any other type of file is imported only as an asset.
3. Over the asset list (**Drop files to import them**): everything is imported as an asset, without creating layers.

In the terminal: `add_layer` with `file` (asset and layer at once) or `import` (asset only).

## How to see and manage assets

Where it is: **Assets** tab.

The list shows each asset with a thumbnail and a line: id, type, size and extra data (**HDR**, cube map, number of frames, cover art of what is playing). A click on a row chooses the asset and its inspector. At the end of each row there are buttons.

Steps:

1. **Import…** opens the file picker (images, videos, sounds, fonts, color tables and models) and lets you choose several.
2. **Import a sprite sheet or a data map…** imports an image with its cells or with data for another use. See [drawing](../editor-drawing.md).
3. **Import a script…** brings a JavaScript script file. **New script…** creates one from a template: **Empty module** or modules that follow the player, the pointer, the sound, the time of day, a setting or a launch counter. See [scripts](../scripts.md).
4. On each row: **Rename…** (changes its id and what names it), **Cut sprites…** and **Convert to KTX2…** (images only) and **Remove**.
5. **Remove** warns if the asset is in use: it lists the places that use it and offers **Remove anyway**; those places are left without an image, video or file.
6. **Remove unused files** deletes from the assets folder what the scene does not use.
7. Below the list are also the lists of scripts, templates, package effects and pools.

In the terminal: `import`, `remove` (with the asset's pointer), `rename` with `kind` `asset`, `asset`, `import_script`, `sprites` and `reiwand-ctl scene prune DIR` to delete what is not used.
