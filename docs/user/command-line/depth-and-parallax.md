# Depth maps and parallax

A still image can move in depth (parallax), guided by a gray depth map. This page covers how
parallax moves, where the depth map comes from, and the depth model. How the image is split into
layers is in [3D parallax layers](parallax-layers.md).

## How parallax moves

With a gray depth map (white near), the daemon shifts the near more than the far. It follows one of
these sources:

- a slow orbit;
- the pointer;
- the computer's accelerometer.

It moves only while the wallpaper is seen, at the pace the screen allows, with the same pauses as
video. It stays still if the system asks to reduce motion.

Choosing the pointer as the source for an image is your permission to follow it while it is seen,
as in scenes.

With software rendering (no GPU) the image stays still unless you turn on
`playback.software_parallax`, which moves it at 15 frames per second at most.

## Where the depth map comes from

The map can be:

- **your own**: `reiwand-ctl edit depth import`, with the same proportion as the image;
- **computed on the computer itself** with Depth Anything V2 Small (Apache-2.0) through ONNX
  Runtime, in a separate process and never inside the daemon. Only the layers (see
  [3D parallax layers](parallax-layers.md)) are computed in a background job of its own.

The most recent map wins, and the cache of computed maps does not exceed 256 MB.

## Downloading the model

Neither the model (99 MB) nor ONNX Runtime (11 MB) are included. They are only downloaded from
GitHub when you ask for them:

1. Size, license and servers are shown first.
2. The files are checked with their pinned SHA-256.
3. They are stored alongside the full text of their license (and ONNX Runtime alongside the
   third-party notices of its archive).

The image never leaves the computer.

| Setting or command | Effect |
|--------------------|--------|
| `models.allow_download = false` | prevents any download |
| `reiwand-ctl models install` | installs files obtained another way |

## Computing the map

In the app, when the download ends the map is computed by itself. It is seen already adjusted
(near, far, curve, smoothing and edges), in gray or false color.

The model runs on the processor or, with an ONNX Runtime built with CUDA (for example given in
`ORT_DYLIB_PATH`), on the NVIDIA graphics card. *The fastest device* uses it when it can and if it
fails goes back to the processor.

```sh
reiwand-ctl models list                                 # size, license, server and state
reiwand-ctl models download depth-anything-v2-small     # asks before downloading
reiwand-ctl edit depth generate photo.jpg               # map computed here; Ctrl-C cancels
reiwand-ctl edit depth generate photo.jpg --device cuda --bench   # measure the model on the graphics card
reiwand-ctl edit depth export photo.jpg map.png --adjusted --colour   # the adjusted map in false color
reiwand-ctl edit depth photo.jpg --dilate 4 --smooth 2   # adjust near, far, curve and edges
reiwand-ctl edit depth remove photo.jpg                  # delete the computed map, the imported one and the one next to it
reiwand-ctl edit parallax photo.jpg --source pointer --strength 0.02 --focus 0.3
```
