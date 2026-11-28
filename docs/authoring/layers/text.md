# Text layer (`text`)

```json
    {
      "blend_space": "canvas", "id": "title", "type": "text",
      "text": { "en": "Good morning", "de": "Guten Morgen" }, "font": "sans-serif", "size": 160,
      "color": "#ffffff", "align": "center", "outline": { "color": "#3b4a6b", "width": 6 },
      "shadow": { "color": "#00000099", "offset": [6, 8], "blur": 12 },
      "transform": {
        "position": [960, 300], "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0],
        "skew": [0.0, 0.0]
      },
      "blend": "normal", "cache": false, "depth_test": false, "effects": [], "fallback": [],
      "line_height": null, "opacity": 1.0, "padding": 0, "parallax": [0, 0], "render": "auto",
      "solid": false, "valign": "center", "visible": true, "wrap": null
    }
```

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `text` | required | | A text, a text per language (`{ "en": "Good morning", "de": "Guten Morgen" }`, which asks for `text_languages`) or a value: `{ "clock": "HH:mm" }`, `{ "date": "EEE d MMM yyyy" }`, `{ "prop": "message" }`, `{ "media": "title" }`. |
| `font` | the system's | | The id of a font asset, or a system family (`sans-serif`, `serif`, `monospace`). |
| `fallback` | none | up to 8 | More fonts for the characters that `font` does not have. |
| `size` | 48 | 1 to 1024 px | The size of the letters: pixels from the top of the font's tallest letters to the bottom of its lowest (ascender to descender), not the em. |
| `color` | white | | The color. |
| `align` | `center` | `left`, `center`, `right` | Alignment of the lines. |
| `valign` | `center` | `top`, `center`, `bottom` | Which edge of the text stays at the position while the lines change. |
| `wrap` | not broken | `width` 1 to 16384 px, `lines` 1 to 1000, `ellipsis` | Breaks lines at the given width, leaves at most `lines` and ends a cut text with an ellipsis. |
| `line_height` | | 0.5 to 4 times the size | The line height. |
| `letter_spacing` | 0 | -100 to 1000 px | Space between letters. |
| `advances` | `exact` | `exact`, `whole` | `whole` moves the pen after each letter by its advance cut down to whole pixels (kerning cut toward zero), so each letter starts on a whole pixel of the layer. |
| `background` | no box | `color`, `padding` `[x, y]`, `radius` | A box behind the text. |
| `padding` | 0 | 0 to 1024 px | Margin around, for effects and for letters that stick out of the line. |
| `outline` | no outline | `color`, `width` 0 to 64 px | An outline. |
| `shadow` | no shadow | `color`, `offset` `[x, y]`, `blur` 0 to 64 px | A shadow. |
| `blur` | 0 | 0 to 64 px | Blurs the text. |
| `render` | `auto` | `auto`, `raster`, `sdf` | How it is drawn: `sdf` uses a distance field so the size can change without redrawing the letters; `raster` draws again at the size it has on screen. |

`letter_spacing`, `outline`, `shadow` and `blur` can follow values. The text is drawn in the line
box that its font declares (what rises above and what drops below the baseline, times
`line_height`) plus `padding`. A letter that sticks out of that box, like a tall accent or the
letters of a font that declares wrongly what drops below, is cut at the edge if `padding` leaves it
no room. A text is composed again only when the text, the size on screen or the composition change;
its color and position do not. Each character comes from the first font of `font` and `fallback`
that has it, and if none does, from `sans-serif`. Letters are placed one after another with the
spacing of the font, without joining or reordering: right-to-left writing and scripts that need
joined forms (Arabic, scripts of India) do not look right. A font that cannot be read is replaced
by the system one.

A text per language works like the package title and the labels of the properties: each key is a
language (`de`, `pt-BR`), the text of the main language of the package (`default_locale` of the
manifest) is required and is used when the viewer's language is not there. The full language
(`de-AT`) is chosen first and then its base (`de`). A text that follows a value (clock, date,
property) has no languages; a script that sets the text of the layer sets it in all languages, and
a script that reads it receives that of the language being shown.

Clocks and dates are explained in [scene.md](../scene.md). The example
[`text`](../examples/text/scene.json) carries text with outline, shadow, box, line wrapping, a title
in English and German, time and date:

```json
    {
      "blend_space": "canvas", "id": "paragraph", "type": "text",
      "text": "A long text breaks into lines of the width you choose and ends with an ellipsis if it does not fit.",
      "font": "serif", "size": 44, "color": "#dbe4f5", "align": "left", "valign": "top",
      "line_height": 1.3, "letter_spacing": 1,
      "wrap": { "width": 900, "lines": 3, "ellipsis": true },
      "background": { "color": "#00000066", "padding": [24, 16], "radius": 18 },
      "transform": {
        "position": [510, 520], "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0],
        "skew": [0.0, 0.0]
      },
      "blend": "normal", "cache": false, "depth_test": false, "effects": [], "fallback": [],
      "opacity": 1.0, "padding": 0, "parallax": [0, 0], "render": "auto", "solid": false,
      "visible": true
    }
```

```json
    {
      "blend_space": "canvas", "id": "time", "type": "text",
      "text": { "clock": "HH:mm:ss", "invert": false }, "font": "monospace", "size": 90,
      "color": "#7dd3fc", "anchor": "bottom_right",
      "transform": {
        "position": [-80, -60], "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0],
        "skew": [0.0, 0.0]
      },
      "align": "center", "blend": "normal", "cache": false, "depth_test": false, "effects": [],
      "fallback": [], "line_height": null, "opacity": 1.0, "padding": 0, "parallax": [0, 0],
      "render": "auto", "solid": false, "valign": "center", "visible": true, "wrap": null
    }
```
