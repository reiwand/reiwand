# Quality levels and the budget

On this page: moving the level while playing, the level the user chooses and the hard budget.

## Moving the quality level while playing

While a scene plays, the GPU time of its frames (measured as described below) moves the quality
level. The desktop daemon and the phone steer by measured time alone, with the same controller
(`Pilot` in `crates/reiwand-scene/src/pilot.rs`): its ceiling is `q3` (one step under the tier level
with power saving, lower while warm or hot, and never above the `cap` of the package), and the
level the cost model fits is only where it starts, before any frame is measured. On the desktop
the level never rises on battery, with power saving or while the GPU or CPU is warm, and the
reason stays the one of the start (`battery`, `saver` or `heat`) while the level holds there; the
phone also rises on battery. It keeps the GPU times of the last 32 frames and,
from the 12th on, averages those between half and twice their median, so a few spikes do not
count, against the frame interval of the rate it plays at (the user's cap or the display rate).
When the average stays over 85 % of the interval for 2 seconds the level goes one step down; when
it stays under half of the interval for 6 seconds (once the level above was measured: when the
average times the ratio measured there fits three quarters of the interval), one step up, unless the power source, power saving or heat hold it. Two changes are at least 5 seconds apart, the window
starts again after each, and a step up that comes back down within 30 seconds doubles the wait for
the next one, up to four times; one that holds for 30 seconds halves it again, so a level measured
too dear is not tried again while the scene costs the same. Under `auto` a scene whose frames take
more than twice the interval at the lowest level for 2 seconds stays at `poster_time` until the
choice changes or the scene is loaded again, with the notice of a still scene (`quality_auto` is
`still`): on the phone the cost model never stops a scene, only the model triangle limit and that
measurement do; the desktop also keeps still a scene whose estimate at `q0` is over the budget of
the tier. The phone settings, the app, the tray and `reiwand-ctl quality` (from `GetState`, see `dbus.md`) show the measured average
against the interval and the last decision (`model`, `steady`, `climbed`, `dropped`, `top`, `held`,
`still`, `halved` or `doubled`), and `--quality auto` of `reiwand-ctl scene bench` and `--level auto`
of the phone bench report the same under `auto` (`frame_ms`, `period_ms`, `step`, `still`, `pace`,
`pace_fps` and every change of level or pace). The pilot decides a pair, the level and a pace
divisor, and the interval it judges is the one of the pace in use. The divisor is 1 (the pace of
the user's cap or the display) or 2 (half of it: 30 frames a second on a 60 Hz display), and the
pace never goes under 30 frames a second, so a cap under 60 stays whole. With `playback.auto_pace`
at `detail`, the default, a level above that does not fit the whole interval, but whose predicted
average fits three quarters of the doubled one, is climbed to at half pace; a level at half pace
that averages under half of the whole interval for 6 seconds goes back to the whole pace before
any level goes up; and an overload halves the pace before it drops a level while the level is above
`q0`, and drops a level once the pace is already halved. `smooth` never changes the pace. The same
waits, cool-downs and bounce guards apply to the pace (a pace that flips again within two minutes
doubles the wait before it returns to whole, up to four times), a level the user fixes never
changes its pace, and the user's cap bounds it (half pace halves the cap). On the phone the halved
pace presents every second vsync of Choreographer, the skipped ones do no work, and the measured
time stays the GPU time of each frame drawn; on the desktop the halved pace halves the rate the
daemon schedules the frames of that output at. The frame rate otherwise never depends on the level
(60 on battery unless the user sets another cap). It
presents through three swapchain images (latency 2), so the GPU draws a frame while the next one is
recorded.
Two changes are at least 5 seconds apart. The quality only moves on devices that time their frames,
which all do, with timestamps or by waiting (see below). `GetState().outputs[x].quality` and
`quality_reason` tell the level and why it is below the highest; the app, the tray and the phone
show both, translated.

## The quality the user chooses

The user can take the level out of these hands with `playback.quality` (see `SetOption` in
`dbus.md`): `auto`, the default, is everything above; `q0` to `q3` fix the level, and `min-q1` or
`min-q2` keep the automatic choice but never let it go below that level. A level the user sets
always holds, also on battery, with power saving, while the device is warm and when the scene costs
more than the tier holds: only the frame rate still follows the tier, the power source and the
temperature, so a heavy scene shows fewer frames a second instead of fewer details. Above a floor,
the automatic choice and the GPU time of the frames move the level as usual. The reason is then
`user`, and when the frames measured at the chosen level take more than 85 % of their interval
(before 12 frames are measured: when the level costs more than the budget of the tier) the app
(next to the quality choice in the settings and on each screen that shows the scene), the tray, the
phone (under its quality setting) and `reiwand-ctl quality` and `reiwand-ctl state` warn: the measured
time against the interval and the rate it allows (or the cost against the budget), that frames may
drop and battery use rise, and whether the automatic choice would lower the level or keep the scene
still. Frames measured to fit silence the estimate. Only the automatic choice stops a scene: a scene
over budget even at the lowest level (on the desktop), one measured far too slow there, or with more
model triangles than the
tier draws, stays at `poster_time` under `auto`
and plays at the chosen level under `q0` to `q3`, and at the floor (or higher, when it fits) under
`min-q1` and `min-q2`; clients say so too, and for a scene heavy only in model triangles, whose
cost fits, they warn that it draws more model triangles than the device holds and that the
automatic choice would keep it still. Only the hard limits that protect memory and the driver
stay with every choice: the model data and triangles a scene may load, texture sizes, and the
effects left out by the hard budget below or after they stopped the GPU, which the warning counts.
The phone keeps its own value, and stopping on battery saver or at a critical temperature still
stops it.

## The hard budget

Every view of a scene also holds a hard budget, whatever the quality level: each effect use, built
in or packaged, does its units (at the highest level, four times for an effect with a `state`, which
may run four steps a frame) times the share it covers times the megapixels drawn, at least half a
unit, and the uses together may do at most 2 000, 4 000, 8 000 or 16 000 of those units a frame on
the four tiers; at most 16 uses with a `state` may run in a scene. The engine adds the uses in the
order of the layers, a group before its layers, and leaves out each use that does not fit with
`the effect <id> on <layer> was left out: the effects of the scene would do more than <limit> units
of work a frame on this GPU` (or `... the scene has more than 16 effects that keep a state`). When
the scene loads, before it builds anything, it leaves out only the uses that cannot fit at any size
(past the 16 stateful uses, or past the limit at half a unit each), so a package effect whose uses
are all left out is never built; it counts the work whenever the size a view draws at changes, at
that size and over the part of the canvas the screen shows, so a large canvas shown on a small
output keeps the effects that fit there. The state textures of all uses together may hold at most
512 MiB, beside the 512 MiB of textures a frame may borrow; a use whose state would go past it is
left out with `its state textures would take the scene past 512 MiB`. A frame may borrow at most
512 MiB of textures, or four full frames of the output if that is more, and a view holds at most
1 GiB of textures in all, or its states, its cached layers and its target plus the frame's share
and two full frames more if that is more (so an 8K output keeps its effects): before it makes
another texture it drops the ones it used longest ago, then the cached layers this frame has not
drawn, and a use whose texture still does not fit is left out with `its textures would take more
than the <n> MiB of textures this view may hold` (or `... a frame may borrow`). Layers draw into textures
whose sides grow in steps of an eighth of a power of two (at least 64 pixels), so a zooming layer
reuses a few sizes. The first pass that writes a target of a package effect clears
it as it draws instead of in a pass of its own, and an adjustment layer reads what is below it
directly instead of a copy. `reiwand-ctl scene bench` prints these notes after its
numbers.
