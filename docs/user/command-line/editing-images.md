# Editing images

Image edits are non-destructive: they never touch the image file. This page describes the editor in
the app, the `reiwand-ctl edit` commands, and spanning an image over several screens.

## Where edits are stored

Edits are saved in `<image>.reiwand.json` next to the image, or in `~/.local/share/reiwand/edits/`
if the folder cannot be written. The GPU applies them, and the daemon reads them again as soon as
they change.

## The editor in the app

The app edits them from the card of each image ("Edit…"). The window has three parts:

| Part | Content |
|------|---------|
| Left | The sections, always in view, each with its icon; below them, the *Operations, in order*. The arrows go through the list. |
| Middle | The controls of the chosen section. |
| Right | The edited image. |

The sections are *Crop and rotate*, *Screens*, *Light and color*, *Curves*, *Effects*, *Masks*,
*Depth* and *Parallax*.

### The edited image

The app itself draws the image on the GPU, with the same development as the daemon and
`reiwand-ctl edit export`. It is reduced (1024 pixels per side at most, 512 with software rendering)
and drawn only when a value changes or a new depth map arrives.

- If it cannot draw it, a notice says why.
- Another notice warns if the image changed after editing it. `reiwand-ctl edit` and the daemon log
  say so too.

### Sections that work on the image

- **Crop and rotate**: the whole image is seen with the crop on top, to move it and change its
  size.
- **Masks**: linear and radial masks are placed by dragging their handles over the image. Brush
  masks are painted over it, with their size, their hardness and *Erase*. Each stroke is a step that
  can be undone.
- **Screens**: shows the crop of each screen over the result, and the screens drawn to scale in
  millimeters. Drag them to where they really are, with the width of their bezels. A pattern of
  lines and circles over the real screens lets you check it: the lines continue straight from one
  screen to another.

### Editor actions

| Action | What it does |
|--------|--------------|
| *Undo* and *Redo* (Ctrl+Z and Ctrl+Shift+Z) | go through the same steps as `reiwand-ctl edit undo` and `redo` |
| *Compare with the original* (the `\` key) | shows the image without operations |
| *Export…* | saves the result as PNG, JPEG, lossless WebP or AVIF |
| *Show on the desktop* | also takes the unsaved edit to the real wallpaper, wherever that image is the wallpaper; the window says so under the image |

## Editing from the terminal

Each action of the editor also exists in `reiwand-ctl edit`, which changes them without the daemon.

Light, color and crop:

```sh
reiwand-ctl edit set photo.jpg ev=0.3 contrast=10 temperature=5200 tint=4
reiwand-ctl edit crop photo.jpg 0.1,0.05,0.8,0.9 --rotate 90
reiwand-ctl edit crop photo.jpg --aspect 16:9 -o DP-1      # crop of its own for a screen
reiwand-ctl edit overlay photo.jpg '#ff8800' --blend soft-light --opacity 0.3 --mask linear:0.5,0,0.5,0.5
```

Preview, export and history:

```sh
reiwand-ctl edit preview photo.jpg ev=1                  # see it on the desktop for 60 s without saving
reiwand-ctl edit export photo.jpg -o edited-photo.png    # same result, without the daemon
reiwand-ctl edit export photo.jpg -o photo.avif --quality 70   # or .jpg and .webp; --before without operations
reiwand-ctl edit undo photo.jpg                          # and redo, also after changes made in the app
```

Masks and fit:

```sh
reiwand-ctl edit set photo.jpg ops.bl.mask=sky           # give a mask to an operation; empty removes it
reiwand-ctl edit mask photo.jpg face 'brush:0.04,0.6:0.41,0.33;0.43,0.35'   # a brush stroke; erase: erases
reiwand-ctl edit fit photo.jpg center                    # its own fit; none goes back to that of the screen
```

## Spanning and screen layout

```sh
reiwand-ctl set photo.jpg -f span && reiwand-ctl edit span photo.jpg --zoom 1.2
reiwand-ctl config layout.bezel_mm 8                    # width of the bezel of each screen when spanning
reiwand-ctl config layout.monitors.DP-1 0,0,597,336     # place a screen by hand, in millimeters
reiwand-ctl config layout.follow.STREAM-1 DP-1         # an output without a location shows the part of DP-1
reiwand-ctl calibrate on                                # pattern to check it; off removes it
```

Depth maps and parallax are covered in [Depth maps and parallax](depth-and-parallax.md).
