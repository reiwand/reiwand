# Timelines, sound and video clocks

Scenes animate with timelines, play sound layers and show videos. This page explains timeline
events and controls, how sound is played on each platform, video clocks and HDR video.

## Named events and loops

The desktop and the phone draw the named events of timelines. Each one:

- fires once per pass, even if a frame skips it;
- starts other timelines or sounds without waking the scene early.

They also draw the closing of loops, which blends the last key with the first.

## Timeline controls in scripts

Scripts can:

- change the speed of a timeline;
- stop it;
- read its time, its duration and whether it is still running;
- receive its events and its end.

## Timeline options

| Option | Effect | Status |
| --- | --- | --- |
| `frames` and `fps` | Timelines measured in frames. | Already works on both platforms. |
| `wrap` | Links the end of a loop to its start. | Rejected while the engine does not draw it. |
| `"seed": null` | Gives a scene a new seed every time it starts. | — |

## Sound layers

The desktop plays complete sound layers:

- MP3;
- several files in order or at random with pauses;
- long sounds up to ten minutes, read as a stream without loading them whole;
- sounds triggered by events;

with the volume and the panning smoothed.

Sound follows the scene's time:

- loops are realigned;
- on resuming it sounds where the timeline says;
- latency drops from about 160 ms to about 50 ms.

The phone plays the same layers, with about 200 ms of latency.

## Thumbnails

A scene without a preview image receives a thumbnail drawn from the scene itself at its poster
moment.

## Video clocks

Several layers that show the same video share their clock, led by the first.

With `playback.own_clock` a `video` layer plays it with a clock of its own: its speed, span, pause
and jumps from scripts, at the cost of one more decoder.

- A video chosen by a property with `map` carries a clock of its own for each video of the list,
  and the jumps from scripts move them all.
- A video that a file property chooses on the fly carries its own for the file the user chooses.

> **Warning:** `own_clock` is rejected on an image layer, on a video bound without `map` to a
> property that is not a file one, or on a layer that shows script geometry.

The editor's inspector offers it as *Own clock*. The editor's timeline shows each own clock with:

- a strip of video frames;
- the span as a bar whose ends can be dragged;
- its speed, loop and playback;
- where it is at the chosen instant.

`reiwand-ctl scene ask FOLDER clocks` prints it, with N frames, and this command does too:

```sh
reiwand-ctl scene act DIR set '{"pointer":"/scene/layers/N/playback/own_clock","value":true}'
```

## HDR video

A package with an HDR video (PQ or HLG) receives the `hdr` facet, and `reiwand-ctl pack check`
says so.

On the computer it is decoded with 10 bits, and the graphics adapts its tones to normal colors,
also inside scenes:

| Item | Value |
| --- | --- |
| Color space | BT.2020 to sRGB |
| Screen white | 203 nits |
| Strongest light | 1000 nits |

Normal videos do not change.
