# What a scene costs

Whether your scene looks good on a phone depends on how much work each frame asks for. The program
estimates it before drawing, without a graphics card, in **units**. A unit is a full-screen pass
that reads pixel by pixel.

- A layer adds the units of its effects, one more if it is drawn separately (with effects, mask,
  `cache` or a blend mode other than `normal`, `add`, `multiply` or `screen`) and two more if its
  blend mode copies what is underneath.
- All that is multiplied by the part of the canvas that the layer covers. An adjustment layer
  covers everything.
- A layer whose position can change (animated, with `parallax`, `parent`, `tilt` or with scripts
  in the scene) counts as if it covered the whole canvas.
- Each frame adds 1.5 units for composing and copying. A `camera` makes every layer count as the
  whole canvas.

### Quality levels

The program draws the scene at one of four quality levels, from the lowest `q0` to the highest `q3`
(in the app: lowest quality, low, medium and highest quality). Each level makes the effects
smaller and leaves some aside. Each type of computer can handle a number of units:

| Computer | Units per frame |
|----------|-----------------|
| No graphics card, or no compute shaders | 4 |
| Old integrated graphics and phones | 8 |
| New integrated graphics | 16 |
| Dedicated graphics cards | 40 |

The level used is the highest whose cost fits. Afterwards, the program measures the real time of
each frame and raises or lowers the level by itself. Running on low battery it goes down one step,
with power saving another one, and with heat it goes down and limits the rate. If a scene does not
fit even at `q0`, the program shows its `poster_time` and does not move it.

The user can set the level (`q0` to `q3`) or a minimum; that setting always rules, also on battery.
Then a heavy scene loses frames instead of detail.

### What to do so it runs on a phone

1. Look at the warning. `reiwand-ctl pack check` warns when the scene costs more than 8 units at
   `q2`: `the scene costs 8.3 units a frame at quality level 2, more than the 8 that older
   integrated and phone GPUs have, so they show it with fewer details`, followed by how to bring
   it down (remove layers and effects that are not needed or lighten the heaviest ones in each
   effect's panel, which shows its cost at each level). In the app the warning is translated. This
   is produced by [`examples/stacked-effects`](../examples/stacked-effects/scene.json), with glow,
   blur, vignette and grain over the whole scene.
2. Prefer a few low-cost effects. The heavy effects (`fluid`, `bloom`, `god_rays`,
   `local_contrast`, `tilt_shift`, `water_ripple`) do a lot of work; mark them with
   `"optional": true` so the program removes them before the others.
3. Use adjustment layers with `size` instead of the whole screen, and masks instead of effects on
   huge layers.
4. Lower `clock.max_fps` if nothing moves fast.
5. Every blend other than `normal`, `add`, `multiply` or `screen` reads what is underneath with a
   copy, which on a phone costs about a millisecond per layer. If the look allows it, use `add` or
   `screen`.
6. Part of the work is skipped by itself: what does not change is not drawn again, and a scene that
   does not move does not ask for frames.
7. A hard cap applies to everyone: among all the effects of a scene only an amount of work per
   frame runs (2,000, 4,000, 8,000 or 16,000 units depending on the computer) and at most 16
   effects with memory of their own (`state`). What does not fit is left out with a warning.

To really measure, build the package and use `bench`:

```
reiwand-ctl pack build MY-FOLDER -o my.wpak
reiwand-ctl scene bench my.wpak --quality q2 --tier t1
```

It prints the estimated cost, the time per frame, the memory it takes and the notes of what was
left out. `--shot frame.png` saves the last frame as an image and `--tier t0` to `t3` applies the
limits of each class of computer (`t1` is that of phones). The same happens with `--quality auto`,
which raises and lowers the level as the phone would. `bench` measures the package as `scene.json`
describes it: it does not run the scripts. To see the result of the scripts at a fixed time,
`reiwand-ctl scene render MY-FOLDER -o view.png --time-of-day 15:42`.

### When the user asks for fewer flashes

As long as the user does not turn on `playback.allow_flashing` (it comes off), packages marked
`photosensitive` are not played and are listed with the warning "Has flashes, turned off"; besides,
the program measures the brightness of each frame and, if the scene flashes more than three times
a second, plays it more slowly until it flashes at most three times, and if it still flashes
faster, it stops on its frame. If your scene flashes on purpose, say so in the manifest with
`photosensitive` (see [package.md](../package.md)).

### When the user asks for less motion

If the system asks to reduce motion, the scene stays at `poster_time`: the parallax, the camera
shake and the timelines stop. The frame is redrawn once a minute, so that clocks stay right.
