# The parts of the editor

- **Layers**: a tree with all the layer types (image, video, text, shape, color, spectrum,
  particles, sound, group and adjustment) to add, remove, duplicate, raise and lower, group (Ctrl
  and click choose several), move into a group or out of it, hide (or mute a sound), lock, and leave
  a layer *solo* in the preview. Each row's handle drags it to another place in the list, also
  inside another group or out of it (the same action as `reiwand-ctl scene move-layer`).
- **Canvas**: the preview is drawn by the daemon itself on a virtual output that arrives through
  PipeWire, like the Plasma wallpaper, so it needs the KPipeWire QML module. Ctrl and the wheel
  zoom in, the wheel or a drag with the middle or right button scroll. An unlocked layer is dragged
  directly, and the chosen one rotates, scales, tilts in depth (round handle on the left edge,
  `transform.tilt`) and moves nearer or farther (diamond at the bottom, `transform.z`) with its
  handles, and a tilted or distant layer shows its outline as a dashed line with the same
  perspective the scene draws it with; the handles write with the `layer` action, as
  `reiwand-ctl scene act DIR layer`; on an animated value they put a key at the playhead
  (`--at S`), also on each part of a position or scale that is animated separately, and a part that
  follows a link rejects the drag; with *Write over keyframes* in the bar they leave a fixed value
  over the keys or links (`--replace`); a drag that starts outside the layers, or any with Shift,
  traces a rectangle that chooses the layers it touches, like Ctrl in the list, ready to group them.
  The guides mark what is seen on 16:9, 21:9, 32:9, 4:3, 16:10 and 9:16 screens. *Screens* shows the
  scene spread over two or three screens side by side, two stacked or your own, with the frame of
  each and what the frames between them cover darkened, and *Spread over every screen* asks for it
  in the manifest. *Show on the desktop* takes the unsaved scene to all the screens without applying
  it; it is removed when you turn it off or close the editor, and at most a minute later if the app
  closes abruptly.
- **Inspector**: it is generated from the scenes' JSON schema (`reiwand-ctl scene schema`), so new
  fields of the format appear by themselves: numbers with a slider and unit, colors, options,
  vectors, curves (a click adds a point, you drag it and a double click removes it), assets,
  effects with their parameters and the return to the default value of each optional field. The
  names of the fields and of their values (blend modes, shapes, interpolation curves, effects,
  layer and asset types, languages, *Match the display* for frames per second) come out in the
  desktop's language, and a test walks through the inspector of each starting point with all its
  variants and fails if anything it shows has no Spanish translation. The new layers and options
  that the app creates carry names and texts in that language. The diamond of each animatable value
  puts or removes a keyframe at the current instant or links it to a user property, to the sound or
  to the pointer, and *Edit the source…* adjusts the timeline, the scale or the smoothing of that
  link. Above the fields, a layer offers its blend mode, the canvas switches and sliders for glow
  and parallax (they change the scene live while you drag them and each drag is a single undo step,
  even if it pauses halfway; so with all the editor's sliders, the range ones too, and with the
  canvas handles; undoing halfway through a drag leaves the rest as a step of its own), and a slider
  property its minimum, maximum, step and default value; *Import a sprite sheet or a data map…* in
  the assets panel cuts an image into rows and columns or reads it as data. In the terminal
  `reiwand-ctl scene act DIR effects`, `scene act DIR layer`, `scene act DIR manifest`,
  `scene act DIR property`, `scene act DIR canvas` and `scene import --linear --atlas ROWS,COLUMNS`
  do the same, with the same checks.
- **Timeline**: play, pause and move the playhead (the preview stays at that instant), one track
  per animated value with its keyframes, which are dragged to another instant, the interpolation of
  each one and the timeline that drives it; *Curve and base of the keyframe* drags live the two
  Bézier handles of the curve up to the next key, a single undo step per drag even if it pauses
  halfway, also on positions, colors and lists of numbers, where the handle's height is the share of
  the way between the two keys (except between two keys written as a single number, such as a scale
  `2`, which curve in units of the value, the same in the preview as in the wallpaper); and the list
  of timelines to create, rename, remove and edit them: *Edit the timeline and its events…* opens
  its panel with the duration in seconds or in frames and frames per second, how it repeats, the
  blending of the end into the start, the speed (slider and field), the delay, whether it starts on
  its own and what starts it, and a strip of named events: a double click adds one at that instant,
  they are dragged to another, renamed and removed (`reiwand-ctl scene act DIR timeline` with
  `add_events`, `drop_events` and `clear_events`).
