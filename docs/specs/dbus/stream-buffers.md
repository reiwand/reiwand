# Stream buffers

Each stream offers its consumers two formats, in this order:

1. GPU memory: the raw video format with the `SPA_FORMAT_VIDEO_modifier` property, mandatory and
   not fixated, listing every DRM format modifier that the GPU can export as a single-plane
   `B8G8R8A8` color attachment. PipeWire keeps the modifiers that the consumer can also import, as
   it announces them for screen casts (KPipeWire does so when the scene graph can import
   DMA-BUFs). The daemon allocates a test image with the remaining modifiers, lets the driver
   choose one and announces the format again with that modifier fixed. The buffers are then
   `SPA_DATA_DmaBuf`, one plane each, with the offset and stride of the plane in their chunk.
2. Shared memory: the same format without a modifier, one memfd per buffer mapped from offset 0.

A consumer never gets a placeholder picture: until a frame has been drawn into shared memory (for
example right after GPU memory was used, or when a stream is made again in shared memory) nothing is
sent and the output renders at once. The picture kept for shared memory is freed while no consumer
streams the output.

With GPU memory, each frame is drawn straight into one of the 4 to 8 stream buffers: the CPU
neither reads it back nor copies it, and a video frame is uploaded to the GPU once and leaves in
the exported buffer. A buffer is drawn into only after the consumer gave it back, the one sent
longest ago first, and never while it holds the last frame, which keep-alives send again when the
consumer has given it back. The daemon releases the image to external use and waits for the GPU to
finish the frame before it queues the buffer. When the consumer still holds every other buffer,
the frame is skipped and tried again one frame later.

The stream offers every single-plane layout (modifier) of the format that the driver can export:
first those the GPU can draw into, then those it can only copy into, such as linear images on
NVIDIA GPUs. A buffer in a layout of the second kind gets its frame drawn into a texture of the
daemon and copied on the GPU, which still avoids any read back. Linear buffers are placed in
memory the CPU can map where the GPU has it, so consumers that cannot import GPU memory may still
read them; the consumer's list decides which layout is used, and linear is the one that works
between different GPUs, such as a desktop on an integrated GPU and the daemon on a discrete one.

The stream uses shared memory, without any action from the consumer, when:

- the renderer is not Vulkan (the GLES backend), or the driver cannot export such images through
  `VK_KHR_external_memory_fd` and `VK_EXT_external_memory_dma_buf` (without
  `VK_EXT_image_drm_format_modifier` only linear images, modifier `0`, are offered);
- rendering is in software, unless the daemon runs with `REIWAND_DMABUF=1`, or the daemon runs with
  `REIWAND_DMABUF=0`;
- the consumer does not offer DMA-BUFs or none of its modifiers is exportable;
- a buffer cannot be allocated. The daemon allocates the first four buffers of a stream as soon as
  the consumer has chosen their layout, before any is handed out, so a failure there gives that
  consumer shared memory; a failure for a fifth or later buffer recreates the stream in shared
  memory. The stream then offers shared memory only, and new streams of that output do too for a
  minute.

With `REIWAND_SYNCOBJ=1`, when a DRM render node offers timeline synchronization objects, the GPU
buffers also offer explicit synchronization to consumers and PipeWire versions that support it
(`SPA_META_SyncTimeline`, two `SyncObj` blocks after the image): the daemon signals the acquire point
of a buffer once its frame is finished and draws into it again only after the consumer signalled its
release point, at once when the consumer did not schedule one, or after one second. Otherwise the
buffers rely on the implicit synchronization of DMA-BUFs.

GPU memory stays in use on battery, since it only removes work. All consumers of one stream share
its buffers: the first consumer decides their kind, a consumer that links later receives the same
buffers, and the kind is chosen again once all consumers have left. `GetState` shows the kind of
buffers of each output in `buffers`. Debug builds also accept `REIWAND_DMABUF=emulate`, which
exports the Vulkan memory of software rendering as memfds marked as DMA-BUFs, so that tests can
follow the whole path with lavapipe (with a GPU it turns streams in GPU memory on, as `1` does), and `REIWAND_DMABUF=emulate-flaky`, where every second
allocation fails.
