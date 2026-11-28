# Online authors, Operator workflow, Client update, Package installation, Broken repositories and Client storage

On this page: online authors, operator workflow, client update, package installation, broken repositories, client storage.

## Online authors

A root with `policy.authors = "online"` lets authors register without a root ceremony, as a
server with open registration needs. The authors that the root lists keep priority: an online
author with the id of a root author is never used, and `register_author` refuses such ids and keys
that belong to the root or to another author. Because the online key alone vouches for the
authors file, the client trusts each package on first use: `install` records in
`<data>/anchors.json` the fingerprint and public key that signed the first version it installed of
`(repo_id, id)`, and accepts a later version (or another version installed again) only when the
same key signed it or a chain of rotations from the anchored key reaches the new one, each signed
by its `from` key and none from a revoked key; the anchor then moves to the new key. Uninstalling
the last version of `(repo_id, id)` drops its anchor, so a package whose anchored key was revoked can
be trusted again on first use by uninstalling it and installing it again. A package
whose author is in the root needs no anchor, but once a signed version of `(repo_id, id)` is
installed, `install` refuses an unsigned release, an online author for a package installed from an
author of the root, and another author of the root than the one that signed the installed versions (`installed.json`
keeps the author id, so this holds after that author's key leaves the root);
a release from an author of the root drops the anchor. Whoever steals the online key can therefore publish new
authors and new packages, but not update a package already installed; a stolen author key is
revoked through `revoked`.

## Operator workflow

| Step | Function |
|------|----------|
| Create keys (`root`, a backup root key, `online`, authors) | `keygen(password)` |
| Write `root/1.json` | `init(dir, settings, now)` |
| Add root signatures until the threshold is met | `sign_root(dir, signer)` |
| Sign a package source before building it | `sign_source(dir, signer)` |
| Add a package (validated, never `imported`, provider packages under their licenses only, signature checked against root authors, preview copied) | `add(dir, wpak, author, now)` |
| Revoke a package version or a key, or withdraw the revocation of a version | `revoke(dir, id, version, reason, message)`, `revoke_key(dir, fingerprint, reason)`, `unrevoke(dir, id, version)` |
| Hide every version of a package from browsing, or show it again | `hide(dir, id, hidden)` |
| Unlist a package or one version, or list it again | `list(dir, id, version, listed)` |
| Feature a package with an optional note, or stop featuring it | `feature(dir, id, note, on)` |
| With online authors: register an author key, hand an author over to a new key, drop a key that no rotation names | `register_author(dir, id, key)`, `rotate_author(dir, id, from, key, sig)` with `sign_rotation(signer, id, key)`, `drop_author_key(dir, id, fingerprint)` |
| Write shards, revocations and the next entry; repeat before expiry | `publish(dir, online, lifetime_days, now)` |
| Replace keys or policy: write `root/<N+1>.json`, then sign with the old and the new root threshold, then publish with the current online key | `rotate_root(dir, settings, now)`, `sign_root`, `publish` |
| Check a tree as a client would, without storing anything | `verify(fetch, url_or_dir, now)` |

`status(dir)` reports the signers of the newest root, whether it is complete and the share
URL query. `settings(dir)` returns the current settings for editing before a rotation, with the
lifetime the current root was written with: `init` and `rotate_root` keep `root_days` in
`catalog.json`, and a catalog without it means 365. A `Charter` changes those settings: the keys it
names for an author replace the keys that author had, `removed` drops authors before that, and
`mirrors` replaces the mirrors when present, so an empty list drops them all
(`repo rotate-root --no-mirrors`) and a missing one keeps them.

## Client update

Each step that fails for a base (network, size, hash, signature or format) is retried on the
next base: the user URL first, then the root mirrors. If every base fails nothing is stored. A
cancelled transfer (`io::ErrorKind::Interrupted`) stops at once without trying further bases.

1. `T = max(now, max_timestamp)`.
2. First contact: fetch `root/1.json`, check the anchor, `genesis`, `version` and its own root
   threshold.
3. Walk `root/N+1.json` while it exists (at most 64 per run). Each must meet the threshold of
   the previous root and of itself. If the `online` keys changed, reset the entry state.
4. Abort if the current root expired (`Expired`, possible freeze attack). Installed packages
   keep working.
5. Read `entry/latest` (at most 32 bytes), start at `max(latest, last seen)` and probe up to 16
   newer entries.
6. The entry must meet the `online` threshold, pin the current root by version and hash, have
   a valid lifetime, a timestamp not older than the last one and not more than 24 hours after
   `T`. The same version with different bytes is an equivocation.
7. If `expires ≤ T` the repository is stale: it is shown as such, installs need confirmation
   and revocations still apply. `repos(data, now)` recomputes staleness and expiry warnings at
   every call with the same `T`, so listings and installs agree.
8. Fetch every shard, the revocation list and the files the entry cites not already
   cached, cut at the declared size, check size and SHA-256, then parse them.
9. Under the exclusive lock, read the stored state again: when another process stored a newer
   root or entry meanwhile, keep it and report it instead (nothing is ever rolled back); the same
   entry version with other bytes is an equivocation. Otherwise store root, entry, shards
   and revocations, then `state.json`, and drop unreferenced shards.
10. Apply revocations to installed packages and list available updates (newer SemVer, not
    revoked).

## Package installation

1. Resolve `id[@version]` (optionally `<repo_id>/id[@version]`) against verified shards only.
2. Refuse ids installed from another repository, expired roots, stale
   repositories without confirmation and revoked versions. Without an explicit version nothing
   is downgraded; an explicit version older than the newest installed one needs confirmation.
3. Refuse a release whose declared size exceeds `policy.max_package_size` or what a package may
   hold (2 GiB of files plus 64 MiB), before anything is downloaded. Download
   `packages/<id>/<version>.wpak` from the bases in order into a temporary file in
   `<data>/packages/<id>/`, cut at the declared size, and check size and SHA-256.
4. Validate the package, check `id`, `version`, `kind`, `license` and `manifest_sha256` against
   the release, and check the author signature when the root requires it or the listing names
   an author (key bound to that author in the root, or in the online authors file when the root
   allows them, and not revoked).
5. Under the exclusive lock, re-read `installed.json` and the repository, check that it is still
   there and that neither the version nor the signing key is revoked, check the anchor of an online
   author (above), then rename into place and record it. `current(data, repo_id, id, version)` repeats that check for an installed record, so
   a caller can check again right before it activates the package.

`preview(data, fetch, repo_id, id, version, dest)` downloads `previews/<sha256>.<ext>` of a
release from the bases in order, cut at the signed size, checks SHA-256 and writes `dest`
(`0600`) atomically. It returns `false` when the release has no preview. `release(data, repo_id,
id, version)` returns any listed release, so callers can cache the preview of every version by its
hash, and `previews(data, repo_id)` lists the preview hashes of every release of a repository, to
delete them with it.

## Broken repositories

A folder under `<data>/repos/` whose snapshot cannot be read (no `state.json`, or a damaged or
missing file) is reported by `repos` with `broken` set to the reason; `browse` and `install` skip
it, so the other repositories keep working. `update_repo` fetches it again, anchored on its
`repo_id`, from the address kept in its `state.json` or else from the network mirrors of its
stored root, keeping the stored entry version and timestamps when they can be read, so it cannot
be rolled back either. When no address is left, `add_repo` with its URL repairs it and
`remove_repo` deletes it.

## Client storage

```
<data>/repos/<repo_id>/state.json             versions, hashes, max_timestamp, online keys, stale
<data>/repos/<repo_id>/root.json[.minisig]    current verified root
<data>/repos/<repo_id>/entry.json[.minisig]   current verified entry
<data>/repos/<repo_id>/blobs/<sha256>.json    verified shards and revocations
<data>/packages/<id>/<version>.wpak           installed packages
<data>/installed.json                         origin, hash, signer and revocation of each package
<data>/anchors.json                           first signing key of each package of an online author
<data>/.repos.lock                            flock serializing writers
<data>/format-version                         number of the stored format (6)
```

The client knows only the current format. A data folder without `format-version` that already holds
repositories, or with another number, is refused with a message that says it was made by another version
and that nothing was changed; an empty folder is marked as current. Nothing converts data of another
version; stored repository files of an older format are refused naming it.

Files are written to a temporary file in the same directory, synced and renamed. Files are
`0600` and directories `0700`. Nothing is stored before it is verified. Committing a snapshot,
removing a repository and saving `installed.json` hold an exclusive `flock` on
`<data>/.repos.lock`; reading snapshots or `installed.json` holds a shared one. `browse` and
`install` read every snapshot and `installed.json` under one shared lock, so they never mix a
catalogue with an older or newer list of installed packages. Each `browse` item carries the
display name of its repository. Downloads finish
before the lock is taken, so readers never wait for the network.
