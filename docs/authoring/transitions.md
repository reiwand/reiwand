# Make and try your own transition

A wallpaper can bring its own transition for when it comes in ("When it comes in") and another for
when it leaves ("When it leaves"). Without its own, whoever uses it sees the one chosen in their
settings. The format saved in `scene.json` is in [effects](effects/own-shader.md#your-own-transition) and in
[`examples/own-transition`](examples/own-transition/scene.json).

## Make it

1. In the editor, open **Options** and scroll down to **Transitions**.
2. Press **Make a transition…** under "When it comes in" or "When it leaves". If the wallpaper does
   not have a transition effect yet, the editor writes a starting one, "Soft reveal"
   (`transitions/opening.json` and `transitions/opening.wgsl`), which opens the scene from a center.
   If the other direction already has an effect, the new one uses the same.
3. The tool opens:
   - the canvas shows the crossing between a **previous wallpaper** and your scene, with two labels
     ("... leaves", "... comes in · 45 %");
   - at the bottom, **Progress** moves the crossing by hand and the curve editor shows how it
     advances over time;
   - on the right, the panel has the **Effect**, the **Duration** and the values of the effect.
4. Press **Ready** to go back to the normal canvas. The transition is saved with the scene.

## Adjust it

- **Duration**: from 0.1 to 10 s. A value outside that range is not saved and the panel says why
  next to the field.
- **Curve**: the **Curve…** button at the bottom brings the named curves ("soft at the end, medium",
  "even"…). The two handles of the curve editor draw a curve of your own; with Tab you reach each
  handle and the arrow keys move it.
- **Effect values**: numbers are adjusted with their slider in the panel. Points (for example the
  center of "Soft reveal") are handles on the canvas: drag them, or move them with the arrow keys
  after reaching them with Tab.
- **Effect**: the list brings all the descriptors of the package with `"stage": "transition"`. To
  write your own, follow the guide in [effects](effects/own-shader.md#your-own-transition) and put it in the
  package folder; it appears in the list.
- **Remove the transition** deletes that direction. Undo brings it back.

## Try it

- **Try the transition** plays it in full with its duration and its curve.
- **Previous wallpaper** chooses what it is tried against: a plain color or an image of yours.
- The crossing looks as it does in **See as**: change the computer at the top to try each quality
  level. If the transition does not play at a level, the panel warns you and the canvas shows the
  simple fade that whoever uses it would see.
- With **Less motion**, whoever uses it always sees a simple fade; the canvas also shows it that way
  when you turn it on in the app.

## From the terminal

All of the above is the editor's `transition` action:

```sh
reiwand-ctl scene act DIR transition '{"way": "in", "make": true}'
reiwand-ctl scene act DIR transition '{"way": "in", "duration_ms": 1800, "curve": "out_cubic"}'
reiwand-ctl scene act DIR transition '{"way": "in", "params": {"center": [0.3, 0.6]}}'
reiwand-ctl scene ask DIR transition
reiwand-ctl scene act DIR transition '{"way": "out", "remove": true}'
```

`scene actions transition` shows all its fields. The action checks the same things as the loading
of the wallpaper and never writes anything that the check rejects.

## Limits

- While a transition plays, the program keeps three images the size of the screen in graphics card
  memory for each screen: the wallpaper that leaves, the one that comes in and the crossing. On very
  large screens or with many screens that adds up; it ends when the transition ends.
- A transition costs at most 8 units and does not use images, color tables, masks or emitters; if
  the effect asks for more, the panel says so and it is not saved.
- The files of the starting effect stay in the folder even if you undo the creation; delete them by
  hand if you do not use them.
- The preview of the crossing uses a still picture of the scene at the moment of the editor's clock,
  not the animated scene.
