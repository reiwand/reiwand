# Facets and tags

Repositories, the library and the server compute facets from the package itself, so that authors
cannot lie: the inputs the scene uses (`audio`, `pointer`, `clicks`), `sound`, `scripts`, `clock` (a
text shows the time), `customizable` (it has properties to change), `photosensitive`, `localized`
(more than one language), `animated` (scenes, videos and web pages), `desktop` and `android` (it
runs fully there, see "Platforms"), `compute` (it sets `requires.gpu` to `compute`), the orientation (`landscape`, `portrait`, `square`),
`ultrawide` from a 2.2 ratio and the size (`720p`, `1080p`, `1440p`, `4k`, `5k`, `8k`) of scenes
and images, and `hdr` when a video of the package (its entry, or a video asset of its scene) writes
the PQ or HLG transfer in its colour description (the `colr` box of MP4 or the colour elements of
Matroska and WebM). On the computer such a video is decoded with 10 bits and its tones are mapped to
standard colors on the graphics card (PQ and HLG, BT.2020 to sRGB, with 203 nits as the white of
the screen and 1000 nits as the brightest light); on the phone the system maps it when it plays
straight on the screen, and when a filter, a flip, a fit other than fill or stretch or a setting of
the package sends it through the graphics card, the app maps the 8 bits the decoder gives it the
same way (BT.2020 YCbCr, PQ or HLG read from that colour description, the same knee toward 1000
nits). When `playback.hdr` lights the screen in HDR for such a video, the app writes extended linear
values instead, with 203 nits at 1.0 and the knee toward the headroom of the screen, and a color
filter keeps the highlights above white; when the decoder reports that its frames are already
standard (it mapped them itself), the app shows them as they come and does not light the screen in
HDR for them. A video asset inside a scene with `canvas.hdr`, a tone map and `q3` takes the same
extended values when the screen is lit in HDR, so the tone map of the scene bends its highlights;
in other scenes it keeps the standard curve. The last frame held by a video that ended, or by a
paused clip, is converted again when the headroom changes or the graphics card is opened anew.
Once a picture or nothing takes the place of the video, the screen leaves HDR. `reiwand-ctl pack check` says so. Readers look for it only when they
record facets (adding to a repository, indexing the library, the server and `pack check`), never
when they open a package to show it. The inputs of a web page come from its manifest as declared;
for the other kinds every input, `media` included, comes from what the package uses, and an
`audio`, `pointer`, `clicks`, `media` or `network` input that nothing uses gets a warning and no
facet. Since only web pages use `network`, released second-version packages that declare it
elsewhere get that warning and keep working. Searches filter by
them (`facet:audio`, `facet:4k`) and by rating (`content:general`, `content:mature`). Tags stay free; these lowercase slugs are recommended for genres: `abstract`,
`anime`, `animals`, `cars`, `cityscape`, `fantasy`, `games`, `minimal`, `mountains`, `music`,
`nature`, `pixel-art`, `retro`, `sci-fi`, `space`, `water`, `winter`.
