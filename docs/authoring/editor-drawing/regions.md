# Regions and the parallax model

On this page: regions, how the image moves and downloading the model.

## How to join, split and move regions

**Where it is:** **Parallax**, **Regions** group (the image must have a depth map).

A region is a part of the image between depth jumps; it moves as one piece and goes whole to a layer. The tool paints each region over the image, from blue (the farthest) to red (the nearest), with a number that is its place in the depth order: 1 is the nearest.

**Steps:**

1. Click regions on the image, or mark them in the list, to choose them; another click releases them.
2. **Join** makes a single region of the chosen ones: they share a layer and move together. It is useful when one thing was split into pieces that should not be separated.
3. **Nearer** and **Farther** move the chosen regions in depth, as much as **Move** says.
4. To cut a region in two, turn on **Split along a stroke** and draw across it; when you release, it is cut. Each part can go to a different layer.
   Without a mouse: choose a region, turn on **Straight split of the picked region** and move **Where to cut** with the arrows; the dashed line follows the longer side of the region. **Cut along the line** cuts it.
5. **Undo** and **Redo** work for each join, cut and move. **Put the regions back** removes them all.
6. **Preview the movement while you edit** (when the layers are built) shows the image moving: the pointer over the image, the box's point or the **Orbit** act as the pointer. The arrows move the point when the box has focus.

A join, split or move that no longer applies (because the image, the map or another mark changed: its two points fall in one region, the stroke crosses none or the region has already been moved or joined) is not applied and does not break the layers; the tool and `reiwand-ctl edit depth regions` report it with the mark's number.

If a change lowers the movement limit, the tool reports it with the change that caused it and the limit from before and now; less movement keeps the image from stretching. At the top, a bar shows how the building of the layers is going, in the app and on the desktop.

**In the terminal:**

```sh
reiwand-ctl edit depth join picture.png "0.25,0.5;0.7,0.5"
reiwand-ctl edit depth split picture.png "0.0,0.5;1.0,0.5" --radius 0.004
reiwand-ctl edit depth move picture.png 0.25,0.5 --by -0.2
reiwand-ctl edit depth move picture.png --clear
reiwand-ctl edit undo picture.png
```

Before those commands, `reiwand-ctl edit depth regions picture.png` (with the depth map already made) lists the regions from the nearest, each with a point inside: that point is the one given to `join` and to `move`. `--png` saves the painted regions and `--json` prints the list as JSON. With `edit depth layers picture.png --list` you see the movement limit of each set of layers.

## How the image moves and why it sometimes stays still

Each layer moves with its own smooth depth field, and the total movement has a limit: it never goes beyond what the layers cover, so the edge of the background is not seen and no piece of the image is stretched. With a high strength, the limit trims the displacement instead of deforming the image. The layers are built at the screen's resolution (the app and `reiwand-ctl` build them at 1920 at most, and so does the phone).

While the layers are not ready, or if they cannot move it without stretching it, the image stays still. The reason is read in the red line of **Layer cuts**, in the **Parallax** tool and in `reiwand-ctl edit depth layers FILE` (on the error output; `--list` also shows the regions, the safe displacement and whether each set moves). The reasons are: there is no depth map, there is no depth model, the layers are being built, they could not be built, or their depth does not allow moving it without stretching it. The daemon says the same in `GetState`, in the `still` field of each output.

**Images with color operations.** An image with active color operations moves the same as an unedited one: each layer is revealed with the same operations, once when it is shown (no per-frame work), and editing the operations does not rebuild the layers. A crop, rotation or flip already rotates the layers. A blur operation also blends the transparent edge of each layer, so around the outline of a near object a very faint halo may be seen.

**Layer limits.** On sharp cut-outs and on hair, region layers come out somewhat less clean than a simple cut, even with an exact depth map; with an estimated map the result is the same either way, because the limit is the estimate. Very fine hair beyond the band of about 4 pixels next to the edge is left as a faint fixed outline, and a semi-transparent edge (a flower petal) may leave tiny gaps. The movement preview is an approximation computed on the processor at 480 pixels, not compared with what the graphics card draws; the region map is computed at 512 pixels, so a very small region may differ from the built layers.

## How to download the model from the parallax tool

**Where it is:** **Parallax**, in **Details > Edit picture**.

If you turn on the parallax and the depth model is missing, the tool immediately shows **The picture stays still: the depth model is not installed** with the download size, and the **Download the depth model…** button. **Nothing is downloaded until you ask**: the button opens the **Download the depth model?** consent window, and afterwards the progress is seen with **Cancel**. If the model is already there but the image has no map, the button is **Generate the map**. The phone shows the same notice with a download button.

**Fill and layers of each image.** In the **Parallax** tool, **Smart fill** or **Simple fill** chooses how what a near object leaves uncovered is painted, and **Depth layers** (4, 5 or 6) how many layers are computed. They belong to that image: two images can use different fills, they are saved and synced with the rest of their settings and are undone with them. If an image should not move, turn off **Move the picture in depth**. If smart fill is not installed, the simple one is used and the notice offers to download it.

**In the terminal:** `reiwand-ctl edit parallax FILE --fill lama|simple --layers 4..6` sets the image's fill and layers. Without settings, `reiwand-ctl edit parallax FILE` prints the notice with the size and the command `reiwand-ctl models download depth-anything-v2-small`.
