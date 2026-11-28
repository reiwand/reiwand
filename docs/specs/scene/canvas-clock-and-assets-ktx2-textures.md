# KTX2 textures

A picture asset may be a `.ktx2` file holding a Basis Universal texture: ETC1S, UASTC or XUASTC of
low dynamic range, or UASTC HDR (4 × 4 or 6 × 6) of high dynamic range; one picture, an array of
up to 256 frames (layers, or the frames of an ETC1S video) or a cube map of six square faces; at
most 16 384 pixels a side, 15 levels and 256 MiB. Once read, the engine transcodes it to the
compressed format the graphics card reads best: BC7 on desktop cards, ASTC 4 × 4 on most phones,
ETC2 on older phones, and uncompressed RGBA where none of them is offered. An HDR texture keeps its
light past white: BC6H on desktop cards, ASTC HDR where the card reads it and half floats
elsewhere; it gains range on canvases with `hdr` and is clamped at white on others and wherever
the CPU reads it. The frames of an array are laid out in a grid of ⌈√n⌉ columns, the same grid as
an animated GIF: `atlas` and `playback.frame` pick one, and `animated: true` plays them with the
frame times of the file: the own time of each frame under the key `reiwandFrameTimes` (a 32-bit count of milliseconds per frame, written when the frames differ), else the one of `KTXanimData` (1/24 s without one). A cube map becomes an equirectangular
panorama twice as wide as high (`+Z` in the middle, `+Y` up, `+X` to the right), uncompressed, so it
fits wherever a panorama does, such as the `env` of `lighting`. A side that is not a multiple of 4
pixels stays compressed: the texture is padded to whole blocks and every reader of the picture samples only the real pixels:
layers, sprite frames, particles, the masks of layers and effects, normal, roughness and emissive maps,
reflections and projected pictures through its crop, and molds, meshes, the maps of particles, the
reflections of models and the textures of effects through an uncompressed copy of the real pixels; the mipmaps whose blocks no longer line up with the
padded texture are left out, and the frames of an array whose sides are not a multiple of 4 are
kept uncompressed. The levels of the file are its mipmaps: the first level no larger than the largest texture the card
takes is drawn, `mipmaps: false` keeps only that one, and a file of one level is mipped like any
other picture. `linear`, `filter`, `wrap`, `atlas` and the frames of a sprite sheet apply as to
other pictures. The file stays packed in memory and only becomes plain pixels
when the CPU needs them (pointer tests against its alpha, particles shaped by it), so a scene of
KTX2 pictures takes about a quarter of the video memory of the same pictures in PNG. The 512 MiB that
the pictures of a scene may take once decoded counts them as uncompressed, as on cards that get
RGBA. Picture packages may use a `.ktx2` entry too; it is shown uncompressed, and an HDR one keeps its light above white on screens with HDR output (`playback.hdr` in `dbus.md`) unless it has photo edits.

KTX2 files are made where scenes are made: Convert to KTX2 in the desktop editor and `reiwand-ctl
scene asset --ktx2`, with the Basis Universal encoder: a picture becomes a texture, an animation an array
that keeps the delay of each frame, a Radiance `.hdr` or OpenEXR `.exr` picture (pictures of a scene too,
drawn on the desktop and the phone with their light clamped at white and encoded in sRGB, so that they look as their KTX2 texture does, or kept as plain values when the asset is `linear`; six faces that mix such pictures with 8-bit ones take the 8-bit ones as sRGB light) a UASTC HDR texture, and a panorama or six square faces a cube map. The phone only reads them: it has no scene
editor, and its app carries the transcoder written in Rust but not the C++ encoder, which would add
a C++ toolchain to the phone build and several megabytes to the app for a tool it has no screen
for. Packages made on the desktop play on the phone with the same files.
