# Jobs and Thumbnails

On this page: jobs, thumbnails.

## Jobs

Job ids start at 1 for every daemon run and are never reused in a run; clients notice a restart
of the daemon through `NameOwnerChanged`. Starting a job whose kind and target equal a queued or
running job returns that job's id (`import` jobs are never merged, and a `scan` only joins a queued
scan, never a running one, so a change made while a folder is scanned is seen by the next scan). Library jobs (`scan`,
`import`, `analyze`) run one at a time, repository jobs (`repo_add`, `repo_refresh`, `install`)
run one at a time, at most two `provider_sync` jobs download at once, `account` and
`upload` jobs run one at a time, and so do `layers` jobs; a job waiting for its
turn is `queued`. A sync after its downloads and an install after its commit stay `running`
while they wait for the library.

| Kind | Started by | Target | Progress unit | Output |
|------|------------|--------|---------------|--------|
| `scan` | `AddSource`, `Rescan`, folder changes | source uid, or `""` for all | files (`0/0` while walking) | `ScanOutput` |
| `import` | `Import` | `""` | files | `ImportReport` |
| `analyze` | after a scan or import that added or changed items | `""` | items | `AnalyzeReport` |
| `provider_sync` | `Providers1.Sync`, schedule | provider id, or `""` | items | `SyncOutput` |
| `repo_add` | `Repos1.Add` | repository URL without query | `0/0` | `Repo` |
| `repo_refresh` | `Repos1.Refresh`, schedule | repository id, or `""` | `0/0` | `RefreshOutput` |
| `install` | `Repos1.Install` | `repo_id/id@version` | bytes | `InstalledPackage` |
| `layers` | showing a picture whose parallax is on, that has a depth map and no layers for its settings yet | picture path | percent (`0` to `100`) | `{ file, fill, layers }` |
| `account` | `Accounts1.SignIn` | `sign-in <repository URL>` | `0/0` | `{account, user, server}` |
| `upload` | `Publish1.Upload` | random id | bytes | `UploadOutput` |
| `carry_export` | `Carry1.Export` | file path | sections | `{path, sections, own, encrypted}` |
| `carry_import` | `Carry1.ImportChosen` | file path | steps | `{added, pending, unresolved, notes}` |

`CancelJob` drops a queued job; a running job stops at its next checkpoint (between scan
batches of at most 256 files, imported files, analyzed items, provider downloads and download
chunks). Finished steps stay: a cancelled scan keeps the batches it stored and marks nothing
missing, and a package whose commit finished stays installed. Cancelling a finished job changes
nothing. A cancelled `layers` job is not started again by the daemon on its own (when a screen
is reconfigured, covered or uncovered): only when the picture is applied again (`SetWallpaper`,
`Library1.Apply`), when a depth or parallax setting that is part of the key of the set changes, or
with `RetryLayers`. The daemon keeps that in memory only.

## Thumbnails

Thumbnails are JPEG files (quality 85) whose longest edge is 256, 512, 1024, 2048 or 4096 pixels, never
upscaled, in `~/.cache/reiwand/thumbs/<size>/<key>.jpg`, where `key` is the lowercase hex
BLAKE3 hash of `"<item path>\0<mtime>\0<size>"`; a changed file gets a new key. Directories are
`0700` and files `0600`. Clients only get paths from the daemon and open them read-only.

Only an isolated worker process (`reiwandd --worker`, at the lowest CPU and I/O priority) decodes
files: images with size limits (at most 32768 pixels a side and 1 GiB of memory, the same limits
under which the renderer, the app and `reiwand-ctl` show and edit a picture; a baseline JPEG of up to
300 million pixels is decoded at a reduced scale, a progressive one only while the coefficients it
keeps need at most 512 MiB), EXIF orientation applied and the first frame of animations;
videos at 10 % of their duration (between 1 s and 30 s), scaled to at most 8192 pixels a side
keeping their shape (a larger video is also measured at that size); packages from their preview, else from
their image or video entry. The same run measures the item (dimensions, duration, audio,
dominant color) or records why it cannot be decoded. At most `max(1, min(4, cores / 2))` files
are decoded in parallel, each within 15 s plus 1 s for every MiB of the file (at most 120 s), and
a worker exits after 30 s without work. A file whose decoding takes longer or stops the worker is
not marked as broken: it stays unanalyzed, background analysis leaves it until the daemon starts
again (or the file changes), and `Thumbnail` keeps trying it when asked. Requests waiting in
`Thumbnail` come first, then `RequestThumbnails`, then background analysis.

Every returned path has its modification time refreshed. When the cache grows beyond
`library.thumb_cache_mb`, the oldest files are deleted until it is below 90 % of the limit.
Removing items deletes their thumbnails of every size, also those a worker finishes after the item
was removed or its file changed.
