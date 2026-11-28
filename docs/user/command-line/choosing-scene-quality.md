# Choosing the scene quality

The quality level of scenes can be chosen by hand with `playback.quality`, for all screens or for
one. This page covers the values, what the warnings say, and the limits that always stay. How the
automatic choice works is in [Frame rate and automatic scene quality](frame-rate-and-automatic-quality.md).

## Values

| Value | Meaning |
|-------|---------|
| `auto` | As always: the automatic choice. |
| `q0` to `q3` | A fixed level. |
| `min-q1`, `min-q2` | Keep the automatic choice but never below that level (it can go higher). |

What you choose is always respected, also on battery, with power saving, with heat and even if the
scene weighs more than the GPU can bear. Then the scene is seen with all its details and fewer
frames per second.

## When a scene stays still

Only the automatic choice stops a scene. A scene stays still on its poster with `auto` when:

- it weighs more than the computer's budget even at the lowest level (only on the computer), or
- it was measured too slow at that level, or
- it draws more model triangles than its class admits.

Such a scene plays at the chosen fixed level and, with `min-q1` or `min-q2`, at that minimum (or
higher if it fits).

## Cost warnings

These places warn of what a scene costs against the computer's budget:

- the app, next to the quality choice in *Settings* and on each screen with a scene;
- the tray;
- the phone, under its quality setting, of the home or lock wallpaper it drew last (never of a
  preview, and with no notice as soon as that wallpaper becomes an image, a video or a page);
- `reiwand-ctl quality` and `reiwand-ctl state`.

As soon as its frames are measured, they say how long they take on the GPU against the interval
and at how many frames per second. They do not warn if they fit. They warn that it may drop frames
and use more battery, and whether the automatic choice would lower it or leave it still.

`quality_auto` in the state says this:

| Value | Meaning |
|-------|---------|
| `still` | It is still on its poster. |
| `stop` | A fixed level or a minimum plays it, and `auto` would leave it still. |
| `lower` | `auto` would only lower it. |

If only the model triangles weigh it down and its cost fits, they warn that it draws more triangles
than the computer bears and that the automatic choice would leave it still.

## Limits that always stay

Only the limits that protect memory and the driver are kept with any choice:

- the data and triangles of models that a scene can load;
- the size of textures;
- the effects that are left out by the fixed budget or because they stopped the GPU, which the
  notice counts.

Automatic drops only apply in `auto` or above the chosen minimum, and the reason is `user`.

## Changing it

The app offers it in *Settings* → *Scene quality* and on each screen with a scene or web page, with
its own value. *Same as all screens* goes back to the general one. A page takes the level of the
screen that shows it, and screens with different levels run it separately. The phone offers it in
its settings.

From the terminal:

```sh
reiwand-ctl quality q3
reiwand-ctl quality min-q1 -o DP-1
reiwand-ctl quality none -o DP-1
reiwand-ctl config playback.quality q2
reiwand-ctl config playback.quality.DP-1 q1
```

`reiwand-ctl quality` shows the choice, the level and the reason for each screen.
`reiwand-ctl config playback.quality.DP-1` reads the own value of a screen even if it is not
connected.
