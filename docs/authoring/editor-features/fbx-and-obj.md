# Converting FBX and OBJ models to glTF with Blender

Reiwand reads only binary glTF (`.glb`). For a model in FBX or OBJ, with Blender 3.6 or newer:

1. Open Blender, delete the cube of the initial scene and use *File → Import → FBX* (or *OBJ*). If
   the model comes with separate textures, leave the images next to the file so Blender finds them.
2. Check the scale and the orientation: glTF uses meters, +Y up and −Z forward; the exporter
   converts Blender's axes, so it is enough for the model to look upright in Blender. Apply the scale
   and the rotation with *Object → Apply → All Transforms*.
3. Review the materials: the exporter translates the *Principled BSDF* (color, metal, roughness,
   normals, emission and alpha); other nodes are lost. For a texture to travel inside the file,
   connect it directly to the *Principled BSDF*.
4. Use *File → Export → glTF 2.0*, choose the *glTF Binary (.glb)* format and, under *Include*,
   *Selected Objects* if you only want a part. Leave *Normals*, *UVs* and *Tangents* on, and under
   *Animation* the actions and the blend shapes if it has them. Each Blender action arrives as an
   animation of the file with its name; armatures arrive as skins of up to 4 bones per vertex (in
   *Mesh Data → Weights*, *Limit Total* to 4 before exporting if needed). For foliage, paint the
   vertices' color in *Vertex Paint*: the red says how much each vertex sways with the wind.
5. If the file weighs a lot, reduce textures to 2048 pixels or less (what phones receive) and use the
   *Decimate* modifier to lower triangles; `reiwand-ctl scene ask DIR model_info` says whether it
   fits in each equipment class.
6. Import it in the editor with *3D model from a file* or with
   `reiwand-ctl scene import DIR model.glb --id model` and
   `reiwand-ctl scene act DIR scene3d '{"model":{"add":{"asset":"model"}}}'`.

Files that load images or buffers from outside, or that require unsupported extensions (such as
Draco compression), are rejected: export them without mesh compression.

Each change is an operation on a JSON Pointer. Undo and redo (Ctrl+Z and Ctrl+Shift+Z) keep up to
500 steps or 64 MiB, and a drag counts as a single step. What has not been saved is copied every 30
seconds and on closing to `~/.local/state/reiwand/editor/`, which only your user can read, and the
app offers to recover it when you open the folder again; the copy is deleted when closing with
everything saved and those unused for 30 days delete themselves. Importing, building and applying
work in the background, without blocking the window. *Publish…* opens a three-step page (review,
sign and choose where) that creates the `.wpak` with the same code as `reiwand-ctl pack build`, signs
it with your author key (like `reiwand-ctl pack sign`) and saves it, uploads it to your server or
adds it to a repository; *Set as wallpaper* applies it to one screen or to all.

Every editor action can also be done with `reiwand-ctl scene act DIR ACTION 'JSON'`, which works on
the folder without a daemon, saves immediately and shares with the app the undo history, where each
action is a step; `reiwand-ctl scene actions` lists them and `scene actions NAME` gives its shape and
an example (all in [`docs/authoring/actions.md`](../actions.md)). `preview` and `apply` use the service:

```sh
reiwand-ctl scene new sea --from parallax --title Sea --locale en   # empty, particles, parallax, clock or web
reiwand-ctl scene open copy --from scene.wpak            # an unsigned, derived copy, with the id <id>.copy
reiwand-ctl scene import sea wave.ogg                       # copy an asset into assets/
reiwand-ctl scene import sea foam.png --atlas 4,4         # sprite sheet; --linear for data maps
reiwand-ctl scene act sea add_layer '{"kind":"sound","source":"wave"}'   # and remove_layer, duplicate_layer, move_layer
reiwand-ctl scene act sea add_layer '{"kind":"text","name":"Title"}'   # id «title», with that text
reiwand-ctl scene act sea add_layer '{"kind":"mesh","file":"fox.glb"}'   # imports the .glb and shows it in a mesh layer
reiwand-ctl scene act sea set '{"pointer":"/scene/layers/1/opacity","value":0.5}'   # any field by its JSON Pointer
reiwand-ctl scene act sea keyframe '{"pointer":"/scene/layers/1/opacity","time":2,"value":1,"ease":"in_out_sine"}'
reiwand-ctl scene act sea keyframe '{"pointer":"/scene/layers/1/opacity","time":3,"from":2}'   # move it to another instant
reiwand-ctl scene act sea timeline '{"id":"waves","add":true,"duration":6,"loop":"repeat","add_events":[{"t":3,"name":"wave"}]}'   # and to, remove, frames with fps, drop_events
reiwand-ctl scene act sea keyframe '{"pointer":"/scene/layers/1/opacity","time":0,"value":-0.5,"timeline":"waves","base":0.5}'   # relative to 0.5
reiwand-ctl scene act sea keyframe '{"pointer":"/scene/layers/1/opacity","time":0,"out_handle":[1,0]}'   # Bézier handle up to the neighboring key (here the 3 s one); also in_handle
reiwand-ctl scene act sea keyframe '{"pointer":"/scene/layers/1/opacity","time":0,"straighten":true}'   # straight up to the next key, without touching the neighboring curves
reiwand-ctl scene act puppet clip '{"layer":"fox","id":"greet","add":true,"play":true}'   # a new clip that already plays on an animation layer
reiwand-ctl scene act sea translate '{"pointer":"/manifest/title","language":"de","text":"Meer"}'   # another language; "add":true overwrites nothing, without "text" it removes it
reiwand-ctl scene act sea set '{"pointer":"/manifest/content","value":{"rating":"mature","descriptors":["violence"]}}'
reiwand-ctl scene thumbnail sea capture.png                # thumbnail in assets/, shrunk if needed
reiwand-ctl scene act sea add_property '{"kind":"combo","id":"weather"}'   # and move_property, rename_property, remove_property
reiwand-ctl scene act sea set '{"pointer":"/manifest/properties/weather/options","value":["calm","storm"]}'
reiwand-ctl scene act sea set '{"pointer":"/manifest/properties/weather/default","value":"calm"}'
reiwand-ctl scene act sea add_property '{"kind":"slider","id":"speed","label":"Speed"}'
reiwand-ctl scene act sea translate '{"pointer":"/manifest/properties/speed/label","language":"de","text":"Geschwindigkeit"}'
reiwand-ctl scene act sea condition '{"id":"speed","text":"weather == \"storm\"","enabled":true}'   # checked as in validate
reiwand-ctl scene act sea add_preset '{"label":"Calm"}' && reiwand-ctl scene act sea set '{"pointer":"/manifest/presets/0/values/weather","value":"calm"}'
reiwand-ctl scene act sea effects '{"add":{"layer":"far","effect":"vignette","params":{"intensity":0.5}}}'   # and set, remove, move, mask
reiwand-ctl scene act sea layer '{"transform":{"layer":"far","position":[960,400],"scale":[1.2,1.2],"tilt":[15,0],"z":40}}'   # and blend, instance
reiwand-ctl scene act sea property '{"id":"speed","min":0,"max":10,"step":0.5,"default":2}'
reiwand-ctl scene act sea manifest '{"tags":{"tags":["sea","night"]}}' && reiwand-ctl scene act sea manifest '{"license":{"license":"CC-BY-4.0"}}'   # and origin, authors, display
reiwand-ctl scene act sea canvas '{"bloom":{"strength":1.5}}' && reiwand-ctl scene act sea canvas '{"parallax":{"amount":2}}'
reiwand-ctl scene act sea group '{"ids":["near","far"],"id":"hills"}' && reiwand-ctl scene act sea hide '{"id":"sky","hidden":true}'
reiwand-ctl scene act sea undo && reiwand-ctl scene history sea
reiwand-ctl pack check sea                             # problems, each with its JSON Pointer
reiwand-ctl scene preview sea -o DP-1 --time 2             # see it on the desktop for 60 s without applying it
reiwand-ctl scene preview sea --layout dual --out dual.png # spread over two screens, without a desktop
reiwand-ctl scene preview sea --layout dual --out dual.png --time 4   # second 4 instead of the poster
reiwand-ctl scene preview sea --quality q1 --battery --calm   # as the daemon draws it at level Q1, on battery and with less motion
reiwand-ctl scene render sea --out sea.png --quality q1 --battery      # one frame to a file, without a desktop
reiwand-ctl scene render sea --out sea.gif --frames 90 --pointer 0.2,0.5:0.8,0.5 --click   # a clip with a simulated pointer
reiwand-ctl scene render sea --out glow.png --against q0 --without-bloom   # compare levels or the glow, split down the middle
reiwand-ctl scene render sea --out time.png --time-of-day 15:42               # runs the scripts with that wall-clock time
reiwand-ctl pack sign sea --key author.key && reiwand-ctl pack build sea   # sign and create the .wpak
reiwand-ctl scene apply sea -o DP-1                        # add it to the library and show it
reiwand-ctl scene apply sea --span                         # show it spread over all the screens
```
