# Adjustments and spanning

On this page: how the adjustments are applied and one picture over several screens.

## How the adjustments are applied

Exposure, white balance, contrast, saturation, vignette and blur work in linear light, curves and
overlays (with their `opacity`) in sRGB. The GPU renders an edit once, at the size the output
needs, and dithers the result; nothing is rendered again while the image, the edit and the output
stay the same. Outputs that show the same image with the same edit and depth map (the per-screen
crops of `outputs` and the screens of a `span`) share one copy on the GPU when they need about the
same size, no more than twice it, and a larger copy made for one of them replaces the smaller ones.
JPEG images larger than needed are decoded at 1/2, 1/4 or 1/8 of their size and then reduced, and so
are the thumbnails of the library.
`reiwand-ctl edit export` renders with the same code at the size of the edited image, or at `--size`
after cropping the edited image to that shape. The extension of the file chooses PNG, JPEG
(`--quality`), lossless WebP or AVIF (`--quality`, encoded without native libraries at speed 6),
and `--before` renders the picture with its geometry but without its operations and masks, as
*Compare with the original* shows it in the app, whose *Export…* writes the same files: its panel asks for the quality (1 to 100, 90 filled in), an optional size (width and height) and *Without my edits*, as `--quality`, `--size` and `--before` do.

Every `reiwand-ctl edit` command that changes an edit, and *Save* in the image editor of the app,
keep the edits that came before in
`$XDG_STATE_HOME/reiwand/edit-history/<SHA-256 of the canonical image path>.json` (mode
`0600`, 100 steps and 8 MiB at most, the oldest dropped first), so `reiwand-ctl edit undo` and
`redo` and *Undo* and *Redo* in the app walk through the same steps. In the app a slider or a
handle dragged in one go is one step, and the steps since the last save are kept when it saves.

## Spanning several screens

With the `span` fit, the outputs that use it form one canvas in millimetres. A screen measures its
physical pixels divided by its density (96 dpi times its scale when the density is unknown or
outside 2 to 20 px/mm). Screens that touch in the logical layout touch on the canvas, centred along
the shared edge and `2 × layout.bezel_mm` apart; `[layout.monitors."<identity or output>"]` with
`mm = [x, y, w, h]` places a screen by hand. The edited image covers the canvas, moved by
`span.zoom` and `span.center`, and every output shows its part of it, so a picture keeps its
physical size across screens of different densities. An output without `SetOutputInfo` (a
stream, a preview or a screen the desktop does not place) shows `span` like `fill` until the user
places it: `layout.monitors.<output>` puts it on the canvas like any screen, and
`layout.follow.<output>` makes it show the part of the output it follows. The same holds for the
parts of a spread scene. `GetState().outputs[x].mm` tells where each screen sits on the canvas; the
*Screens* section of the image editor draws them to scale, a screen dragged there is placed with
`layout.monitors`, the outputs without a place get a choice there (the whole picture, placed by
hand on the board, or the same part as another output), and `ShowCalibration` shows a pattern to check the result on the real screens.
