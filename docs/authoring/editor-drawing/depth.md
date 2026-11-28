# Depth maps and cuts

On this page: the depth map, its brush, the layer cuts and fills.

## How to generate or import the depth map

**Steps:**

1. In **Depth**, the image has no map yet: white is near and black is far.
2. Choose in **Run the model on** whether it runs on **The fastest device**, **The processor** or the graphics card.
3. Click **Generate the map**. The first time it asks **Download the depth model?**: the map is computed on this computer and the image does not leave it; **Download** fetches the files only once, or **Install from a file…** uses files you already have. If downloads are turned off in the settings, only that second option is left.
4. While it works, the progress is seen (**Downloading the model…**, **Loading the model…**, **Estimating depth…**, **Saving the map…**) and the **Cancel** button.
5. If you already have a finished map, use **Import a map…**. **Save next to the picture** saves it as a file next to the image. **Remove the map** deletes it.
6. Adjust the map with **Near level**, **Far level**, **Curve**, **Smoothing (px)** and **Grow near edges (px)**. The **False colour, red near and blue far** checkbox changes how the map looks in the window.

The following tools appear as soon as there is a map.

**In the terminal:** `reiwand-ctl edit depth` (see [cli](../cli.md)): `generate`, `import`, `remove`, `export`, and the options `--near`, `--far`, `--gamma`, `--smooth`, `--dilate` on the image.

## How to see the map over the image

**Where it is:** **Depth over the picture**.

**Steps:** turn on **Show the depth map over the picture**, adjust **Opacity** and choose with **False colour, red near and blue far** whether it is seen in grays or in color. The map that is drawn already includes your strokes.

## How to touch up the map with the depth brush

**Where it is:** **Depth brush**.

**Steps:**

1. Click **Paint depth**.
2. Choose **Raise** (brings closer) or **Lower** (moves away).
3. Adjust **Brush size**, **Strength** and **Soft edge**.
4. Drag over the image. The brush circle and the stroke are drawn; when you release, a step is saved.
5. Below you read how many strokes there are (**N strokes**), with **Undo** (removes the last) and **Remove the strokes** (removes all).

**In the terminal:** `reiwand-ctl edit depth brush FILE "X,Y;X,Y" [--raise|--lower] [--by] [--radius] [--falloff] [--clear]`. The coordinates are fractions of the original image.

```sh
reiwand-ctl edit depth brush picture.png '0.2,0.2;0.4,0.3' --raise --by 0.2 --radius 0.05 --falloff 0.5
reiwand-ctl edit depth brush picture.png '0.5,0.5' --lower
reiwand-ctl edit depth brush picture.png --clear
```

## How to change the layer cuts

**Where it is:** **Layer cuts**.

The strip shows the depth histogram, **Far** on the left and **Near** on the right, colored by layer, with a handle for each cut. Below it says **N layers, cut by the picture itself** or **N layers, cut where you set the handles**, followed by **N regions** when the image already has layers. The cuts group the regions into layers by their average depth.

**Steps:**

1. Drag a handle to move a cut. While you drag, the layers' outlines are seen over the image, and when you release the layers are rebuilt half a second after the last change.
2. Double-click a handle to remove the cut, or click on the strip to add one. **Add a cut** puts it in the widest gap. There are at most 5 cuts and each one is at least 0.01 from its neighbor.
3. **Remove the cut** deletes the chosen one (or the last). **Back to automatic** lets the image itself decide.
4. **Show the layers on the picture** paints each layer's outline tinted.

The window's help says: **Drag a handle to move a cut, double-click to remove it and click on the strip to add another.**

If the image stays still, a red line says why (see below).

**In the terminal:** `reiwand-ctl edit depth cuts FILE 0.3,0.6` or `--auto`.

```sh
reiwand-ctl edit depth cuts picture.png 0.3,0.6
reiwand-ctl edit depth cuts picture.png --auto
```

## How to decide what gets filled behind near objects

When a near layer moves, it uncovers a bit of what was behind it; that area is filled. The fill brush marks where.

**Where it is:** **Fill brush**.

**Steps:**

1. Click **Paint fill marks**.
2. **Refill** marks areas that are always filled; **Protect** marks areas that are never filled (they stay transparent in the layers behind).
3. Adjust **Brush size** and **Soft edge**. There is no strength here: a mark counts in full where it covers at least half of the brush.
4. Drag over the image. A later mark overrides the earlier one where they cross. When you release, the layers are rebuilt.
5. **N marks** counts the marks; **Remove the marks** deletes them all.

**In the terminal:** `reiwand-ctl edit depth fill-mask FILE "X,Y;X,Y" (--refill|--protect) [--radius] [--falloff] [--clear]`.

```sh
reiwand-ctl edit depth fill-mask picture.png '0.5,0.5;0.6,0.6' --protect --radius 0.08
reiwand-ctl edit depth fill-mask picture.png --clear
```

## How to see a layer and compare fills

**Where it is:** **Layers and fill**.

**Steps:**

1. Choose a layer in the list: **None**, or **Layer 1, the farthest**, **Layer 2**… up to **the nearest**. The image shows that layer alone, with its transparency. If the set of layers is not built, it is built then, with a progress bar and **Cancel**.
2. **Highlight the band that moving can uncover** tints the pixels that were filled.
3. **Compare LaMa with the simple fill** splits the image: on the left the LaMa fill and on the right the **Simple fill**. **Where LaMa ends** moves the split (the line can also be dragged). It is only available if LaMa is installed; otherwise it says **LaMa is not installed, so there is nothing to compare. Download it in the settings.**

**In the terminal:** `reiwand-ctl edit depth layers FILE` builds the layers, `show FILE --layer N --out PNG [--band PNG]` saves one and `compare FILE --out PNG` saves the comparison. In the app's settings there are **Check LaMa**, **Remove LaMa** and **Rebuild the layers**; in the terminal, `reiwand-ctl models verify` and `models remove`.
