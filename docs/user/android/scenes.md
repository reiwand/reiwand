# Scenes

Scenes are drawn on the phone with the same engine as the desktop. This page covers what the
engine supports, scene properties and presets, folder properties, scripts, and how the phone picks
a quality level and keeps scenes fast.

For sound, media and touch in scenes, see [Media, sound and touch](media-sound-touch.md).

## What the engine draws

- layers;
- effects;
- particles: on the GPU if it supports compute shaders and, if not, on the CPU with fewer
  particles;
- texts;
- timelines;
- properties.

## Properties and presets

The page of a scene shows its properties and its presets. They are stored in the same
configuration as the desktop.

- Your own presets are saved, renamed and deleted there, and are exported and imported as a file.
- The scene's buttons work.
- Secret texts are sealed by Android's keystore.

## File and folder properties

File and folder properties are chosen with Android's document picker.

| Property | What is copied to the app's files |
| --- | --- |
| File | The chosen file: an image or, if the property accepts them, a video. |
| Folder | The images and videos it accepts, with its subfolders to the depth it asks for and at most the files it asks for. |

The app keeps the folder's read permission and copies again what changed in it:

- every time the app is opened;
- for the phone's own folders, in the background when Android reports a change in the folder or in
  its photos and videos. WorkManager waits for the changes to settle for 10 seconds (2 minutes at
  most), without low battery and without polling anything at intervals. So the copy is usually up
  to date in less than a minute.

### Cloud folders

Notices from a cloud or from the network do not wake the copy, because many notify after each
read. A cloud folder is brought up to date:

- on opening the app;
- in the background, after each copy of a folder of the phone and once a day, only over Wi-Fi or
  another network without a data limit (except *Use mobile data in the background too*, in the
  settings). Without that network it waits for one, polling nothing meanwhile.

The folder of a cloud that does not notify is brought up to date without opening the app.

*Remove* deletes the copy and returns the permission.

As on the desktop, the scene shows the file of the folder that is due, in order or at random and
every the time it asks, and only changes while it is seen.

## Scripts

Scene scripts store data, create shapes, move the camera, change the parents of layers and list
the chosen folder just as on the desktop.

They run on the phone as on the desktop, in a thread of their own with the same limits:

| Limit | Value |
| --- | --- |
| Engine | QuickJS without files or network |
| Memory | 32 MiB |
| Time | 4 ms per frame |

If a script fails three times in a row it is stopped and the scene continues without it. The
diagnostics of *Settings* say so.

Their sounds play on the phone (see [Media, sound and touch](media-sound-touch.md)) and their
volume appears among the properties.

## Quality level

The quality level comes from the phone's class and from the battery.

The phone's class:

| Class | When |
| --- | --- |
| basic | with little memory or without compute shaders |
| high | if the manufacturer declares a performance class of Android 13 or later |
| medium | for the rest |

The class is adjusted with the same short measurement as on the desktop and stored in the app's
files.

Then:

- with low battery it drops one level;
- with heat it drops further;
- if frames take too long on the GPU it drops another;
- it only goes up again with the charger plugged in and the phone cool.

With automatic quality, a scene that is too heavy for the phone stays still on its sample frame.
With a fixed level or a chosen minimum it plays anyway, and the settings warn of what it costs.

## When scenes draw

- Each scene draws at its own frames per second, at most those of the settings.
- It draws only while something changes and never while it is not seen.
- The scene engine stays alive when the system changes the wallpaper surface, so it is not
  reloaded or compiled again.
- Videos inside a scene are decoded by the hardware and copied to the GPU frame by frame, without
  sound, and pause if the scene stays still.

## Parallax in scenes

Scene layers with parallax follow the phone's tilt with the same sensors as photo parallax (see
[Parallax and depth](parallax-depth.md)), even if the scene does not ask for it with `gyro`.
Without sensors they follow the launcher's page swiping. With power saving they stay still.

## Performance on phone GPUs

On the tile GPUs of phones each render pass costs about a millisecond by itself, more than its
pixels. So there the scene draws fewer passes:

- the first effect of an image, a video or a mesh reads the image where it is instead of a copy of
  the layer;
- a hidden mesh that gives its mask to another of the same place and size is read without drawing
  it apart;
- the effects that blend in one pass stay blended when the level removes one in the middle.

Each frame reaches the GPU in a single submission, with the posed meshes, the stencils, the 3D
models and the particles inside, so the time the levels measure includes them. The meshes of the
same pose are posed together in one pass, and the layers in exclusion mode are blended without
copying the canvas.

> **Note:** Layers in lighten or darken mode still copy what is underneath and cut the canvas pass.

Measured on the test phone (60 Hz, screen off). The first two rows came from drawing fewer passes;
the last two from the single submission and the blending changes above:

| Scene | Change |
| --- | --- |
| An image and two waves | 15.5 to 9.9 ms per frame at Q0 |
| An animated mesh | 16.9 to 15.1 ms at Q2 |
| The animated-mesh scene | 50 to 57 frames per second shown at Q0 |
| The two-wave scene | 57 to 59 frames per second |

## Diagnostics

The diagnostics also say whether the effects of the packages are timed with GPU timestamps or by
waiting for them to finish.

When the wallpaper closes, the phone finishes the shader compilations and clears its test marks,
so that a normal close does not count as an effect that hung the GPU.
