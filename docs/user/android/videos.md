# Videos

Videos (MP4, MOV, 3GP, WebM or MKV) loop as the wallpaper. This page covers how they play, the
conversion of videos the phone cannot play smoothly, HDR, and the settings of video packages.

## How videos play

The path depends on the fit:

| Fit | How it plays |
| --- | --- |
| *Fill* or *Stretch* | The phone's hardware decodes them directly onto the wallpaper, without going through the GPU or copying frames. |
| *Whole image* or *Center* | Drawn with the GPU, like the videos of scenes: the hardware decodes them and each frame is copied to the GPU, at the allowed frames per second at most. Bars of the background color go around them. |

With *Fill* the app asks Android to fill the screen cropping the edges. That crop is done by the
system compositor.

> **Note:** A phone that does not honor the *Fill* request will show the video stretched.

- Videos pause as soon as the wallpaper stops being seen.
- They release the decoder if they stay hidden more than 30 seconds.
- When coming back they continue where they were.
- If home and lock show the same video, they share a single player.
- They play without sound, except with *Play the sound of videos*.

## Fades

The fade between wallpapers works between all types: from an image, a scene or a video to another
of any type, the one leaving and the one entering are mixed.

A video enters or leaves through the GPU during the fade. Then it goes back to the direct path
without cutting or restarting.

## Adding a video

When adding a video, the app checks that the phone has a decoder for its video track and takes its
first frame, as the desktop checks that GStreamer can open it. If it cannot, it does not add the
video and counts it in the notice of files that could not be opened.

## Converting videos

Some videos the phone can only decode with the processor: it has no hardware decoder for their
format, size or speed. If the phone does decode H.264 in hardware at that size or at 1080p or
720p, the app converts the video when adding it to an H.264 copy, with Android's own encoder
(Media3 Transformer, no encoders added to the APK).

While it converts:

- The app shows progress with a *Stop* button that leaves the video as it was.
- A silent, ongoing notification, in its own channel, shows the progress, how many are waiting and
  a *Stop* button. It disappears when finished or stopped.

The first conversion asks for the notifications permission with a line that explains what for. If
you refuse, it converts anyway without a notification and does not ask again.

### The setting

*Convert videos this phone cannot play smoothly* in *Settings* has three options:

| Option | Result |
| --- | --- |
| *Convert them* | The copy takes the place of the original, which is deleted, and keeps its lists and its choice. |
| *Convert and keep the original* | The copy is added as another video. |
| *Never* | Nothing is converted. |

### Power saving

With power saving the app converts nothing when adding, and says so. The page of that video says
why it is worth converting it and offers *Convert for this phone* and *Convert and keep the
original*. These also work with power saving, because you ask for it.

### If Android closes the app

The conversion asks WorkManager to keep the app alive while it lasts (without low battery) and
notes what is left in a file.

- If Android closes the app earlier, the conversion is resumed from scratch as soon as the app
  starts again or WorkManager wakes it.
- It is discarded if the video already changed or does not exist.

### Output format

- 10-bit HDR videos (PQ or HLG) always come out as 8-bit H.264 with the standard range (tone
  mapping).
- Audio that the MP4 container does not support is re-encoded to AAC.
- Videos that the phone already decodes in hardware play as they are.

## Filter, flip and HDR

The filter and the flip of images and videos are applied on the phone too (see
[Settings](settings.md#filter)). A video with a filter or flipped goes through the GPU instead of
playing directly on the screen.

The tones of an HDR video (PQ or HLG) are adapted:

- by the system, when it plays directly;
- by the app itself, when it goes through the GPU, with the same curve as the desktop (white of
  203 nits and peak light of 1000 nits).

If `playback.hdr` turns on the screen's HDR, the app keeps the lights above white up to what the
screen supports, also with a filter. If the decoder already delivers the frames in standard colors,
the app shows them as they are without turning on HDR.

When a photo or nothing takes the place of the video, the screen leaves HDR.

## Video packages

The page of a video package (see [Packages](packages.md)) has its settings and its presets, as on
the desktop:

| Setting | Notes |
| --- | --- |
| speed | |
| start, end | A two-handle trim over the video's real duration, measured when the page opens if the library does not have it, with no one-hour cap. |
| volume | |
| zoom, position | A framing that is dragged over the cover frame. With zoom or position the video goes through the GPU. |

### Frame limit

With a frame limit, the video stays direct on the screen. The screen is requested at that
frequency, and in addition the extra frames are skipped before showing them. So a screen that does
not drop to that frequency does not receive more either. The decoder keeps decoding all of them.

The author's frame limit (in videos, with that nuance), the silent start and the single playback
hold for videos and pages as on the desktop. With single playback, when finished, the last frame
stays on the screen without being drawn again, also if the fit, the filter, the flip, the zoom or
the position change afterwards.

### Sound

- A page without a volume of its own has the *Volume* setting, which is multiplied by the sound of
  videos.
- A video that is silent or at volume 0 does not decode its sound or occupy the audio output.
- When two screens or a preview show the same video with different settings (a preset and its
  base, for example), each one plays it with its own when seen.
