# Streams and video decoding

This page explains how frames move between the GPU, streams and the video decoder without copies,
the environment variables that control it, and measurements on an NVIDIA GPU.

## Streams in GPU memory

Streams carry each frame in GPU memory (DMA-BUF) when the Vulkan driver can export it and the
consumer (KPipeWire on Plasma) import it. The daemon then draws directly into the stream's buffers,
without reading the frame back or copying it with the CPU, also on battery.

Where the GPU cannot draw in the format the consumer asks for, such as linear images on NVIDIA GPUs,
it draws into a texture of its own for each screen and copies it on the GPU to the stream's buffer.

### Only what changed

The daemon composes and sends only the rectangle that changed:

- In GPU memory, it draws into each buffer what changed since the last time it used it.
- In shared memory, it reads from the GPU and copies only those rows.
- Both kinds of buffer carry `SPA_META_VideoDamage` with what changed since the previous buffer, for
  the consumers that read it.

A small clock over a 4K image thus copies about 55 KiB per frame instead of 32 MiB.

Wayland and X11 surfaces and the phone keep composing the whole frame, because the graphics API does
not say what each screen buffer contains.

### Fallback and variables

If something is missing (OpenGL, software rendering, a consumer without DMA-BUF), the stream uses
shared memory without intervention.

| Variable in the daemon's environment | Effect |
|--------------------------------------|--------|
| `REIWAND_DMABUF=0` | Forces shared memory. |
| `REIWAND_DMABUF=1` | Asks for GPU memory also with software rendering. |
| `REIWAND_SYNCOBJ=1` | GPU buffers also offer explicit synchronization (syncobj) to consumers and PipeWire versions that support it. |

`reiwand-ctl state` shows the buffer type of each screen (`buffers`: `dmabuf` or `memfd`).

> **Note:** `REIWAND_SYNCOBJ=1` is experimental and off by default. Without that variable the
> buffers are synchronized only implicitly.

## Zero-copy video decoding

When the hardware video decoder delivers frames in GPU memory (DMA-BUF in NV12, such as VA-API on
Intel and AMD GPUs) and the Vulkan driver can import them with their modifier, the daemon reads them
in place. It converts the color, rotates the video if asked and reduces it to the screen size on the
GPU, without passing the frame through system memory.

| Path | CPU copies per frame |
|------|----------------------|
| Zero-copy | zero |
| Copy | three: download the frame, convert it to RGBA and upload it |

With a 4K video at 30 fps decoded by VA-API on an AMD iGPU, that is about 0.3 ms of CPU per frame
instead of about 16 ms.

The video goes back to the copy path by itself if the decoder is software, if the format or the
modifier cannot be imported, or if the import fails.

On NVIDIA GPUs, GStreamer 1.24 does not deliver DMA-BUF. NVDEC gives CUDA or OpenGL memory, and the
VA-API driver over NVDEC, `nvidia-vaapi-driver`, only system memory. So there videos keep being
copied, although the import works. With lavapipe and OpenGL there is no import.

`reiwand-ctl state` shows the chosen path in `decode` (`zero-copy` or `copy`).
`REIWAND_ZERO_COPY=0` in the daemon's environment forces the copy.

Pausing a video stops it where it is, without decoding again from the previous keyframe. On a 4K
video with keyframes every 10 s, a pause went from about 2.3 s of CPU to under 20 ms.

## Measurements on NVIDIA

Test setup:

- an NVIDIA dedicated GPU with the proprietary driver 615;
- GStreamer 1.24;
- a `--release` daemon on a private instance;
- a 30 s H.264 video (1080p60) or 20 s (4K60) on a screen of the same size;
- 30 s per test, average of two rounds.

GPU power varies about 5 W between rounds, probably because other processes share it. At idle,
without video, it measured between 17 and 32 W.

"Copy" is what the daemon does today (NVDEC through `nvh264dec`). "VA-API" forces the
`varenderD129h264dec` decoder of `nvidia-vaapi-driver` (`NVD_BACKEND=direct`,
`GST_VA_ALL_DRIVERS=1`).

| Test | Frames per second | CPU per frame | Daemon CPU | Memory (RSS) | GPU power | GPU / NVDEC use |
|------|-------------------|---------------|------------|--------------|-----------|-----------------|
| 1080p60, copy | 59.8 | 4.7 ms | 28 % of a core | 673 MiB | 45.7 W | 3 % / 2 % |
| 1080p60, VA-API | 59.8 | 4.5 ms | 27 % of a core | 730 MiB | 46.3 W | 3 % / 2 % |
| 4K60, copy | 59.7 | 18.9 ms | 113 % of a core | 1124 MiB | 48.4 W | 8 % / 10 % |
| 4K60, VA-API | 59.5 | 19.0 ms | 113 % of a core | 1112 MiB | 48.0 W | 8 % / 8 % |

NVIDIA's VA-API decodes fine (NV12, at all frames per second), but changes nothing measurable. Its
frames arrive in system memory just the same and follow the same copy path, with about 50 MiB more
memory and nearly 1 GiB more video memory.

The driver does know how to export NV12 as DMA-BUF (two separate layers, one per plane, with
NVIDIA's block modifier `0x0300000000606014`), but not composed layers. The GStreamer 1.24 `va`
plugin fails to export (`vaExportSurfaceHandle: an unsupported memory type was supplied`), so it
does not offer DMA-BUF.

That is why the daemon neither prefers nor detects it. Giving it zero copies on NVIDIA would require
decoding with NVDEC or `libva` directly, or a GStreamer that accepts separate layers.
