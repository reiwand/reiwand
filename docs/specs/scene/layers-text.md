# Text

A `text` layer also takes these fields:

| Field | Meaning | Feature |
|-------|---------|---------|
| `fallback` | at most 8 more fonts (asset ids or system families) tried in order for characters `font` lacks; only `sans-serif`, `serif` and `monospace` are sure to exist, and content never sees the list of installed fonts | `fonts2` |
| `valign` | `top`, `center` (prefilled) or `bottom`: which edge of the text stays at the layer position while lines come and go (the vertical part of the default `origin`) | `text_layout` |
| `wrap` | `{ "width" (1 to 16384 px), "lines" (1 to 1000), "ellipsis" }` breaks lines at that width (after spaces and hyphens, between the letters of Chinese, Japanese and Korean, and inside a word longer than the line), keeps at most `lines` of them and ends a cut text with an ellipsis; `align` then works within that width | `text_layout` |
| `line_height` | 0.5 to 4 times the size: the distance between two baselines is `line_height` × `size` (it replaces the whole step); without it the step is the font's ascender − descender + line gap, at `size` (no rounding) | `text_layout` |
| `background` | `{ "color", "padding": [x, y], "radius" }`, a box behind the text | `text_layout` |
| `padding` | 0 to 1024 px of margin around the text for its effects | `text_layout` |
| `letter_spacing` | −100 to 1000 px between letters | `text_styles` |
| `advances` | `exact` (absent, today) or `whole`: how far the pen moves after each letter; `whole` cuts each advance down to whole pixels of the layer (`floor(advance · size / (ascender − descender))`) and each kerning toward zero, before `letter_spacing` is added, so the pen after n letters is a whole number of layer pixels | `text_advances` |
| `outline` | `{ "color", "width" (0 to 64 px) }` | `text_styles` |
| `shadow` | `{ "color", "offset": [x, y], "blur" (0 to 64 px) }` | `text_styles` |
| `blur` | 0 to 64 px | `text_styles` |
| `render` | `auto` (prefilled), `raster` or `sdf`, which draws from a distance field so that the size can change without drawing the letters again | `text_styles` with `sdf` |

Outline, shadow, blur and letter spacing may be bound like other values.

Outlines, shadows and blurs are drawn from a distance field: the letters are drawn once, the GPU
spreads their edges into distances with a jump flood (a few passes over a texture of the text, made
once each time the text, its layout or the room of its styles changes), and the shader draws the
fill, the outline, the shadow and the blur from it, so a change of their colors, widths, offsets or
blurs changes only numbers. With `render: auto` a text with a style uses the field at a reference
size (twice its size, from 16 to 256 pixels), so it grows and shrinks without being drawn again;
`raster` draws it again at its size on the screen, as plain texts are, and `sdf` uses the field
even without a style. The texture keeps room around the letters for the widest style, rounded up to
a power of two, so animating a style within that room draws nothing again. At the lowest quality
level shadows blur at most 4 pixels. The GPU makes the field in plain 8-bit textures, which every
GPU can draw into, packing each offset and each distance into two channels (16 bits), so styled
texts look the same on GPUs without 16-bit float textures and the processor never builds it.

A text is laid out again only when its text, size on screen or layout changes; its color, the color of
its background and its position never lay it out again. Each character comes from the first font of
`font` and `fallback` that has it, then from the system `sans-serif`; letters are placed one after
another with the kerning of their font, without joining or reordering them, so scripts that need
shaping (Arabic, Indic scripts) or right-to-left text do not show correctly yet. A font file is at most
32 MiB, also once a `.woff2` is unpacked; a font that cannot be read is replaced by the system one.
The service and the phone take the size of a font in a package from its entry in the archive and do
not read a larger one at all.
`size` is the height in layer pixels from the ascender to the descender of `font` (OS/2 typographic values when
the font asks for them, else the `hhea` ones), not the em: a font of 1000 units per em with an ascender of 900 and a
descender of −250 at `size` 115 has 100 pixels per em. Advances, kerning (the `kern` table, between letters of the
same font) and `letter_spacing` (layer pixels after each letter, not rounded) are in layer pixels at that scale; the
letters are drawn at the size of the screen, which changes nothing of where they are placed in the layer.
The text is drawn in the line box its font declares (its ascent and descent, times `line_height`)
plus `padding`: a letter that reaches beyond that box, such as a tall accent or the letters of a font
that declares its descent above the baseline, is cut at its edge unless `padding` leaves room for it.
