# Settings

The *Settings* tab holds the app-wide choices: fit, fade, sound, frame rate, system colors,
graphics and the color filter. Below them are the diagnostics and the licenses.

## Main settings

| Setting | Options |
| --- | --- |
| Fit of images that have no fit of their own, and of videos | fill, whole image, stretch or center |
| Fade between wallpapers | — |
| Sound of videos | — |
| Frames per second on battery | 60, 30, 15 or 10 |
| Frames per second on the charger | 60 or 30 |
| *System colors*: when the system colors change | see below |
| Graphics | automatic, Vulkan or OpenGL ES |

For *Convert videos this phone cannot play smoothly*, see [Videos](videos.md). For *Allow
downloading the depth model*, see [Parallax and depth](parallax-depth.md). For mature content and
flashes, see [Packages](packages.md#mature-content-and-flashes).

## Filter

*Settings > Filter* chooses the color filter of the home screen and that of the lock screen.

- The same looks as the desktop app, each with a sample of already filtered colors.
- Sliders, as the desktop app, with their value in view: strength, exposure, contrast, saturation,
  warmth and tone.
- *Show as it is*.

The lock screen uses the filter of the home screen while *Same filter as the home screen* is on.
The wallpaper changes as each control is moved.

## System colors

The system colors (Material You) come from the image or the first frame of the video. They are
computed only once, when adding them.

*System colors* decides whether Android changes theme:

- when a wallpaper is chosen (the normal);
- also on each change of a list;
- or never.

Android applies it the next time the screen is turned off and on.

## Frame rate and battery

The app only draws when something changes and never while the wallpaper is not seen. With a still
image there is not a single extra frame.

The fade and the shifting when swiping pages in the launcher:

- run at most at the chosen frames per second: 60 on battery and on the charger if not changed,
  and 30 in the preview;
- stop with power saving or with low battery;
- drop when the phone heats up.

Videos pause with power saving, with low battery or with a very hot phone.

## Diagnostics

Below the settings, the diagnostics show:

- the GPU;
- the frames drawn and per second;
- the limit applied at that moment;
- the state of charge, power saving and temperature;
- with a scene: the phone's class (and whether it was measured on its GPU), the quality level and
  why it dropped.

They also say whether the phone's class was measured with GPU timestamps, which count only the
GPU's work, or by waiting for the GPU to finish, on phones that do not have them. The desktop
measures the same way.

The notices that come from the engine (the failures of a package, of a property or of depth) carry
a message identifier. They are shown in the phone's language, with the same translations as the
rest of the app.

## Licenses

*Licenses* shows the texts of ONNX Runtime, of the components it includes and of the depth and
fill (LaMa) models.
