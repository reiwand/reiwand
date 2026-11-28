# Features

`requires.features` lists, from a closed registry, what the package needs from the engine, so that
an older version or a weaker device says that it needs a newer one before reading anything else.
`reiwand-ctl pack build` computes the list from the manifest and the scene and writes the missing
names into `manifest.json` before building (a signed folder must be signed again: `pack sign`
fills the list before signing); readers refuse a package that uses a feature it does not list,
and `reiwand-ctl pack check` explains each one and warns about listed features the package does not
use.

Each feature also says what each platform does with it: the desktop and the Android app either run
it, play the package without it (and its page on the phone says so), or refuse the package with
`this package needs a newer version of Reiwand: it uses the feature ktx2 (uses compressed textures in the KTX2 format)`
before drawing anything, so that nothing is drawn wrongly or silently left out. The library still
lists a refused package with its thumbnail, title and that reason, and reads it again after an
update of Reiwand. `reiwand-ctl pack check` prints what each platform does. The Android app reads a chosen
file or folder (`folder`, `folder2`, `user_videos`) through the system document picker: it copies
the file, or the accepted files of the folder up to `max_files` and `recursive` levels deep, into
its own storage, keeps the read permission of the folder and copies it again when the app opens and
the folder changed, and the stored value is the path of that copy.

| Feature | The package | Desktop | Android |
|---------|-------------|---------|---------|
| `inputs.audio`, `inputs.pointer`, `inputs.clicks`, `inputs.media`, `inputs.network` | declares or uses that input | runs | runs |
| `scripts` | runs scene scripts | runs | runs |
| `sound` | plays sound layers | runs | runs |
| `properties2` | has settings of the newer kinds or fields: sliders without `step` or with `decimals`, `editable` or `log`, combos of numbers or flags or with icons, `secret` texts, `button`, `divider`, `folder`, `shortcut`, collapsed groups, icons, fractional `order`, `visible_if` expressions, `enabled_if` and `default_asset`, or settings written in a way the first version cannot read (a field it does not know, even with its default value, a missing `default` of a text or `accept` of a file, or an `order` such as `2.0`) | runs | runs |
| `folder` | shows pictures from a folder the user chooses | runs | runs |
| `folder2` | reads the subfolders of a chosen folder (`recursive`) or fewer of its files (`max_files`) | runs | runs |
| `folder_lists` | lets its page list a chosen folder (`mode: all` in a `web` package) | runs | runs |
| `script_folders` | lets its scene scripts list a chosen folder (`mode: all` in other kinds) | runs | runs |
| `user_videos` | lets the user choose videos for its files or folders | runs | runs |
| `shortcut` | has a `shortcut` property | runs | runs |
| `quality_settings` | has properties with `quality` values | runs | runs |
| `video_properties` | sets `playback` or `display` of a video | runs | runs |
| `playback2` | sets `max_fps`, `muted` or `loop`, or `playback` of a web page or a scene | runs | runs |
| `display2` | sets `display` of a picture, or its `background` or `span` | runs | without |
| `flip` | sets `display.flip` to `horizontal` or `vertical` | runs | runs |
| `accent` | sets `display.accent` | runs | runs |
| `web` | is a web page (`kind: web`, see `web.md`) | runs | runs |
| `preset` | sets the values of another package (`kind: preset`) | runs | runs |
| `assets` | holds resources for other packages (`kind: assets`) | refused | refused |
| `packages` | lists `requires.packages` | runs | runs |
| `gpu` | sets `requires.gpu` | runs | runs |
| `platforms` | sets `platforms` | runs | runs |
| `locale_files` | keeps texts in `locale/<language>.json` | runs | runs |
| `engine_strings` | names a text of the engine, `"@reiwand.<key>"` | runs | runs |
| `origin2` | is `imported` or from a `provider`, or sets `origin.source_url` | runs | runs |
| `credits2` | credits files to the package they come from (`credits[].source`) | runs | runs |
| `ktx2` | uses KTX2 textures (Basis Universal pictures, `scene.md`) | runs | runs |
| `avif` | uses AVIF pictures | refused | refused |
| `spanned_scenes` | sets `display.span` of a scene to `all-outputs` | runs | without |
| `rgb` | sets `permissions.rgb` or sends colors to RGB lighting from a scene | runs | without |
| `effect.<id>` | uses a built-in effect that older versions do not have; `pack build` and `pack check` name the ones a package needs | runs | runs |
| `effect.<id>.<parameter>` | sets a parameter that an older version of a built-in effect does not have | runs | runs |

Every effect and parameter listed in "Newer effects and parameters" of `scene.md` runs on both
platforms. A package that uses one names it with these names, so that older versions, which do not
draw it, refuse the package with the newer-version message.

The fields of `scene.json` that came after the first version name these features (`scene.md` says
what each field does). Each one is refused on a platform until its engine draws it, and the table
says which ones run today; the phone, which runs scene scripts in a thread of their own and plays
the scene sound, runs `sound2`, `script_sounds`, `scripts2`, `script_layers` and
`script_animations`, and leaves out the `script_` features that change nothing there:

| Feature | The scene | Desktop | Android |
|---------|-----------|---------|---------|
| `blend_modes2` | uses a blend mode after `luminosity` (`linear_burn` … `diffuse_light`) | runs | runs |
| `layer_color` | sets `brightness` on a layer or `color` on a picture, video, particles or group layer | runs | runs |
| `parent` | sets `parent` or `inherit` on a layer | runs | runs |
| `tilt` | sets `transform.tilt`, `transform.z` or `canvas.perspective` | runs | runs |
| `mask_strength` | sets `strength` on a mask | runs | runs |
| `effect_masks` | masks an effect with a `layer` or a `shape` | runs | runs |
| `layer_textures` | feeds a layer into an effect texture with `{ "layer" }` | runs | runs |
| `video_textures` | uses a video asset as a mask, an effect texture or the picture of a light | runs | runs |
| `layer_playback` | sets `playback` on a picture or video layer | runs | runs |
| `video_clocks` | plays the same video in several layers, each on its own clock (`playback.own_clock`) | runs | runs |
| `texture_options` | sets `filter`, `wrap` or `mipmaps` on an asset | runs | runs |
| `animated_textures` | sets `frames`, `fps`, `durations`, `rects` or `animated` on an asset | runs | runs |
| `text_layout` | sets `valign`, `wrap`, `line_height`, `background` or `padding` on a text | runs | runs |
| `text_styles` | sets `letter_spacing`, `outline`, `shadow`, `blur` or `render: sdf` on a text | runs | runs |
| `text_languages` | writes the `text` of a text layer as one text per language | runs | runs |
| `fonts2` | uses ttc, otc or woff2 fonts, `face` or `fallback` | runs | runs |
| `sound2` | plays MP3, `sounds` lists, `mode`, `pause`, `stream` or `play_on` | runs | runs |
| `camera` | sets `canvas.camera` | runs | runs |
| `parallax2` | sets `canvas.parallax` or a parallax with two different parts | runs | runs |
| `parallax_values` | binds `amount`, `pointer` or `offset` of `canvas.parallax`, sets an `offset`, or binds the `parallax` of a layer | runs | runs |
| `bloom` | sets `canvas.bloom` | runs | runs |
| `hdr` | sets `canvas.hdr` or `canvas.tonemap` | runs | runs |
| `keep_frame` | sets `canvas.clear` to `false` | runs | runs |
| `lights` | has `light` layers | runs | runs |
| `volumetric_lights` | sets `volumetric` on a light | runs | runs |
| `light_textures` | sets `texture` on a spot light | runs | runs |
| `lighting2d` | sets `lighting` on a layer | runs | runs |
| `ambient_light` | sets `canvas.ambient` or `canvas.sky` | runs | runs |
| `capture` | sets `size` or `redraw` on an adjustment layer | runs | runs |
| `timeline_events` | names `events` of a timeline or triggers on `event:<name>` | runs | runs |
| `timeline_frames` | gives the length of a timeline in `frames` at `fps` | runs | runs |
| `timeline_wrap` | sets `wrap` on a looping timeline | runs | runs |
| `fresh_seed` | sets `clock.seed` to `null`, a new seed every start | runs | runs |
| `tangents` | gives keyframes `{ "in", "out" }` handles | runs | runs |
| `tracks` | animates the parts of a position or a color on their own (`x`, `y`, `r`, `g`, `b`, `a`) | runs | runs |
| `relative_values` | adds a value to `base` | runs | runs |
| `clock_values` | follows `{ "clock": "day" }` and the like with a number | runs | runs |
| `clock_dates` | writes dates with a `date` text | runs | runs |
| `audio_ranges` | follows a range of bands (`audio: [lo, hi]`, `of`, `reduce`) | runs | runs |
| `shaping` | sets `bounds`, `exponent` or `smooth: { "up", "down" }` | runs | runs |
| `audio_response` | sets the scene `audio` block | runs | runs |
| `custom_effects` | brings effects of its own (`effects`) | runs | runs |
| `effect_options` | brings effects with `options` | runs | runs |
| `vertex_effects` | brings effects with a `vertex` stage | runs | runs |
| `custom_effects2` | brings effects with `compose`, `swap`, `min_fps` or `velocity`, or whose shader calls `cursor_last`, `grid_uv`, `grid_position` or `fs_copy` | runs | runs |
| `lighter_effects` | brings effects with a `lighter` variant for low quality levels | runs | runs |
| `mesh` | has `mesh` layers, with their parts, depth, clips and animation layers | runs | runs |
| `ik` | sets `ik` chains on a mesh or model | runs | runs |
| `bone_physics` | sets `physics` on a mesh or model, or `canvas.wind` | runs | runs |
| `bone_rules` | sets `rules` on a mesh or model | runs | runs |
| `blend_shapes` | sets `shapes` or `expressions` on a mesh or model | runs | runs |
| `attach` | sets `attach` on a layer | runs | runs |
| `model3d` | has `model` layers or groups with `space: 3d` | runs | runs |
| `model_animation` | sets `clips`, `animations`, `ik`, `physics`, `rules`, `shapes`, `expressions` or `root_motion` on a model | runs | runs |
| `material_shaders` | shades a model with `fur`, `foliage`, `chroma` or a material shader of its own | runs | runs |
| `camera3d` | sets `camera3d` | runs | runs |
| `camera_paths` | sets `camera3d.paths` | runs | runs |
| `shadows` | sets `shadows` on a light, or `cast_shadows` or `receive_shadows` on a material | runs | runs |
| `particles2` | moves particles in world space, with control points, per-layer `instance` or `render`, scripts that use `particles`, emitter timing, newer emitter shapes, initializers and forces, operator windows, render `brightness`, `cycles`, `order` or a stretch range, `init.gradient`, a `mask` collider, or beyond the older limits (8 systems, 4 emitters, 8 forces, 4 colliders, 10 s of prewarm) | runs | runs |
| `particle_children` | sets `children` on a system | runs | runs |
| `particle_depth` | sets `perspective`, `init.depth`, `init.depth_speed` or an emitter shape `sphere` | runs | runs |
| `particle_push` | sets `push` on an emitter shape `sphere` | runs | runs |
| `particle_lighting` | sets `render.lit`, `render.normal` or `render.refract`, or `lighting` on a particles layer | runs | runs |
| `particle_layers` | emits from a `layer` | runs | runs |
| `particle_fields` | uses `turbulent`, `offset_noise`, `init.remap`, `around` or `between` | runs | runs |
| `particle_steering` | uses `keep_distance`, `keep_between` or a `remap` force | runs | runs |
| `particle_values` | binds an end of an `init` range or a field of an emitter shape | runs | runs |
| `particle_collisions` | collides with a `box`, a `segment`, a `layer` or the inside of a circle, or sets `stick` | runs | runs |
| `boids` | sets the `boids` force | runs | runs |
| `particle_noise` | sets the `noise_field` force | runs | runs |
| `layer_margin` | sets `margin` on a layer | runs | runs |
| `text_advances` | sets `advances` on a text | runs | runs |
| `tonemap_srgb_to_linear` | sets `canvas.tonemap` to `srgb_to_linear` | runs | runs |
| `stretch_anchor` | sets `anchor` in `render.stretch` | runs | runs |
| `oscillate_random` | sets `random` in an `oscillate` force | runs | runs |
| `window_ramp` | sets `ramp` next to the `window` of a force | runs | runs |
| `particle_ropes` | sets `render.rope` or trails longer than 16 steps or with `segments`, `subdivide`, `uv_scale`, `uv_scroll` or `textured` | runs | runs |
| `particle_looks` | sets `render.cutout`, `render.orient` or `render.angle` | runs | runs |
| `particle_shaders` | sets `render.shader` | runs | runs |
| `scripts2` | has more than 16 script modules, modules that declare newer events, parts or `params`, values with `fn` or `params`, or named effect uses | runs | runs |
| `script_layers` | has `templates` or scripts that use `layers` | runs | runs |
| `script_geometry` | has `pools` or scripts that use `geometry` | runs | runs |
| `script_storage` | has scripts that use `storage` | runs | runs |
| `script_sounds` | has scripts that use `sounds` | runs | runs |
| `script_animations` | has scripts that use `animations` | runs | runs |
| `script_camera` | has scripts that use `camera` | runs | runs |
| `script_shortcuts` | has scripts that use `shortcuts` | runs | runs |
| `script_hierarchy` | has scripts that use `hierarchy` | runs | runs |
| `layer_events` | sets `solid: { "alpha" }`, `hit` bones, `click:`, `enter:` or `leave:` triggers or scripts that handle pointer events | runs | runs |
| `transition` | declares `transition` in `scene.json`, its own transition when it comes in or leaves | runs | without |
| `media_info` | follows `media` values, cover assets, `media:` triggers or scripts that use the media player | runs | runs |
