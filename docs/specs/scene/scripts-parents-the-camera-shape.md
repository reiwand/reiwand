# Parents, the camera, shapes, storage and folders

With `hierarchy`, `layer.parent` is the id of the layer it follows (its `parent`, or the group that
holds it) or `null`, `layer.children` the ids of the layers that follow it or that its group holds,
and `layer.world` a `reiwand/math` `Mat3` from the pixels of the layer to canvas pixels, without the
camera, as the scene evaluated it at the last frame. `layer.setParent(id, keep)` makes a layer follow
another layer of the scene (`null` to follow none) without changing the order of drawing, as `parent`
does in `scene.json`; with `keep` the layer stays where it is, its position, rotation and scale
turned into ones relative to the new parent. A layer cannot follow itself or one that follows it, nor
more than 8 parents deep: the call throws and the engine drops such a parent too. `lookAt(from, to)`
of `reiwand/math` gives the angle in degrees that turns a layer at `from` towards `to`.

With `camera`, `scene.camera` (`{ center, zoom, rotation, shake: { enabled, amplitude, roughness,
speed } }`), `scene.camera3d` (`{ fov, eye, center, up }`), `scene.parallax` (`{ amount, delay, pointer, offset }`), `scene.bloom` (`{ threshold, strength, radius,
tint }`), `scene.background`, `scene.ambient` and `scene.sky` (colors) read what the scene sets in
`canvas` and what the script wrote; writing an object changes only the fields it names, within the ranges of `canvas`, and `null`
gives the field back to the scene. The camera is the same transformation as `canvas.camera`, without
passes, and its shake stays still with *reduce motion* and at Q0. Bloom also reaches a scene whose
`canvas` sets no `bloom`: writing it adds the glow of `canvas.bloom` with the defaults of that object
for the fields it does not name (threshold 0.8, strength 1, radius 16, `canvas` unit, white, knee 0.5, saturation 1, no limit), kept only while the
script keeps it (`null` takes it away again), dropped at Q0 and given its plain set on battery like
the glow a scene declares; until a script writes it nothing is added and nothing costs more.

With `geometry`, `scene.mesh(pool)` gives a pool of `pools`: `create({ vertices, indices, format,
visible })` adds a shape and returns it, with `update(fields)`, `destroy()`, `id` and `alive`;
`shapes` lists them and `clear()` removes them. `vertices` is a `Float32Array` or an array of numbers
in pixels of the pool, by vertex `x, y` (`format: "xy"`), `x, y, u, v` (`"xyuv"`, prefilled), `x, y,
r, g, b, a` (`"xyrgba"`) or `x, y, u, v, r, g, b, a` (`"xyuvrgba"`), colors from 0 to 1 that tint
`texture` (white without one); `indices` (a `Uint32Array` or numbers, three a triangle, at most six a
vertex) default to the vertices in order. A pool holds at most `max` shapes and `max_vertices`
vertices, the shapes of a frame take at most 1 MiB, and the engine drops a pool whose shapes name a
vertex that is not there or a number that is not finite. The shapes are drawn into a picture of the
pool of `size` (prefilled `[w, h]`, 1 to 4096, the canvas) that a `video` layer shows with `"video":
"<pool id>"`, so they take the place, transform, effects, masks and blending of that layer; a pool
that does not change draws nothing again and asks for no frames, and what the scripts send is binary,
not text.

With `storage`, `storage.get(key, scope)`, `set(key, value, scope)`, `delete(key, scope)`,
`clear(scope)` and `keys(scope)` keep JSON values for the package (`"package"`, the default) or for
the screen (`"output"`), at most 64 KiB in each scope: `set` throws past that or for a value that is
not JSON. They are kept on the device only (never synced or exported), in a file of the package
(mode 0600) that is written at most every 5 s and when the scene stops, and removed with the package;
a scene whose scripts use storage runs one script process per screen. The screens showing a package
share its `package` scope: what one stores reaches the scripts of the others on their next frame,
and each keeps its own `output` scope (at most 16 screens, the one used longest ago going first;
on the phone the home screen and the lock screen are two screens, each with its own `output` scope).
Previews in the editor read what is kept but never write it.

With `folders`, `folder(prop, options)` lists a `folder` property with `mode: all`: `files` are opaque
ids of its pictures, and also of its videos with `options.videos: true` when the property `accept`s
`video` (an id changes when its file is
written again), `name(id)` the name of the file, `kind(id)` `"image"` or `"video"`, and
`on("added" | "removed" | "changed", handler)` gets `{ prop, files }` (`changed` as `[old, new]`
pairs, only of the files that `folder` lists) when the folder changes: while the scene is shown, the
desktop watches the chosen folder and its listed subfolders for change notices and reads them again,
away from the thread that draws, once the notices settle (half a second after the last one, at most
5 s after the first, so a copy of many files is read once), leaving out hidden files that sync tools
write while they work; a hidden scene drops the watch and looks once when shown again. A folder that
is missing, removed or moved is awaited in the nearest folder above it that exists and watched again
when it comes back, and subfolders nobody may read are left out, as the listing leaves them out.
Only where the system gives no change notices, on a network file system (a user space file system
counts when it reaches the network, such as `sshfs` or `rclone`, not a local disk or a vault), or
when a folder holds more than 8 192 subfolders to watch, it looks every 2 s while the scene is shown. The phone lists its copy of the chosen folder again only when the
copy changes, which it makes again when the system says the folder changed. `show(layer, id)` makes an image layer show that picture, or a `video` layer play that video
with a decoder of its own that only runs while the scene is shown, on a clock of its own that follows
the `playback` of the layer and that `layer.video` (play, pause, seek, rate, volume, `ended`) controls like the
videos of the package, with its sound at the volume of the videos of the package on the desktop (the
phone plays the videos inside a scene without sound, folder videos too), instead of its own
(`null` gives it back), at most eight files at once, any of them videos; a video on another
kind of layer throws. Scripts never see paths, and the phone lists the folder the user chose in the same way.

`templates` maps at most 64 ids to layers that scripts with `layers` copy into the scene, validated
like the other layers (`script_layers`); a template id may not be the id of a layer, a template holds
at most 256 layers with its children, and templates are pictures, texts, shapes, solids, spectra,
meshes and groups of them. Each copy of a `mesh` gets a puppet of its own, with the bones, clips,
physics, IK and attachments of a mesh layer of the scene (scripts drive it by the id of the copy), at
most 64 puppets in a scene, and the puppet goes with the copy. An `attach`, a `parent`, an IK target
`layer:<id>`, the `mask.layer` of a layer or of an effect, or an effect parameter fed by `{ "layer" }`
inside a template may name a layer of the same template, which each copy turns into its own
`<copy>.<child>` (so a badge pinned to a bone follows the bone of its own copy and a mask cuts its
own copy), or a layer of the scene; pointer hits on the bones of a copied mesh name the bone of that
copy; particle systems are scene-wide, so a particle source written in the scene names a layer of the
scene, and a script points the layer source of an emitter or a layer collider of one particles layer
at any layer, a layer of a copy included, with `particles(layer).from(n, id)` and `.wall(n, id)`; the editor's *Make a template* (`reiwand-ctl scene act DIR duplicate_layer`)
copies the layer under a free name, `<id>_template` or the next `<id>_template_<n>`, and the layers bar
of the editor shows one template alone on the canvas, as a copy of it draws, with the layers panel,
the canvas handles (move, bones, chains, weights, points, attachments) and every layer action on the
layers of that template, named by their own ids (`reiwand-ctl scene --inside TEMPLATE …` likewise). `scene.spawn(template, options)` adds a
copy on top of the layers (or at `options.at`, 0 being the bottom) with the id `options.id` (letters,
digits, `_` and `-`, at most 64) or `<template>_<n>`, sets the fields `options` names and returns its
handle; the children of a copied group are `<id>.<child>`. `destroy()` removes it, `scene.order(id,
index)` moves a top-level layer, and at most 256 copies live at once, and only while the layers of
the scene and of every copy, children included, are at most 512: `spawn` throws past that and the
engine drops what goes beyond it. Copies count as layers of the scene for the inputs it uses and for
its cost, and are drawn with the same resources as the other layers: pictures share the texture of
their asset and equal texts one raster.

The `reiwand/math` module, written for this engine, has `Vec2`, `Vec3` and `Vec4` (`add`, `sub`, `mul`,
`div`, `min`, `max`, `clamp`, `scale`, `negate`, `abs`, `floor`, `ceil`, `round`, `dot`, `length`,
`lengthSquared`, `normalize`, `distance`, `mix`, `lerp`, `reflect`, `project`, `equals`, `isZero`,
`toArray` and, by size, `cross`, `angle`, `rotate` and `perpendicular`), `Mat3` (2D, with
`translation`, `rotation`, `scaling`, `transformPoint`, `transformVector` and `decompose`) and `Mat4`
(`translation`, `scaling`, `rotationX/Y/Z`, `fromEuler`, `perspective`, `lookAt`, `transformPoint`,
`transformVector` and `decompose`), both with `multiply`, `transpose`, `determinant`, `invert`,
`equals` and `get(row, col)`, and `clamp`, `mix`, `smoothstep`, `step`, `fract`, `remap`, `degrees`,
`radians`, `angleDelta`, `mixAngle`, `angleVector`, `vectorAngle`, `lookAt`, `rgbToHsv` and `hsvToRgb` (hue
from 0 to 1). Vectors and colors go in and out as these classes or as arrays.

`Math.random` follows `clock.seed`, and `clock.seed: null` gives it a new seed on every start.
Scripts never see paths, device names, the battery or the list of fonts.
