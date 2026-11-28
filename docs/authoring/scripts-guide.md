# Guide for script authors

A script is a JavaScript module of the package that the scene names in `scripts` (at most 64 and
1 MiB in total). On the computer all the scripts of a scene share one isolated process, and on the
phone one thread, also when several screens show the same scene with the same settings, with 32 MiB
of memory and 4 ms of CPU per frame for all of them together (100 ms to load and for the first
frame). What they return arrives on the next frame, so a slow script never slows down drawing, and
only what changed travels: each field a script writes is compared with the one it already had,
without turning the whole output into text on every frame, and a frame without changes sends
nothing. `docs/specs/scene.md` describes the full interface; the essentials:

- **Lifecycle**: the module exports the functions it needs: `init(ctx)` on the first frame,
  `update(frame)` on every one, `destroy()` on stop, `resize({screen, canvas})` when the screen
  changes or rotates, `properties(changed)` with the settings that change (all of them at the
  start), `settings({language, fps, quality})` and `timelineEvent(e)`. Without `update` the script
  only wakes on events and the scene does not draw extra frames.
- **One value, one function**: `{"script": "scripts/waves.js", "fn": "sway", "params": {"speed":
  2}}` calls `sway(value, ctx)` on every frame with what it returned the previous time and
  `ctx.path`, `ctx.layer` and `ctx.params`; one module thus serves many properties. The `params` are
  declared only in the script entry in `scene.json`, with type, default value and limits: the module
  does not export them and `reiwand-ctl scene act FOLDER import_script` does not propose them, but
  it keeps them when the module is imported again. In the editor they are written in a table typed
  per module (id, type, default value and, for numbers, minimum and maximum) and each property
  follows them with *Follow a script*, which lists the functions of the module.
- **Layers**: `scene.layer(id)` reads the position, scale, rotation, opacity, color, size, text and
  the rest as the scene evaluated them, and writes those fields, which last until the script
  changes them or calls `reset()`. `layer.effect("name")` turns an effect off, changes its
  parameters within their range or empties what it holds. With templates (`templates`),
  `scene.spawn("bar", {position: [x, y]})` creates up to 256 copies, which together with the layers
  of the scene and those inside each copy add up to at most 512 (a template has at most 256),
  `destroy()` removes them and `scene.order(id, index)` reorders them: this is how visualizers are
  made. A template can also be a `mesh` layer (or a group with them): each copy gets its own puppet,
  with bones, clips, physics, IK and attachments like the meshes of the scene (64 puppets at most
  in total); an `attach`, a `parent`, a `layer:<id>` target, a layer mask (of the layer or of an
  effect) or an effect parameter that takes a layer, inside the template and naming another layer
  of it, points in each copy to the layer of that same copy, and touches on the bones of a copied
  mesh name the bone of that copy; particle systems belong to the whole scene, but
  `particles(layer).from(n, "copy.layer")` and `.wall(n, …)` make a particle layer spawn from the
  layer of a copy or collide with it, and the
  *Make a template* button of the editor copies the chosen layer into `templates` with a free name
  (`<id>_template`, then `<id>_template_2`…; `reiwand-ctl scene act FOLDER duplicate_layer` does the
  same) and the layer bar shows a template alone on the canvas, the way a copy is drawn;
  while it is shown, the layers panel, the canvas handles (move, bones, chains, weights, points and
  attachments) and every layer action work on the layers of that template with their own ids
  (`reiwand-ctl scene --inside TEMPLATE …` likewise), and since a template has a single layer, new
  ones go inside its group. An attachment, a chain target or a mask of a layer of the template can
  also name a layer of the scene (the inspector offers *Follow a mesh of the scene*, with its bone
  or point), and whatever is not about layers (properties, conditions, settings, resources, canvas,
  particles…) acts on the whole scene even if a template is shown, so renaming or removing a
  property also sees and changes the links of the scene.
- **Input data**: `audio.buffers(32)` returns the left, right and mid bands in `Float32Array`s that
  the engine fills by itself on every frame; `input.cursor` gives the position in fractions, in
  canvas pixels and screen pixels, and the left button; `engine` gives the size of the canvas and of
  the screen, the orientation, the language, the quality, the frames per second and whether it is a
  wallpaper, lock screen or editor. Sound and the pointer still need their permission.
- **Pointer and clicks**: a layer with `"solid": true` (or `{"alpha": 0.5}` to count only where it
  is opaque enough) receives the pointer. Without scripts, a timeline with `"trigger":
  "click:button"`, `"enter:button"` or `"leave:button"` starts on click, enter or leave, and a sound
  with that `play_on` plays. With scripts, `scene.layer("button").on("click", (e) => …)` receives
  `{type, layer, local, scene}` for `click`, `down`, `up`, `enter`, `leave` and `move`; the layer
  that receives `down` keeps the pointer until release, even if it leaves the layer, so dragging is
  following `move` between `down` and `up`, and a click only counts if it is released over the same
  layer. It is declared in `events` and the scene asks for the `pointer` and `clicks` inputs: the
  user allows them per package and, for clicks, also per screen, where they start off. On the phone
  a touch is a click.
- **Parents, camera and scene settings**: with `uses: ["hierarchy"]`, `layer.parent`,
  `layer.children` and `layer.world` (a `Mat3`) tell whom a layer follows, who follows it and where
  it ends up on the canvas, and `layer.setParent("sun", true)` makes it follow another layer without
  moving it or changing the drawing order; `lookAt(from, to)` of `reiwand/math` gives the angle to
  look toward a point. With `uses: ["camera"]`, `scene.camera = {zoom: 2, shake: {amplitude: 6}}`,
  `scene.parallax`, `scene.bloom` and `scene.background` change the camera, the parallax, the glow
  and the background color of the scene, and `null` returns them to what `scene.json` says. The glow
  also turns on in scenes that do not declare it (with the default values of `canvas.bloom` for
  whatever the script does not name) and turns off with `null`; it follows the same quality and
  battery rules and costs nothing until a script asks for it; the script declares it with `bloom` in
  `uses` (the editor adds it by itself when it sees `.bloom` in the code) and the cost of the scene,
  that of each level in the editor and `reiwand-ctl pack check` count it from the start.
- **Custom shapes**: with `uses: ["geometry"]` and a `pools` in `scene.json`,
  `scene.mesh("bars").create({vertices, indices, format: "xyuv"})` creates triangles with
  `Float32Array`s that `update()` changes and `destroy()` removes, within the limits of the pool and
  of 1 MiB per frame. They are drawn in the image of the pool, which a `video` layer shows with
  `"video": "bars"`; a pool that does not change asks for no frames.
- **Saving data**: with `uses: ["storage"]`, `storage.set("visits", n)` and `storage.get` save JSON
  of the package or, with `"output"`, of the screen: at most 64 KiB each, only on the device (not
  synced), in a private file that is written at most every 5 s and on stop, and is deleted when the
  package is removed or uninstalled. The screens that show the package share what belongs to the
  package (what one saves reaches the others on their next frame) and the editor previews read it
  without writing it.
- **Chosen folder**: with `uses: ["folders"]` and a `folder` property with `"mode": "all"`,
  `folder("photos").files` gives opaque identifiers of its images, `name(id)` the file name,
  `on("added" | "removed" | "changed", …)` reports the changes and `show("frame", id)` chooses which
  photo a layer shows. With `folder("photos", {videos: true})` it also lists its videos if the
  property accepts `video` (`kind(id)` says `image` or `video`) and `show("screen", id)` plays one
  in a `video` layer with its sound, with a clock of its own that follows the layer's `playback` and
  that `layer.video` controls (play, pause, seek, speed, volume, end) like the videos of the
  package, up to eight files at a time, videos or photos, on the desktop and on the phone. On the
  desktop, while the scene is in view, the folder is watched with system change notices and only
  read again when the notices calm down (half a second after the last one, 5 s at most after the
  first, so a copy of many files is read once), not counting the hidden files that sync tools write,
  off the thread that draws; when hidden, it stops watching it and looks at it once on return. If
  the folder is missing, is deleted or is moved, it waits in the nearest folder above that exists
  and watches it again when it appears, and it leaves out subfolders that nobody can read. Only on
  network folders (also user-space ones that go to the network, such as `sshfs` or `rclone`, not a
  local disk or a vault) or with more than 8,192 subfolders to watch does it look at it every 2 s
  while in view. The phone lists its copy of the folder again only when the copy changes, and
  rebuilds it when the system reports that the folder changed. Scripts never see paths.
- **Sound**: with `uses: ["sounds"]`, `sound("bell")` controls a sound layer with `play()`,
  `pause()`, `stop()`, `playing` and `volume`, from 0 to 1 and below the general volume, which also
  silences it. The first call takes over the sound even if it had `play_on`. The phone does not play
  scene sounds, so there those calls do nothing.
- **Time and debugging**: `setTimeout` and `setInterval` follow the scene clock (at most 64, once
  per frame and never below 16 ms) and return a function to cancel them; `console.log` and its
  siblings write to the log and to the editor, at most 20 lines per second. `shared` holds values
  common to all the modules (a frozen copy of what is assigned: to change it, assign again), which
  can also be imported from each other with relative paths (`import { media } from "./lib/media.js"`).
- **Math**: `reiwand/math` brings `Vec2`, `Vec3`, `Vec4`, `Mat3` and `Mat4` with their operations
  (product, inverse, decomposition, reflection, projection…), `smoothstep`, `mix`, `remap`, angles,
  `lookAt`, `rgbToHsv` and `hsvToRgb`.
- **Declaring what is used**: each entry of `scripts` lists its `events` and its `uses`; a module
  that declares something can only use the timers, the created layers and the effects if it
  declares them, and `pack build` deduces from that the new functions the package asks for, so an
  older engine warns instead of running it wrong. A module that declares nothing receives the same
  interface and uses what it imports from `reiwand`; the editor lists what it uses without
  declaring.
- **Errors**: `reiwand-ctl pack check` and the editor compile each script without running it and
  report, with the file and the line, a syntax error or a missing script; `reiwand-ctl scene render
  --time-of-day TIME` runs the scripts in their isolated process and writes their console (with
  `--media track.json`, they see an invented media player track). A function that throws an error
  writes it to the console and the rest carry on; a module that fails three frames in a row stops
  by itself, with its timers, and the scene carries on without it with a translated warning in the
  app, the tray and `reiwand-ctl status`. If the scripts run out of time or memory three frames in a
  row, they are turned off and the scene carries on without them.

```js
import { scene, audio, sound, console } from "reiwand";
import { smoothstep } from "reiwand/math";

const bands = audio.buffers(32);
const bars = [];

export function init() {
  for (let i = 0; i < 32; i++) bars.push(scene.spawn("bar", { position: [40 + i * 60, 900] }));
  scene.layer("drum").on("click", () => sound("hit").play());
  console.log("bars ready");
}

export function update() {
  bars.forEach((b, i) => (b.scale = [1, 0.1 + smoothstep(0, 1, bands.average[i]) * 4]));
}
```
