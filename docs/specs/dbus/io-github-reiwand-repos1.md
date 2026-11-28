# `io.github.reiwand.Repos1`

Signed repositories of `.wpak` packages (see `repo.md`), kept in `~/.local/share/reiwand`.
`repo` arguments take a full repository id or a unique prefix of at least 8 hex digits. Trust is
anchored by the first `root/1.json` read at the address it is added with (its hash is the `repo_id`; an address has no parameters); repository
jobs (`repo_add`, `repo_refresh` and the download of `install`) run one at a time. Requests to
a repository follow redirects only to its declared bases (the URL it was added with and the
mirrors of its signed root), never from HTTPS to HTTP. Installed packages are imported into the
library; revoked ones are never applied again (`Apply` returns `Failed`), rotation skips them,
and an output showing one when its revocation arrives shows its background color instead.

### `List() → (s json)`

A `ReposReply`. `stale` and `warnings` are computed at every call; `last_error` is the message
of the last failed refresh since the daemon started and `last_error_name` its error name (`Offline`,
`Untrusted` or another of the table above), so a client tells the kinds apart without reading the
message; both are dropped when the repository is removed or added again. A repository whose stored copy cannot be read (its `state.json` missing,
a damaged file) is listed with the reason in `broken` and its other fields as far as they can be
read; browsing and installing skip it, and the other repositories keep working.

### `Add(s url) → (t job)`

Adds a repository by its address. The URL is `https://` with a certificate valid for a public
authority, `file://` or `http://` to this computer, without user information or fragment; any
other `http://` is refused with `InvalidArgs`; so is an address with a query (`?fingerprint=`, `?repo_id=`), with a message that says to use the plain address.
The job fails with `CertificateUnknown` when the server's certificate is not from a known
authority (the message carries its SHA-256 fingerprint; nothing is pinned), with
`CertificateChanged` when the origin is already pinned (or was just confirmed) and the server
presents another one, and with `NotARepository` when the address is a server (it answers
`/.well-known/reiwand-server` and has no `root/1.json`; the message names its repositories and the job's `output` is `{"repositories": [{"name", "url"}]}`, at most five, so a client lists them without reading the text). `Exists` when the job finds it already added; a `broken` one is
replaced. The job verifies the repository fully and stores its signed state.

### `AddTrusting(s url, s certificate) → (t job)`

Like `Add`, for a home server with its own certificate: `certificate` is the lowercase SHA-256
fingerprint that the failed `Add` reported as `CertificateUnknown`, which the user confirmed.
The connection to that origin is accepted only with that certificate, and after the job succeeds
the fingerprint is kept in `repo-pins.json`, per origin (`scheme://host:port`), for every later
request to repositories at that origin. A different certificate later makes `Add` and `Refresh`
fail with `CertificateChanged` and list the warning `CertificateChanged`; nothing is applied.
A pin does not affect the sync server.

### `AddExamples() → (t job)`

Adds again the repository of the example wallpapers that come with the program (`repo.md`,
«Bundled example repository»), also after the user removed it. `NotFound` when the
installation carries none; `Exists` when it is already added. `List` marks that repository with
`bundled: true`, which is how a screen gives it its own name and icon. On its first start the
daemon adds it by itself; once removed with `Remove` it is not added again until this call, and the
daemon updates it with the program, at startup and once a day.

### `Refresh(s repo) → (t job)`

Refreshes a repository (`""` = all) and applies revocations to the installed packages, also
when the repository is stale. A failed refresh keeps the last verified state. A `broken`
repository is fetched again from its stored address, or from the network mirrors of its stored
root, anchored on its `repo_id`; when neither is left the refresh fails and the repository has to
be added again with its URL or removed. With
`repos.auto_refresh`, every repository is refreshed 10 minutes after the daemon starts and then
every 24 hours. Updates are installed only with `repos.auto_update` (off by default), once the
refresh ends and also when the option is turned on; the desktop has no metered or battery rule for
refreshes, so none applies to them. A version that asks for new permissions (see `ListInstalled`) is
downloaded, checked, discarded and remembered until a newer version appears; its job ends `failed`
with the permissions in the message. `Install` of that package by hand installs it. With the option
off, updates show as a list with the `Update` action of each package.

### `Remove(s repo, b uninstall)`

Removes a repository with the cached previews of all its versions (those another repository
also offers stay); `uninstall` also uninstalls its packages
(`Failed` if one of them is shown on an output). Without it the packages stay usable, without
updates or revocations. Queued or running jobs of this repository (its refreshes and installs
from it) are cancelled first and the call waits only for them; jobs of other repositories go on.
An install from it that starts while it is being removed ends `cancelled`.

### `Browse(s query, u offset, u limit) → (s json)`

A `BrowsePage` of the packages offered, matched without case on id, title, description (in every
language), tags, authors and facets, sorted by id then repository. `limit` is `0..=500`. Titles,
descriptions and change notes come in the user's language; with `library.hide_mature`, packages
rated `mature` are left out and `hidden` counts them (Explore says «N hidden by your content
filter» and `reiwand-ctl pkg search` prints `hidden = N`). Packages that the moderators of the repository hid are left out unless
a version of them is installed; they can still be installed by id. `revealed` tells whether the
user chose to see the previews of a mature package without blur (`Reveal`).

### `Reveal(s spec)`

Remembers, for `[repo/]id[@version]`, that the user wants to see the previews of that package
without blur, in every version, until the repository is removed; clients show them unblurred when
`revealed` is set or `library.show_mature` is on. `NotFound` when no repository offers it,
`Invalid` when several do without a repository prefix. Emits `repos`.

### `RequestPreviews(as specs) → (a{ss} paths)`

Returns `{spec: path}` for the cached previews of `[repo/]id[@version]` specs (at most 500,
keys as given) and queues the others; `PreviewReady` follows for each of them. A new call from
the same sender replaces its queued requests. Previews are downloaded within their signed size
and hash, never when that size is over 16 MiB, re-encoded by the thumbnail worker to JPEG files of at most 1024 px in
`~/.cache/reiwand/previews/`, and only the re-encoded file is handed out. Previews of older
versions are cached the same way, named by their signed hash, and downloaded only once.

### `RequestAnimatedPreviews(as specs) → (a{ss} paths)`

Like `RequestPreviews` for the animated previews (`previews.animated`, `animated` in `Browse`): the
gif or webp file is downloaded within its signed size and hash, never when that size is over the
8 MiB a package may carry, checked to be a picture of at most
640 px a side and 15 s, and handed out as it is from `~/.cache/reiwand/previews/`, counted in
`library.thumb_cache_mb` and removed with its repository. `PreviewReady` follows for each queued spec
with the spec prefixed by `animated:`; a new call from the same sender replaces its queued animated
requests only. Clients ask for one only on mains power, when the pointer or the focus is on its
package, and play it only then.

### `Install(s spec, b allow_stale, b allow_downgrade) → (t job)`

Installs `[repo/]id[@version]`, plus the packages it lists in `requires.packages` that are missing; when one of those cannot be installed the job still finishes, its output lists the ids in `missing` and the `JobFinished` message says the package is incomplete. Without a version, the newest release when it is newer than the
installed one (this is how updates are installed), else the job finishes at once with the
installed record. Checked before the job starts: `InvalidArgs` for a malformed spec, a package
offered by several repositories without `repo`, a package installed from another repository or an
explicit version older than the installed one without `allow_downgrade`;
`NotFound` for an unknown package or version; `Failed` for an expired root (`root expired:
possible freeze attack`) or a revoked version; `Stale` when the repository's index expired and
`allow_stale` is false. Before downloading, the job refreshes the repository that offers the
package, as `Refresh` does: what the server offers now is installed, and a refresh that fails (for
example `the signature of the repository at … changed; nothing was applied`) fails the job, with the
repository's warning set. When the download ends, and again before the package reaches the library,
the job checks that the repository is still there and that neither the version nor its signing
key was revoked meanwhile; otherwise it fails and nothing is kept. The first installed version is
imported into the library; later
versions relink the same item, keeping its title, tags, rating, presets and playlist entries.

### `Uninstall(s id, s version)`

Uninstalls one version, or every version for `version = ""`. `Failed` when one of them is shown
on an output. `Uninstall`, `Library1.Apply`, `Profiles1.Activate` and the last step of an `install`
job never run at the same time, so an item is never shown after its package is removed, and an
install whose package was removed meanwhile fails instead of leaving an item without its file. When the version linked to the library item goes, the item moves to the newest
remaining version, or is removed with its thumbnails when none remains.

### `ListInstalled() → (s json)`

An `InstalledReply`; `item` is set on the version linked to the library item and `update` on
the newest installed version when a newer one is available, with `changes` holding the change
notes of that version in the user's language and with its markup. `asks` lists the permissions that
version asks for and the installed one does not, when automatic updates held it back; clients show
them next to the update and leave the package out of their "update all" action.
`RefreshOutput.updates` carries the same notes.

### Signal `PreviewReady(s spec, s path)`

A preview queued by `RequestPreviews` is ready, or could not be made (`path = ""`: unknown
package, no preview, or a failed download or check).

**Data of another version.**

The daemon knows only the current format of `~/.local/share/reiwand` (`format-version`, 6). When the
folder holds data of another version (repositories or an account file without that mark, or another
mark) it refuses to start, changes nothing and logs "the stored repositories in DIR were made by another
version, which this version does not read; nothing was changed". The daemon, `reiwand-ctl` and the app
contain no reader of an older format and nothing converts one.
