# Curve editor

The *Curve editor* button of the timeline bar (and that of the clips bar of a mesh or a model, for the
channels `rotation`, `position`, `scale`, `angles`, `shift`, the weights of the shapes and the order of
the parts) opens a graph of the keys of the animated values of the chosen layer, or of all of them if
no key is chosen (twelve at most). Each value is split into tracks, one per part: x, y and z of a
position, red, green, blue and alpha of a color, or the number alone; each track is drawn with its own
scale, shown or hidden with its checkbox, and comes from the `curves` query (`reiwand-ctl scene ask DIR
curves '{"pointer": P}'` or, for a clip channel, `{"layer", "clip", "row": ["bones", BONE, CHANNEL]}`),
which gives the keys with their parts, the position of their handles in each track (those missing, with
their default value of a third of the span; `split` says whether they are per part and `each` gives
each pair in the units of its part) and the curve sampled by the engine. A click chooses a key, Ctrl
or Shift plus click adds or removes it, and dragging over the background chooses the keys in a
rectangle. The chosen keys are dragged in time and, if their values are of the same type and unit (the
`curves` query gives `kind` and `unit` of each value), also in the value of the track being dragged,
adding the same to all; with Alt the vertical drag scales their values around the middle of the chosen
ones of that part, and *Scale values ×* does the same with a factor, part by part; the handle at the
right of the selection box stretches their times from the first, and *Stretch times ×* does the same
with a factor; each is an undo step with the `edit_keys` action. The handles of the chosen keys are
dragged live, as one step per drag, with the time limited to the neighboring key. With *Linked parts*
(the starting mode), in lists and colors the handle height is the share of the path, the same for all
parts, so the handle drawn on one track moves the curve of all; with *Separate parts* each track has
its own handles, in the units of that part (`edit_keys` with `part`), and going back to *Linked parts* with a chosen key that has them separate gives all of them the shape of its first part
(`link`). With *Aligned handles* the opposite handle turns with the one being dragged, keeping its
length; with *Broken handles* each one moves alone. *Straighten*, copy (Ctrl+C), paste at the playhead
(Ctrl+V; the keys of another value are pasted into the chosen value; the copied keys also go to the
system clipboard as the text `reiwand-keys:{"picks": [{"pointer", "keys"}]}`, so they are pasted in
another window or another project, and a clipboard with any other text is ignored and the last keys
copied in the app are pasted; the `key_clip` query builds that text with `{"copy": PICKS}` and reads it
with `{"read": TEXT}`, which gives a null `picks` if it is not one) and remove (Delete) act on the
selection. The wheel zooms the time around the pointer, with Ctrl the value and with Shift (or the
horizontal wheel) it scrolls the time; dragging with the middle button moves the view in time and
value. *Fit the picked keys* (F) and *Fit all keys* (Home) come from `bounds` of the `curves` query: `all`
gives, part by part, from where to where the curve, the keys and their pinned handles reach, and with
`picked` (the seconds of the chosen keys) `picked` gives the time and value span of those keys with
their handles. Each track keeps its own scale; the value zoom is the same for all.

The template picker of the layers bar shows the scene, or one script template alone on the canvas as
a copy of it draws (the preview look `template`). While a template is shown, the layers panel lists
its layers, the canvas puts its move, bone, chain, weight, point and attachment handles on them, and
every action and query that names layers, from `bone`, `ik`, `attach`, `weights` and `parts` to
`hide` and `add_layer`, works on the layers of that template by their own ids, so an attachment, a
parent or a chain target names a layer of the same template as each copy needs; a template holds one
layer, so new layers go inside its group (`group` on its layer makes one) and an action that would
add a second one beside it is refused. An attachment, a chain target `layer:<id>` or a mask may also
name a layer of the scene: the lookup tries the template first and then the scene, a point placed on a
mesh of the scene goes into that mesh, the inspector of a template layer offers *Follow a mesh of the
scene* (a mesh, then one of its bones or points; the canvas draws the bones and points of that mesh, and tapping one makes every copy follow it; the outline lists the meshes in `followed`), the chain target picker lists the layers of the
scene after those of the template, and `reiwand-ctl scene --inside TEMPLATE attach DIR LAYER --to MESH`
does the same. Actions that are not about layers (properties, conditions, presets, assets, imports,
the manifest, the canvas, timelines, wind, ambient light, particle systems and every `set`, `add`,
`remove`, `move` or `translate` outside `/scene/layers`) act on the whole scene while a template is
shown, so renaming or removing a property sees and changes the bindings of the scene too. Its fields
are the fields of `/scene/templates/<name>` in the inspector. Bones, weights and points live in the
mesh asset, so a mesh layer in a template and the layers of the scene that name the same asset change
together, as any two layers sharing a mesh do; chains, attachments, parts and depth live in the layer
and stay its own. `reiwand-ctl scene --inside TEMPLATE …` does the same for every scene command, and `reiwand-ctl scene open DIR --json --inside TEMPLATE` prints what the editor shows.

The timeline shows a lane for each own video clock (`playback.own_clock`): a strip of eight frames
taken evenly over the video (decoded once off the interface thread and kept in the cache of the
user, `$XDG_CACHE_HOME/reiwand/strips`, at most 512 files), the range of the video as a bar whose
grips trim `start` and `end`, its speed, loop and play switches and a mark where the clock is at the
playhead; `reiwand-ctl scene ask DIR clocks` prints the same lanes as JSON, with the
files of N frames (at most 16) of each with `--strip`.

`import` copies pictures, videos, sounds, fonts and `.cube` tables. It refuses a file over the
limits of the format (256 assets, 1.5 GiB per file, 512 MiB of pictures once decoded, sounds of 192
MiB and 3 hours at most, the ones up to 60 s 180 s together and the longer ones to be streamed,
tables of 16 MiB); MP3 sounds and `.ttc`, `.otc` and `.woff2` fonts are imported too or an asset id in use before it
copies anything. The file is copied in blocks to a hidden file in `assets/` that is renamed when
complete. A file with the same name and other content is stored as `name-2.ext`; one with the same
size and SHA-256 is reused. The app imports, builds and applies on a worker thread, one task at a
time, so the window keeps answering. `prune` deletes the files of `assets/` that the scene, its
previews and its credits no longer name, and the meshes of `meshes/` that neither the scene nor the
history names.

Particle look panel: *Stretch by speed* sets the check box, the mode (*Add pixels to the side* or *Multiply the side*), *Centre the stroke on the particle* (`anchor: centre`, kept across mode changes; hidden for `segment`, which is centred anyway), whether the trail comes from the speed without perspective or follows the motion on screen (`depth`, with a nearest and a farthest drop drawn with their trails when the system has perspective; `handles` returns them as `drops`), the seconds of motion and the shortest and longest length, with a sprite that shows the length at the sample speed (`handles` returns it as `stretch`). *Turn sprites in depth* (`init.tilt`, always present with zeros for no turn) has two sliders (from and to) for the angle and the spin about the horizontal and the vertical line, with five sprites drawn at the steps of the angle range; it writes through the `particles` action `start`. It has no handle on the canvas: the turn has no direction in the scene to drag. *Fades and pulses* lists the `oscillate` forces on opacity and size with a plot of three particles (`handles` returns it as `waves`), adds one and edits `what`, `from` and the scale range; *Noise fields* lists the `noise_field` forces (`handles` returns them as `noises`, each with its `index`), adds one with the `Offer.noise_field` row of the prefill table and edits the strength and phase ranges, `scale`, `drift` and the three parts of `mask`, or removes it; all of it writes through the `particles` actions `render` and `force`.
