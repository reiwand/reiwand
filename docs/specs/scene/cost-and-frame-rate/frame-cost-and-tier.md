# The cost of a frame and the tier

On this page: how a frame is estimated and the tier of the graphics card.

## The cost of a frame

The engine estimates what each frame of a scene costs before it draws it, without the GPU. A unit
is one full-screen pass that reads one texel per pixel. Each layer adds the units of its effects
(taps × passes × scale², at the size the quality level gives them), one unit when it is drawn
apart (effects, masks, `cache`, blend modes that are not `normal`, `add`, `multiply` or `screen`)
and two more when its blend mode copies what lies below it, all multiplied by the share of the
canvas the layer is drawn over. That share is the area the effects actually cover, clipped to the
canvas: an adjustment layer covers its whole target (the canvas, or its group, which an adjustment
layer widens to the canvas) whatever `size` it declares; a group covers the bounding box of its
layers, not the sum of their areas; any other layer covers the box around its `size` (an image or
video without one, text, particles and lights count as the whole canvas) moved, turned, skewed and
scaled by fixed transforms of its own and its groups, grown by the `reach` of its effects. A layer
whose place can change while it plays (a bound or animated `position`, `rotation`, `skew` or
`scale`, a `tilt`, an `anchor`, `parallax`, a `parent`, an `attach`, a bound `reach`, or any script
in the scene) counts as the whole canvas. A
single-pass effect that the render draws only over a box (`bounded`, or a fixed texture mask in `strength` mode that is
not inverted, feathered or weaker than 1) counts its units times the share of its texture that the lit texels' box covers
(the box of the channels the mask reads, or of red, green and blue for `bounded`; the box of both when it has both),
with a floor of 5 % for the fixed cost of the pass; the box grows on every side by the `reach` of the effect (its value over the canvas size). The boxes come from the
pictures the renderer already decodes (`SceneAssets::ink_scene`, before the quality level is chosen), so loading a
wallpaper decodes nothing twice; where no renderer exists (editor, `pack check`, `scene catalog`) `Scene::read_inked`
decodes them. A texture that does not clamp counts whole, as does a multi-pass effect. `pack check`, `scene catalog`, the effect panel (`reach` query), the phone and the
automatic quality level all use that same function (`Scene::effect_units`). A
layer that masks or feeds others adds one more unit times its share, once however many read it,
since it is drawn on its own at most once a frame. Particles add their overdraw (`capacity·size²·min(aspect, 1/aspect)` over the canvas), `canvas.bloom` adds the units of its effect over the whole canvas, a
`canvas.camera` makes every layer count as the whole canvas, and every frame costs 1.5 units for composing and copying.
An image, video or mesh layer with effects runs them at the density of its picture: its target holds
at most one texel of the picture per pixel of the screen, so a picture that the screen magnifies
(a 2560×1440 canvas filling a 1440×3040 phone shows each texel on 2.1 pixels) runs its effects on
its own texels and the composite scales the result up, as it scales the picture itself. At Q3 an
image that is neither turned nor stretched lines its target up with its texels, so the picture is
copied as it is and only the effects change; a turned image or a mesh keeps one and a half times
that density. Lower levels multiply the density by 0.75 at Q2, 0.6 at Q1 and 0.5 at Q0, also on a
screen that does not magnify the canvas. The canvas itself renders at full size at Q3 and half size
at Q0; at Q2 it renders at one pixel per canvas unit, between three quarters and full size, and at Q1
at 0.85 pixels per unit, between 0.6 and three quarters, so a screen that magnifies the canvas draws
fewer pixels that hold no more detail. The cost counts each such layer at that density, from the
magnification of the screen (pixels per canvas unit), the fixed scale of the layer and one texel
per unit of its `size`. A scene
that costs more at level `q2` than the 8 units of older integrated and phone GPUs gets a warning
from `reiwand-ctl pack check`, `pack build` and the scene editor, since those devices show it with
fewer details. The quality level is the highest whose cost fits the budget of the device: 4, 8, 16
or 40 units for the tiers of software rendering or GPUs without compute shaders, older integrated
and phone GPUs (Intel HD and UHD, AMD Vega and other older APUs, Mali-G up to G710, Adreno 5xx and
6xx, or less than 4 GiB of memory), newer integrated GPUs and discrete GPUs. With the automatic choice, a scene over budget at the
lowest level stays at `poster_time`.

## The tier of the graphics card

The tier starts from the model of the graphics card, as above. The first time a program renders
scenes with a graphics card and driver, it times a short render: frames of 16 passes over a
1920×1080 target, each reading a texture of that size, for about 300 ms, and the time of a unit is
taken from the fastest quarter of the frames. A unit of at most 0.2 ms gives the tier of discrete
GPUs, 0.5 ms that of newer integrated GPUs, 2 ms that of older integrated and phone GPUs, and slower
ones the lowest tier; the measure moves the tier of the model by one step at most, up or down, and
less than 4 GiB of memory or textures smaller than 8192 pixels still cap it.
Software rendering and GPUs without compute shaders keep the lowest tier without a measure. The
result is kept in `gpu-tier.json` in the state folder (`$XDG_STATE_HOME/reiwand/`, shared by
the daemon and `reiwand-ctl scene bench`) or in the files of the phone, with the name, ids and backend
of the adapter and the version of its driver, and it is measured again when any of them changes.
The file is a list that starts with the last card and keeps up to seven other cards, so a
computer that switches between two cards (`render.gpu` in `dbus.md`) measures each of them once.
A measure that lowered the tier, which a busy start can cause, is taken again on the next starts,
three times in all, and the fastest one counts (`runs` in the file).
`GetState().tier` (`"t0"` to `"t3"`) and `tier_measured` tell the result, and the phone lists it
among its diagnostics. A phone takes the tier of its GPU and this measure, at most that of newer
integrated GPUs, and the lowest tier when Android marks it as a low-memory device. Its settings can
record the performance of the shown wallpaper: while that switch is on, each frame that the home
or the lock screen presents while it is visible (never a preview) keeps its time, the delay of its
callback after the vsync, the time of the whole frame loop, the processor time of the scene, of
encoding and of presenting, its GPU time, passes, draws, allocations and the refresh rate of the
display, and every 15th frame times each pass, as `reiwand-ctl scene bench --passes` does. A button
copies a report of the last 30 seconds for the screen shown last: it gives the frames presented,
the seconds the screen was shown (from the moment it became visible to the moment it was hidden,
while any engine showed it) and the seconds it was animating (between frames of the same run,
since hiding the wallpaper starts a new run), and the rate both per second shown and per second
animating, so a wallpaper that stays still while shown lowers the first one. It also gives the level, its
reason, the render and screen sizes, the rate, its cap and the present mode. With the switch off a
frame costs one atomic read and nothing is measured.
