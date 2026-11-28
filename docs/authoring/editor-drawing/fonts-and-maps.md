# Fonts, light maps and sprite sheets

On this page: fonts, lit images and their maps, normal maps, sprite sheets and depth tools.

## How to choose the font and the fallback fonts

**Where it is:** the text layer's inspector, "Font" and "Fallback" fields.

**Steps:**

1. In the "Font" field, click the button with the font's name (it says **Default font** if there is none). A list opens.
2. Type in **Search fonts** to filter. Each name is written in its own font, and below it says where it comes from: **In this package** (fonts you imported), **Whatever the system uses for this style** (generic ones, like serif or monospace) or **Installed on this computer**.
3. Click a font to choose it. The delete button, **Use the default font**, removes it.
4. In the "Fallback" field there is the list of fallback fonts, up to 8. A letter the font does not have is drawn with the first of this list that has it. **Add a fallback font** opens the same selection list. Each row has **Raise**, **Lower** and **Remove**.
5. If a font in the list is no longer in the package or installed, the notice **%1 is not in the package or installed here, so the next font is used** appears, with its name.

For a font to travel with the scene, import it first in the **Assets** tab with **Import…** (it accepts ttf, otf, ttc, otc and woff2 files). The list's preview is drawn by the editor with the family name: a font that resolves differently there than when drawing the scene may look slightly different in the list. Trust the canvas preview.

**In the terminal:** `layer` with `figure` and `fonts`: `font` (an empty text removes it) and `fallback` (list of names).

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"title"}'
reiwand-ctl scene act my-scene layer '{"figure":{"fonts":{"layer":"title","font":"serif","fallback":["monospace"]}}}'
```

## How to turn on light and choose maps

An image, video, text, mesh, model or particles layer can receive the light of light layers. Its maps say how it receives it (roughness, glow and normals).

**Where it is:** the layer's inspector, **Lighting** section.

**Steps:**

1. Turn on the **Take the light of light layers** switch.
2. Choose, in each list, an image from the assets: **Normal map**, **Reflected picture**, **Roughness map** and **Glow map**. **Not set** leaves the field empty.
3. Adjust the **Roughness**, **Reflection**, **Glow** and **Strength** sliders. On particles only **Glow** and **Strength** are there; their normal map is in the particle system.
4. If a map was loaded as a color image, a notice appears: **… is read as colors and shades wrong; pick it again to read it as a map.**

**In the terminal:** `lighting` with `layer`, and `off` to turn it off. The maps are `normal`, `env`, `roughness_map` and `emissive_map`.

## How to paint roughness or glow on the preview

**Where it is:** **Lighting** section, below the sliders.

**Steps:**

1. Click **Paint roughness on the preview** or **Paint glow on the preview** (it turns on like a switch).
2. Adjust **Brush size**, **Hardness** and **Gray**. **Gray** is the value that is written: from 0 to 1.
3. Drag over the layer in the preview. While you paint, the layer is tinted with a heat map of the map you are painting.
4. Release: the stroke is saved as a step. If the layer had no map of its own, one is created with the size of its image (up to 512 on the longer side), white for roughness and black for glow, and it is left chosen in the list.

To drop the brush, click the button again.

**In the terminal:** `paint_map` (`layer`, `map` with `roughness` or `emissive`, `radius`, `hardness`, `value`, `points`); the points are fractions of the layer, from 0 to 1. The `map` query gives the heat map.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"image","id":"background","source":"picture"}'
reiwand-ctl scene act my-scene lighting '{"layer":"background","roughness":0.6,"emissive":2,"strength":0.8}'
reiwand-ctl scene act my-scene paint_map '{"layer":"background","map":"roughness","radius":0.1,"hardness":0.5,"value":0.8,"points":[[0.2,0.2],[0.4,0.4]]}'
reiwand-ctl scene ask my-scene map '{"layer":"background","map":"emissive"}'
```

## How to generate a normal map

**Where it is:** **Lighting** section, **Generate normals…** button (only on image, video and text layers, with light turned on).

**Steps:**

1. Click **Generate normals…**. The title says whether it is generated from the picture, from the letters or from a frame.
2. On a video layer, choose the frame with the slider (**Frame at N s**); **Playhead** takes the second you are at.
3. Adjust **Strength** (from 0 to 16) and **Blur** (from 0 to 32). The window shows three images: the relief, the map and a sphere lit with it. **The relief is lit from the top left; the map is kept as a picture of the package.**
4. Click **Generate**. The map is saved as a picture of the package and chosen in **Normal map**.

**In the terminal:** `normals` (`layer` or `asset`, `strength`, `blur`, `time`). The `normals` query shows the preview without saving anything.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"image","id":"background","source":"picture"}'
reiwand-ctl scene act my-scene lighting '{"layer":"background"}'
reiwand-ctl scene ask my-scene normals '{"layer":"background","strength":3,"blur":1}'
reiwand-ctl scene act my-scene normals '{"layer":"background","strength":3,"blur":1}'
```

The maps of 3D models' materials are painted in another window: see [editor-3d](../editor-3d.md).

## How to cut a sprite sheet

**Where it is:** **Assets** tab, in each image's row, the **Cut sprites…** button (icon only; the help gives the name).

**Steps:**

1. Import the image with **Import…** if it is not in the scene yet.
2. Click **Cut sprites…** in its row. The **Cut the sprites of NAME** window opens.
3. To cut in a grid, fill in **Rows**, **Columns**, **Margin**, **Spacing** and **Frames (0 for all)**, choose the order (**Left to right, top to bottom**, **Top to bottom, left to right**, **Backwards, by rows** or **Backwards, by columns**) and click **Cut the grid**.
4. To draw a frame by hand, drag over the image. Drag a frame to move it and its corner to resize it. Each frame carries its number.
5. Below there is the strip of frames. Choose one and use **Earlier** and **Later** to order it, **Remove the frame** to delete it. **Remove all frames** deletes them all.
6. The strip's preview plays the frames: **Play** and **Pause**. **Frames per second** sets the speed and **Frames played** limits the animation to the first ones. The extra frames are seen dimmed.

Up to 256 frames per image. The scene shows those frames on animated image layers and in particles.

**In the terminal:** `sprites`, one of these variants: `grid`, `rects`, `add`, `change`, `remove`, `reorder`, `play` or `clear`. Each rectangle is `[x, y, width, height]` in image pixels. `import` also accepts `atlas` for a sheet of equal cells.

```sh
reiwand-ctl scene act my-scene import '{"file":"sheet.png"}'
reiwand-ctl scene act my-scene sprites '{"grid":{"asset":"sheet","rows":1,"columns":4}}'
reiwand-ctl scene act my-scene sprites '{"add":{"asset":"sheet","rect":[0,0,50,50]}}'
reiwand-ctl scene act my-scene sprites '{"change":{"asset":"sheet","index":0,"rect":[2,2,60,60]}}'
reiwand-ctl scene act my-scene sprites '{"reorder":{"asset":"sheet","from":0,"to":1}}'
reiwand-ctl scene act my-scene sprites '{"play":{"asset":"sheet","fps":12,"frames":3}}'
reiwand-ctl scene act my-scene sprites '{"remove":{"asset":"sheet","index":0}}'
```

## How to open an image's depth tools

With depth, an image is separated into layers that the parallax moves at different rates. The layers are cut by regions: the connected parts of the image between one depth jump and another. That way a continuous surface, like a floor, is never split or stretched. These tools are not in the scene editor but in the image editor.

**Where it is:** in the library, open an image and click **Edit…**; in the image editor's tool column choose **Depth**.

The editor then shows the whole image, without rotating or cropping, so that what you paint matches the original. The other tools (crop, rotate) are seen when you go back to them.

The changes are image editor steps: **Undo** and **Redo** (with the system's standard shortcuts) work for each stroke, each change of cuts and each mark.
