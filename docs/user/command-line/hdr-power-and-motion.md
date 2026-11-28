# HDR, power, motion and flashing

This page covers HDR screens, what happens on battery and with heat, reduced motion, software
rendering, and wallpapers that flash.

## HDR screens

HDR screens show the colors brighter than the white of scenes with `canvas.hdr`. If the desktop and
the graphics driver offer an HDR surface (on Wayland, with the color management protocol), the
scene keeps 16 bits to the end. It folds what exceeds 0.8 of the white toward the maximum
brightness of that screen. The desktop communicates that brightness along with its reference white.
If it does not say, double the white is assumed, a decision that is kept.

The colors go to the desktop in one of three ways:

1. In extended linear sRGB, if the driver offers it.
2. In PQ (BT.2100), encoded by the daemon, if the driver only offers PQ.
3. If it offers neither but the desktop accepts image descriptions of its own, the daemon encodes
   in PQ and describes the surface itself with its HDR metadata (MaxCLL, the highest light it can
   draw, and MaxFALL, its white).

The app, `reiwand-ctl quality` and `reiwand-ctl state` say which of the three is used.

### The `playback.hdr` setting

| Value | Meaning |
|-------|---------|
| `auto` | Only if the desktop or the phone say the screen shows HDR, on power and with a scene with `canvas.hdr` and tone mapping at `q3`, with an HDR video or with an HDR KTX2 image, because it uses more. |
| `on` | Whenever the surface supports it, even if the screen does not say so. |
| `off` | Off. |

Each screen can have its own value (`playback.hdr.DP-1`). If the screen cannot, it is seen in SDR as
always.

### What HDR changes

With HDR on:

- An HDR video (PQ or HLG) keeps its lights above white, adjusted to the peak of the screen (the
  lowest of the screens that show it).
- An HDR KTX2 image (UASTC HDR, without photo retouching) also keeps its light above white, softly
  folded toward the screen's peak, on the desktop and on the phone.
- The other images, normal videos and pages, made for the standard range, put their white at the
  white of the screen.

### Changing it

The app offers it in *Settings* → *HDR* and on each screen, with what the screen supports and why
it is on or not. The editor's tone panel warns how those scenes look on an HDR screen.

On the phone (Android 14 or later) it is in its settings. The app uses `COLOR_MODE_HDR` in its
window, and the diagnostics say whether the screen shows HDR and its peak.

```sh
reiwand-ctl quality --hdr on
reiwand-ctl quality --hdr off -o DP-1
reiwand-ctl config playback.hdr.DP-1 auto
```

`reiwand-ctl quality` and `reiwand-ctl state` show the state of each screen.

> **Note:** The setting does not travel in the exported file or enter profiles, because it depends
> on each computer. When using a profile, each screen keeps its own value. Web pages follow the
> general one.

## Battery and heat

On battery or with the power-saving profile of power-profiles-daemon, scenes go down in level and
frames. If a system sensor says the GPU or the CPU is near its critical temperature, they go down
further until they cool.

## Reduced motion

When the desktop asks to reduce motion (`reduced-motion` of the settings portal), scenes stay still
on their sample frame. They are only redrawn once a minute so clocks stay on time.

The exception is scenes that you let keep moving, meant for calm scenes:

- in the app: *Keep moving when the system asks for less motion* in the package details;
- on the phone: the same switch on its page;
- in the terminal: `reiwand-ctl lib confirm motion ITEM`, and `reiwand-ctl lib unconfirm motion ITEM`
  to undo it.

It is not offered for photosensitive packages.

## Drawing only what is needed

- Slow effects, such as clouds or waves, declare at how many frames per second they look good. A
  scene that only moves through them does not go beyond that.
- Each frame redraws only the part of the screen that changed and, if nothing changed, draws
  nothing. Still particles and scenes stopped on a frame do not count as a change.

## Software rendering

With software rendering:

- Each scene is drawn at one megapixel per core at most and scaled to the screen.
- The wait for the GPU reaches up to 60 s with few cores. After a frame with package effects, also
  when composing it, the wait stays at 10 s to detect hangs.
- If a frame takes more than an eighth of that wait, the resolution goes down in steps and goes back
  up when there is time to spare. So a heavy frame is never taken for a hung GPU.

Blurs and deformations no longer darken the right and bottom edges, and motion blur can follow the
velocity of its layer (`follow`).

## Wallpapers that flash

Wallpapers that flash are disabled by default: `playback.allow_flashing` comes off. You can change
it in these places:

- *Allow wallpapers that flash* on the app's *Settings* page, which the tray's *Settings…* button
  opens;
- a switch of its own in the tray;
- `reiwand-ctl config playback.allow_flashing true`;
- a switch in the phone's settings.

While it is off:

- Each photosensitive package carries the notice "flashes and is disabled" in lists, is not played
  and rotation skips it.
- The program measures the brightness of each changed frame of scenes. If one flashes more than
  three times per second, it plays it more slowly or stops it. `reiwand-ctl state` indicates it in
  `flashes`.

That measurement costs a small GPU read per changed frame. With the setting on, nothing is measured
or warned.
