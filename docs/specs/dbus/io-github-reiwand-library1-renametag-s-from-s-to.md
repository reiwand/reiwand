# `io.github.reiwand.Library1`: `RenameTag` to Signal `ThumbnailReady`

On this page: `renametag`, `thumbnail →`, `requestthumbnails →`, `apply`, `confirm`, `unconfirm`, `cleardata`, signal `thumbnailready`.

## `RenameTag(s from, s to)`

Renames a tag; if `to` exists (without regard to case) the two merge. Renaming to the same name
with another case changes the spelling. Packages of the renamed items no longer bring `from` back.
`NotFound` for an unknown `from`.

## `Thumbnail(s uid, u size) → (s path)`

Returns the path of the item's thumbnail of `size` (`256`, `512`, `1024`, `2048` or `4096`, else
`InvalidArgs`), making it first when it is not cached, ahead of every other thumbnail work.
`Failed` with the reason when it cannot be made or takes more than 20 s; a decoding problem also
sets the item's `error`. A scene package without preview gets a thumbnail drawn from the scene at
its `poster_time`, with the default values of its settings, the local time at which it is drawn and
the language of the package, in the decoding process; it returns
`Failed` (`no preview: …`) only when no graphics device can draw it. A
package that needs a newer Reiwand keeps that `error` and still gets the thumbnail of its
preview. The library reads every package again once after the daemon is upgraded, on its own
connection and in short transactions like a scan.

## `RequestThumbnails(as uids, u size) → (a{ss} paths)`

Returns `{uid: path}` for every item whose thumbnail of `size` is cached, and queues the others
ahead of background analysis; `ThumbnailReady` follows for each of them (`path = ""` when it
cannot be made or the file is missing). A new call from the same sender replaces that sender's
queued requests, so a fast scroll does not queue the whole library. Unknown uids are ignored.

## `Apply(s uid, s output, s fit, s preset)`

Shows the item on `output` (`""` = every output, clearing per-output wallpapers and
playlists) and stops the playlists of the affected outputs. `fit` is a fit or `""` to keep the
current one. `preset` is `""`, a preset uid, or a preset name looked up first among the item's
presets and then among the global ones; its `fit` and `background` are applied first, and an
explicit `fit` wins over the preset's. `Failed` before changing anything when the item is
missing, has an `error` (invalid packages), or is a revoked package, and `FlashingDisabled` when it
is a photosensitive package and `playback.allow_flashing` is off: clients show the notice `flashes and is disabled` and offer the setting instead. The item's path,
the effective fit and background and the preset uid (as a label) are written to the affected
configuration sections, and the use is recorded once per affected connected output; when the
library cannot record it (a full disk, a locked database), the item stays shown, the call
succeeds and the daemon logs a warning. Image and
video packages show their entry: it is copied once to `~/.cache/reiwand/blobs/`. Scene
packages are rendered live (see "Scenes"); the videos they contain are copied there once.

## `Confirm(as uids, s what)`

Records a decision of the user about the packages of these items, once per package id, so that
every version and copy of the package keeps it. `what` is `mature` (clients stop blurring the
thumbnails of a package rated `mature`) or
`motion` (the scene keeps moving while the desktop asks for reduced motion, for gentle scenes the
user chose; every screen that shows a scene is configured again; `InvalidArgs` for a package marked
`photosensitive`, and a choice recorded before a new version marked it so is ignored while it is);
anything else returns `InvalidArgs`, an unknown uid `NotFound`. Items that are no packages are
accepted and change nothing. `PackageInfo.confirmed` lists the decisions, and the `library` change
names every item that shares a package id with these items.

## `Unconfirm(as uids, s what)`

Withdraws a decision recorded with `Confirm`, with the same arguments, errors and `library` change;
withdrawing `motion` makes the scene stay still again while the desktop asks for reduced motion.

`hide_zone`, `hide_language`, `hide_gpu` and `hide_storage` restrict what a web page may learn
(`web.md`): the time zone (it sees UTC), the language (it sees `en-US`), the name of the graphics
card (WebGL keeps working with a generic renderer name) and its own storage (it keeps nothing). By
default a page sees all four, and `PackageInfo.exposes` lists what the package's page sees now:
`zone`, `language`, `gpu`, `storage` (only for pages that declare `web.storage_kib`) and `network`
(only for pages that declare origins and hold the `network` grant of `Inputs1`), so that every
client shows the same notice. `PackageInfo.flash_disabled` is `true` for a photosensitive package while `playback.allow_flashing` is off (`RepoPackage.flash_disabled` likewise, and `RepoPackage.asks` lists what the release asks for); `PackageInfo.unshown` is `null`, or, for a web page that this build or
computer cannot show (no `reiwand-web` helper or no Qt WebEngine 6.8 or newer), the same reason that
applying it would return, so clients warn before it is applied. Every screen that shows a web page is configured again, and its page
starts again with the new settings.

## `ClearData(as uids)`

Deletes what the web pages of these packages stored (`~/.local/share/reiwand/web/<id>/`), once
per package id, and starts again the pages that are shown. Removing the last item of a web package
deletes it too. Items that are no web packages change nothing; an unknown uid returns `NotFound`.

## Signal `ThumbnailReady(s uid, u size, s path)`

A thumbnail queued by `RequestThumbnails` is ready, or could not be made (`path = ""`).
