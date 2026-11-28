# Problems and painted masks

On this page: the problems list, messages and masks painted per channel.

## Problems and messages

- **Problems**: the validator's messages change with each edit; clicking one chooses the affected
  layer or field. Those messages and the editor's rejections carry an identifier with their
  arguments (`scene.color`, `editor.step.add_layer`…) and come out in the desktop's language in the
  app and in `reiwand-ctl`, and in the phone's when it shows the same error; the starting points'
  layers (`sky`, `sun`, `flakes`…) and the undo steps are shown translated without changing the
  stored ids. A starting point created in a language that its labels do not have uses that language,
  the title's, as the default language and copies the English labels into it to be translated.
  Below, the scripts' console shows the last 50 lines they wrote in the preview (`reiwand-ctl state`
  gives them in `console` of `previews`, under the output name `preview:`).

## Painted masks

The scene editor also paints the masks of the effects that ask for them per channel: next to each
mask parameter there is a button to paint, the channel (red, green, blue or alpha), an erase mode
and the brush size, and each stroke on the layer in the preview is saved as a new image in
`masks/`, so that undoing returns to the previous one (`reiwand-ctl scene act DIR paint_mask` does
the same from the terminal). If the starting mask, map or image is KTX2, the new image is too, with
the same codec and the same quality, instead of a PNG. While the app stays open, each stroke starts
from an exact copy of the previous stroke, so the compression loss does not accumulate stroke by
stroke; from the terminal each call starts from the file, so there it is better to paint on PNG and
convert at the end. When you choose a layer's effect, the inspector offers its mask: none, an
image, another layer or a shape (rectangle, ellipse, ring, polygon, star or line), the channel,
invert, the soft edge (with a slider of up to 200 px and a field of up to 10,000) and the strength,
and the canvas draws it over the layer: the image on top (also a KTX2, which the app decompresses
once into a PNG in its cache), the masking layer framed and the shape with its outline, the soft
edge inside and out in dashes, a handle to size it (or to move the line's end) and a point next to
the edge to soften it; each drag is an undo step and follows the `effects mask` action, as
`reiwand-ctl scene act DIR effects`.
