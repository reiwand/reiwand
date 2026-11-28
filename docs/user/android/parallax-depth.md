# Parallax and depth

Parallax moves an image with the phone, so near things shift more than the background. It works
best with a depth map, which the phone can compute itself or import from the desktop.

## Turn on parallax

On the page of an image, turn on *Move with the phone*.

## How it follows the phone

The phone uses the first source it has:

1. The rotation sensor that does not use the compass.
2. The normal rotation sensor.
3. Without a gyroscope, the tilt, with the accelerometer.
4. If the phone has no sensor or gives no data, the launcher's page swiping.

Turning the phone to one side moves the image horizontally. Tilting it forward or back moves it
vertically, also with the phone upright.

The rotation is filtered and recenters by itself in a few seconds, with the same spring and the
same shader as the desktop.

- With a depth map and its layers, the near moves more than the background.
- Without them, the image stays still.

## Intensity and focus

*Intensity* and *Focus* (the depth that stays still) are the same values as on the desktop. On the
phone, though, the intensity is measured over the part that is seen and is worth triple, because
the screen is small and looked at up close.

With the default values:

- the near shifts up to 4 % of the screen width;
- the background shifts up to 2 % to the other side;
- the image is enlarged just enough not to show the edges.

The layers are computed for that movement.

## Status and battery

The page shows whether the image has no depth map yet, whether it is being computed or whether it
is ready.

It warns when the movement is paused because of power saving, low battery or heat. With a normal
battery, with or without a charger, it never stops.

- The sensor is only turned on while a wallpaper with parallax is seen, at the allowed frames per
  second (60 at most), and is turned off when it is covered.
- With the phone still, no frame is drawn.
- On battery the shader takes at most 8 depth steps.

## Make a depth map

*Make a depth map* uses the same model as the desktop, Depth Anything V2 Small, with ONNX Runtime
1.31.0, which no longer goes inside the APK.

The first time, it asks for permission to download both, and shows:

| Download | Size | License | Server |
| --- | --- | --- | --- |
| Model | 99.1 MB | Apache-2.0 | github.com |
| ONNX Runtime package for Android | 55.1 MB | MIT | repo1.maven.org |
| Total | 154.2 MB | | |

About the downloads:

- They carry the desktop's generic User-Agent.
- They resume if cut.
- They are checked with their pinned SHA-256.
- Of the ONNX Runtime package only the library for the phone's architecture is kept (about 34 MB
  on arm64-v8a). It is also checked with its own SHA-256, kept read-only and with the text of its
  license next to it. The package is then deleted.

The map is computed in the background on the phone itself, with ONNX Runtime telemetry off.

> **Note:** The photo does not leave the phone.

*Cancel* stops the download or the computation as soon as it is pressed, also in the middle of the
model.

## Depth map settings

With a map, the page shows the same settings as the desktop (`reiwand-ctl edit depth`):

- near and far level;
- curve;
- smoothing;
- how much the near edges grow.

These change the parallax and the depth masks when released.

The page also shows the adjusted map in gray or in *False colour, red near and blue far*, as in the
app. *Remove the depth map* deletes the computed map and the imported one.

*Run the model on* chooses, as in the app, the fastest device or the processor. On the phone the
fastest is today the processor, because its ONNX Runtime brings no accelerators.

The page warns, like the desktop, if the image changed after editing it.

## Import a depth map

*Use a depth map file* imports a map made on the desktop: the 16-bit PNG that it saves next to the
image (for example `photo.jpg.depth.png`), if it has the same proportion.

## Allow or delete the model

In *Settings*, *Allow downloading the depth model* (`models.allow_download`) blocks the download.
Right there the model can be deleted.

## Depth layers

On the phone, with a map, the engine also computes the layers and draws with them. They are
computed in the background, like the map, with the same progress and *Cancel*.

In the parallax settings of each image:

| Setting | Options |
| --- | --- |
| *Fill behind near objects* | `simple` (the default) or `lama` |
| *Depth layers* | from 4 to 6, with a visible note of their cost |

*Build the depth layers* redoes them with the current settings.

> **Note:** LaMa is optional. It uses more battery and memory and takes minutes per image, so it is
> off by default. It is downloaded only if you choose it (208 MB, Apache-2.0, huggingface.co, with
> the same prior permission and its SHA-256). It never runs with battery saving, which uses the
> simple fill.

The phone still has no author tools.
