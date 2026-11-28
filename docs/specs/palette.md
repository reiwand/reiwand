# Interface palette

The `reiwand-palette` crate computes the interface tones from the wallpaper in use. It is a pure
module: it does not decode images and uses no dependencies; `derive` allocates no memory (everything
lives on the stack).

## The tones

All come from the wallpaper, except the four fixed ones. The interface is always dark and text goes
over Night.

| Tone | Field | Use |
|---|---|---|
| Night | `night` | Deep tone of the wallpaper; surfaces and panel background |
| High night | `night_high` | Raised panels |
| Night line | `night_line` | Lines and borders |
| Mist | `mist` | Secondary text |
| Low mist | `mist_low` | Footers and help |
| Moon | `moon` | Accent at rest |
| Deep moon | `moon_deep` | Pressed accent |
| Veil | `veil` | Intensity of the veil under the text (0.62 to 0.96) |
| Frost, Amber, Coral, Sprout | `FROST`, `AMBER`, `CORAL`, `SPROUT` | Fixed: main text and states |

## How they are derived

Work is done in OKLab. The hue of Night is that of the darker half of the image (if that half is
almost gray, that of the whole image; if there is no hue, blue). Its chroma is that of that half
times 1.3, between 0.014 and 0.040. Night, High night and Night line have fixed lightness (0.185;
0.235; 0.345). Mist and Low mist use the same hue with lightness 0.81 and 0.69 and almost no chroma.
The accent takes the most present hue: pixels with lightness greater than 0.30 weigh their chroma
raised to 1.5, are split into 36 hue sectors, smoothed with the neighbors, and the winning sector
is averaged with its two adjacent ones; if nothing weighs, it uses the hue of Night. Moon and Deep
moon have lightness 0.82 and 0.71. Each color is lowered in chroma until it fits in sRGB. The veil
is 0.50 plus 0.55 times the mean lightness, between 0.62 and 0.96, rounded to two decimals.

## The light set of the editor

The editor may be drawn light, as a preference of its own (`paper`, in `paper.rs`): from the dark
set of the same picture it keeps the hue of Night for the surfaces, as paper (Night lightness
0.975, High night 0.945, Night line 0.85, chroma 0.012), Mist and Low mist as soft ink (0.42 and
0.47, chroma 0.02) and the accent's hue, darker (Moon 0.46 and Deep moon 0.38, chroma at most
0.12). Text that is Frost on a dark set is ink, `#1c1a17`. Every text tone reads at 4.5:1 or more
on Night and High night, and Night on Deep moon too. The rest of the app never uses this set.

## Guarantees

Since the lightness is fixed, it holds for any wallpaper: Night is never light, Frost over Night has
a contrast of at least 12, Mist over Night at least 8 and Low mist at least 4.5. Without a
wallpaper, `fixed()` gives the fixed set.

## Use

`derive(pixels, channels, width, height) -> Option<Palette>`: RGB or RGBA bytes (alpha is ignored).
The caller reduces the poster to about 96 × 54 pixels; the function reads 8192 pixels at most (if
there are more, it takes one in every n). It returns `None` if the data do not add up. `Rgb` prints
as `#rrggbb`. Tests against an independent reference calculation in
`crates/reiwand-palette/tests`, with a tolerance of one step per channel.

## Main colors of an image

`dominant(pixels, channels) -> Option<[Rgb; 3]>`: RGB or RGBA bytes (pixels with alpha below 128 are
ignored). It groups the pixels into three groups (k-means in OKLab, 12 rounds at most) and returns
the center of each, from the largest group to the smallest. It is the project's only extractor: it
is used by the color and hues of the library thumbnails (`reiwand-render`), the colors of the cover
in progress (`reiwand-scene`, also for the editor) and the Android app. A single-color image returns
that color three times. `oklab` and `from_oklch` are the only conversion to OKLab; the color
blending of scenes uses them too. It returns `None` if no visible pixel remains.
