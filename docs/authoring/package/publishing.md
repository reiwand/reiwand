# Publishing and copies

On this page: repositories, editing a copy of an installed package and videos.

## Publish in a repository

A **repository** is a folder of static files that any web server, shared resource or `file://` path
can serve. Everything a client uses is signed, so a mirror or a server does not need to be
trusted.

### The repository keys

| Role | What it signs | How it is kept |
|------|---------------|----------------|
| **root** (`root`) | The root: the list of keys and the rules of the repository. | Offline. Used a few times a year. There can be several and require *t* of *n* to sign. |
| **online** (`online`) | The index, which expires (7 days by default). | On the computer that publishes, without a password if a program does it. Used on every publication. |
| **author** | The `manifest.json` of each package. | The author's: the same `author.key` as above. |

### Create the repository

```
reiwand-ctl repo keygen root
reiwand-ctl repo keygen online --no-password
reiwand-ctl repo init my-repo --name "My repository" --root-key root.pub --online-key online.pub --author me=author.pub --require-author-signature true
reiwand-ctl repo sign-root --key root.key my-repo
```

`repo init` writes `root/1.json` in an empty folder and says that it still has to be signed;
`sign-root` signs it with the root key and `repo status my-repo` shows the result:

```
repo_id = <64 hexadecimal digits>
version = 1
signers = <fingerprint of the root key>
threshold = 1
complete = true
address = file:///path/to/my-repo/
```

`address` is the full address of the folder, as it is pasted into *Add a repository* or into
`remote add`; the static repository box of the app shows it with a *Copy the address* button.

`--author ID=author.pub` registers an author with their public key (repeat it for several) and
`--require-author-signature true` rejects packages that no author of the root signed. There are
more settings: `--threshold`, `--root-days` (the root lasts 365 days by default),
`--entry-max-days` (14), `--max-package-size`, `--mirror URL`, `--online-authors` (lets authors
register on a server).

### Add packages and publish

```
reiwand-ctl repo add my-repo clock.wpak --author me
reiwand-ctl repo publish --key online.key my-repo
```

`repo add` validates the package, checks its signature against the author and copies it to
`packages/<id>/` with its preview (`--unlisted` leaves it out of browsing and search, though it is
installed by id). It never admits `imported` packages. `repo publish` signs and writes the index:

```
org.example.clock 1.0.0 sha256=<sha256 fingerprint> author=me
index 1 published with 1 packages; it expires <UTC date and time>
```

The index expires. Before the date, sign it again without changing anything:

```
reiwand-ctl repo refresh --key online.key my-repo
```

A repository with an expired index is shown as "out of date" and installations ask for
confirmation. `--days` changes how long it lasts (up to `entry_max_days`).

### Serving it

Copy the whole folder to any static file server. It serves `entry/latest` with `Cache-Control:
no-cache` and everything else as immutable. For others to add it, give them the plain address that
`repo status` prints:

```
https://example.org/wallpapers/
```

Whoever receives it adds it with `reiwand-ctl remote add 'ADDRESS'` or from *Repositories* in the
app. To try it on your computer, a `file://` address works:

```
reiwand-ctl remote add 'file:///path/to/my-repo/' --wait
reiwand-ctl pkg search clock
reiwand-ctl pkg install org.example.clock
```

(With the daemon running, and with `--wait` on `pkg install` so that it says `Installed
org.example.clock 1.0.0.`. `remote add` also accepts the folder, `my-repo`; an address with
`?fingerprint=` or `?repo_id=` from version 1.x is rejected.)

### Update a published package

A published version **cannot be replaced**: adding other content with the same version is rejected
with "is already published with other content". To update, raise the version:

1. Change `version` in the manifest (`1.0.0` to `1.1.0`) and, if you want, `changes`.
2. `reiwand-ctl pack sign --key author.key my-scene`
3. `reiwand-ctl pack build my-scene -o clock-1.1.wpak`
4. `reiwand-ctl repo add my-repo clock-1.1.wpak --author me`
5. `reiwand-ctl repo publish --key online.key my-repo`

Whoever already installed it sees the update when refreshing the repository (`reiwand-ctl remote
refresh`) and installs it with `reiwand-ctl pkg upgrade`. It never downgrades without confirmation.

To withdraw something: `repo unlist my-repo ID [--version V]` takes it out of browsing (it is
installed by id) and `repo list-again` brings it back; `repo revoke my-repo ID --reason broken
[--version V] [--message TEXT]` revokes it (reasons: `malware`, `license`, `takedown`, `broken`,
`other`) and clients turn it off. After each one, `repo publish`. `repo feature my-repo ID --note
TEXT` highlights it.

To change the keys or the rules of the repository: `repo rotate-root` writes the next root, it is
signed with the keys of the previous root and the new one, and published. The app has the same in
*Repositories*, *Static repository* tab.

### A server of your own

If you have a Reiwand server of your own, you do not need a folder: the already signed package is
uploaded with `reiwand-ctl server upload clock.wpak --wait`, and the server checks it before
publishing it (`reiwand-ctl server author-key add` registers your public author key). It only
accepts the licenses of its list and rejects `mature` and `imported` packages.
If the network drops while it uploads, the upload stops (cancelling the job stops it at once) and
nothing is published: upload it again when the network is back. An upload cut that way does not
count against the two open uploads the server allows, once the server can be reached again.

## Edit a copy of an installed package

To start from a package you already have (yours or another author's), it is unpacked as a copy with
a new id: the installed one never changes.

```
reiwand-ctl scene open my-copy --from clock.wpak --facts
reiwand-ctl scene open my-copy --from clock.wpak
```

The first prints, without writing anything, the license with a line that explains what it allows,
the verdict (`allowed` or `refused`), the conditions (in the program's language) and the id of the
copy:

```
{"authors":[],"conditions":[],"credits":0,"id":"org.example.clock","kind":"scene","license":"CC0-1.0","new_id":"org.example.clock.copy","reason":"","summary":"Public domain: anyone may use, change and share it with no conditions","title":"Script clock","verdict":"allowed","version":"1.0.0"}
```

The second unpacks and describes the folder. The copy:

- has the id `<id>.copy` (`--id` sets another; `.copy2` if there is already a copy of a copy);
- loses the signature of the original author (`manifest.json.minisig`);
- keeps the license, the credits and the files;
- becomes `derived`: its `origin` cites the original (authors, title, license and page).

You have to write who you are in `authors`. If the license does not allow derived works, it fails
with "… does not allow derived works: edit a copy only if you made it or its author allows it";
with `--mine` you say that the package is yours or that you have permission. In the app: *Edit a
copy…* on the page of each package of the library and in *Scenes*.

## Videos

A video wallpaper is a `kind: video` package whose `entry` is a `.webm`, `.mp4` or `.mkv`. Its
settings (speed, loop start and end, volume, zoom and position) are set by the author in
`playback` and `display` and the user changes them per screen; there is an example in the package
specification (`docs/specs/package.md`, "Video settings").

The desktop plays whatever its decoders open, and when adding a video it checks that it can open
it. `reiwand-ctl` does not convert videos: prepare them beforehand with the tool you prefer.

The phone does convert. When adding a video, the app checks that the phone has a decoder for its
track. If it can only decode it with the processor, but does decode H.264 in hardware at that size
or at 1080p or 720p, it converts it to an H.264 copy with Android's encoder, and shows the
progress. In *Settings*, *Convert videos this phone cannot play smoothly* chooses between *Convert
them* (the copy takes the place of the original), *Convert and keep the original* and *Never*.
10-bit HDR videos always come out as 8-bit H.264 with the standard range. So, for a video to work
well everywhere without conversion, the safest is H.264 in MP4, at 1080p at most.

Videos inside a scene (`video` layers) are files of the package like the others; they are added
with `reiwand-ctl scene import my-scene background.mp4`.
