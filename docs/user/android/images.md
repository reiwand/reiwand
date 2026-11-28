# Images and setting the wallpaper

On the page of an image you choose how it is placed and which part is seen, then set it on the
home screen, the lock screen or both. This page also covers how the phone draws the wallpaper and
which formats it reads.

## Fit

*Fit* chooses how that image is placed:

- fill;
- whole image;
- stretch;
- center;
- tile;
- or as in *Settings*.

The fit is a field of the same non-destructive edit that the desktop uses:

```sh
reiwand-ctl edit fit
```

## Framing

Drag or pinch the preview to choose the part that is seen on the phone. *Reset framing* removes it
and goes back to the whole image or to the crop made on the computer.

The framing is a crop with the shape of the phone's screen. The edit stores it like that of one
more screen, `Android`:

```sh
reiwand-ctl edit crop -o Android
```

- It synchronizes with the computer without cropping its screens.
- It is made inside the crop of the whole image, if one was chosen on the computer.
- It combines with parallax.
- The launcher's page swiping runs through the image, or the computer's crop if it is wider than
  the screen. It does not move a framing made on the phone, which already has its shape.

The preview uses the shape of the screen in its natural orientation, also with the phone on its
side, where the preview is placed next to the controls.

## Set as wallpaper

1. Tap *Set as wallpaper*.
2. Choose the home screen, the lock screen or both. The choice exists from Android 14; before,
   both show the same.

Each screen keeps its own wallpaper or its own list, just like a desktop screen (`[outputs.lock]`
in the configuration).

What happens next depends on the current wallpaper:

| Situation | Result |
| --- | --- |
| The app is already the wallpaper of that screen | The change comes in with a fade, without going through the system preview. |
| The app is not the wallpaper of that screen, or the same wallpaper of the app covers both screens and only one is chosen | The system preview opens to set it there. |

What is stored is what the system actually set. If in the system preview you choose only the home
screen, the lock screen keeps its wallpaper.

The wallpaper can also be reached from the system's live wallpapers.

## How it is drawn

The wallpaper is drawn with the GPU (Vulkan, or OpenGL ES if Vulkan fails), with the same renderer
as the desktop. It recovers by itself if the system takes the GPU or the surface from it.

The edit made on the desktop is applied just the same:

- the rotation;
- the straightening;
- the color;
- the masks, also the depth ones, which select on the depth map of the photo after the same
  rotation and the same orientation as the desktop.

## Formats

The phone supports JPEG, PNG, WebP, GIF and BMP. With Android's own decoder (`AImageDecoder`, no
libraries added to the APK) it also reads the HEIC and AVIF photos of cameras.

> **Note:** The desktop does not read HEIC. It does not add HEIC files to its library when scanning
> its folders or when receiving the phone's private files.

For videos, see [Videos](videos.md). For parallax, see [Parallax and depth](parallax-depth.md).
