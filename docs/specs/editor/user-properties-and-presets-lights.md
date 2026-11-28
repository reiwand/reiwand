# Lights

The Add layer menu offers Light with its four types. Every light shows over the preview as a marker
to drag, and the selected one draws its handles, in pixels of the canvas through its parents: a ring
at its radius with a handle on it, a square above it whose distance sets the height `z` (half a pixel
of the preview for each pixel of height), for spot lights a handle that turns it and the edges of the
outer and inner cone with a handle on each, for tube lights a handle on each end that moves, turns and
stretches it, and for directional lights an arrow with a handle that turns it. A check box draws the
box a moving light redraws. The marker moves the light with the same `grip` as the layer handles
(through its anchor and the skew of its parents, with snapping), and the aim, tilt, height, radius,
cone and tube end handles and the tilt slider send `light set` with the time of the playhead and the
overwrite switch, so a keyed position, rotation, tilt, height or radius gets a key at the current
time (a tube end moves the middle with `at`, turns it with `aim` towards the other end and sets
`length`), and lights go on the timeline like other layers.
The inspector of a light picks its type and color, slides its intensity, radius, height, length,
cone and, for spot and directional lights, its tilt toward the canvas (also a handle on the aim line,
nearer the light the more it faces the canvas), plots its light against the distance with the curve component and toggles solo and mute.

Selecting the canvas lists every light with its type and solo and mute toggles, how many lights the
scene has against the 12 it may have and how many each device tier keeps (8 on T0 and T1), the
pickers of the light around and from above with a sphere lit by them, and the preview quality picker
of the particle budget, which here shows for each level whether the lighting is worked out once (Q0)
or how many pixel lights it shades each frame while a light moves.

The inspector of an image, video or text layer has a Lighting part (a particles layer has its switch
and the glow and strength sliders): a switch that writes or removes
`lighting`, pickers of the normal map, the reflected picture and the roughness and glow maps, sliders
for roughness, reflection, glow and strength, a note when a map is read as colors, Generate normals (an image, video or text layer), a dialog with
sliders for strength and blur that shows the relief of the map lit from the top left, the map itself
and a lit sphere before it keeps the map, and Paint roughness and Paint glow, which paint on the layer
in the preview with a brush size, hardness and gray while the map shows over it as a heat view.
`reiwand-ctl scene ask DIR lights`, `light … falloff`, `normals --preview`, `paint-map --heat` and
`ambient --preview` print the same data.

A point or spot light has a Light the air part: a switch for `volumetric`, sliders for density,
samples and shafts, the glow plotted against the distance (with shafts, a dashed line shows the glow over a part of the layers below darker than around the light), and for each quality level whether the air is off
(Q0), worked out at a quarter of the size with half the samples (Q1) or at half the size (Q2 and Q3),
with a note that phones work at a quarter; the preview shows the glow live at the picked level, and
the selected light shades the ring of the lit air. A spot light has a Projects picker that lists the
pictures and videos of the scene and the layers it may project without a loop, with a thumbnail of
the picture; the preview draws what the light projects inside the cone where it lands on the
canvas, with a dashed outline: the picture, the frame of the video at the playhead (from the cached
strip of 16 frames) or, for a projected image, video or text layer, its picture, frame or letters
(wrapped and on its background plate, as the text is laid out); a group, shape, spectrum or any other
layer, and a layer with effects or a mask, is drawn alone by the renderer at the playhead over black
and over white (whose difference gives its coverage) and cut to what it covers. When part of the beam runs along the canvas the footprint is drawn
out to the radius of the light, fainter, with a note. The list of lights marks lights that light the air or project, and the cost of each
quality level adds the samples of the air, the pixels of the layers below that the shafts copy (`shaft_copy`, at the size of the air; once while they stay still, each frame while one moves), the lights shaded on particles and the pixels refraction
reads, with the same rows for phones.

The particle tools have a Light part: Take the lights of the scene, a normal map picker and a preview
of a sprite lit from the front and from behind, and Bend what is behind the particles with a normal
map picker, an amount slider, the displacement map, the shift each quality level keeps and a switch
that makes the renderer draw the displacement of the frame on the preview, whether the particles
run on the graphics card or the processor: the scene is dimmed and each refracting particle, trail
or rope shows its shift as a color (red to the right, green up, saturated at 16 px). The app asks
for it with `preview.shift` in the document of `PreviewScene`; it adds no work while it is off. The
particle budget lists how many lights light the particles and whether they refract at each level.
`reiwand-ctl scene act DIR light`, `projection --show`, `particles … lit --show`,
`refract --show` and `lights --cost` print the same data.
