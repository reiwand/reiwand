# Frame rate and automatic scene quality

This page explains how many frames per second each screen is drawn at, and how scenes pick their
quality level by themselves. To choose the level by hand, see
[Choosing the scene quality](choosing-scene-quality.md).

## Frame rate cap

`playback.max_fps` is `display` by default (*Match the display* in the app).

- **On power**, each screen is drawn at its refresh rate at most. That is the rate the compositor
  gives when presenting each surface or that of the `wl_output` mode, the RandR one on X11 or, on
  Plasma, the one KWin gives.
- **On battery**, it is drawn at that rate up to 60 frames per second. Battery lowers the quality
  (resolution and effects), not the pace.
- **A number** sets the cap:

```sh
reiwand-ctl config playback.max_fps 60
```

Except for videos, the GPU sets its own limit:

| GPU | Limit |
|-----|-------|
| Software rendering | 24 |
| Integrated GPU | 60, except light scenes |
| Dedicated GPU | follows the screen up to 240 (60 on battery) |

Videos stay at 30 with `display` and software rendering, and never exceed their own frames.

Nothing is drawn if nothing changes. `reiwand-ctl state` shows the rate (`refresh`) and the cap
(`fps`) of each screen.

## How a scene estimates its cost

Before drawing, a scene computes how much each frame costs. It counts its effects, the layers drawn
separately and the blends that copy the background, each by the part of the screen it occupies. It
then chooses the highest quality level that fits the GPU.

The effects of an image, a video or a mesh run at the density of their image. If the screen
enlarges it (a 2560×1440 canvas on a 1440×3040 phone shows each texel on 2.1 pixels), they work on
their own texels and the result is enlarged when composing, like the image itself. At Q3 they look
the same and cost a quarter.

| Level | Effect density | Canvas drawn at, if the screen enlarges it |
|-------|----------------|--------------------------------------------|
| Q3 | full | full size |
| Q2 | 0.75 | one pixel per canvas unit (between three quarters and full size) |
| Q1 | 0.6 | 0.85 (between 0.6 and three quarters) |
| Q0 | 0.5 | |

The cost counts the densities that way. The reduced canvas sizes are pixels that would have no
more detail.

Old integrated GPUs (Intel HD and UHD, AMD APUs with Vega and similar) and computers with less than
4 GiB of memory have an intermediate level of their own.

## Measuring the graphics card

The first time a graphics card is used with a driver, a short render of about 300 ms measures how
long it takes and adjusts its class.

- The class is stored in `~/.local/state/reiwand/gpu-tier.json`, one per card.
- It is measured again only if the driver of that card changes.
- The measurement moves the class one step at most.
- If it lowers the class (a start with the computer busy can do so), it is repeated on the next
  starts, up to three times, and the best counts.

## Measuring each frame

While the scene moves, the time that each frame adds on the GPU, particles included, guides the
quality. That is what runs from when the GPU finishes the previous frame until it finishes this
one. So neither the wait behind a frame still on the GPU nor the passes that run at the same time
on tile GPUs count. The per-pass measurements only split that time among the effects.

Without GPU timestamps, it measures one frame in fifteen by waiting for the GPU.

## The quality pilot

On the computer and on the phone, the level is guided only by the measured time, with the same
pilot. The cost model only gives the level of the first frames.

From the twelfth frame, the average of the last 32 counts. Frames that exceed double or fall below
half of their median are left out, so a stray spike does not count. The average is compared with
the interval of the pace at which it is drawn (the cap you choose or that of the screen).

| Condition | Result |
|-----------|--------|
| For 2 s it exceeds 85 % of the interval | It goes down a level. |
| For 6 s it does not reach half (or, once it has measured the level above, the average by what that level made more expensive fits in three quarters) | It goes up one, up to Q3. |

The rise stops below Q3 with power saving, heat or the cap of the package.

- On the phone it also goes up on battery.
- On the computer it does not go up on battery, and keeps the reason ("on battery") while it stays
  at the starting level.

To avoid jumping between two levels:

- At least 5 s pass between two changes.
- A rise that drops again in under 30 s doubles the wait of the next one (up to four times).

With `auto`, a scene that at the lowest level takes more than double the interval for 2 s stays
still on its poster until you change the choice or it is loaded again.

*Settings* shows the measured time, the target and the last decision. The rate does not depend on
the level: 60 frames per second on battery unless you choose another, except for what follows.

## Half pace

The pilot decides a pair, the level and a divisor of the pace, and judges against the interval of
the pace in use. `playback.auto_pace` has two values:

| Value | Behavior |
|-------|----------|
| `detail` (the default value) | With a pace of 60 or more, a level that does not fit in the whole interval but whose expected average fits in three quarters of the double is reached at half pace (30 frames per second at 60 Hz, never fewer than 30). |
| `smooth` | The pace never changes, nor does a fixed level, and the cap you choose bounds it. |

With `detail`:

- If the level at half pace would also fit whole (less than half of the whole interval for 6 s),
  it goes back to the whole pace before going up another level.
- An overload goes down first to half pace and only then a level.
- On the computer, half pace draws that screen at half the frames.

*Settings* shows it in the decision ("q2 at 30 fps"). The app and this command change it:

```sh
reiwand-ctl config playback.auto_pace detail|smooth
```

`reiwand-ctl scene bench --quality auto` prints the pace.

It presents with three images so that the GPU draws one frame while the next is prepared.

## Where to see it

`reiwand-ctl state`, the app and the tray show the measured class, the level and why it dropped.
They also show the GPU time measured per frame against the interval, the pace and the last decision
of the pilot.

| Field in `reiwand-ctl state` | Meaning |
|------------------------------|---------|
| `tier`, `tier_measured` | the measured class |
| `quality` (each screen) | the level |
| `quality_reason` (each screen) | why it dropped |
| `quality_frame_ms` | GPU time measured per frame |
| `quality_period_ms` | the interval |
| `quality_pace` | the pace |
| `quality_step` | the last decision of the pilot |

The `MEASURED` column of `reiwand-ctl quality` shows the measurement too.
