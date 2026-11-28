# 3D parallax layers

*3D parallax computed once per image.* An image only moves in layers, without stretching anything.
This page explains how the layers are built and filled, the commands to inspect them, and the
author tools in the editor. Depth maps themselves are covered in
[Depth maps and parallax](depth-and-parallax.md).

## How the layers are built

The daemon prepares a set of layers of the image once, in the background, and never per frame. It
runs as a `Layers` job with its progress, which shows in `reiwand-ctl jobs`.

1. It refines the map with a filter guided by the image itself.
2. It looks for the depth jumps and splits the image into connected regions between them. The fine,
   like a lock of hair, stays with its object; small pieces join the neighbor with which they share
   the most edge.
3. It distributes whole regions into 4 to 6 layers (`layers` of the edit of each image, 5 by
   default) according to their average depth, so that a continuous surface is never cut.
4. Inside each layer, the depth is smoothed and its slope limited so that nothing stretches more
   than 4 % with the chosen strength. A floor, a bridge or a wall move with their depth, and what
   does not fit in that limit is flattened toward a rigid card.

## Edges and fill

The edge of each near object takes the transparency of the image itself, without strips of the
background. Behind it, the strip that the movement can uncover is marked. It is filled without
copying the edge of the object, according to the `fill` of the edit of each image:

| `fill` | Method |
|--------|--------|
| `lama` (the default on the desktop) | uses the LaMa model on 512 px crops at the resolution of the image |
| `simple` | extends the nearest background by pyramid |

Both are chosen per image in the parallax panel (*Fill*, *Depth layers*) and on the phone.

### The LaMa model

LaMa (big-lama in ONNX, 208 MB, Apache-2.0) is not included. It is downloaded with
`reiwand-ctl models download lama-big-fp32` (or from the app's settings) after showing size, license
and server, with the same rules as the depth model. Without it, or without ONNX Runtime, `simple` is
used.

It runs on the processor, or on CUDA if `ORT_DYLIB_PATH` points to an ONNX Runtime with CUDA, with
minimum priority and at most four threads.

## Resolution and cache

The set is computed at the resolution the screen asks for: 1024, 1920, 2560 or 3840 px on the
longer side.

It is stored in the map cache (`~/.cache/reiwand/depth/<SHA-256>-<key>.layers.png`, `.fields.png`
and `.layers.json`, within the same 256 MB). The key gathers the image, the model, the fill, the
number of layers, the strength, the resolution and the settings of the map.

## Movement

- The movement is clipped per image so as never to exceed the filled strip.
- The enlargement is computed with the strength, the focus and the shape of the image so that the
  edge of the frame is never seen.
- When moving, the GPU draws the layers from back to front, and at rest shows the original image.
- A crop, rotation, straightening or flip also turns the layers.

The image stays still while the layers are being computed, if they fail, or if their depth does not
allow moving it without stretching. The app, `reiwand-ctl edit parallax`,
`reiwand-ctl edit depth layers` and the phone say why in one line.

> **Note:** If the depth model is missing, they say so when turning on parallax, with its size and
> the button or the command that downloads it. It is never downloaded by itself.

## Commands

```sh
reiwand-ctl models download lama-big-fp32                          # the LaMa fill (208 MB)
reiwand-ctl edit depth layers photo.jpg                            # compute the layers with the settings
reiwand-ctl edit depth layers photo.jpg --fill simple --layers 4 --force
reiwand-ctl edit depth layers photo.jpg --list                     # the layer sets of the image
reiwand-ctl edit depth layers photo.jpg --clear                    # delete them
reiwand-ctl edit depth show photo.jpg --layer 0 --out layer.png    # inspect a layer
reiwand-ctl edit depth show photo.jpg --layer 1 --out layer.png --band strip.png   # and its strip to fill
reiwand-ctl edit depth compare photo.jpg --out cmp.png --layer 1   # LaMa on the left, simple fill on the right
reiwand-ctl edit depth brush photo.jpg "0.3,0.4;0.5,0.4" --raise --by 0.2 --radius 0.05   # depth brush
reiwand-ctl edit depth cuts photo.jpg 0.2,0.45,0.7                 # layer cuts by hand (--auto goes back to the automatic ones)
reiwand-ctl edit depth fill-mask photo.jpg "0.5,0.5" --protect     # areas that are always (--refill) or never (--protect) filled
reiwand-ctl edit parallax photo.jpg --fill simple --layers 6       # fill (lama or simple) and layers of this image
```

## Author tools

*Author tools for the layers.* On the edit page of an image with a depth map, the *Depth* tool shows
the whole image, unrotated. Under the map settings, it brings the tools with which the author
decides how the image is split and filled. Each has its command in `reiwand-ctl edit depth` and
supports undo.

| Tool | What it does | Command |
|------|--------------|---------|
| *Depth over the picture* | Shows the adjusted map, with the strokes, on top of the image, in gray or in false color and with an opacity control. | |
| *Depth brush* | Raises or lowers the depth with a radius, a strength and a soft edge to choose. Each stroke is a step of the history and is applied to the adjusted map. | `edit depth brush` |
| *Layer cuts* | Over the depth histogram, the cuts are handles that can be dragged (a double click removes one, a click on the strip adds another), while the image shows the tinted outline of each layer live. On release the layers are computed again with those cuts, which automatic grouping no longer overrides until *Back to automatic* is pressed. | `edit depth cuts` |
| *Fill brush* | Paints on the image what must always be filled behind a near object (even if it falls outside the reach of the displacement) or what is never filled. The layers are recomputed on release. | `edit depth fill-mask` |
| *Layers and fill* | Choose a layer to see it alone, with the strip that the movement can uncover highlighted, or compare the LaMa fill on one side of a divider that can be dragged and the simple one on the other. What is missing is then computed, with its progress, and the comparison is only offered with LaMa installed. | `edit depth show --band`, `edit depth compare` |

Layers are recomputed by themselves half a second after the last change while the tool is open, and
a new change cancels the computation in progress.

## Layer settings

In *Settings*, next to the LaMa download, there are:

- buttons to check it (`models verify`) and remove it (`models remove`);
- *Rebuild the layers*, which deletes the stored layer sets and makes the daemon prepare again those
  of the images it shows. The button indicates how many sets are stored or how far the daemon's
  computation is.
