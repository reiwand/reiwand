# Frame rate and Video decoding

On this page: frame rate, video decoding.

## Frame rate

Every output knows the refresh rate of its screen: layer-shell outputs take it from the presentation
feedback of their surface (`wp_presentation`, asked for once after the surface is created or changes,
and again when the current mode changes its rate, which drops the rate measured in the old mode)
when the compositor gives a refresh period, and from the current mode of their `wl_output` otherwise,
X11 outputs from the RandR mode of their CRTCs (the fastest when a
monitor spans several) and streams from the current mode of the `wl_output` with the same name
while the daemon is connected to the Wayland compositor, which on Plasma is KWin's own output
information. With variable refresh the mode gives the highest rate. An output never renders more
often than the lowest of:

- `playback.max_fps`: its number, or with `display` (the default) the refresh rate of the output
  (60 while it is unknown), on battery no more than 60;
- a known refresh rate;
- except for videos, the GPU class: software renderers 24, older integrated and phone GPUs 30,
  or 60 for a scene whose estimated cost at that rate fits their budget, newer integrated GPUs 60
  or the rate above for a scene whose estimated cost at that rate fits their budget, dedicated
  GPUs the rate above; with `display` on battery none goes above 60, and a number the user chose
  in `playback.max_fps` keeps the caps of mains power on battery;
- for scenes, their `clock.max_fps` (`display` follows the refresh rate); the quality level changes
  what a frame draws, not the rate;
- with power saving and `display`, 30, and for scenes while the GPU or CPU is hot, 30 or 15 (see
  "Cost and frame rate" in `scene.md`), and for scenes whose only motion comes from effects with a
  `min_fps`, the highest of those;
- for parallax images with software rendering, 15;
- for videos with `display` and software rendering, 30.

A fade to a video or a web page starts once its first frame is there, or once it failed. A fade
between two wallpapers runs at 60 frames per second on mains power, or faster when the
rate above is higher, never above a known refresh rate; on battery it keeps the rate above.
Videos play at their own frame rate and drop the frames above the rate of the fastest output that
shows them, never repeating one. Nothing is rendered while nothing changes, whatever the rate. A
frame that cannot be drawn is tried again 1 s later, then after twice as long each time up to once a
minute, until it is drawn or the picture, the size or the surface changes, which is drawn at once.

The renderer uses Vulkan and opens OpenGL ES through EGL only when Vulkan offers no adapter or only a
software one (`WGPU_BACKEND` chooses one instead). A swapchain that the driver reports as
suboptimal is configured again, with the present modes and formats it offers then, right after the
frame it gave.

### Graphics card

`render.gpu = auto` takes the card the compositor renders with, so that the frames reach it without a
copy: at start the daemon asks the compositor for the default feedback of `linux-dmabuf-v1` (its
`main_device`, a device number that the daemon turns into the PCI bus id of the card behind it
through `/sys/dev/char`, within 2 s and only on Wayland) and opens the adapter on that bus, Vulkan
before OpenGL ES. When the compositor does not say (X11, no `linux-dmabuf-v1` of version 4, a card that
is not in the list, or one that cannot be opened) it keeps the choice above: the adapter that saves the
most power, so a computer with an integrated and a dedicated card renders on the integrated one,
which a compositor on the dedicated card (such as KWin when plasmashell offers only that card's
modifiers to PipeWire) then reads through the processor. `integrated`, `dedicated`
and `software` take the first Vulkan adapter of that kind, and any other value the first one whose
name contains it, without regard to case, so two cards of the same kind are told apart by their
names, and `pci:` with a PCI bus id (`pci:0000:03:00.0`) the card on that bus, which tells two
identical cards apart (`GetState().adapters[x].bus`, empty when the driver gives none; the app offers
it only for cards that share a name). The list of cards follows the graphics nodes of `/dev/dri`: a
card plugged in or taken out shows up or goes away a second later, without polling; OpenGL ES is looked at only when Vulkan has no hardware adapter or the choice is `software`.
An OpenGL ES adapter whose driver names no kind takes the kind the system gives its card in
`/sys/class/drm` (Intel integrated on PCI bus 0 and dedicated on another bus, or, without a PCI
address, dedicated for the device ids of its dedicated families and integrated for the others;
NVIDIA dedicated; AMD integrated when its own memory is at most 2 GiB and dedicated above), matched
by vendor or, with a single card, that card; with several cards of the vendor and different kinds
it takes the kind of the card that starts the screen (`boot_vga`), which OpenGL uses by default, or
of the other cards when `DRI_PRIME` asks for another one, and stays `other` when those still differ
or no card starts the screen. So `integrated` also picks an integrated card
that only OpenGL ES reaches.
`WGPU_BACKEND` still limits the backend, also when the daemon lists the cards, so no other
driver is loaded. When no adapter matches, or the matching adapter cannot open a device, the daemon
starts with the automatic choice, logs a warning and sets `GetState().gpu_warning`; it never fails
to start because of this setting. The renderer is made once at start and shared by every output, video and scene,
so a change of `render.gpu` takes effect the next time the daemon starts
(`Restart()`, `reiwand-ctl restart` or the app's button); until then `GetState().gpu_restart` is true, and both
`reiwand-ctl gpu` and `reiwand-ctl config render.gpu` say so. Names are compared without regard to case,
so changing only the case of the name asks for no restart. The class of the
card (see "Cost and frame rate" in `scene.md`) is measured and kept for each card and driver, so
going back to a card does not measure it again; `reiwand-ctl scene bench` renders on the card that
`render.gpu` of the configuration file chooses. The phone has one graphics card and ignores this
setting.

## Video decoding

The player asks GStreamer for NV12 frames in DMA-BUF memory (`video/x-raw(memory:DMABuf)`,
`format=DMA_DRM`) with every DRM format modifier that the Vulkan device can import as single-plane
`R8` and `R8G8` sampled images, and for RGBA frames in system memory as the second choice. A hardware
decoder that exports DMA-BUFs (VA-API on Intel and AMD) hands its frames over directly: the daemon
imports the luma and chroma planes of each decoder buffer once (up to 16 buffers per video), turns
them into RGBA on the GPU with the colour matrix and range of the stream, applies the orientation
tag and reduces the frame to the largest output that shows it, and keeps the decoder buffer until
the next frame arrives. No CPU copy happens per frame. Software decoders, other formats (10-bit
video), devices without `VK_EXT_external_memory_dma_buf` and `VK_EXT_image_drm_format_modifier`
(lavapipe, OpenGL) and any failed import use the copy path: the frame is converted, scaled and
turned by its orientation tag in system memory and uploaded, without opening the video again. A failure switches the video to the copy path once and for good.
`REIWAND_ZERO_COPY=0` in the environment of the daemon forces the copy path. Per driver: Intel (iHD)
and AMD (radeonsi) VA-API export NV12 DMA-BUFs; with GStreamer 1.24, NVIDIA offers no DMA-BUF from
NVDEC (CUDA or OpenGL memory) nor from its VA-API driver (`nvidia-vaapi-driver`, system memory), so
its videos are copied, although its Vulkan driver imports them. That driver decodes and can export
NV12 as two separate layers with an NVIDIA block-linear modifier, but not as composed layers, and the
`va` plugin of GStreamer 1.24 then offers no DMA-BUF; forcing it (`NVD_BACKEND=direct`,
`GST_VA_ALL_DRIVERS=1`) measured the same CPU time per frame, memory and GPU power as the copy path
(table in docs/user/command-line/streams-and-video-decoding.md).

Pausing a video only pauses the pipeline: the frame on screen stays and nothing is decoded again.
If the pause has not settled after 1 s, the player seeks to the nearest key frame to finish it.
