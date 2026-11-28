# `io.github.reiwand.Library1`

The library catalogs images, videos and `.wpak` packages found in folder sources or imported
one by one. Every method returns `Failed` while the library is unavailable.

## `AddSource(s path, b recursive) → (s source, t job)`

Adds a folder and starts a `scan` job of it. `path` is canonicalized. `InvalidArgs` when it is
not an existing directory, when it would share files with an existing source (the same folder, a
folder inside a recursive source, or a recursive folder containing a source: `overlaps the
source <path>`), and when it lies inside `~/.cache/reiwand` or
`~/.local/share/reiwand` or, being recursive, contains one of them through non-hidden
folders (`contains Reiwand data`). Scans skip hidden entries, so adding the home folder is
allowed, and depth maps (`*.depth.png`), which are never items: `Import` refuses them and a scan
marks one already in the library missing.

## `RemoveSource(s source)`

Cancels a queued or running scan of the source, then removes it with its items (their tags,
ratings, playlist entries and thumbnails). Sources that belong to a provider return
`InvalidArgs` (`belongs to provider "<id>"; remove the provider`).

## `ListSources() → (s json)`

A `SourcesReply`: every source with its item count and, for provider cache folders, the
provider id.

## `Rescan(s source) → (t job)`

Starts a `scan` job. `source = ""` scans every source in one job and then checks the items that
belong to no source (imported files and packages), so that deleted files become `missing`. Files
are read (metadata, quick hash, package manifest) outside any database transaction; their changes
are then written in short transactions of at most 256 files, and a file whose size, time and
place did not change is not written at all, so other writers never wait for file reads. A
source whose folder cannot be read, or is completely empty (no entries, hidden ones included)
while it still has items that are not missing, as happens with the mount point of a disk that
is not mounted, gets an `error` (`the folder is empty; its disk may not be mounted`) and keeps
its items as they were; the next scan that reaches the folder clears it. To empty a folder on
purpose, remove its source.

## `Import(as paths, b copy) → (t job)`

Imports files one by one in an `import` job. Each path must be absolute (no URI); directories go
through `AddSource`. With `copy = false` the file is cataloged where it is. With `copy = true`
it is first copied to `~/.local/share/reiwand/files/` (on a name collision as
`<stem> (2).<ext>`, `(3)`, …; the copy keeps the modification time of the original) and takes
the original file name without extension as title. A `.wpak` file always goes through the rules
of `ImportPackages` (verification, version rules, always copied; `copy` is ignored) with `allow_older = false`. A
file that fails is listed in `ImportReport.failed` with its reason; the job fails only on a global
error.

## `ImportPackages(as paths, b allow_older) → (t job)`

Imports `.wpak` files into the library as the implicit local source, in an `import` job; the
files are always copied to `~/.local/share/reiwand/files/`. Each package is opened and its content
hashes are verified; a damaged or modified one is listed in `ImportReport.failed` with the reason.
There is no signature requirement: a package whose author is not known imports and the app shows
«Unverified signature». A package whose id is installed from a repository is refused. Importing the
same version again changes nothing and lists the file in `ImportReport.kept` with `newer = false`,
or, when the file changed, updates the item in place (listed in `imported`); a newer version
updates the item in place (same uid, its decisions and favorite stay); an older version is not a
failure: unless `allow_older` is true the library keeps its newer version and the file is listed
in `kept` with `newer = true` and `version` set to the version the library holds. The search key
`local:yes` matches the items imported this way (files cataloged by path and packages installed
from a repository are excluded), and `source:none` matches every item without a source.
`ImportReport` carries `imported[]` (`path`, `uid`), `kept[]` (`path`, `uid`, `version`, `newer`)
and `failed[]` (`path`, `error`, a readable reason). The daemon logs one `WARN` line per failed
file, `not imported: PATH: REASON`, and one line with the three counts when the job ends.
`reiwand-ctl lib import` and `pkg import` with `--wait` print those counts, one line per kept or
failed file with its reason, and exit with an error when any file failed.

## `Resolve(as refs) → (as uids)`

Turns references into uids, in order. A reference is an exact uid (any case), a unique uid
suffix of at least 6 characters (any case), or the absolute path of a known item. The first
failing reference returns `NotFound` (`no item matches "<ref>"`) or `InvalidArgs`
(`"<ref>" matches N items; use more characters`, or a suffix shorter than 6 characters).

## `Remove(as uids, b delete_files)`

Removes items with their thumbnails and materialized package entries, and with `delete_files`
their files. Every uid is checked first (`NotFound` names the first unknown one); nothing
changes and `InvalidArgs` is returned when an item is a package installed from a repository
(`use Repos1.Uninstall`), belongs to a source while its file still exists and is no depth map
(`remove the file or the folder`: the next scan would add it back), or lies outside
`~/.local/share/reiwand/files/` while `delete_files` is true. `ItemSummary.removable` and
`imported` tell clients in advance.

## `Search(s query, u offset, u limit) → (s json)`

A `SearchPage`. Free text matches the start of words in titles, tags and paths; `key:value`
filters (listed in `GetSchema().query`), `-` negates and quotes group words. `sort:` orders the
results (newest first by default). `uid:` keeps up to 500 items named by their uids, separated by
commas, which lets a client ask again for the items a `library` change names. `limit` is
`0..=500`; `0` returns only `total`.

`first:<uid>,<uid>…` (up to 500) is no filter: the named items come first, in that order, ahead of
the rest in the usual order (the app's «See all» leads with what is in use and the list in use).
`provider:yes` keeps the photos a photo source brought, `provider:no` everything else; a source
counts as a photo source when its folder lies in the service's provider cache.
`ItemSummary.provider` names the photo source of a source photo whose title is still its file name,
so clients can call it «Photo from <source>».

`folder:<absolute path>` keeps the items whose file lies inside that folder or one of its subfolders (quote paths with spaces), whether or not the folder is a source; a playlist whose query is `folder:<path> -kind:package` follows the folder as it changes.

The query is normalized first and `SearchPage.query` echoes it: `in:<name>` becomes `in:<uid>`
(`in:` takes a manual playlist; a smart or unknown playlist returns `InvalidArgs`), and a bare
`sort:random` becomes `sort:random:<seed>` with a new seed, which orders the same way every time.
Clients pass the normalized query unchanged for later pages, so random pages neither repeat nor
skip items. `ItemSummary.thumb` is the cached 256 px thumbnail or `null`: `Search` never makes
thumbnails. `ItemSummary.package` describes package items, whose `kind` tells scenes apart.

## `GetItem(s uid) → (s json)`

An `ItemDetails`: the item with its path, the cached 512 px thumbnail, its presets, the manual
playlists that contain it, and its provider credit, installed package record and revocation
reason when they apply. For a valid package, `about` holds its manifest metadata (`package.md`)
with the texts in the user's language: `description` and `changes` keep their markup, which
clients render without loading anything remote, opening a link only after a click and a
confirmation. Every other text is plain.
`provider` is the credit of the online source it comes from, `platforms` the devices it is made
for, `packages` the packages it needs (`<id> <version requirement>`) and a credit's `source` the
package its files come from (`<id> <version>`), and `spread` is `true` when its author asks to
spread it over every screen (`display.span` is `all-outputs`), so that clients propose `span` when
it is applied. `animated` is the path of its animated preview
(`previews.animated`), copied once into the thumbnail cache, or `null`; clients play it only while
it is on screen, on mains power, and once a mature package was allowed. A package this device cannot play keeps its
details, with the reason in `error` (`this package needs a newer version of Reiwand: it uses
…` or `this package is made for … only`). `variants` lists the installed preset packages whose base
is this package (`{uid, name}`), and for a preset package `item.package.base` is the id of its base
and `base` the installed base (`{uid, name}`) or `null`. `Apply` of a preset package applies its
base with the values of the preset (`package.md`, "Presets").

## `SetTitle(s uid, s title)`

Renames an item.

## `SetRating(as uids, u rating)`

Sets the rating, `0` to `5`, of every item.

## `SetSound(as uids, s muted, s volume)`

Sets the own sound of wallpapers. It belongs to the wallpaper, not to a screen: wherever the wallpaper is
shown (any screen, a playlist, the lock), it plays with it. It is a setting of this computer: it lives in the
library database (columns `sound_muted` and `sound_volume` of `items`), is not part of the package and is not
synced. `muted` is `true` (silent), `false` (plays), `none` (follows the global setting again) or empty (keep it
as it is); `volume` is a whole percent `0` to `100`, `none` or empty. Every uid is checked first. The volume
multiplies the global one, and the global `playback.muted` stays the master over everything: while it is on,
nothing plays whatever a wallpaper says. It reaches videos (also the videos of a package and of a scene), the
sound layers of scenes (they scale with the `volume` property of the scene) and web pages (the volume the daemon
tells the page engine; the page decides what it does with it, and `0` mutes it). `Item` and `ItemSummary` carry
`sound_muted` and `sound_volume` (`null` follows). Outputs showing the wallpaper change at once.

## `SetFavorite(as uids, b favorite)`

Sets or clears the favorite flag of every item.

## `SetTags(as uids, as add, as remove)`

For every item, adds the tags of `add` and then removes those of `remove`, in one transaction.
Tag names compare without case; the first spelling stored is kept. A tag removed from an item that
had it stays removed when the package brings it again (a changed file, a new version or a
relink); adding it again lifts that. Removing the item forgets it.

## `ListTags() → (s json)`

A `TagsReply`: every tag in use with its item count.
