# Editor panels

## Canvas bar

Above the canvas there is a 44 px bar in two groups with text. On the left, what changes the view:
the zoom ("100 %", a menu with zoom in, zoom out and fit), "Try the pointer" (the mouse over the
preview acts as the scene's pointer and clicks), "Guide: …" (no guide or the chosen screen
template) and "Several screens" (one screen, two side by side, two stacked, three side by side, my
screens or span across all). On the right, "See as: <computer>", which opens the see-as panel (the
four computer levels, with battery, reduced motion, language, comparing two levels, restarting the
preview and the fold "Effects on this computer"), and "Capture", which saves an image or a clip of
what the canvas shows with the Try values. Below, the row of what is chosen names the layer (or
"Canvas" if nothing is chosen) and offers what is done with it on the canvas: fit to the canvas
(cover, contain or stretch), before and after, canvas and clock, and "More" with follow a layer,
magnets and the rest of its tools. While a particle system or a particles layer is chosen the row also names
its handles, "Emitter handles", "Force arrows" (pulls, distances and swirls), "Control points" and
"Colliders" (`SceneSwarmBar`), each checked while the canvas draws them; unchecking one hides that
kind for every system until the app closes (`SceneSwarmShow`, not saved), and the four are on when
it starts.

## Right side

The right side (320 px) shows at the top the name of what is chosen and its type, and below its
values, each in a row with the name on the left and the control on the right; nothing it shows goes
beyond its width, not even with the smallest window: a long name is shortened with "…" and a long
text is split into lines. On a layer, the everyday values go at the top (position, size, opacity,
visible and anchor), then "Moves on its own" and the list of effects by name, with "Add an effect";
when an effect is chosen, its card opens under the list with its values. The rest goes in folded
sections that remember whether they were left open: that of the layer type ("Image and crop",
"Text and color", "Shape and fill", "Particles", "Light", "Model and materials"…), "Color and
blending", "Depth" and "Mask". With the canvas chosen it shows the glow and parallax with their
values, the ambient light (the sphere with the surrounding light and that of the sky), the scene's
lights, the preview quality and what it costs at each level.

## Problems

Below the values, each problem is a row in a person's words with the name of the layer ("layer: what
happens"), "Go", which chooses the value, and "Fix" when the editor knows how to repair it; the
technical pointer stays under "Details". Without problems the panel is one line, "No problems". A
script of the preview that stopped with an error (`GetState().outputs[OUTPUT].error`) is a problem
too: it is shown at the top, "Error in the script, line N" with the engine's text and "Go to the
line", which opens the script at that line in Code, and it counts in "Problems (N)", in the tab and
in the status bar (`SceneScriptFault`). The script console stays in Console.

## Timeline

The lanes are grouped under the name of their layer, each lane named by its value, and a ruler over
them marks every half second and writes the seconds; a click or a drag on the ruler moves the
playhead. On a window narrower than 1500 px the footer starts folded on its header even with keys,
to leave the canvas its room, and its caret unfolds it.

The Timeline panel starts with a header that says the clock in words ("Lasts 10 s · repeats") and
"Change…", which opens the menu of the scene's timelines (choose, add and edit each one in its
panel), then, while it is open on the scene's keys, "Key" and "Trigger". "Key" adds a key at the
playhead to the track of the chosen key, with the value the inspector shows for it there (one
`keyframe`); it is off until a key is chosen. "Trigger" adds an event at the playhead (within the
timeline's length) to a timeline, named `event_1`, `event_2`… like the lane of events of its panel
(`timeline` with `add_events`); with several timelines it asks which, and with none it is off and
says to make one in "Change…". Below them are the playhead, the curve of the chosen key and a lane
per animated value (or the clip line of the chosen mesh or model clip). Under the ruler, before the
values, each sound layer has a lane with the waveform of its sounds one after the other from 0 s
(each as long as its sound, cut at the end of the ruler) and, before it, what starts it ("Starts with
the scene" or "Starts with …"); and while a timeline has events, a lane "Events" marks each at its
second with its name, as seconds from the start of its timeline; a click on a mark moves the playhead
there. The ruler shows with these lanes even when nothing is keyed. While its group sits right under the canvas, in the
canvas's column, the panel is a footer: if the scene has no keys it stays folded on its header, with
"Timeline" to unfold it. Anywhere else (the bottom row of Animation, a side, a group of its own) it
keeps the whole place of its group, keyed or not.

## Effects

The Effects panel shows the effects of the layer chosen, or of the layer of the effect chosen
("Of “Reflection”"), in their order: a click on one opens its values, its cost at each level of the
preview quality (as an effect's inspector says it) and "Open the effect", which chooses it; choosing
an effect anywhere else opens it here too. "Add an effect" opens the catalog. A fold "Package
effects" lists the package's own effects with the same buttons as in Assets (edit the WGSL code or
the descriptor, reimport, remove, import, new effect). Without a layer chosen it says "Choose a layer
to see its effects."; a sound layer has none. While the workbench has the Effects panel (open in any
group), the Inspector of a layer leaves out its Effects section; with the panel closed the section
is back.

## Particles and Cost

The Particles panel (`SceneSwarmTree`) holds the scene's particle systems as the systems list has
them (make, copy, rename, remove, a system without a layer) and, under the system in hand, its parts
in order: its settings, each emitter (by its shape), how particles start, each force, each collider,
the look, the control points and the child systems. Choosing a part selects it, and the Inspector
shows it: the system's settings with its whole editor, a part with its fields.

The Cost panel (`SceneCost`) gathers what the scene costs to draw, from where each cost was shown
before, in five tabs: Scene (the preview quality), Particles (the budget of each system), Lights
(the lights and lit layers per level), 3D (the models) and Effects (what each effect of the layer in
hand costs per level). The number of a single layer stays in the Inspector.

A `workbench.json` of format 6, from before these two panels, is read with them closed in every
layout it holds; the Window menu opens them, and "Reset this layout" places them as the layout came.

## Scripts, Code, Reference and Console

Scripts (`SceneScriptsPanel`) holds the scene's scripts and the `scene.spawn` templates, which were
sections of Assets (see "Scripts and templates"). Code (`SceneCodePanel`) edits a script or the WGSL
of a package effect (`SceneScriptBody`, `SceneShaderBody`; the dialogs they were are gone); every
file opened from Scripts or Effects stays in the list at its top, with what was typed and not saved,
and Ctrl+Tab or Ctrl+Shift+Tab, while Code has the keyboard, goes to the next or previous one. Ctrl+S
saves. Reference (`SceneApiPanel`) is the scripting API, in folds, with a search over names and
descriptions, and the picker of bones, shapes and impulses, which inserts its call in the script open
in Code. Console (`SceneConsolePanel`) shows the console and the error of the scripts of the preview
(`previews[OUTPUT].console` and `error`), with All, Warnings or Errors; clicking a line of a script,
or *Go to the line* of the error, opens that script in Code at that line. Problems keeps the
problems and the inputs a script uses and is not granted. A web package has Console too.

A `workbench.json` of format 7, from before these four panels, is read with them closed in every
layout it holds, as with format 6.

## Curves

The Curves panel is the curve editor (see "Curve editor") in a panel of its own, for the line in
view in the Timeline panel: the keys of the chosen value's layer, or of every animated value, or of
the clip of a mesh or a model. It follows the scene as it changes while it is in view, and its keys
(copy, paste, Del, F, Home) work while it holds the keyboard focus. While the workbench has the
Curves panel, the *Curve editor* button of the timeline bar and of the clips bar brings it into view
and focuses it; without it, the button opens the curve editor as a dialog, as before.
While a link (a bound field whose `smooth` the Inspector offers) is chosen, Curves also has a
*Smoothing* tab, which it opens: the same `SceneSmooth` control as the Inspector, on the field host
of that link (the plot of the `response` query and the seconds up and down), writing `smooth` as the
Inspector does. With no link chosen the tab is not there and Curves is as before.

## Layers and Assets

The head of Layers, under its buttons, has the blend mode and the opacity (in percent) of the chosen
layer, the same values as its Inspector, for any layer but sound and light. Assets lists its assets
in groups by kind (images, videos, sounds, meshes and models, fonts, colour tables), each under a head
with its count; a click on a head folds or opens the group, and the filter words and the search
narrow the groups as before.

## See as, Inputs and Screens

The See as panel holds what the canvas bar's "See as: …" opens (`ScenePreviewPanel`): the device
whose quality level the preview draws, on battery, reduced motion, the two levels to compare,
"Restart the preview" and the effects on that device. While the workbench has the panel, "See as:
…" (and View › "See as…") brings it into view and focuses it; without it, the button opens the same
panel under it, as before.

The Inputs panel gathers what the scene would get from its user while it is tried here: "Test the
pointer" (the same state as the button of the canvas bar: the mouse over the canvas acts as the
pointer and its clicks reach the scene), "Try the parallax" when the canvas has parallax, the bands
of the sound playing on the computer as the scene hears them (`SceneBandMeter`), and the simulated
player (`SceneMedia`). The See as panel heads its parts "Device" (a choice per level), "How its
user would see it" and "Compare". While the workbench
has the Inputs panel, the Try panel leaves out its "Simulated player" fold; with Inputs closed the
fold is back.

The Screens panel lists the screen layouts of the canvas bar's "Several screens" (one screen, two
side by side, three side by side, two stacked, the author's own) as choices, with "Spread over every
screen" when more than one screen is chosen, and the guides of "Guide: …"; the panel, the button and
the guide box show and change the same state. With more than one screen it also shows a map: the
box around the screens at the panel's width, cut into the part each screen shows (the same parts the canvas frames,
from the query `spread`), each with the thumbnail of the package under it, or a plain tile when it
has none, and the screen's name.

While the Timeline panel is not in view (another tab of its group is shown, or it is closed), the
canvas shows a row of its time at the foot of the canvas (`SceneCanvasTime`): play or pause, the clock
in seconds, the playhead, "Restart the preview" and the *Canvas and clock* menu. It adds no control
of its own: play, pause and the playhead are those of the timeline bar.

## The wallpaper panels

The Wallpaper sheet shows the package's own data, the fields the Inspector shows with the package
chosen (`/manifest`), in sections named on its left (a click scrolls to one): Data (title,
description, notes of the version, version, authors, tags, accent, base language), Rating and
flashes (content, photosensitive), Inputs and permissions (inputs, permissions, web), Device and
platforms (requires, platforms, playback, display), Credits and license (license, credits, origin,
links), Dependencies (`requires.packages`) and Advanced (every other field). A section with no
field is not drawn.

Texts and languages holds the base language and every text of the package shown to people (title,
description, notes of the version and the package's strings), each with its translation editor.

How it will look draws the package as a cover: its thumbnail in 16:9 (or the note that it has none),
its title and kind; beside it, when the panel holds two covers of at least 280 px, another cover at
rest, dimmed as a cover that is not chosen (the thumbnail of a scene that comes with the app, not the
one being edited), with "next to another cover, at rest" under it; "Update the thumbnail" (the
capture of the canvas the publishing page offers); "Record 6 s" for a scene or a video, which records
six seconds from the second the preview is on as the animated preview (`shot` with `recording`, 10
frames a second, as a WebP that keeps every color); and the thumbnail, animated preview and accent
fields.

Review holds the path to publishing: the three numbered steps described under publishing below
(Review, Sign, Publish) and its «Done» part, one under the other in the panel, so checking the
package, answering its questions and signing it happen beside the canvas. *Publish…* in the top bar
chooses the Wallpaper layout and brings this panel into view at the step publishing is at. The
steps are refreshed each time the panel comes into view.
