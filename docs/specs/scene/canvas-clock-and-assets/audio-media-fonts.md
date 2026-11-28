# Scene audio, media and fonts

On this page: scene audio, newer asset fields, sheets, pictures, videos and fonts.

## Scene audio and newer asset fields

The scene may also set `audio: { "attack" (0.001 to 2 s), "release" (0.01 to 10 s), "gain_release" (0.05 to 10 s) }`, a block that may be absent and that carries all three fields when present, how fast audio values rise and fall
and how the relative reference recovers (`audio_response`). With the `relative` kind of audio binding
every band is measured against the loudest band of the last moments; `gain_release` is the time in which
that reference falls toward the current level after a loud sound (a scene without an
`audio` block follows the 5 s reference of the sound system, and the editor prefills 5 when it adds the block): a short one makes bands dip right after each
hit and recover before the next one. The reference
is kept by the scene runtime from the unnormalised band powers of each audio frame (`AudioFrame.power`),
so scripts' and shaders' `band()` and every binding see the same values; the audio frames also carry the
reference of the sound system for scenes without an `audio` block. The editor shows it as a slider next to `attack` and `release`, and the
band meter of the sound panel draws the bands against that reference live.

| Asset field | Meaning | Feature |
|-------------|---------|---------|
| `filter` | always written: `linear` or `nearest` (sharp pixels) sampling of a picture | `texture_options` |
| `wrap` | always written: `clamp`, `repeat` or `mirror` outside the picture: with `repeat` or `mirror`, an `image` layer whose `size` is larger than the picture (or than one frame of a sheet) fills it with copies at the picture's own size instead of stretching it, and a picture that replaces the textures of a `mesh` repeats or mirrors over the parts whose texture coordinates go outside 0 to 1, as many times as they reach | `texture_options` |
| `mipmaps` | `true` or `false` overrides whether smaller copies are made for pictures drawn smaller than they are; they are made once, the first time a picture is drawn at less than three quarters of its size | `texture_options` |
| `atlas`, `rects` | a sprite sheet as `[rows, columns]` of equal cells (1 to 256 each; as for particles) or as at most 256 `[x, y, width, height]` rectangles in pixels, not both | `animated_textures` with the fields below |
| `frames` | how many cells play (1 to the number of cells, at most 256) | `animated_textures` |
| `fps`, `durations` | 0.1 to 120 frames a second, or one duration per frame (0.001 to 60 s), not both | `animated_textures` |
| `animated` | `true` plays the frames of an animated GIF or WebP with their own durations (at most 256 frames and 64 MiB once decoded; a frame shorter than 20 ms lasts 100 ms, as in browsers) | `animated_textures` |
| `face` | which font of a `.ttc` or `.otc` collection (0 to 255) | `fonts2` |
| `media` | `current` or `previous`: the cover of what the media player plays now or played before, usable wherever a picture is; `path` is then the picture shown while nothing plays | `media_info` |
| `shapes` | on a `.glb`, at most 64 blend shapes sculpted beside the model, each name of a shape to the path of its file (`WPSH`, then the version 1 and the count as 32-bit little-endian numbers, then per vertex its node and primitive as 16-bit numbers, its vertex as a 32-bit number and the change of its place and normal as six 32-bit floats, in the space of the primitive); a model takes them as blend shapes of the file, animated or not (a still model that has any is posed), with the weight 0 until `shapes` or keys set one; the bytes of the files count in the 128 MiB of meshes of a scene; a name the file already has is refused | `model_animation` |

## Sheets, pictures and videos

An `image` layer that shows a sheet plays its frames on the scene clock, so that every layer showing
the same sheet at the same rate shows the same frame; its `playback` sets the rate, the range, whether
it loops and the frame (`layer_playback`). A sheet without `fps` or `durations` shows its first frame. The scene asks for a frame only when the next frame of a shown
sheet is due, never while the layer is hidden, and the lowest quality level shows the first frame of
the range without moving.

Each picture is decoded once and uploaded once per GPU, and every screen that shows the scene
draws from that one texture. Once a picture is on the GPU its decoded copy leaves main memory; the
engine reads it again from the package only when no screen holds the texture any more. Every
shader and pipeline a scene needs is built when the scene loads, never during a frame, and a frame
hands the textures of a finished layer to the next layer that needs one of the same size, so a
steady frame creates nothing and makes no allocations of its own. Evaluating a frame allocates
nothing either: `Runtime::frame_in` takes the lists of the previous frame back (`Frame::spare`).
On the phone a scene script that changes layers each frame still makes a few allocations per changed layer
on the drawing thread (the copy of its answer and the layer names it reads back: about 15 a frame for the
four planets of the `orbits` example); the phone test bounds them at 16 a frame.

Each video asset has one decoder, shared by every layer, mask and effect that shows it on every
screen. The first layer of the scene that shows the asset, as a `video` layer or as the picture of
an `image` layer, steers it with its `playback`: `rate` (a video plays from 0.1 to 4 times, and its
time, its end and what scripts read follow the rate it plays at; 0 holds the video), `playing`,
`loop` (prefilled on), `start` and `end` in seconds. The time of a video runs on the scene clock
while it plays and something shows it: a video that no visible layer, mask or effect shows waits
where it is and its decoder pauses, as for a paused or finished video, and none of them asks for
frames. Without `loop` the video stops at `end`, or at its own end (read from the header of its
WebM, MKV or MP4 file), and tells scripts. On battery its decoder runs no faster than the frame rate
of the scene, which `clock.max_fps` caps. On the phone the decoder is the platform's MediaCodec,
through the system player, which plays only the part between `start` and `end`.

On the desktop at most 16 video decoders run at once, for all screens together (the video of a
wallpaper, the videos of scenes, own clocks and videos that scripts show). Clocks that play the same
file in step (the same part, speed, play state and sound level, no seek of a script, and less than
0.1 s apart) share one decoder. A video that would need a decoder past the limit shows a still frame
of its video, the one at its `start`, scaled to at most 1920 pixels a side, and the daemon writes a
warning that says how many videos show a still; a decoder that runs keeps running, and a video
showing a still plays again as soon as a decoder is free.

## Fonts

Fonts may also be TrueType or OpenType collections (`.ttc`, `.otc`) and compressed `.woff2` files
(`fonts2`). Videos may be used as masks, effect textures and the picture of a spot light
(`video_textures`), and wherever a picture is; sounds may also be MP3 (see "Sound"). Pictures may
also be KTX2 textures (`ktx2`, see "KTX2 textures"). Meshes and models are binary glTF 2.0
files (`.glb`, at most 64 MiB each; FBX or OBJ convert to it with common 3D tools): readers read
only the header and the JSON part of each file once (at most 4 MiB, and 16 MiB for all the models
of a scene), refuse files that load anything from outside themselves or require an extension other
than `KHR_materials_emissive_strength`, `KHR_materials_unlit`, `KHR_mesh_quantization` and
`KHR_texture_transform` (model layers draw the offset, rotation, scale and `texCoord` of each map,
read the first two sets of texture coordinates, `TEXCOORD_0` and `TEXCOORD_1`, and sample each
texture with the wrap (repeat, clamp or mirror) and the filters (nearest or linear, with or
without mipmaps) of its sampler; the editor warns about a map that reads a third set, drawn with the
first), skins of more than 256 bones, more than 64 blend shapes per primitive,
more than 65 536 nodes or 4096 meshes, materials, animations or skins, more than 262 144 list
entries for all the models of a scene (buffer views have a cap of their own, 262 144 per file, and
do not count there), pictures shown by textures whose distinct byte ranges together pass the size
of the file (pictures no texture shows are never read, and a model keeps its pictures as ranges of
the file it read instead of copies) and more than 2 000 000 triangles, and read the names of its nodes (attachment points), skin joints
(bones), meshes (parts), materials, blend shapes (`extras.targetNames`, or `shape_<n>`) and
animations with their length, against which every name the scene gives is checked. Unnamed
elements are called `node_<n>`, `mesh_<n>`, `material_<n>` and `animation_<n>`.
